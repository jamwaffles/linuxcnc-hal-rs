//! Create a component that adds some pin types

use linuxcnc_hal::{hal_pin::HalPinF64, HalComponent};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component called `empty` and register signal handlers
    let mut comp = HalComponent::new("pins")?;

    let input_1 = comp.register_input_pin::<HalPinF64>("input_1")?;

    let output_1 = comp.register_output_pin::<HalPinF64>("output_1")?;

    // All pins added, component is now ready. This must be called after pins are registered.
    // LinuxCNC will hang if this method is not called.
    comp.ready()?;

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
