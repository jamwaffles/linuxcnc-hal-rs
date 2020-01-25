use linuxcnc_hal_sys::{hal_exit, hal_init, hal_ready};

pub struct HalComponent {
    /// Component name
    ///
    /// Examples:
    ///
    /// * `wj200_vfd`
    /// * `hy_vfd`
    name: &'static str,

    /// Component ID
    id: i32,
}

impl HalComponent {
    /// Create a new HAL component and begin initialisation
    ///
    /// This calls [`hal_init`] under the hood. Do any init work between calling this function and
    /// [`ready()`]. The component name must be unique, and be no longer than [`HAL_NAME_LEN`].
    pub fn new(name: &'static str) -> Result<Self, ()> {
        let id = unsafe { hal_init(name.as_ptr() as *const i8) };

        println!("Init component {} with ID {}", name, id);

        Ok(Self { name, id })
    }

    /// Mark the component as ready
    pub fn ready(&mut self) -> Result<(), ()> {
        unsafe { hal_ready(self.id) };

        println!("Component is ready");

        Ok(())
    }
}

impl Drop for HalComponent {
    fn drop(&mut self) {
        println!("Closing component ID {}, name {}", self.id, self.name);

        unsafe {
            hal_exit(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
