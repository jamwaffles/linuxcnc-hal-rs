//! HAL pins

#[macro_use]
mod macros;

mod hal_pin;
mod input_pin;
mod output_pin;
mod pin_direction;

use self::pin_direction::PinDirection;
pub use self::{hal_pin::HalPin, input_pin::InputPin, output_pin::OutputPin};
use linuxcnc_hal_sys::{hal_pin_bit_new, hal_pin_float_new, hal_pin_s32_new, hal_pin_u32_new};

impl_pin!(
    HalPinF64,
    "HalPinF64",
    hal_pin_float_new,
    "hal_pin_float_new",
    f64,
    "f64"
);

impl_pin!(
    HalPinI32,
    "HalPinI32",
    hal_pin_s32_new,
    "hal_pin_s32_new",
    i32,
    "i32"
);

impl_pin!(
    HalPinU32,
    "HalPinU32",
    hal_pin_u32_new,
    "hal_pin_u32_new",
    u32,
    "u32"
);

impl_pin!(
    HalPinBool,
    "HalPinBool",
    hal_pin_bit_new,
    "hal_pin_bit_new",
    bool,
    "bool"
);
