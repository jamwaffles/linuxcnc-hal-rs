//! Error handling example

use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::convert::TryInto;
use std::{ffi::CString, mem, thread, time::Duration};

fn main() {
    unsafe {
        let name = CString::new("pins").unwrap();

        let ret = hal_init(name.as_ptr().cast());

        // Check that component was created successfully
        let component_id = match ret {
            x if x == -(EINVAL as i32) => panic!("Failed to initialise component"),
            x if x == -(ENOMEM as i32) => panic!("Not enough memory to initialise component"),
            id if id > 0 => id,
            code => unreachable!("Hit unreachable error code {}", code),
        };

        println!("Component registered with ID {}", component_id);

        let mut signals =
            Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT]).unwrap();

        let storage = hal_malloc(mem::size_of::<*mut f64>().try_into().unwrap()) as *mut *mut f64;

        if storage.is_null() {
            panic!("Failed to allocate storage");
        }

        let pin_name = CString::new("pins.input-1").unwrap();

        let ret = hal_pin_float_new(
            pin_name.as_ptr().cast(),
            hal_pin_dir_t_HAL_IN,
            storage,
            component_id,
        );

        // Check that pin was registered successfully
        match ret {
            0 => println!("Pin registered successfully"),
            x if x == -(EINVAL as i32) => panic!("Failed to register pin"),
            x if x == -(EPERM as i32) => {
                panic!("HAL is locked. Register pins before calling hal_ready()`")
            }
            x if x == -(ENOMEM as i32) => panic!("Failed to register pin"),
            code => unreachable!("Hit unreachable error code {}", code),
        }

        let ret = hal_ready(component_id);

        // Check that component is ready
        match ret {
            0 => println!("Component is ready"),
            x if x == -(EINVAL as i32) => panic!("HAL component was not found or is already ready"),
            code => unreachable!("Hit unreachable error code {}", code),
        }

        while !signals.pending().any(|signal| match signal {
            signal_hook::consts::SIGTERM
            | signal_hook::consts::SIGINT
            | signal_hook::consts::SIGKILL => true,
            _ => false,
        }) {
            println!("Input {:?}", **storage);

            thread::sleep(Duration::from_millis(500));
        }
    }
}
