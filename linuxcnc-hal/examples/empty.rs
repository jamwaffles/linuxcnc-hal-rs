//! Create a component that does nothing except init and exit

use linuxcnc_hal::HalComponent;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component called `empty` and register signal handlers
    let comp = HalComponent::new("empty")?;

    // All pins added, component is now ready. This must be called otherwise LinuxCNC will hang.
    comp.ready()?;

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
