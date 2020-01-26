use linuxcnc_hal_sys::{hal_pin_dir_t_HAL_IN as HAL_IN, hal_pin_dir_t_HAL_OUT as HAL_OUT};

/// Pin direction
#[derive(Copy, Clone, Debug)]
pub enum PinDirection {
    /// The pin is an input to the component
    In = HAL_IN as isize,

    /// The pin is an output from the component
    Out = HAL_OUT as isize,
}
