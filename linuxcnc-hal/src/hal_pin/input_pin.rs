use crate::hal_pin::pin_direction::PinDirection;
use crate::hal_pin::PinRead;
use linuxcnc_hal_sys::hal_pin_bit_new;
use linuxcnc_hal_sys::hal_pin_float_new;
use linuxcnc_hal_sys::hal_pin_s32_new;
use linuxcnc_hal_sys::hal_pin_u32_new;

/// An input pin readable by the component
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
