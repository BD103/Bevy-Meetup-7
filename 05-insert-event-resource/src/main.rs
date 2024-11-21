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
        if let ExprKind::MethodCall(path, src, _, method_span) = expr.kind {
            let src_ty = cx.typeck_results().expr_ty(src).peel_refs();

            if !match_type(cx, src_ty, &["bevy_app", "app", "App"]) {
                return;
            }

            if path.ident.name != sym!(init_resource) {
                return;
            }

            if let Some(&GenericArgs {
                args: &[GenericArg::Type(resource_hir_ty)],
                ..
            }) = path.args
            {
                let resource_ty = cx.typeck_results().node_type(resource_hir_ty.hir_id);

                if match_type(cx, resource_ty, &["bevy_ecs", "event", "Events"]) {
                    span_lint(
                        cx,
                        INSERT_EVENT_RESOURCE,
                        method_span,
                        "called `App::init_resource::<Events<T>>()` instead of `App::add_event::<T>()`",
                    );
                }
            }
        }
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::RunCompiler::new(&args, &mut LinterCallbacks).run()
}
