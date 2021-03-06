//! Periodically print the f32 value of an input pin
//!
//! There is absolutely no error checking in this example - use at your own risk!
//!
//! Hook this up to your LinuxCNC instance (simulator or no) with something like the following:
//!
//! ```
//! loadusr -W /path/to/linuxcnc-hal-rs/target/debug/examples/pins
//! net input-1 spindle.0.speed-out pins.input_1
//! ```

use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::convert::TryInto;
use std::{ffi::CString, mem, thread, time::Duration};

fn main() {
    unsafe {
        let name = CString::new("pins").unwrap();

        let id = hal_init(name.as_ptr().cast());

        println!("ID {}", id);

        let mut signals =
            Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT]).unwrap();

        let storage = hal_malloc(mem::size_of::<*mut f64>().try_into().unwrap()) as *mut *mut f64;

        println!("Storage {:?}", storage);

        let pin_name = CString::new("pins.input-1").unwrap();

        let ret = hal_pin_float_new(pin_name.as_ptr().cast(), hal_pin_dir_t_HAL_IN, storage, id);

        println!("Pin init {}", ret);

        let ret = hal_ready(id);

        println!("Ready {}", ret);

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
