//! Safe wrappers for LinuxCNC's HAL (Hardware Abstraction Layer)
//!
//! # Examples
//!
//! These examples can be loaded into LinuxCNC using a HAL file similar to this:
//!
//! ```text
//! loadusr -W /path/to/your/component/target/debug/comp_bin_name
//! net input-1 spindle.0.speed-out pins.input-1
//! net output-1 pins.output-1
//! ```
//!
//! ## Create a component with input and output
//!
//! This example creates a component called `"pins"` with a single input (`"input-1"`) and output
//! pin (`"output-1"`). It enters an infinite loop updates the value of `output-1` every second.
//! Signals are registered when `builder.ready()` is called which allow LinuxCNC to close the
//! component gracefully.
//!
//! ```rust,no_run
//! use linuxcnc_hal::{hal_pin::{InputPin, OutputPin}, prelude::*, HalComponentBuilder};
//! use std::{
//!     error::Error,
//!     thread,
//!     time::{Duration, Instant},
//! };
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Create a new HAL component called `empty`
//!     let mut builder = HalComponentBuilder::new("pins")?;
//!
//!     let input_1 = builder.register_pin::<InputPin<f64>>("input-1")?;
//!
//!     let output_1 = builder.register_pin::<OutputPin<f64>>("output-1")?;
//!
//!     // All pins added, component is now ready. This consumes the builder and registers signal
//!     // handlers.
//!     let comp = builder.ready()?;
//!
//!     let start = Instant::now();
//!
//!     // Main control loop
//!     while !comp.should_exit() {
//!         let time = start.elapsed().as_secs() as i32;
//!
//!         // Set output pin to elapsed seconds since component started
//!         output_1.set_value(time.into())?;
//!
//!         // Print the current value of the input pin
//!         println!("Input: {:?}", input_1.value());
//!
//!         // Sleep for 1000ms. This should be a lower time if the component needs to update more
//!         // frequently.
//!         thread::sleep(Duration::from_millis(1000));
//!     }
//!
//!     // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is
//!     // called at this point. Registered signal handlers are also deregistered.
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

#[macro_use]
extern crate log;

mod check_readme;
mod component;
pub mod error;
pub mod hal_pin;
pub mod prelude;

pub use crate::component::HalComponent;

use crate::error::PinRegisterError;
use crate::hal_pin::HalPin;

/// Resources for a component
pub trait Resources: Sized {
    /// Register resources against a component
    fn register_resources(comp: &RegisterResources) -> Result<Self, PinRegisterError>;
}

/// Component metadata used when registering resources
pub struct RegisterResources {
    /// Component name
    name: &'static str,

    /// Component ID
    id: i32,
}

impl RegisterResources {
    /// Register a pin with this component
    ///
    /// The pin name will be prefixed with the component name
    pub fn register_pin<P>(&self, pin_name: &'static str) -> Result<P, PinRegisterError>
    where
        P: HalPin,
    {
        let full_name = format!("{}.{}", self.name, pin_name);

        let pin = P::register(&full_name, self.id)?;

        Ok(pin)
    }
}
