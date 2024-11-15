//! This driver prints out a list of all types defined in a crate, along with their locations.
//! 
//! Try running this on itself:
//! 
//! ```bash
//! cd simple-driver
//! cargo run -- rustc src/main.rs
//! ```

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface::Compiler, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_span::ErrorGuaranteed;

/// A driver callback that prints all types (structs, enums, and unions) defined in a crate.
struct TypeFinderCallback;

impl Callbacks for TypeFinderCallback {
    // The following method is run after the compiler finishes analyzing and verifying the crate.
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        // Retrieve the global type context `TyCtxt` and execute the following closure with it.
        queries.global_ctxt().unwrap().enter(|tcx: TyCtxt<'tcx>| {
            let hir = tcx.hir();

            // Iterate over all items in a crate.
            for id in hir.items() {
                // Retrieve a reference to the item definition from its ID.
                let item = hir.item(id);

                // If the item is an algebraic data type (struct, enum, or union)...
                if item.is_adt() {
                    // Print the type name and span.
                    println!("{} - {:?}", item.ident, item.span);
                }
            }
        });

        // No need to continue compiling, our job here is done.
        Compilation::Stop
    }
}

fn main() -> Result<(), ErrorGuaranteed> {
    // Usually in the format of ["path/to/driver", "path/to/rustc", "additional", "arguments"]. We
    // skip the driver path, but keep the `rustc` path because that is what `RunCompiler` expects.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Run the compiler with our custom callback.
    rustc_driver::RunCompiler::new(&args, &mut TypeFinderCallback).run()
}
