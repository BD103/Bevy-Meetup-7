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

struct MyCallbacks;

impl Callbacks for MyCallbacks {
    fn config(&mut self, _: &mut Config) {
        println!("Configuring!");
    }

    fn after_analysis<'tcx>(&mut self, _: &Compiler, _: &'tcx Queries<'tcx>) -> Compilation {
        println!("Analysis complete!");
        Compilation::Continue
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    let args: Vec<String> = std::env::args().collect();

    rustc_driver::RunCompiler::new(&args, &mut MyCallbacks).run()
}
