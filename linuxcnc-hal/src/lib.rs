pub mod hal_pin;

use crate::hal_pin::{HalPinF64, PinDirection, PinType};
use linuxcnc_hal_sys::{hal_exit, hal_init, hal_ready, EINVAL, HAL_NAME_LEN};
use signal_hook::iterator::Signals;
use std::collections::HashMap;

use std::error::Error;
use std::ffi::CString;
use std::fmt;

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

    /// Pin resources registered by this component
    pins: HashMap<String, HalPinF64>,

    /// Handles to Unix exit signals
    signals: Signals,
}

#[derive(Debug, Copy, Clone)]
pub enum ComponentError {
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
    /// [`ready()`]. The component name must be unique, and be no longer than [`HAL_NAME_LEN`].
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

        if id < 0 {
            println!("Create comp error code {}", id);

            // TODO handle errors better
            // -EINVAL
            // -ENOMEM
            Err(ComponentError::Unknown("Failed to initialise component"))
        } else {
            println!("Init component {} with ID {}", name, id);

            // Register signals so component closes cleanly. These are also required for the component to
            // pass initialisation in LinuxCNC. If LinuxCNC hangs during starting waiting for the component
            // to become ready, signal handlers might not be registered.
            let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
                .map_err(|_| ComponentError::Unknown("Failed to register signals"))?;

            Ok(Self {
                name,
                id,
                pins: HashMap::new(),
                signals,
            })
        }
    }

    /// Signal that the component is ready
    ///
    /// This method must be called after any pins or signals are set up. The HAL does not allow
    /// adding any more resources after this method is called.
    pub fn ready(&self) -> Result<(), ComponentError> {
        let ret = unsafe { hal_ready(self.id) };

        if ret == 0 {
            println!("Component is ready");

            Ok(())
        } else if ret == -(EINVAL as i32) {
            Err(ComponentError::Unknown(
                "HAL component was not found or is already ready",
            ))
        } else {
            // At time of writing, the body of hal_ready() can only return `0` or `-EINVAL`
            unreachable!("Error status {} returned from hal_ready()", ret)
        }
    }

    /// Register a pin with this component
    ///
    /// The pin name will be prefixed with the component name
    pub fn register_pin(
        &mut self,
        pin_name: &'static str,
        _pin_type: PinType,
        direction: PinDirection,
    ) -> Result<(), ComponentError> {
        let full_name = format!("{}.{}", self.name, pin_name);

        let pin = HalPinF64::new(full_name.clone(), _pin_type, direction, self.id)?;

        self.pins.insert(full_name.clone(), pin);

        Ok(())
    }

    /// Check whether the component was signalled to shut down
    pub fn should_exit(&self) -> bool {
        self.signals.pending().any(|signal| match signal {
            signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
            _ => false,
        })
    }

    pub fn pins(&mut self) -> &mut HashMap<String, HalPinF64> {
        &mut self.pins
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
