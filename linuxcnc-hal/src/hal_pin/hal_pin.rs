use crate::error::PinRegisterError;
use crate::error::StorageError;
use crate::hal_pin::PinDirection;
use linuxcnc_hal_sys::hal_malloc;
use std::{convert::TryInto, mem};

fn is_aligned_to<T: ?Sized>(ptr: *const T, align: usize) -> bool {
    assert!(align.is_power_of_two());
    let ptr = ptr as *const u8 as usize;
    let mask = align.wrapping_sub(1);
    (ptr & mask) == 0
}

/// HAL pin trait
///
/// Implemented for any HAL pin. Handles allocation of backing storage in LinuxCNC's memory space.
pub trait HalPin: Sized {
    /// The underlying storage type for the given pin
    ///
    /// This will usually be a scalar value such as `u32` or `bool`
    type Storage;

    /// Allocate memory using [`hal_malloc()`] for storing pin value in
    ///
    /// # Errors
    ///
    /// This method will return an `Err` if [`hal_malloc()`] returns a null pointer.
    ///
    /// # Safety
    ///
    /// This method attempts to allocate memory in LinuxCNC's shared memory space with the unsafe
    /// method [`hal_malloc()`].
    fn allocate_storage() -> Result<*mut *mut Self::Storage, StorageError> {
        let storage_ptr = unsafe {
            let size = mem::size_of::<Self::Storage>();

            println!("Allocating {} bytes", size);

            let ptr = hal_malloc(size.try_into().unwrap()) as *mut *mut Self::Storage;

            if ptr.is_null() {
                return Err(StorageError::Null);
            }

            if !is_aligned_to(ptr, size) {
                return Err(StorageError::Alignment);
            }

            println!("Allocated at {:?}, value {:?}", ptr, *ptr);

            ptr
        };

        Ok(storage_ptr)
    }

    /// Get the pin's name
    fn name(&self) -> &str;

    /// Get pointer to underlying shared memory storing this pin's value
    fn storage(&self) -> Result<&mut Self::Storage, StorageError>;

    /// Register the pin with the LinuxCNC HAL
    ///
    /// Returns a raw pointer to the underling HAL shared memory for the pin
    fn register_pin(
        full_pin_name: &str,
        direction: PinDirection,
        component_id: i32,
    ) -> Result<Self, PinRegisterError>;
}
