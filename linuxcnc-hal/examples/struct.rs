//! Store pin resources on a struct

use linuxcnc_hal::{
    hal_pin::{InputPin, OutputPin},
    prelude::*,
    HalComponentBuilder,
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

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component called `pins`
    let mut builder = HalComponentBuilder::new("pins")?;

    let pins = Pins {
        input_1: builder.register_pin::<InputPin<f64>>("input-1")?,
        output_1: builder.register_pin::<OutputPin<f64>>("output-1")?,
    };

    // All pins added, component is now ready. This consumes the builder and registers signal
    // handlers.
    let comp = builder.ready()?;

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
