//! Create a component that does nothing except init and exit
//!
//! This component doesn't register any pins or other resources.

use linuxcnc_hal::error::ResourcesError;
use linuxcnc_hal::HalComponent;
use linuxcnc_hal::RegisterResources;
use linuxcnc_hal::Resources;
use std::{error::Error, thread, time::Duration};

/// An empty resources struct that doesn't register any resources
#[derive(Debug)]
struct EmptyResources {}

impl Resources for EmptyResources {
    type RegisterError = ResourcesError;

    fn register_resources(_comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
        Ok(Self {})
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HAL component
    let comp = HalComponent::<EmptyResources>::new("empty")?;

    while !comp.should_exit() {
        // Main control loop code goes here. This example prints `Poll` every 1000ms. This code can
        // block - LinuxCNC handles component threading.
        println!("Poll");

        thread::sleep(Duration::from_millis(1000));
    }

    // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
    // at this point. Registered resources and signal handlers are also deregistered.

    Ok(())
}
