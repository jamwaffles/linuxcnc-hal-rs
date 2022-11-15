use crate::error::{PinRegisterError, StorageError};
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
    type Storage: std::fmt::Debug;

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

            let ptr_size = mem::size_of::<*mut Self::Storage>().try_into().unwrap();

            debug!("Allocating {} bytes (ptr size {})", size, ptr_size);

            let ptr = hal_malloc(ptr_size) as *mut *mut Self::Storage;

            if ptr.is_null() {
                return Err(StorageError::Null);
            }

            if !is_aligned_to(ptr, size) {
                return Err(StorageError::Alignment);
            }

            debug!("Allocated value {:?} at {:?}", *ptr, ptr);

            ptr
        };

        Ok(storage_ptr)
    }

    /// Get the pin's name
    fn name(&self) -> &str;

    /// Get a mutable pointer to underlying shared memory storing this pin's value
    fn storage_mut(&self) -> Result<&mut Self::Storage, StorageError>;

    /// Get a reference to the underlying shared memory storing the pin's value
    fn storage(&self) -> Result<&Self::Storage, StorageError>;

    /// Register the pin with the LinuxCNC HAL
    ///
    /// Returns a raw pointer to the underling HAL shared memory for the pin
    fn register(full_pin_name: &str, component_id: i32) -> Result<Self, PinRegisterError>;
}
