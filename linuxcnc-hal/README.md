# LinuxCNC HAL interface for Rust

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/linuxcnc-hal.svg)](https://crates.io/crates/linuxcnc-hal)
[![Docs.rs](https://docs.rs/linuxcnc-hal/badge.svg)](https://docs.rs/linuxcnc-hal)
[![Liberapay](https://img.shields.io/badge/donate-liberapay-yellow.svg)](https://liberapay.com/jamwaffles)

A safe, high-level interface to LinuxCNC's HAL (Hardware Abstraction Layer) module.

For lower level, unsafe use, see the [`linuxcnc-hal-sys`](https://crates.io/crates/linuxcnc-hal-sys) crate.

```bash
cargo add linuxcnc-hal
```

Please consider [becoming a sponsor](https://github.com/sponsors/jamwaffles/) so I may continue to maintain this crate in my spare time!

# [Documentation](https://docs.rs/linuxcnc-hal)

# Example

More examples can be found in the `examples/` folder.

```rust,no_run
use linuxcnc_hal::{hal_pin::HalPinF64, HalComponentBuilder};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component called `empty`
    let mut builder = HalComponentBuilder::new("pins")?;

    let input_1 = builder.register_input_pin::<HalPinF64>("input-1")?;

    let output_1 = builder.register_output_pin::<HalPinF64>("output-1")?;

    // All pins added, component is now ready. This consumes the builder and registers signal
    // handlers.
    let comp = builder.ready()?;

    let start = Instant::now();

    // Main control loop
    while !comp.should_exit() {
        let time = start.elapsed().as_secs() as i32;

        // Set output pin to elapsed seconds since component started
        output_1.set_value(time.into())?;

        // Print the current value of the input pin
        println!("Input: {:?}", input_1.value());

        // Sleep for 1000ms. This should be a lower time if the component needs to update more
        // frequently.
        thread::sleep(Duration::from_millis(1000));
    }

    // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
    // at this point. Registered signal handlers are also deregistered.

    Ok(())
}
```

# Development

## Setup

[`bindgen`](https://github.com/rust-lang/rust-bindgen) must be set up correctly for `linuxcnc-hal-sys` to work correctly. Follow the [requirements section of its docs](https://rust-lang.github.io/rust-bindgen/requirements.html).

To run and debug any HAL components, the LinuxCNC simulator can be set up. There's a guide [here](https://wapl.es/cnc/2020/01/25/linuxcnc-simulator-build-linux-mint.html) for Linux Mint (and other Debian derivatives).

## Build

```bash
cargo build
```

You can also run `./build.sh` to run all the commands that would normally be run in CI.

## Test

```bash
cargo test
```

## Build docs

The docs make heavy use of [intra-rustdoc-links](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html). To get the links to render correctly, run with nightly:

```bash
rustup toolchain add nightly
cargo +nightly doc
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
