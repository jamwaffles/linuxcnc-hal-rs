//! Safe wrappers for LinuxCNC's HAL (Hardware Abstraction Layer)

#![deny(missing_docs)]

mod error;
pub mod hal_pin;

use crate::{
    error::{ComponentInitError, ComponentReadyError, PinRegisterError},
    hal_pin::{HalPin, InputPin, OutputPin},
};
use linuxcnc_hal_sys::{hal_exit, hal_init, hal_ready, EINVAL, ENOMEM, HAL_NAME_LEN};
use signal_hook::iterator::Signals;
use std::ffi::CString;

/// HAL component wrapper
///
/// Create a new HAL component with [`HalComponent::new`]
#[derive(Debug)]
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

impl HalComponent {
    /// Create a new HAL component and begin initialisation
    ///
    /// Do any init work between calling this function and [`HalComponent::ready`]. The component
    /// name must be unique, and be no longer than [`HAL_NAME_LEN`].
    ///
    /// # Safety
    ///
    /// This calls [`hal_init`] internally, which may panic or leak memory.
    pub fn new(name: &'static str) -> Result<Self, ComponentInitError> {
        if name.len() > HAL_NAME_LEN as usize {
            println!(
                "Component name must be no longer than {} bytes",
                HAL_NAME_LEN
            );

            return Err(ComponentInitError::NameLength);
        }

        let name_c = CString::new(name).map_err(|_| ComponentInitError::InvalidName)?;

        let id = unsafe { hal_init(name_c.as_ptr() as *const i8) };

        match id {
            x if x == -(EINVAL as i32) => Err(ComponentInitError::Init),
            x if x == -(ENOMEM as i32) => Err(ComponentInitError::Memory),
            id if id > 0 => {
                println!("Init component {} with ID {}", name, id);

                // Register signals so component closes cleanly. These are also required for the component to
                // pass initialisation in LinuxCNC. If LinuxCNC hangs during starting waiting for the component
                // to become ready, signal handlers might not be registered.
                let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
                    .map_err(ComponentInitError::Signals)?;

                Ok(Self { name, id, signals })
            }
            code => unreachable!("Hit unreachable error code {}", code),
        }
    }

    /// Signal that the component is ready
    ///
    /// This method must be called after any pins or signals are set up. The HAL does not allow
    /// adding any more resources after this method is called.
    pub fn ready(&self) -> Result<(), ComponentReadyError> {
        let ret = unsafe { hal_ready(self.id) };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentReadyError::Invalid),
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
    ) -> Result<InputPin<P>, PinRegisterError>
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
    ) -> Result<OutputPin<P>, PinRegisterError>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_too_long() {
        let comp = HalComponent::new(
            "name-thats-way-too-long-for-linuxcnc-to-handle-wow-this-is-ridiculous",
        );

        assert!(comp.is_err());
    }
}
