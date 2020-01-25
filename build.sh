#!/bin/bash

set -xe

cargo clean --doc

cargo fmt --all -- --check
cargo test --release
cargo test --release --all-features
cargo bench --no-run

cargo doc --all-features
linkchecker target/doc/linuxcnc_hal_sys/index.html
linkchecker target/doc/linuxcnc_hal/index.html

# Check that packaged crate builds (doesn't push to crates.io)
pushd linuxcnc-hal-sys
cargo package
popd
