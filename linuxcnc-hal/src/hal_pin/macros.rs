macro_rules! impl_pin {
    ($type:ident, $type_str:expr, $hal_fn:expr, $hal_fn_str:expr, $storage:ty, $storage_str:expr) => {

        #[doc = $type_str]
        #[doc = "HAL pin."]
        #[doc = ""]
        #[doc = "Equivalent to the [`"]
        #[doc = $hal_fn_str]
        #[doc = "`] HAL function, backed internally by a [`"]
        #[doc = $storage_str]
        #[doc = "`]"]
        #[derive(Debug, PartialEq)]
        pub struct $type {
            name: String,
            storage: *mut *mut $storage,
        }

        impl $crate::hal_pin::HalPin for $type {
            type Storage = $storage;

            fn name(&self) -> &str {
                &self.name
            }

            fn storage(&self) -> Result<&mut Self::Storage, $crate::ComponentError> {
                if self.storage.is_null() {
                    Err($crate::ComponentError::Unknown("Value pointer is null"))
                } else {
                    Ok(unsafe { &mut **self.storage })
                }
            }

            fn register_pin(
                full_pin_name: &str,
                direction: $crate::hal_pin::PinDirection,
                component_id: i32,
            ) -> Result<Self, $crate::ComponentError> {
                if full_pin_name.len() > linuxcnc_hal_sys::HAL_NAME_LEN as usize {
                    return Err($crate::ComponentError::Unknown("Pin name is too long"));
                }

                let storage = Self::allocate_storage().map_err(|_| {
                    $crate::ComponentError::Unknown("Failed to allocate storage in HAL shared memory")
                })?;

                let ret = unsafe {
                    $hal_fn(
                        full_pin_name.as_ptr() as *const i8,
                        direction as i32,
                        storage,
                        component_id,
                    )
                };

                match ret {
                    x if x == -(linuxcnc_hal_sys::EINVAL as i32) => {
                        Err($crate::ComponentError::Unknown("Failed to create pin"))
                    }
                    x if x == -(linuxcnc_hal_sys::EPERM as i32) => Err($crate::ComponentError::Unknown("HAL is locked")),
                    x if x == -(linuxcnc_hal_sys::ENOMEM as i32) => {
                        Err($crate::ComponentError::Unknown("Insufficient memory for pin"))
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

        impl Drop for $type {
            fn drop(&mut self) {
                println!("Drop HalPinF64 {}", self.name);
            }
        }
    };
}
