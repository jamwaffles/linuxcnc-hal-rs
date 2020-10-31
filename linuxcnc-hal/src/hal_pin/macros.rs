macro_rules! impl_pin {
    ($type:ident, $storage:ty, $hal_fn:expr, $direction:expr) => {
        impl $crate::hal_pin::HalPin for $type<$storage> {
            type Storage = $storage;

            fn name(&self) -> &str {
                &self.name
            }

            fn storage_mut(&self) -> Result<&mut Self::Storage, $crate::error::StorageError> {
                if self.storage.is_null() {
                    Err($crate::error::StorageError::Null)
                } else {
                    Ok(unsafe { &mut **self.storage })
                }
            }

            fn storage(&self) -> Result<&Self::Storage, $crate::error::StorageError> {
                if self.storage.is_null() {
                    Err($crate::error::StorageError::Null)
                } else {
                    Ok(unsafe { &**self.storage })
                }
            }

            fn register(
                full_pin_name: &str,
                component_id: i32,
            ) -> Result<Self, $crate::error::PinRegisterError> {
                if full_pin_name.len() > linuxcnc_hal_sys::HAL_NAME_LEN as usize {
                    return Err($crate::error::PinRegisterError::NameLength);
                }

                let full_pin_name_ffi = std::ffi::CString::new(full_pin_name).map_err(|e| {
                    log::error!("Failed to convert name to C string: {}", e);

                    $crate::error::PinRegisterError::NameConversion
                })?;

                let full_pin_name_ffi = full_pin_name_ffi.as_c_str();

                log::debug!("Full pin name {:?}", full_pin_name);

                let storage =
                    Self::allocate_storage().map_err($crate::error::PinRegisterError::Storage)?;

                let ret = unsafe {
                    $hal_fn(
                        full_pin_name_ffi.as_ptr() as *const std::os::raw::c_char,
                        $direction as i32,
                        storage,
                        component_id,
                    )
                };

                match ret {
                    x if x == -(linuxcnc_hal_sys::EINVAL as i32) => {
                        Err($crate::error::PinRegisterError::Invalid)
                    }
                    x if x == -(linuxcnc_hal_sys::EPERM as i32) => {
                        Err($crate::error::PinRegisterError::LockedHal)
                    }
                    x if x == -(linuxcnc_hal_sys::ENOMEM as i32) => {
                        Err($crate::error::PinRegisterError::Memory)
                    }
                    0 => {
                        debug!("Make pin {} returned {}", full_pin_name, ret);

                        Ok(Self {
                            name: full_pin_name.to_string(),
                            storage,
                        })
                    }
                    code => unreachable!("Hit unreachable error code {}", code),
                }
            }
        }
    };
}
