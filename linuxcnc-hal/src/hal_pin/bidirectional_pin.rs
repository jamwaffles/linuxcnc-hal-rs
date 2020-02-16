use crate::hal_pin::{pin_direction::PinDirection, PinRead, PinWrite};
use linuxcnc_hal_sys::{hal_pin_bit_new, hal_pin_float_new, hal_pin_s32_new, hal_pin_u32_new};

/// A pin that can be both read from and written to
///
/// Supported pin types are as follows
///
/// | Type                     | Storage | Equivalent `linuxcnc_hal_sys` function |
/// | ------------------------ | ------- | -------------------------------------- |
/// | `BidirectionalPin<f64>`  | `f64`   | [`hal_pin_float_new`]                  |
/// | `BidirectionalPin<u32>`  | `u32`   | [`hal_pin_u32_new`]                    |
/// | `BidirectionalPin<i32>`  | `i32`   | [`hal_pin_s32_new`]                    |
/// | `BidirectionalPin<bool>` | `bool`  | [`hal_pin_bit_new`]                    |
///
/// # Examples
///
/// ## Create a pin
///
/// This example creates a `BidirectionalPin` under `demo-component.named-pin`.
///
/// ```rust,no_run
/// use linuxcnc_hal::{
///    error::PinRegisterError,
///    hal_pin::{BidirectionalPin},
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
///    pin: BidirectionalPin<f64>,
/// }
///
/// impl Resources for Pins {
///    type RegisterError = PinRegisterError;
///
///    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
///        Ok(Pins {
///            pin: comp.register_pin::<BidirectionalPin<f64>>("named-pin")?,
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
///         while !comp.should_exit() {
///             println!("Input: {:?}", pin.value());
///
///             pin.set_value(123.45f64);
///
///             thread::sleep(Duration::from_millis(1000));
///         }
///
///    Ok(())
/// }
/// ```
pub struct BidirectionalPin<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut *mut S,
}

impl<S> Drop for BidirectionalPin<S> {
    fn drop(&mut self) {
        debug!("Drop BidirectionalPin {}", self.name);
    }
}

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
