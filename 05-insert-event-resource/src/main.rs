//! Based off of <https://github.com/TheBevyFlock/bevy_cli/blob/lint-v0.1.0/bevy_lint/src/lints/insert_event_resource.rs>.
//! Please consider reading that instead, it has better documentation and more features! :)

#![feature(rustc_private)]

use clippy_utils::{diagnostics::span_lint, sym, ty::match_type};
use rustc_driver::Callbacks;
use rustc_hir::{Expr, ExprKind, GenericArg, GenericArgs};
use rustc_interface::interface::Config;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::ErrorGuaranteed;

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

struct LinterCallbacks;

impl Callbacks for LinterCallbacks {
    fn config(&mut self, config: &mut Config) {
        config.register_lints = Some(Box::new(|_, lint_store| {
            lint_store.register_lints(&[INSERT_EVENT_RESOURCE]);
            lint_store.register_late_pass(|_| Box::new(InsertEventResource));
        }));
    }
}

declare_lint! {
    pub INSERT_EVENT_RESOURCE,
    Warn,
    "called `App::init_resource::<Events<T>>()` instead of `App::add_event::<T>()`"
}

declare_lint_pass! {
    InsertEventResource => [INSERT_EVENT_RESOURCE]
}

impl<'tcx> LateLintPass<'tcx> for InsertEventResource {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // There many different kinds of expressions. (See `ExprKind`!) In this case, we're looking
        // for method calls in the form of `receiver.path()`.
        if let ExprKind::MethodCall(path, receiver, _, method_span) = expr.kind {
            // Find the receiver type. This may be `App`, which is what we want, or something else.
            // We call `peel_refs()` to unwrap any references, getting the underlying type. (For
            // example, `&&App` becomes `App`.)
            let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();

            // Check if the receiver type is a Bevy `App`. If it's not, exit early.
            if !match_type(cx, receiver_ty, &["bevy_app", "app", "App"]) {
                return;
            }

            // Check if the method name is `init_resource()`, else exit early.
            if path.ident.name != sym!(init_resource) {
                return;
            }

            // Pull out the generic argument `T` from `app.init_resource::<T>()`.
            if let Some(&GenericArgs {
                args: &[GenericArg::Type(resource_hir_ty)],
                ..
            }) = path.args
            {
                // There are two relevant representations of types: `rustc_hir::Ty` and
                // `rustc_middle::ty::Ty`. The HIR representation is more of a name than anything
                // else, while the middle's representation has actual type information. For more
                // info, check out <https://rustc-dev-guide.rust-lang.org/ty.html#rustc_hirty-vs-tyty>.
                //
                // In this case, we convert an HIR `Ty` to a middle `Ty` so we can figure out if it
                // is `Events<T>` or not.
                let resource_ty = cx.typeck_results().node_type(resource_hir_ty.hir_id);

                if match_type(cx, resource_ty, &["bevy_ecs", "event", "Events"]) {
                    // Emit the lint! The compiler figures out whether this should be displayed to
                    // the user, depending on whether it is `#[allow(...)]`'d or not.
                    span_lint(
                        cx,
                        INSERT_EVENT_RESOURCE,
                        // The span is a range of bytes in the source code. By passing the span of
                        // the method call, we can attach our lint warning to that specific line of
                        // code in the diagnostics.
                        method_span,
                        "called `App::init_resource::<Events<T>>()` instead of `App::add_event::<T>()`",
                    );
                }
            }
        }
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    // Unlike previous examples, this is no longer meant to be used as `rustc` directly. Instead, it
    // is designed to be a driver that can be called by Cargo. Cargo calls drivers in the format or
    // `path/to/driver path/to/original/rustc --addition --args`. We skip the first argument, so
    // `RunCompiler` just sees the path to `rustc` and not our driver.
    //
    // This also means that if you wish to run this executable manually, you need to call it as
    // `cargo run -- rustc --actual --arguments`.
    let args: Vec<String> = std::env::args().skip(1).collect();

    rustc_driver::RunCompiler::new(&args, &mut LinterCallbacks).run()
}
