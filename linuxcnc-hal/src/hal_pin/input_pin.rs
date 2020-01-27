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
    /// Register a new input pin with the HAL
    ///
    /// Requires the full pin name including component like `vfd.speed-in` or `jog-pendant.enabled`.
    /// The component ID should be fetched from [`HalComponent::id`].
    pub fn new(name: String, component_id: i32) -> Result<Self, ComponentError> {
        let pin = P::register_pin(&name, PinDirection::In, component_id)?;

        Ok(Self { pin })
    }

    /// Get a reference to this pin's value
    pub fn value(&self) -> Result<&P::Storage, ComponentError> {
        let v = self.pin.storage()?;

        Ok(v)
    }
}
