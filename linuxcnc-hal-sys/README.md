# LinuxCNC HAL Rust bindings

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/linuxcnc-hal-sys.svg)](https://crates.io/crates/linuxcnc-hal-sys)
[![Docs.rs](https://docs.rs/linuxcnc-hal-sys/badge.svg)](https://docs.rs/linuxcnc-hal-sys)
[![Liberapay](https://img.shields.io/liberapay/patrons/jamwaffles.svg?logo=liberapay)](https://liberapay.com/jamwaffles)

Provides unsafe, **non-realtime** Rust bindings for the LinuxCNC `hal` module. Useful for writing drivers for external hardware.

For a safe, high-level interface see the [`linuxcnc-hal`](https://crates.io/crates/linuxcnc-hal) crate.

```bash
cargo add linuxcnc-hal-sys
```

Please consider [becoming a sponsor](https://github.com/sponsors/jamwaffles/) so I may continue to maintain this crate in my spare time!

# [Documentation](https://docs.rs/linuxcnc-hal-sys)

# Example

More examples can be found in the `examples/` folder.

```rust,no_run
use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::{ffi::CString, mem, thread, time::Duration};

unsafe {
    let ret = hal_init(CString::new("pins").unwrap().as_ptr() as *const i8);

    // Check that component was created successfully
    let component_id = match ret {
        x if x == -(EINVAL as i32) => panic!("Failed to initialise component"),
        x if x == -(ENOMEM as i32) => panic!("Not enough memory to initialise component"),
        id if id > 0 => id,
        code => unreachable!("Hit unreachable error code {}", code),
    };

    println!("Component registered with ID {}", component_id);

    let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();

    let storage = hal_malloc(mem::size_of::<f64>() as i64) as *mut *mut f64;

    if storage.is_null() {
        panic!("Failed to allocate storage");
    }

    let pin_name = CString::new("pins.input-1").unwrap();

    let ret = hal_pin_float_new(
        pin_name.as_ptr() as *const i8,
        hal_pin_dir_t_HAL_IN,
        storage,
        component_id,
    );

    // Check that pin was registered successfully
    match ret {
        0 => println!("Pin registered successfully"),
        x if x == -(EINVAL as i32) => panic!("Failed to register pin"),
        x if x == -(EPERM as i32) => {
            panic!("HAL is locked. Register pins before calling hal_ready()`")
        }
        x if x == -(ENOMEM as i32) => panic!("Failed to register pin"),
        code => unreachable!("Hit unreachable error code {}", code),
    }

    let ret = hal_ready(component_id);

    // Check that component is ready
    match ret {
        0 => println!("Component is ready"),
        x if x == -(EINVAL as i32) => panic!("HAL component was not found or is already ready"),
        code => unreachable!("Hit unreachable error code {}", code),
    }

    while !signals.pending().any(|signal| match signal {
        signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
        _ => false,
    }) {
        println!("Input {:?}", **storage);

        thread::sleep(Duration::from_millis(500));
    }
}
```

# Development

## Setup

[`bindgen`](https://github.com/rust-lang/rust-bindgen) must be set up correctly. Follow the [requirements section of its docs](https://rust-lang.github.io/rust-bindgen/requirements.html).

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
