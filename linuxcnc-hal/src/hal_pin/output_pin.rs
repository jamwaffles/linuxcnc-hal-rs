use crate::hal_pin::pin_direction::PinDirection;
use crate::hal_pin::PinWrite;
use linuxcnc_hal_sys::hal_pin_bit_new;
use linuxcnc_hal_sys::hal_pin_float_new;
use linuxcnc_hal_sys::hal_pin_s32_new;
use linuxcnc_hal_sys::hal_pin_u32_new;

/// A pin that can be written to by the component
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
