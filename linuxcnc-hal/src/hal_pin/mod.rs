//! HAL pins

mod bidirectional_pin;
mod input_pin;
mod output_pin;
mod pin_direction;

pub use self::{bidirectional_pin::BidirectionalPin, input_pin::InputPin, output_pin::OutputPin};
use crate::hal_resource::HalResource;

/// HAL pin marker trait.
pub trait HalPin: HalResource {
    // /// Register the pin with the LinuxCNC HAL.
    // ///
    // /// Returns a raw pointer to the underling HAL shared memory for the pin.
    // fn register(full_name: &str, component_id: i32) -> Result<Self, ResourceRegisterError>;
}
