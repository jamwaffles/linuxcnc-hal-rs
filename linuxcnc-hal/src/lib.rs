//! Safe wrappers for LinuxCNC's HAL (Hardware Abstraction Layer)
//!
//! # Examples
//!
//! ## Create a component with input and output
//!
//! This example creates a component called `"pins"` with a single input (`"input_1"`) and output pin
//! (`"output_1"`). It enters an infinite loop until asked to exit by the LinuxCNC parent process which
//! polls `input_1` and updates the value of `output_1` every second.
//!
//!
//! ```rust,no_run
//! use linuxcnc_hal::{hal_pin::HalPinF64, HalComponentBuilder};
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
//!     let input_1 = builder.register_input_pin::<HalPinF64>("input_1")?;
//!
//!     let output_1 = builder.register_output_pin::<HalPinF64>("output_1")?;
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
//!     // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
//!     // at this point. Registered signal handlers are also deregistered.
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]

mod builder;
mod check_readme;
mod error;
pub mod hal_pin;

pub use crate::builder::HalComponentBuilder;
use linuxcnc_hal_sys::hal_exit;
use signal_hook::iterator::Signals;

/// HAL component
///
/// Use [`HalComponentInit::new`] to create a new component builder. Once all pins are registered,
/// calling `.ready()` on the builder will convert it to a `HalComponent` ready for use in the main
/// loop.
#[derive(Debug)]
pub struct HalComponent {
    /// Component name
    name: &'static str,

    /// Component ID
    id: i32,

    /// Handles to Unix exit signals
    signals: Signals,
}

impl HalComponent {
    /// Get the HAL-assigned ID for this component
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the component name
    pub fn name(&self) -> &str {
        self.name
    }

    /// Check whether the component was signalled to shut down
    pub fn should_exit(&self) -> bool {
        self.signals.pending().any(|signal| match signal {
            signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
            _ => false,
        })
    }
}

impl Drop for HalComponent {
    /// Clean up resources
    ///
    /// Cleans up HAL resources by calling [`hal_exit`] as required by the docs.
    ///
    /// Also deregisters signal handlers
    fn drop(&mut self) {
        println!("Closing component ID {}, name {}", self.id, self.name);

        self.signals.close();

        unsafe {
            hal_exit(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ComponentInitError;

    #[test]
    fn name_too_long() {
        let comp = HalComponentBuilder::new(
            "name-thats-way-too-long-for-linuxcnc-to-handle-wow-this-is-ridiculous",
        );

        assert_eq!(comp, Err(ComponentInitError::NameLength));
    }
}
