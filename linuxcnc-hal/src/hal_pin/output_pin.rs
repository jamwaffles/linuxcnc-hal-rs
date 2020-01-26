use crate::hal_pin::{HalPin, PinDirection};
use crate::ComponentError;

/// Wrapping struct to specialise a HAL pin to an output
pub struct OutputPin<P> {
    pin: P,
}

impl<P> OutputPin<P>
where
    P: HalPin,
{
    pub fn new(name: String, component_id: i32) -> Result<Self, ComponentError> {
        let pin = P::register_pin(&name, PinDirection::Out, component_id)?;

        Ok(Self { pin })
    }

    /// Set the pin's value
    pub fn set_value(&self, value: P::Storage) -> Result<(), ComponentError> {
        Ok(*self.pin.storage()? = value)
    }
}
