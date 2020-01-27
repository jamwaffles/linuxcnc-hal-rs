//! Safe wrappers for LinuxCNC's HAL (Hardware Abstraction Layer)

#![deny(missing_docs)]

mod error;
pub mod hal_pin;

use crate::hal_pin::{HalPin, InputPin, OutputPin};
use linuxcnc_hal_sys::{hal_exit, hal_init, hal_ready, EINVAL, ENOMEM, HAL_NAME_LEN};
use signal_hook::iterator::Signals;
use std::{error::Error, ffi::CString, fmt};

/// HAL component wrapper
///
/// Create a new HAL component with [`HalComponent::new`]
pub struct HalComponent {
    /// Component name
    ///
    /// Examples:
    ///
    /// * `wj200_vfd`
    /// * `hy_vfd`
    name: &'static str,

    /// Component ID
    id: i32,

    /// Handles to Unix exit signals
    signals: Signals,
}

/// Component error
///
/// Any error occurring in a component
#[derive(Debug, Copy, Clone)]
pub enum ComponentError {
    /// Unknown error occurred
    Unknown(&'static str),
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(msg) => write!(f, "HAL component error: {}", msg),
        }
    }
}

impl Error for ComponentError {}

impl HalComponent {
    /// Create a new HAL component and begin initialisation
    ///
    /// This calls [`hal_init`] under the hood. Do any init work between calling this function and
    /// [`HalComponent::ready`]. The component name must be unique, and be no longer than [`HAL_NAME_LEN`].
    pub fn new(name: &'static str) -> Result<Self, ComponentError> {
        if name.len() > HAL_NAME_LEN as usize {
            println!(
                "Component name must be no longer than {} bytes",
                HAL_NAME_LEN
            );

            return Err(ComponentError::Unknown("Name too long"));
        }

        let name_c = CString::new(name)
            .map_err(|_| ComponentError::Unknown("Failed to convert name to CString"))?;

        let id = unsafe { hal_init(name_c.as_ptr() as *const i8) };

        match id {
            x if x == -(EINVAL as i32) => {
                Err(ComponentError::Unknown("Failed to initialise component"))
            }
            x if x == -(ENOMEM as i32) => Err(ComponentError::Unknown(
                "Insufficient memory to create component",
            )),
            id if id > 0 => {
                println!("Init component {} with ID {}", name, id);

                // Register signals so component closes cleanly. These are also required for the component to
                // pass initialisation in LinuxCNC. If LinuxCNC hangs during starting waiting for the component
                // to become ready, signal handlers might not be registered.
                let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
                    .map_err(|_| ComponentError::Unknown("Failed to register signals"))?;

                Ok(Self { name, id, signals })
            }
            code => unreachable!("Hit unreachable error code {}", code),
        }
    }

    /// Signal that the component is ready
    ///
    /// This method must be called after any pins or signals are set up. The HAL does not allow
    /// adding any more resources after this method is called.
    pub fn ready(&self) -> Result<(), ComponentError> {
        let ret = unsafe { hal_ready(self.id) };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentError::Unknown(
                "HAL component was not found or is already ready",
            )),

            0 => {
                println!("Component is ready");

                Ok(())
            }
            ret => unreachable!("Unknown error status {} returned from hal_ready()", ret),
        }
    }

    /// Register an input pin with this component
    ///
    /// The pin name will be prefixed with the component name
    pub fn register_input_pin<P>(
        &mut self,
        pin_name: &'static str,
    ) -> Result<InputPin<P>, ComponentError>
    where
        P: HalPin + 'static,
    {
        let full_name = format!("{}.{}", self.name, pin_name);

        let pin = InputPin::<P>::new(full_name.clone(), self.id)?;

        Ok(pin)
    }

    /// Register an output pin with this component
    ///
    /// The pin name will be prefixed with the component name
    pub fn register_output_pin<P>(
        &mut self,
        pin_name: &'static str,
    ) -> Result<OutputPin<P>, ComponentError>
    where
        P: HalPin + 'static,
    {
        let full_name = format!("{}.{}", self.name, pin_name);

        let pin = OutputPin::<P>::new(full_name.clone(), self.id)?;

        Ok(pin)
    }

    /// Check whether the component was signalled to shut down
    pub fn should_exit(&self) -> bool {
        self.signals.pending().any(|signal| match signal {
            signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
            _ => false,
        })
    }

    /// Get the HAL-assigned ID for this component
    pub fn id(&self) -> i32 {
        self.id
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
