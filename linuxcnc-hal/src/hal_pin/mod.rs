//! HAL pins

#[macro_use]
mod macros;

mod bidirectional_pin;
mod hal_pin;
mod input_pin;
mod output_pin;
mod pin_direction;

use self::pin_direction::PinDirection;
pub use self::{
    bidirectional_pin::BidirectionalPin, hal_pin::HalPin, input_pin::InputPin,
    output_pin::OutputPin,
};
use crate::error::StorageError;
use linuxcnc_hal_sys::{hal_pin_bit_new, hal_pin_float_new, hal_pin_s32_new, hal_pin_u32_new};

/// TODO: Docs
pub trait PinRead: HalPin {
    /// TODO: Docs
    fn value(&self) -> Result<&<Self as HalPin>::Storage, StorageError> {
        self.storage()
    }
}

/// TODO: Docs
pub trait PinWrite: HalPin {
    /// TODO: Docs
    fn set_value(&self, value: <Self as HalPin>::Storage) -> Result<(), StorageError> {
        Ok(*self.storage_mut()? = value)
    }
}

impl_pin!(InputPin, f64, hal_pin_float_new, PinDirection::In);
impl_pin!(InputPin, u32, hal_pin_u32_new, PinDirection::In);
impl_pin!(InputPin, i32, hal_pin_s32_new, PinDirection::In);
impl_pin!(InputPin, bool, hal_pin_bit_new, PinDirection::In);

impl PinRead for InputPin<f64> {}
impl PinRead for InputPin<u32> {}
impl PinRead for InputPin<i32> {}
impl PinRead for InputPin<bool> {}

impl_pin!(OutputPin, f64, hal_pin_float_new, PinDirection::Out);
impl_pin!(OutputPin, u32, hal_pin_u32_new, PinDirection::Out);
impl_pin!(OutputPin, i32, hal_pin_s32_new, PinDirection::Out);
impl_pin!(OutputPin, bool, hal_pin_bit_new, PinDirection::Out);

impl PinWrite for OutputPin<f64> {}
impl PinWrite for OutputPin<u32> {}
impl PinWrite for OutputPin<i32> {}
impl PinWrite for OutputPin<bool> {}

impl_pin!(
    BidirectionalPin,
    f64,
    hal_pin_float_new,
    PinDirection::Bidirectional
);
impl_pin!(
    BidirectionalPin,
    u32,
    hal_pin_u32_new,
    PinDirection::Bidirectional
);
impl_pin!(
    BidirectionalPin,
    i32,
    hal_pin_s32_new,
    PinDirection::Bidirectional
);
impl_pin!(
    BidirectionalPin,
    bool,
    hal_pin_bit_new,
    PinDirection::Bidirectional
);

impl PinWrite for BidirectionalPin<f64> {}
impl PinWrite for BidirectionalPin<u32> {}
impl PinWrite for BidirectionalPin<i32> {}
impl PinWrite for BidirectionalPin<bool> {}

impl PinRead for BidirectionalPin<f64> {}
impl PinRead for BidirectionalPin<u32> {}
impl PinRead for BidirectionalPin<i32> {}
impl PinRead for BidirectionalPin<bool> {}

// impl_pin!(
//     HalPinI32,
//     "HalPinI32",
//     hal_pin_s32_new,
//     "hal_pin_s32_new",
//     i32,
//     "i32"
// );

// impl_pin!(
//     HalPinU32,
//     "HalPinU32",
//     hal_pin_u32_new,
//     "hal_pin_u32_new",
//     u32,
//     "u32"
// );

// impl_pin!(
//     HalPinBool,
//     "HalPinBool",
//     hal_pin_bit_new,
//     "hal_pin_bit_new",
//     bool,
//     "bool"
// );
