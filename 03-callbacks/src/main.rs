#![feature(rustc_private)]

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{
    interface::{Compiler, Config},
    Queries,
};
use rustc_span::ErrorGuaranteed;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_span;

/// This is a simple compiler callback that prints when certain stages of the compilation are
/// reached.
struct MyCallbacks;

impl Callbacks for MyCallbacks {
    fn config(&mut self, _: &mut Config) {
        println!("Configuring!");
    }

    fn after_analysis<'tcx>(&mut self, _: &Compiler, _: &Queries<'tcx>) -> Compilation {
        println!("Analysis complete!");

        // This tells the compiler whether to continue or exit early.
        Compilation::Continue
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    let args: Vec<String> = std::env::args().collect();

    // In order to inject our own code into the compiler process, we must pass a custom callback to
    // the `RunCompiler` structure.
    rustc_driver::RunCompiler::new(&args, &mut MyCallbacks).run()
}
