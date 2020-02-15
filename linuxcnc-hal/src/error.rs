//! Error types

use linuxcnc_hal_sys::HAL_NAME_LEN;

/// Errors returned by LinuxCNC bindgen functions

/// Pointer error
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum StorageError {
    /// Pointer is null
    #[error("pointer is null")]
    Null,

    /// Pointer is not aligned
    #[error("pointer is not aligned")]
    Alignment,
}

/// Pin registration error
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PinRegisterError {
    /// Pin name is too long
    ///
    /// The maximum length is dictated by the [`HAL_NAME_LEN`] constant
    #[error("pin name is too long. Must be no longer than {} bytes", HAL_NAME_LEN)]
    NameLength,

    /// An error occurred allocating the HAL shared memory storage backing the pin
    #[error("failed to allocate shared memory storage for pin")]
    Storage(StorageError),

    /// An error occurred in the LinuxCNC HAL functions
    ///
    /// This variant is often returned when a HAL function returns [`EINVAL`]. This error code is
    /// returned for various different reasons. Check the LinuxCNC logs for error messages.
    #[error("HAL method returned invalid (EINVAL) status code")]
    Invalid,

    /// The HAL is locked
    ///
    /// Ensure that pins are registered **before** calling [`HalComponent::ready`].
    #[error("HAL is locked. Pins must be registered before call to HallComponent::ready")]
    LockedHal,

    /// There is not enough free memory available to allocate storage for this pin
    #[error("not enough free memory to allocate storage")]
    Memory,
}

/// HAL component initialisation error
#[derive(thiserror::Error, Debug)]
pub enum ComponentInitError {
    /// Component name is too long
    ///
    /// The maximum length is dictated by the [`HAL_NAME_LEN`] constant
    #[error(
        "component name is too long. Must be no longer than {} bytes",
        HAL_NAME_LEN
    )]
    NameLength,

    /// Component name could not be converted to C type
    #[error("component name cannot be converted to valid C string")]
    InvalidName,

    /// There is not enough free memory available to allocate storage for this pin
    #[error("not enough free memory to allocate storage")]
    Memory,

    /// Failed to register signal handlers
    #[error("failed to register signal handlers")]
    Signals(std::io::Error),

    /// Resource (pin, signal, etc) registration failed
    #[error("failed to register resources with component")]
    ResourceRegistration(PinRegisterError),

    /// An error occurred when initialising the component with [`hal_init`]
    #[error("failed to initialise component")]
    Init,

    /// An error occurred when calling [`hal_ready`] on the component
    #[error("failed to ready component")]
    Ready,
}
