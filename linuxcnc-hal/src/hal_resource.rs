use crate::error::{ResourceRegisterError, StorageError};
use linuxcnc_hal_sys::hal_malloc;
use std::{convert::TryInto, mem};

fn is_aligned_to<T: ?Sized>(ptr: *const T, align: usize) -> bool {
    assert!(align.is_power_of_two());
    let ptr = ptr as *const u8 as usize;
    let mask = align.wrapping_sub(1);
    (ptr & mask) == 0
}

/// HAL resource trait.
///
/// Implemented for any HAL pin, parameter, etc. Handles allocation of backing storage in LinuxCNC's
///  memory space.
pub trait HalResource: Sized + Drop {
    /// The underlying storage type for the given pin
    ///
    /// This will usually be a scalar value such as `u32` or `bool`
    type Storage: std::fmt::Debug;

    /// Allocate memory using [`hal_malloc()`] for storing item value in
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

    /// Get the item's name.
    fn name(&self) -> &str;

    /// Get a mutable pointer to underlying shared memory storing this item's value.
    fn storage_mut(&self) -> Result<&mut Self::Storage, StorageError>;

    /// Get a reference to the underlying shared memory storing the item's value.
    fn storage(&self) -> Result<&Self::Storage, StorageError>;

    fn register(full_resource_name: &str, component_id: i32)
        -> Result<Self, ResourceRegisterError>;
}

/// Readable resource trait
///
/// Implemented for any resource that can only be read by a component
pub trait ResourceRead: HalResource {
    /// Get the value of the resource
    fn value(&self) -> Result<&<Self as HalResource>::Storage, StorageError> {
        self.storage()
    }
}

/// Writable resource trait
///
/// Implemented for any resource that can be only written to by a component
pub trait ResourceWrite: HalResource {
    /// Set the value of the resource
    fn set_value(&self, value: <Self as HalResource>::Storage) -> Result<(), StorageError> {
        let storage = self.storage_mut()?;

        *storage = value;

        Ok(())
    }
}
