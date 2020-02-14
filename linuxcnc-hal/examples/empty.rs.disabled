//! Create a component that does nothing except init and exit

use linuxcnc_hal::HalComponentBuilder;
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component builder
    let builder = HalComponentBuilder::new("empty")?;

    // All pins added, component is now ready. Consumer the builder into an actual HAL component
    let comp = builder.ready()?;

    while !comp.should_exit() {
        // Main control loop code goes here. This example prints `Poll` every 1000ms. This code can
        // block - LinuxCNC handles component threading.
        println!("Poll");

        thread::sleep(Duration::from_millis(1000));
    }

    // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
    // at this point. Registered signal handlers are also deregistered.

    Ok(())
}
