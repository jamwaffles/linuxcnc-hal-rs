use crate::ComponentError;
use linuxcnc_hal_sys::{
    hal_malloc, hal_pin_dir_t_HAL_IN as HAL_IN, hal_pin_dir_t_HAL_OUT as HAL_OUT,
    hal_pin_float_new, EINVAL, ENOMEM, EPERM, HAL_NAME_LEN,
};
use std::convert::TryInto;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
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
    /// `f64` value
    F64,
}

pub trait HalPin {
    type Storage: fmt::Debug + Default;

    /// Allocate memory using [`hal_malloc()`] for storing pin value in
    fn allocate_storage() -> Result<*mut *mut Self::Storage, Box<dyn Error>> {
        let storage_ptr = unsafe {
            let size = mem::size_of::<Self::Storage>().try_into().unwrap();

            println!("Allocating {} bytes", size);

            let ptr = hal_malloc(size) as *mut *mut Self::Storage;

            if ptr.is_null() {
                panic!("Pointer is null");
            }

            println!("Allocated at {:?}, value {:?}", ptr, *ptr);

            ptr
        };

        Ok(storage_ptr)
    }

    /// Get the pin's name
    fn name(&self) -> &str;

    /// Get pointer to underlying shared memory storing this pin's value
    fn storage(&self) -> Result<&mut Self::Storage, ComponentError>;
}

#[derive(Debug, PartialEq)]
pub struct HalPinF64 {
    name: String,
    storage: *mut *mut f64,
}

impl HalPin for HalPinF64 {
    type Storage = f64;

    fn name(&self) -> &str {
        &self.name
    }

    fn storage(&self) -> Result<&mut Self::Storage, ComponentError> {
        if self.storage.is_null() {
            Err(ComponentError::Unknown("Value pointer is null"))
        } else {
            let value = unsafe { &mut **self.storage };

            Ok(value)
        }
    }
}

impl HalPinF64 {
    pub fn new(
        pin_name: String,
        _pin_type: PinType,
        direction: PinDirection,
        component_id: i32,
    ) -> Result<Self, ComponentError> {
        if pin_name.len() > HAL_NAME_LEN as usize {
            return Err(ComponentError::Unknown("Pin name is too long"));
        }

        let storage_ptr = Self::allocate_storage().map_err(|_| {
            ComponentError::Unknown("Failed to allocate storage in HAL shared memory")
        })?;

        println!("PTR for {}: {:?}", pin_name, storage_ptr);

        let full_name = CString::new(pin_name.clone())
            .map_err(|_| ComponentError::Unknown("Failed to convert pin name to CString"))?;

        let ret = unsafe {
            hal_pin_float_new(
                full_name.as_ptr() as *const i8,
                direction as i32,
                storage_ptr,
                component_id,
            )
        };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentError::Unknown("Failed to create pin")),
            x if x == -(EPERM as i32) => Err(ComponentError::Unknown("HAL is locked")),
            x if x == -(ENOMEM as i32) => {
                Err(ComponentError::Unknown("Insufficient memory for pin"))
            }
            0 => {
                println!("Make pin {} returned {}", pin_name, ret);

                Ok(Self {
                    name: pin_name,
                    storage: storage_ptr,
                })
            }
            code => unreachable!("Hit unreachable error code {}", code),
        }
    }

    /// Set the pin's output value
    pub fn set_value(&mut self, value: f64) -> Result<(), ComponentError> {
        *self.storage()? = value;

        Ok(())
    }

    /// Get this pin's value
    pub fn value(&self) -> Result<f64, ComponentError> {
        self.storage().map(|v| *v)
    }
}

impl Drop for HalPinF64 {
    fn drop(&mut self) {
        println!("Drop HalPinF64 {}", self.name);
    }
}
