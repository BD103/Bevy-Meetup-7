#![feature(rustc_private)]

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;
use rustc_lint::LateLintPass;
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::ErrorGuaranteed;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

// Lints are created with the `declare_lint!` macro. This generates a `static MY_LINT: &Lint`.
declare_lint! {
    // Name and visibility of the static.
    pub MY_LINT,
    // The default warning level. Other options are `Deny` and `Allow`.
    Warn,
    // A short description of the lint. This is printed when you run `rustc -W help`, or in this
    // case `cargo run -- -W help`.
    "an example lint"
}

// Lint passes are also created with a macro. This generates a zero-sized structure named `MyLint`
// that implements `LintPass`, `Clone`, and `Copy`.
declare_lint_pass! {
    MyLint => [MY_LINT]
}

// Lint passes implement either `EarlyLintPass` or `LateLintPass`. In this case we implement
// `LateLintPass`, but do not override any of its methods, meaning it will do nothing.
impl LateLintPass<'_> for MyLint {}

/// This is a compiler callback that registers a lint and lint pass.
///
/// See [`LinterCallbacks::config()`].
struct LinterCallbacks;

impl Callbacks for LinterCallbacks {
    fn config(&mut self, config: &mut Config) {
        // `Config::register_lints` is a callback function that is called to register custom lints
        // into a `LintStore`.
        config.register_lints = Some(Box::new(|_session, lint_store| {
            // Register a list of lint definitions.
            lint_store.register_lints(&[MY_LINT]);

            // Register the `MyLint` lint pass. Note that we have access to the `TyCtxt` when
            // constructing the lint pass, meaning we can initialize it with custom data.
            lint_store.register_late_pass(|_tcx| Box::new(MyLint));
        }));
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::RunCompiler::new(&args, &mut LinterCallbacks).run()
}
