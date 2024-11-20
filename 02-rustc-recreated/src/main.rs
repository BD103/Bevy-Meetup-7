//! This is a bare-bones example of making a program that acts like `rustc`. Since all the `rustc`
//! binary does is call `rustc_driver::main()`, we do the same.

// This lets use link to `librustc_driver.so`, which gives us access to all of `rustc`'s internal
// crates. Don't forget to install the `rustc-dev` Rustup component! (It should be automatically
// installed with `rust-toolchain.toml`.)
#![feature(rustc_private)]

// Each `rustc` crate that we use needs this line so that the compiler knows to pull it in.
extern crate rustc_driver;

fn main() -> ! {
    rustc_driver::main()
}
