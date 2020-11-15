macro_rules! impl_param {
    ($type:ident, $storage:ty, $hal_fn:expr) => {
        impl $type<$storage> {
            /// Get the value of the parameter
            pub fn value(
                &self,
            ) -> Result<
                &<Self as $crate::hal_parameter::HalParameter>::Storage,
                $crate::error::StorageError,
            > {
                $crate::hal_parameter::HalParameter::storage(self)
            }

            /// Set the value of the parameter
            pub fn set_value(
                &self,
                value: <Self as $crate::hal_parameter::HalParameter>::Storage,
            ) -> Result<(), $crate::error::StorageError> {
                let storage = $crate::hal_parameter::HalParameter::storage_mut(self)?;

                *storage = value;

                Ok(())
            }
        }

        impl $crate::hal_parameter::HalParameter for $type<$storage> {
            type Storage = $storage;

            fn name(&self) -> &str {
                &self.name
            }

            fn storage_mut(&self) -> Result<&mut Self::Storage, $crate::error::StorageError> {
                if self.storage.is_null() {
                    Err($crate::error::StorageError::Null)
                } else {
                    Ok(unsafe { &mut *self.storage })
                }
            }

            fn storage(&self) -> Result<&Self::Storage, $crate::error::StorageError> {
                if self.storage.is_null() {
                    Err($crate::error::StorageError::Null)
                } else {
                    Ok(unsafe { &*self.storage })
                }
            }

            fn register(
                full_param_name: &str,
                component_id: i32,
                direction: ParameterPermissions,
            ) -> Result<Self, $crate::error::ParameterRegisterError> {
                if full_param_name.len() > linuxcnc_hal_sys::HAL_NAME_LEN as usize {
                    return Err($crate::error::ParameterRegisterError::NameLength);
                }

                let full_param_name_ffi = std::ffi::CString::new(full_param_name).map_err(|e| {
                    log::error!("Failed to convert name to C string: {}", e);

                    $crate::error::ParameterRegisterError::NameConversion
                })?;

                let full_param_name_ffi = full_param_name_ffi.as_c_str();

                log::debug!("Full pin name {:?}", full_param_name);

                let storage = Self::allocate_storage()
                    .map_err($crate::error::ParameterRegisterError::Storage)?;

                let ret = unsafe {
                    $hal_fn(
                        full_param_name_ffi.as_ptr() as *const std::os::raw::c_char,
                        direction as u32,
                        storage,
                        component_id,
                    )
                };

                match ret {
                    x if x == -(linuxcnc_hal_sys::EINVAL as i32) => {
                        Err($crate::error::ParameterRegisterError::Invalid)
                    }
                    x if x == -(linuxcnc_hal_sys::EPERM as i32) => {
                        Err($crate::error::ParameterRegisterError::LockedHal)
                    }
                    x if x == -(linuxcnc_hal_sys::ENOMEM as i32) => {
                        Err($crate::error::ParameterRegisterError::Memory)
                    }
                    0 => {
                        debug!("Make pin {} returned {}", full_param_name, ret);

                        Ok(Self {
                            name: full_param_name.to_string(),
                            storage,
                        })
                    }
                    code => unreachable!("Hit unreachable error code {}", code),
                }
            }
        }
    };
}
