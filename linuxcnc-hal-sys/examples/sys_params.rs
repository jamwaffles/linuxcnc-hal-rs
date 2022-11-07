//! Parameters
//!
//! ```
//! loadusr -W /path/to/linuxcnc-hal-rs/target/debug/examples/sys_params
//! setp params.param-float 1.234
//! setp params.param-uint 321
//! ```

use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::convert::TryInto;
use std::{ffi::CString, mem, thread, time::Duration};

fn main() {
    unsafe {
        // The name of this component MUST be the same name as the binary, or LinuxCNC won't pick it
        // up as ready for some reason.
        let name = CString::new("sys_params").unwrap();

        let id = hal_init(name.as_ptr().cast());

        println!("ID {}", id);

        let mut signals =
            Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT]).unwrap();

        let float_storage = hal_malloc(mem::size_of::<f64>().try_into().unwrap()) as *mut f64;
        let uint_storage = hal_malloc(mem::size_of::<u32>().try_into().unwrap()) as *mut u32;

        println!("Float storage {:?}", float_storage);
        println!("Int storage {:?}", uint_storage);

        let float_name = CString::new("params.param-float").unwrap();

        let float = hal_param_float_new(
            float_name.as_ptr().cast(),
            hal_param_dir_t_HAL_RW,
            float_storage,
            id,
        );

        let uint_name = CString::new("params.param-uint").unwrap();

        let uint = hal_param_u32_new(
            uint_name.as_ptr().cast(),
            hal_param_dir_t_HAL_RW,
            uint_storage,
            id,
        );

        println!("Pin float init {}", float);
        println!("Pin int init {}", uint);

        let ret = hal_ready(id);

        println!("Ready {}", ret);

        while !signals.pending().any(|signal| {
            matches!(
                signal,
                signal_hook::consts::SIGTERM
                    | signal_hook::consts::SIGINT
                    | signal_hook::consts::SIGKILL
            )
        }) {
            println!("Float {:?}", *float_storage);
            println!("Int {:?}", *uint_storage);

            thread::sleep(Duration::from_millis(500));
        }
    }
}
