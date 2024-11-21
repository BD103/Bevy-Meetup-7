#![feature(rustc_private)]

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;
use rustc_span::ErrorGuaranteed;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_span;

struct LinterCallbacks;

impl Callbacks for LinterCallbacks {
    fn config(&mut self, config: &mut Config) {
        config.register_lints = Some(Box::new(|session, lint_store| {
            lint_store.register_lints(&[todo!()]);
            lint_store.register_late_pass(|tcx| Box::new(todo!()));
        }));
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::RunCompiler::new(&args, &mut LinterCallbacks).run()
}
