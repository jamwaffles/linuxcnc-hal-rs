//! Example realtime component.
//!
//! This component is loaded using the `loadrt` HAL cmd. The crate type of any realtime component
//! must be `crate-type = [ "cdylib" ]`.
//!
//! To build this example, run `LINUXCNC_SRC=... cargo build --examples`. The resulting file is
//! placed in `target/debug/examples/librtapi.so`. This can be loaded into LinuxCNC with
//! `loadrt /path/to/librtapi`. Note that the `.so` is added for you.

use linuxcnc_hal::{
    error::PinRegisterError,
    hal_pin::{InputPin, OutputPin},
    prelude::*,
    HalComponent, RegisterResources, Resources,
};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

struct Pins {
    input_1: InputPin<f64>,
    output_1: OutputPin<f64>,
}

impl Resources for Pins {
    type RegisterError = PinRegisterError;

    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
        Ok(Pins {
            input_1: comp.register_pin::<InputPin<f64>>("input-1")?,
            output_1: comp.register_pin::<OutputPin<f64>>("output-1")?,
        })
    }
}

/// Component entry point.
#[no_mangle]
pub extern "C" fn rtapi_app_main() -> i32 {
    // Simple wrapper around inner function that turns result into integer.
    match inner() {
        // NOTE: The error code here should be a lot more descriptive in real code.
        Err(e) => -1,
        Ok(_) => 0,
    }
}

/// Inner function that can return a `Result`.
///
/// This is a convenience wrapper so that the `?` syntax sugar can be used.
fn inner() -> Result<(), Box<dyn Error>> {
    rtapi_logger::init().ok();

    // Create a new HAL component called `rust-comp`
    let comp: HalComponent<Pins> = HalComponent::new("rust-comp")?;

    // Get a reference to the `Pins` struct
    let pins = comp.resources();

    let start = Instant::now();

    // Main control loop
    while !comp.should_exit() {
        let time = start.elapsed().as_secs() as i32;

        // Set output pin to elapsed seconds since component started
        pins.output_1.set_value(time.into())?;

        // Print the current value of the input pin
        println!("Input: {:?}", pins.input_1.value());

        // Sleep for 1000ms. This should be a lower time if the component needs to update more
        // frequently.
        thread::sleep(Duration::from_millis(1000));
    }

    // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
    // at this point. Registered signal handlers are also deregistered.

    Ok(())
}
