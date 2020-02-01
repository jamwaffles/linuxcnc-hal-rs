use crate::hal_pin::{pin_direction::PinDirection, PinRead};
use linuxcnc_hal_sys::{hal_pin_bit_new, hal_pin_float_new, hal_pin_s32_new, hal_pin_u32_new};

/// An input pin readable by the component
///
/// Supported pin types are as follows
///
/// | Type             | Storage | Equivalent `linuxcnc_hal_sys` function |
/// | ---------------- | ------- | -------------------------------------- |
/// | `InputPin<f64>`  | `f64`   | [`hal_pin_float_new`]                  |
/// | `InputPin<u32>`  | `u32`   | [`hal_pin_u32_new`]                    |
/// | `InputPin<i32>`  | `i32`   | [`hal_pin_s32_new`]                    |
/// | `InputPin<bool>` | `bool`  | [`hal_pin_bit_new`]                    |
///
/// # Examples
///
/// ## Create a pin
///
/// This example creates an `InputPin` under `demo-component.named-pin`.
///
/// ```rust,no_run
/// use linuxcnc_hal::{
///     hal_pin::InputPin,
///     prelude::*,
///     HalComponentBuilder,
/// };
/// use std::{thread, time::Duration, error::Error};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let mut builder = HalComponentBuilder::new("demo-component")?;
///
///     let pin = builder.register_pin::<InputPin<f64>>("named-pin")?;
///
///     let comp = builder.ready()?;
///
///     // Main control loop
///     while !comp.should_exit() {
///         println!("Input: {:?}", pin.value());
///         thread::sleep(Duration::from_millis(1000));
///     }
///
///     Ok(())
/// }
/// ```
pub struct InputPin<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut *mut S,
}

impl<S> Drop for InputPin<S> {
    fn drop(&mut self) {
        debug!("Drop InputPin {}", self.name);
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
