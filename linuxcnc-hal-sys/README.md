# Generated LinuxCNC HAL Rust bindings

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/linuxcnc-hal-sys.svg)](https://crates.io/crates/linuxcnc-hal-sys)
[![Docs.rs](https://docs.rs/linuxcnc-hal-sys/badge.svg)](https://docs.rs/linuxcnc-hal-sys)
[![Liberapay](https://img.shields.io/badge/donate-liberapay-yellow.svg)](https://liberapay.com/jamwaffles)

# [Documentation](https://docs.rs/linuxcnc-hal-sys)

This crate provides generated bindings for LinuxCNC's HAL using [`bindgen`].

The high level, safe interface at [`linuxcnc-hal`] is recommended for user code.

## Binding versions

| This crate                                                | LinuxCNC version                                           |
| --------------------------------------------------------- | ---------------------------------------------------------- |
| Any version greater than v0.1.7                           | Provide a path to the LinuxCNC source code with `LINUXCNC_SRC` |
| [v0.1.7](https://crates.io/crates/linuxcnc-hal-sys/0.1.7) | [v2.7.15](http://linuxcnc.org/2020/01/03/LinuxCNC-2.7.15/) |
| [v0.1.6](https://crates.io/crates/linuxcnc-hal-sys/0.1.6) | [v2.7.15](http://linuxcnc.org/2020/01/03/LinuxCNC-2.7.15/) |

## Development setup

[`bindgen`](https://github.com/rust-lang/rust-bindgen) must be set up correctly. Follow the [requirements section of its docs](https://rust-lang.github.io/rust-bindgen/requirements.html).

To run and debug any HAL components, the LinuxCNC simulator can be set up. There's a guide [here](https://wapl.es/cnc/2020/01/25/linuxcnc-simulator-build-linux-mint.html) for Linux Mint (and other Debian derivatives).

## Project setup

The `LINUXCNC_SRC` environment variable is required to build this crate. The value must be the absolute path to the root of the LinuxCNC source code.

**The version of the LinuxCNC sources must match the LinuxCNC version used in the machine control.**

```bash
# Clone LinuxCNC source code into linuxcnc/
git clone https://github.com/LinuxCNC/linuxcnc.git

# Check out a specific version tag. This may also be a commit, but must match the version in use by the machine control.
cd linuxcnc && git checkout v2.8.0 && cd ..

# Create your component lib
cargo new --lib my_comp

cd my_comp

# Add LinuxCNC HAL bindings as a Cargo dependency with cargo-edit
cargo add linuxcnc-hal-sys

LINUXCNC_SRC=/path/to/linuxcnc/source/code cargo build
```

## Examples

All functions exported from this crate are `unsafe`, hence each example is wrapped in a big
`unsafe` block for clarity.

The LinuxCNC HAL requires a certain setup procedure to operate correctly. The basic program
structure should be roughly as follows:

1. Call [`hal_init`] to create a new HAL component
1. Register `SIGTERM` and `SIGINT` signals, likely with the [`signal_hook`] crate. LinuxCNC will
hang if these signals are not registered.
1. Register pins with [`hal_pin_float_new`], [`hal_pin_u32_new`], etc
1. Call [`hal_ready`] to signal to LinuxCNC that the component is ready
1. Enter an infinite loop to continuously update input/output pin values and perform component
logic

These examples can be loaded into LinuxCNC using a HAL file similar to this:

```
loadusr -W /path/to/your/component/target/debug/comp_bin_name
net input-1 spindle.0.speed-out pins.input-1
```

### Create an input pin

This example creates a component called `pins` and registers an input pin to it that accepts a
floating point value using [`hal_pin_float_new`]. Each HAL pin requires some memory allocated to
store its value which is performed with [`hal_malloc`].

The example can be loaded into LinuxCNC using a HAL file similar to this:

**Note that there is no error handling in this example for brevity.**

```rust
use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::ffi::CString;
use std::mem;
use std::thread;
use std::time::Duration;

unsafe {
    let id = hal_init(CString::new("pins").unwrap().as_ptr() as *const i8);

    println!("ID {}", id);

    let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();

    let storage = hal_malloc(mem::size_of::<*mut f64>() as i64) as *mut *mut f64;

    println!("Storage {:?}", storage);

    let pin_name = CString::new("pins.input-1").unwrap();

    let ret = hal_pin_float_new(
        pin_name.as_ptr() as *const i8,
        hal_pin_dir_t_HAL_IN,
        storage,
        id,
    );

    println!("Pin init {}", ret);

    let ret = hal_ready(id);

    println!("Ready {}", ret);

    while !signals.pending().any(|signal| match signal {
        signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
        _ => false,
    }) {
        println!("Input {:?}", **storage);

        thread::sleep(Duration::from_millis(500));
    }
}
```

### Error handling

Errors are handled in this crate the same way as in the C code. Some consts are exported like
[`EINVAL`] and [`EPERM`] to allow matching of returned error codes.

```rust
use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::ffi::CString;
use std::mem;
use std::thread;
use std::time::Duration;

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

    let storage = hal_malloc(mem::size_of::<*mut f64>() as i64) as *mut *mut f64;

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
[`linuxcnc-hal`]: https://docs.rs/linuxcnc-hal
[`bindgen`]: https://docs.rs/bindgen
[`signal_hook`]: https://docs.rs/signal_hook

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
