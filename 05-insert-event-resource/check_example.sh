#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# Build out linter driver.
cargo build

# Set our linter driver as the `rustc` wrapper. This means that, for all cases that Cargo normally
# calls `rustc`, it will instead call our driver with the first argument being the path to `rustc`.
# See the `main()` function in `src/main.rs` for more information.
export RUSTC_WORKSPACE_WRAPPER="../target/debug/insert-event-resource"

# Check `examples/insert_event_resource.rs` with our linter driver, because we set
# `RUSTC_WORKSPACE_WRAPPER`.
cargo check --example insert_event_resource
