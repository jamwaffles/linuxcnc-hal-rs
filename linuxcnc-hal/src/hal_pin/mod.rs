//! HAL pins

#[macro_use]
mod macros;

mod bidirectional_pin;
mod hal_pin;
mod input_pin;
mod output_pin;
mod pin_direction;

pub use self::{
    bidirectional_pin::BidirectionalPin, hal_pin::HalPin, input_pin::InputPin,
    output_pin::OutputPin,
};
use crate::error::StorageError;

/// Readable pin trait
///
/// Implemented for any pin that can only be read by a component
pub trait PinRead: HalPin {
    /// Get the value of the pin
    fn value(&self) -> Result<&<Self as HalPin>::Storage, StorageError> {
        self.storage()
    }
}

/// Writable pin trait
///
/// Implemented for any pin that can be only written to by a component
pub trait PinWrite: HalPin {
    /// Set the value of the pin
    fn set_value(&self, value: <Self as HalPin>::Storage) -> Result<(), StorageError> {
        Ok(*self.storage_mut()? = value)
    }
}
