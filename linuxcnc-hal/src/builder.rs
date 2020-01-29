//! HAL component builder

use crate::{
    error::{ComponentInitError, ComponentReadyError, PinRegisterError},
    hal_pin::{HalPin, InputPin, OutputPin},
    HalComponent,
};
use linuxcnc_hal_sys::{hal_init, hal_ready, EINVAL, ENOMEM, HAL_NAME_LEN};
use signal_hook::iterator::Signals;
use std::ffi::CString;

/// HAL component builder
///
/// Create a new HAL component with [`HalComponentBuilder::new`]. Use the builder to register a
/// component with LinuxCNC. Once created, pins can be registered to the component. Finally, call
/// `builder.ready()` to consume the builder and create a [`HalComponent`] ready for use in the
/// component main loop.
#[derive(Debug, PartialEq)]
pub struct HalComponentBuilder {
    /// Component name
    ///
    /// Examples:
    ///
    /// * `wj200_vfd`
    /// * `hy_vfd`
    name: &'static str,

    /// Component ID
    id: i32,
}

impl HalComponentBuilder {
    /// Create a new HAL component builder and begin initialisation
    ///
    /// # Safety
    ///
    /// This calls [`hal_init`] internally, which may panic or leak memory.
    ///
    /// # Errors
    ///
    /// This component will error if the component name is longer than [`HAL_NAME_LEN`], the name
    /// cannot be converted to a valid [`CString`] or the call to [`hal_init`] returns an invalid
    /// ID.
    pub fn new(name: &'static str) -> Result<Self, ComponentInitError> {
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

                    Ok(Self { name, id })
                }
                code => unreachable!("Hit unreachable error code {}", code),
            }
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

    /// Consume the builder and signal that the component is ready
    ///
    /// This method is called after any pins are registered and consumes the builder into a
    /// [`HalComponent`].
    ///
    /// # Safety
    ///
    /// This method calls the unsafe [`hal_ready`] method internally.
    pub fn ready(self) -> Result<HalComponent, ComponentReadyError> {
        let ret = unsafe { hal_ready(self.id) };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentReadyError::Invalid),
            0 => {
                // Register signals so component closes cleanly. These are also required for the component to
                // pass initialisation in LinuxCNC. If LinuxCNC hangs during starting waiting for the component
                // to become ready, signal handlers might not be registered.
                let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
                    .map_err(ComponentReadyError::Signals)?;

                debug!("Signals registered, component is ready");

                let HalComponentBuilder { name, id, .. } = self;

                Ok(HalComponent { name, id, signals })
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
}
