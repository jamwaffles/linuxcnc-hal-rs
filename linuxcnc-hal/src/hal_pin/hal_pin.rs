use crate::hal_pin::PinDirection;
use crate::ComponentError;
use linuxcnc_hal_sys::hal_malloc;
use std::convert::TryInto;
use std::error::Error;
use std::mem;

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
