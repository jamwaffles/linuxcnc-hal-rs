use crate::ComponentError;
use linuxcnc_hal_sys::{
    hal_malloc, hal_pin_dir_t_HAL_IN as HAL_IN, hal_pin_dir_t_HAL_OUT as HAL_OUT,
    hal_pin_float_new, EINVAL, ENOMEM, EPERM, HAL_NAME_LEN,
};
use std::convert::TryInto;
use std::error::Error;
use std::mem;

/// Pin direction
#[derive(Copy, Clone, Debug)]
pub enum PinDirection {
    /// The pin is an input to the component
    In = HAL_IN as isize,

    /// The pin is an output from the component
    Out = HAL_OUT as isize,
}

pub trait HalPin: Sized {
    type Storage: Copy;

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

    /// Register the pin with the LinuxCNC HAL
    ///
    /// Returns a raw pointer to the underling HAL shared memory for the pin
    fn register_pin(
        full_pin_name: &str,
        direction: PinDirection,
        component_id: i32,
    ) -> Result<Self, ComponentError>;
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
            Ok(unsafe { &mut **self.storage })
        }
    }

    fn register_pin(
        full_pin_name: &str,
        direction: PinDirection,
        component_id: i32,
    ) -> Result<Self, ComponentError> {
        if full_pin_name.len() > HAL_NAME_LEN as usize {
            return Err(ComponentError::Unknown("Pin name is too long"));
        }

        let storage = Self::allocate_storage().map_err(|_| {
            ComponentError::Unknown("Failed to allocate storage in HAL shared memory")
        })?;

        let ret = unsafe {
            hal_pin_float_new(
                full_pin_name.as_ptr() as *const i8,
                direction as i32,
                storage,
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
                println!("Make pin {} returned {}", full_pin_name, ret);

                Ok(Self {
                    name: full_pin_name.to_string(),
                    storage,
                })
            }
            code => unreachable!("Hit unreachable error code {}", code),
        }
    }
}

impl Drop for HalPinF64 {
    fn drop(&mut self) {
        println!("Drop HalPinF64 {}", self.name);
    }
}

pub struct InputPin<P> {
    pin: P,
}

impl<P> InputPin<P>
where
    P: HalPin,
{
    pub fn new(name: String, component_id: i32) -> Result<Self, ComponentError> {
        let pin = P::register_pin(&name, PinDirection::In, component_id)?;

        Ok(Self { pin })
    }

    /// Get this pin's value
    pub fn value(&self) -> Result<P::Storage, ComponentError> {
        self.pin.storage().map(|v| *v)
    }
}

pub struct OutputPin<P> {
    pin: P,
}

impl<P> OutputPin<P>
where
    P: HalPin,
{
    pub fn new(name: String, component_id: i32) -> Result<Self, ComponentError> {
        let pin = P::register_pin(&name, PinDirection::Out, component_id)?;

        Ok(Self { pin })
    }

    /// Set the pin's value
    pub fn set_value(&self, value: P::Storage) -> Result<(), ComponentError> {
        Ok(*self.pin.storage()? = value)
    }
}
