use crate::{
    hal_pin::{HalPin, PinDirection},
    ComponentError,
};

/// Wrapping struct to specialise a HAL pin to an input
pub struct InputPin<P> {
    pin: P,
}

impl<P> InputPin<P>
where
    P: HalPin,
{
    pub fn new(name: String, component_id: i32) -> Result<Self, ComponentError> {
        let pin = P::register_pin(&name, PinDirection::In, component_id)?;

        Ok(Self { pin })
    }

    /// Get this pin's value as a reference
    pub fn value(&self) -> Result<&P::Storage, ComponentError> {
        let v = self.pin.storage()?;

        Ok(v)
    }
}
