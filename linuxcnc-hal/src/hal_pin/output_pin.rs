use crate::hal_pin::{pin_direction::PinDirection, PinWrite};
use linuxcnc_hal_sys::{hal_pin_bit_new, hal_pin_float_new, hal_pin_s32_new, hal_pin_u32_new};

/// A pin that can be written to by the component
///
/// Supported pin types are as follows
///
/// | Type              | Storage | Equivalent `linuxcnc_hal_sys` function |
/// | ----------------- | ------- | -------------------------------------- |
/// | `OutputPin<f64>`  | `f64`   | [`hal_pin_float_new`]                  |
/// | `OutputPin<u32>`  | `u32`   | [`hal_pin_u32_new`]                    |
/// | `OutputPin<i32>`  | `i32`   | [`hal_pin_s32_new`]                    |
/// | `OutputPin<bool>` | `bool`  | [`hal_pin_bit_new`]                    |
///
/// # Examples
///
/// ## Create a pin
///
/// This example creates an `OutputPin` under `demo-component.named-pin`.
///
/// ```rust,no_run
/// use linuxcnc_hal::{
///     hal_pin::OutputPin,
///     prelude::*,
///     HalComponentBuilder,
/// };
/// use std::{thread, time::Duration, error::Error};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let mut builder = HalComponentBuilder::new("demo-component")?;
///
///     let pin = builder.register_pin::<OutputPin<f64>>("named-pin")?;
///
///     let comp = builder.ready()?;
///
///     // Main control loop
///     while !comp.should_exit() {
///         pin.set_value(123.45f64);
///         thread::sleep(Duration::from_millis(1000));
///     }
///
///     Ok(())
/// }
/// ```
pub struct OutputPin<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut *mut S,
}

impl<S> Drop for OutputPin<S> {
    fn drop(&mut self) {
        debug!("Drop OutputPin {}", self.name);
    }
}

impl_pin!(OutputPin, f64, hal_pin_float_new, PinDirection::Out);
impl_pin!(OutputPin, u32, hal_pin_u32_new, PinDirection::Out);
impl_pin!(OutputPin, i32, hal_pin_s32_new, PinDirection::Out);
impl_pin!(OutputPin, bool, hal_pin_bit_new, PinDirection::Out);

impl PinWrite for OutputPin<f64> {}
impl PinWrite for OutputPin<u32> {}
impl PinWrite for OutputPin<i32> {}
impl PinWrite for OutputPin<bool> {}
