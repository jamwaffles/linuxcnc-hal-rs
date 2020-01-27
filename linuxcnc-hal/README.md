# LinuxCNC HAL interface for Rust

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/linuxcnc-hal.svg)](https://crates.io/crates/linuxcnc-hal)
[![Docs.rs](https://docs.rs/linuxcnc-hal/badge.svg)](https://docs.rs/linuxcnc-hal)
[![Liberapay](https://img.shields.io/liberapay/patrons/jamwaffles.svg?logo=liberapay)](https://liberapay.com/jamwaffles)

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

    let input_1 = builder.register_input_pin::<HalPinF64>("input_1")?;

    let output_1 = builder.register_output_pin::<HalPinF64>("output_1")?;

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
