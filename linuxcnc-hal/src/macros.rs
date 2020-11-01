macro_rules! impl_resource {
    ($ty:ident, $storage:ty, $register_body:tt) => {
        impl $crate::hal_resource::HalResource for $ty<$storage> {
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

            #[allow(clippy::redundant_closure_call)]
            fn register(
                full_resource_name: &str,
                component_id: i32,
            ) -> Result<Self, $crate::error::ResourceRegisterError> {
                if full_resource_name.len() > linuxcnc_hal_sys::HAL_NAME_LEN as usize {
                    return Err($crate::error::ResourceRegisterError::NameLength);
                }

                let full_resource_name_ffi =
                    std::ffi::CString::new(full_resource_name).map_err(|e| {
                        log::error!("Failed to convert name to C string: {}", e);

                        $crate::error::ResourceRegisterError::NameConversion
                    })?;

                let full_resource_name_ffi = full_resource_name_ffi.as_c_str();

                log::debug!("Full resource name {:?}", full_resource_name);

                let storage = Self::allocate_storage()
                    .map_err($crate::error::ResourceRegisterError::Storage)?;

                $register_body(
                    full_resource_name,
                    full_resource_name_ffi,
                    storage,
                    component_id,
                )
            }
        }
    };
}

macro_rules! impl_pin {
    ($ty:ident, $storage:ty, $hal_fn:expr, $direction:expr) => {
        impl_resource!(
            $ty,
            $storage,
            (|full_resource_name: &str,
              full_resource_name_ffi: &std::ffi::CStr,
              storage: *mut *mut $storage,
              component_id: i32|
             -> Result<Self, $crate::error::ResourceRegisterError> {
                let ret = unsafe {
                    $hal_fn(
                        full_resource_name_ffi.as_ptr() as *const std::os::raw::c_char,
                        $direction as i32,
                        storage,
                        component_id,
                    )
                };

                match ret {
                    x if x == -(linuxcnc_hal_sys::EINVAL as i32) => {
                        Err($crate::error::ResourceRegisterError::Invalid)
                    }
                    x if x == -(linuxcnc_hal_sys::EPERM as i32) => {
                        Err($crate::error::ResourceRegisterError::LockedHal)
                    }
                    x if x == -(linuxcnc_hal_sys::ENOMEM as i32) => {
                        Err($crate::error::ResourceRegisterError::Memory)
                    }
                    0 => {
                        debug!("Make pin {} returned {}", full_resource_name, ret);

                        Ok(Self {
                            name: full_resource_name.to_string(),
                            storage,
                        })
                    }
                    code => unreachable!("Hit unreachable error code {}", code),
                }
            })
        );

        impl $crate::hal_pin::HalPin for $ty<$storage> {}
    };
}

macro_rules! impl_param {
    ($ty:ident, $storage:ty, $hal_fn:expr, $direction:expr) => {
        impl_resource!(
            $ty,
            $storage,
            (|full_resource_name: &str,
              full_resource_name_ffi: &std::ffi::CStr,
              storage: *mut *mut $storage,
              component_id: i32|
             -> Result<Self, $crate::error::ResourceRegisterError> {
                let ret = unsafe {
                    $hal_fn(
                        full_resource_name_ffi.as_ptr() as *const std::os::raw::c_char,
                        $direction as u32,
                        *storage,
                        component_id,
                    )
                };

                match ret {
                    x if x == -(linuxcnc_hal_sys::EINVAL as i32) => {
                        Err($crate::error::ResourceRegisterError::Invalid)
                    }
                    x if x == -(linuxcnc_hal_sys::EPERM as i32) => {
                        Err($crate::error::ResourceRegisterError::LockedHal)
                    }
                    x if x == -(linuxcnc_hal_sys::ENOMEM as i32) => {
                        Err($crate::error::ResourceRegisterError::Memory)
                    }
                    0 => {
                        debug!("Make param {} returned {}", full_resource_name, ret);

                        Ok(Self {
                            name: full_resource_name.to_string(),
                            storage,
                        })
                    }
                    code => unreachable!("Hit unreachable error code {}", code),
                }
            })
        );

        impl $crate::hal_parameter::HalParameter for $ty<$storage> {}
    };
}
