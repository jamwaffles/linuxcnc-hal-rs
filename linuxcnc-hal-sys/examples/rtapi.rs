//! Realtime testing
//!
//! A thread with some tidbits that helped develop this example: https://forum.linuxcnc.org/24-hal-components/40339-developing-hal-component-in-c#186347
//!
//! Build with e.g. `LINUXCNC_SRC=$(realpath ../linuxcnc) cargo build --package linuxcnc-hal-sys --example rtapi`
//!
//! The binary must be available in the LinuxCNC source `/rtlib` folder. A kludge is to symlink it:
//!
//! ```bash
//! ln -s /home/pi/Repositories/linuxcnc-hal-rs/target/debug/examples/librtapi.so /home/pi/Repositories/linuxcnc/rtlib/
//! ```
//!
//! Run with `./linuxcnc/scripts/halrun -I -V -f ./linuxcnc-hal-rs/linuxcnc-hal-sys/examples/rtapi.hal`.

use linuxcnc_hal_sys::*;
use signal_hook::iterator::Signals;
use std::alloc::{alloc, GlobalAlloc, Layout};
use std::ptr::null_mut;
use std::{convert::TryInto, ffi::c_void, os::raw::c_long};
use std::{ffi::CString, mem, thread, time::Duration};

/// Component ID accessible from both `rtapi_app_main` and `hal_exit`.
static mut COMP_ID: i32 = 0;

/// Args that get passed to the function when called
#[repr(C)]
#[derive(Debug)]
struct TestArgs {
    foo: u32,
    bar: bool,
    arr: [u8; 5],
}

/// Component entry point.
///
/// LinuxCNC's HAL guidelines strongly suggest only allocating in here, and not in any handler
/// functions exported by `hal_export_funct`.
///
/// This is called by LinuxCNC and must have the name `rtapi_app_main`.
#[no_mangle]
pub unsafe extern "C" fn rtapi_app_main() -> i32 {
    rtapi_logger::init().unwrap();

    let name = CString::new("librtapi").unwrap();

    let id = hal_init(name.as_ptr().cast());

    log::info!("Component ID {}", id);

    COMP_ID = id;

    // Register a function that gets called in the realtime context.
    let export_result = {
        let ptr_size = mem::size_of::<TestArgs>().try_into().unwrap();

        // Allocate data to be used in the realtime callback function `test_fn`. This MUST be
        // allocated using `hal_malloc` otherwise it will be placed outside the realtime shared
        // memory region.
        let mut arg = hal_malloc(ptr_size) as *mut TestArgs;

        *arg = TestArgs {
            foo: 1234,
            bar: true,
            arr: [10, 11, 12, 13, 14],
        };

        let arg_ptr = arg as *mut c_void;

        // The fn name here is what is used in `addf ...` calls, etc. The actual function name
        // doesn't matter.
        let fn_name = CString::new("librtapi.rtapi-fn").unwrap();

        hal_export_funct(
            fn_name.as_ptr().cast(),
            Some(test_fn),
            arg_ptr,
            // If in doubt, set to true (uses FP flag)
            true as i32,
            // Is not reentrant
            false as i32,
            COMP_ID,
        )
    };

    if export_result != 0 {
        eprintln!("Failed to export fn {}", export_result);
        return export_result;
    }

    let storage = hal_malloc(mem::size_of::<*mut f64>().try_into().unwrap()) as *mut *mut f64;
    let pin_name = CString::new("pins.input-1").unwrap();
    let ret = hal_pin_float_new(pin_name.as_ptr().cast(), hal_pin_dir_t_HAL_IN, storage, id);

    let ret = hal_ready(id);

    log::info!("Component ID {} is ready: {}", COMP_ID, ret);

    ret
}

/// Handler function called from the realtime thread.
///
/// Try not to allocate in there as nothing will be freed until all components have exited.
#[no_mangle]
pub unsafe extern "C" fn test_fn(arg: *mut c_void, period: c_long) {
    let arg: &mut TestArgs = &mut *(arg as *mut TestArgs);
    let period: i64 = period.into();

    log::debug!("Test fn {:?}, {}", arg, period);
}

/// Exit function.
///
/// This is called by LinuxCNC and must have the name `rtapi_app_exit`.
#[no_mangle]
pub unsafe extern "C" fn rtapi_app_exit() -> i32 {
    println!("Exiting...");

    let code = hal_exit(COMP_ID);

    log::debug!("Exit code {}", code);

    code
}
