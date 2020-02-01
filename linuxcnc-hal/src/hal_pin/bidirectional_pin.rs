use crate::hal_pin::pin_direction::PinDirection;
use crate::hal_pin::PinRead;
use crate::hal_pin::PinWrite;
use linuxcnc_hal_sys::hal_pin_bit_new;
use linuxcnc_hal_sys::hal_pin_float_new;
use linuxcnc_hal_sys::hal_pin_s32_new;
use linuxcnc_hal_sys::hal_pin_u32_new;

/// A pin that can be both read from and written to
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
