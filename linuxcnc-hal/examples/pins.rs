//! Create a component that adds some pin types

use linuxcnc_hal::{
    hal_pin::{PinDirection, PinType},
    HalComponent,
};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component called `empty` and register signal handlers
    let mut comp = HalComponent::new("pins")?;

    comp.register_pin("input_1", PinType::F64, PinDirection::In)?;

    comp.register_pin("output_1", PinType::F64, PinDirection::Out)?;

    // All pins added, component is now ready. This must be called otherwise LinuxCNC will hang.
    comp.ready()?;

    let start = Instant::now();

    while !comp.should_exit() {
        // Main control loop code goes here. This example prints `Poll` every 1000ms. This code can
        // block - LinuxCNC handles component threading.

        let time = start.elapsed().as_secs() as u32;

        let pins = comp.pins();

        pins.get_mut("pins.output_1")
            .unwrap()
            .set_value(time as f64)?;

        // TODO: Deref trait?
        println!("Input: {:?}", pins.get("pins.input_1").unwrap().value());

        thread::sleep(Duration::from_millis(1000));
    }

    // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
    // at this point. Registered signal handlers are also deregistered.

    Ok(())
}
