use crate::{hal_pin::pin_direction::PinDirection, hal_resource::ResourceWrite};
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
///    error::PinRegisterError,
///    hal_pin::{OutputPin},
///    prelude::*,
///    HalComponent, RegisterResources, Resources,
/// };
/// use std::{
///    error::Error,
///    thread,
///    time::{Duration, Instant},
/// };
///
/// struct Pins {
///    pin: OutputPin<f64>,
/// }
///
/// impl Resources for Pins {
///    type RegisterError = PinRegisterError;
///
///    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
///        Ok(Pins {
///            pin: comp.register_pin::<OutputPin<f64>>("named-pin")?,
///        })
///    }
/// }
///
/// fn main() -> Result<(), Box<dyn Error>> {
///    let comp: HalComponent<Pins> = HalComponent::new("demo-component")?;
///
///    let Pins { pin } = comp.resources();
///
///    let start = Instant::now();
///
///         // Main control loop
///     while !comp.should_exit() {
///         pin.set_value(123.45f64);
///         thread::sleep(Duration::from_millis(1000));
///     }
///
///    Ok(())
/// }
/// ```
#[derive(Debug)]
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

impl ResourceWrite for OutputPin<f64> {}
impl ResourceWrite for OutputPin<u32> {}
impl ResourceWrite for OutputPin<i32> {}
impl ResourceWrite for OutputPin<bool> {}
