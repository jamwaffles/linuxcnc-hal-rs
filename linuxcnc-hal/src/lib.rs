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

// mod builder;
mod check_readme;
pub mod error;
pub mod hal_pin;
pub mod prelude;

use crate::error::ComponentInitError;
use crate::error::PinRegisterError;
use crate::hal_pin::HalPin;
use linuxcnc_hal_sys::hal_exit;
use linuxcnc_hal_sys::hal_init;
use linuxcnc_hal_sys::hal_ready;
use linuxcnc_hal_sys::EINVAL;
use linuxcnc_hal_sys::ENOMEM;
use linuxcnc_hal_sys::HAL_NAME_LEN;
use signal_hook::iterator::Signals;
use std::ffi::CString;

/// Resources for a component
pub trait Resources: Sized {
    /// Register resources against a component
    fn register_resources(comp: &RegisterResources) -> Result<Self, PinRegisterError>;
}

/// HAL component
///
/// The main HAL component interface. See the [crate documentation](./index.html) for examples.
///
/// `HalComponent` has a custom `Drop` implementation which calls [`hal_exit`] (among other things)
/// when the variable holding the component goes out of scope. Due to this, the component should be
/// initialised in `main()` so it lives for the entire life of the program.
#[derive(Debug)]
pub struct HalComponent<R> {
    /// Component name
    name: &'static str,

    /// Component ID
    id: i32,

    /// Handles to Unix exit signals
    signals: Signals,

    /// Handle to resources (pins, signals, etc) used in the component
    ///
    /// This is an `Option` so that it can be `Drop`ped before the component itself is dropped.
    /// Resources references to shared memory in LinuxCNC's HAL must be freed before [`hal_exit`] is
    /// called.
    resources: Option<R>,
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

impl<R> HalComponent<R>
where
    R: Resources,
{
    /// Create a new HAL component
    ///
    /// `new` registers a new HAL component with LinuxCNC, registers the required UNIX signal
    /// handlers and allocates resources (pins, signals, etc) required by the component.
    pub fn new(name: &'static str) -> Result<Self, ComponentInitError> {
        let id = Self::create_component(name)?;

        let resources = R::register_resources(&RegisterResources { id, name })
            .map_err(ComponentInitError::ResourceRegistration)?;

        let signals = Self::register_signals()?;

        let comp = Self {
            name,
            id,
            resources: Some(resources),
            signals,
        };

        comp.ready()
    }

    /// Register signal handlers so component closes cleanly
    ///
    /// These are also required for the component to pass initialisation in LinuxCNC. If LinuxCNC
    /// hangs during starting waiting for the component to become ready, it might be due to signal
    /// handlers not being registered.
    fn register_signals() -> Result<Signals, ComponentInitError> {
        let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
            .map_err(|_| ComponentInitError::Signals)?;

        debug!("Signals registered");

        Ok(signals)
    }

    /// Create a HAL component
    ///
    /// # Errors
    ///
    /// * [`ComponentInitError::NameLength`] - If the component name is longer than [`HAL_NAME_LEN`]
    /// * [`ComponentInitError::InvalidName`] - If the component name cannot be converted to a
    ///   [`std::ffi::CString`]
    /// * [`ComponentInitError::Init`] - If the call to [`hal_init`] returned an [`EINVAL`] status
    /// * [`ComponentInitError::Memory`] - If there is not enough memory to allocate the component
    fn create_component(name: &'static str) -> Result<i32, ComponentInitError> {
        if name.len() > HAL_NAME_LEN as usize {
            error!(
                "Component name must be no longer than {} bytes",
                HAL_NAME_LEN
            );

            Err(ComponentInitError::NameLength)
        } else {
            let name_c = CString::new(name).map_err(|_| ComponentInitError::InvalidName)?;

            let id = unsafe { hal_init(name_c.as_ptr().cast()) };

            match id {
                x if x == -(EINVAL as i32) => Err(ComponentInitError::Init),
                x if x == -(ENOMEM as i32) => Err(ComponentInitError::Memory),
                id if id > 0 => {
                    debug!("Init component {} with ID {}", name, id);

                    Ok(id)
                }
                code => unreachable!("Hit unreachable error code {}", code),
            }
        }
    }

    /// Signal to the HAL that the component is ready
    fn ready(self) -> Result<Self, ComponentInitError> {
        let ret = unsafe { hal_ready(self.id) };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentInitError::Ready),
            0 => {
                debug!("Component is ready");

                Ok(self)
            }
            ret => unreachable!("Unknown error status {} returned from hal_ready()", ret),
        }
    }

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

    /// Get a reference to the component's resources
    pub fn resources(&self) -> &R {
        // NOTE: Unwrap is safe here due to `Some(resources)` in HalComponent::new
        &self.resources.as_ref().unwrap()
    }
}

impl<R> Drop for HalComponent<R> {
    /// Clean up resources, signals and HAL component
    fn drop(&mut self) {
        // Force resources to be dropped before close
        self.resources = None;

        debug!("Closing component ID {}, name {}", self.id, self.name);

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
    use crate::error::PinRegisterError;
    use crate::RegisterResources;

    struct EmptyResources {}
    impl Resources for EmptyResources {
        fn register_resources(_comp: &RegisterResources) -> Result<Self, PinRegisterError> {
            Ok(Self {})
        }
    }

    #[test]
    fn name_too_long() {
        let comp = HalComponent::<EmptyResources>::new(
            "name-thats-way-too-long-for-linuxcnc-to-handle-wow-this-is-ridiculous",
        );

        assert_eq!(comp.err(), Some(ComponentInitError::NameLength));
    }
}
