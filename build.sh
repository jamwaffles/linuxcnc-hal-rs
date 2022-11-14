#!/bin/bash

set -x

cargo clean --doc

cargo fmt --all -- --check
cargo build --examples
cargo bench --no-run
cargo test --release

cargo doc --all-features
linkchecker target/doc/linuxcnc_hal_sys/index.html
linkchecker target/doc/linuxcnc_hal/index.html
linkchecker target/doc/rtapi_logger/index.html

# Check that sys package builds into something pushable to crates.io
pushd linuxcnc-hal-sys
cargo package
popd

# Check that higher level package builds correctly
pushd linuxcnc-hal
cargo package
popd

# Check that the logger builds correctly
pushd rtapi-logger
cargo package
popd
