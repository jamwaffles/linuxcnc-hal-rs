//! Assign two input pins and store their references in a struct
//!
//! There is only minor error checking in this example - use at your own risk!

use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::convert::TryInto;
use std::{ffi::CString, mem, thread, time::Duration};

#[derive(Debug)]
struct HalData {
    d1: *mut f64,
    d2: *mut f64,
}

fn main() {
    unsafe {
        let id = hal_init(CString::new("struct").unwrap().as_ptr().cast());

        println!("ID {}", id);

        let mut signals =
            Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT]).unwrap();

        let hal_data = hal_malloc(mem::size_of::<HalData>().try_into().unwrap()) as *mut HalData;

        println!("Storage {:?}", hal_data);

        let pin_1_name = CString::new("struct.input-1").unwrap();
        let pin_2_name = CString::new("struct.input-2").unwrap();

        let ret = hal_pin_float_new(
            pin_1_name.as_ptr().cast(),
            hal_pin_dir_t_HAL_IN,
            &mut (*hal_data).d1,
            id,
        );

        if ret != 0 {
            panic!("Failed to initialise input-1");
        }

        let ret = hal_pin_float_new(
            pin_2_name.as_ptr().cast(),
            hal_pin_dir_t_HAL_IN,
            &mut (*hal_data).d2,
            id,
        );

        if ret != 0 {
            panic!("Failed to initialise input-2");
        }

        let ret = hal_ready(id);

        println!("Ready {}", ret);

        while !signals.pending().any(|signal| match signal {
            signal_hook::consts::SIGTERM
            | signal_hook::consts::SIGINT
            | signal_hook::consts::SIGKILL => true,
            _ => false,
        }) {
            // Uncomment the following two lines to inspect memory addresses
            // dbg!(hal_data);
            // dbg!(&*hal_data);

            // Print pin values with null pointer checks
            println!(
                "Pin values D1: {:?}, D2: {:?}",
                *(*hal_data).d1,
                *(*hal_data).d2
            );

            thread::sleep(Duration::from_millis(500));
        }
    }
}
