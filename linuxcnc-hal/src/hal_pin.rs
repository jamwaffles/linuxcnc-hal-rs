use crate::ComponentError;
use linuxcnc_hal_sys::{
    hal_malloc, hal_pin_dir_t_HAL_IN as HAL_IN, hal_pin_dir_t_HAL_OUT as HAL_OUT, hal_pin_u32_new,
};
use std::convert::TryInto;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem;

/// Pin direction
#[derive(Copy, Clone, Debug)]
pub enum PinDirection {
    /// The pin is an input to the component
    In = HAL_IN as isize,

    /// The pin is an output from the component
    Out = HAL_OUT as isize,
}

/// Pin type to use
#[derive(Copy, Clone, Debug)]
pub enum PinType {
    /// `u32` value
    U32,
}

pub struct HalPin {
    name: String,
    storage: *mut c_void,
}

impl HalPin {
    pub fn new(
        pin_name: String,
        _pin_type: PinType,
        direction: PinDirection,
        component_id: i32,
    ) -> Result<Self, ComponentError> {
        // let mut storage_ptr: *mut c_uint = ptr::null_mut();
        // let mut storage: u32 = 0;

        // let storage_ptr: *mut u32 = &mut storage;

        // Allocate some HAL shared memory
        let storage_ptr = unsafe { hal_malloc(mem::size_of::<u32>().try_into().unwrap()) };

        let full_name = CString::new(pin_name.clone())
            .map_err(|_| ComponentError::Unknown("Failed to convert pin name to CString"))?;

        let ret = unsafe {
            hal_pin_u32_new(
                full_name.as_ptr() as *const i8,
                direction as i32,
                storage_ptr as *mut *mut u32,
                component_id,
            )
        };

        if ret != 0 {
            // TODO: Handle return values
            // -EINVAL;
            // -EPERM;
            // -ENOMEM;
            Err(ComponentError::Unknown("Failed to create pin"))
        } else {
            println!("Make pin {} returned {}", pin_name, ret);

            Ok(Self {
                name: pin_name,
                storage: storage_ptr,
            })
        }
    }
}
