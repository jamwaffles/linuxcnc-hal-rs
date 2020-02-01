use crate::{
    error::{PinRegisterError, StorageError},
    hal_pin::{HalPin, PinDirection},
};

/// Wrapping struct to specialise a HAL pin to a bidirectional IO
pub struct BidirectionalPin<P> {
    pin: P,
}

impl<P> BidirectionalPin<P>
where
    P: HalPin,
{
    /// Register a new bidirectional pin with the HAL
    ///
    /// Requires the full pin name including component like `vfd.spindle-speed-out` or
    /// `jog-pendant.is-estopped`. The component ID should be fetched from
    /// [`HalComponentBuilder.id`].
    pub(crate) fn new(name: String, component_id: i32) -> Result<Self, PinRegisterError> {
        let pin = P::register_pin(&name, PinDirection::Bidirectional, component_id)?;

        Ok(Self { pin })
    }

    /// Get the pin's value
    pub fn value(&self) -> Result<&P::Storage, StorageError> {
        self.pin.storage()
    }

    /// Set the pin's value
    pub fn set_value(&self, value: P::Storage) -> Result<(), StorageError> {
        Ok(*self.pin.storage_mut()? = value)
    }
}
