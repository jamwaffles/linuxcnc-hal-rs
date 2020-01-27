//! This crate uses [`bindgen`] to create bindings to the LinuxCNC HAL module.
//!
//! The high level, safe interface at [`linuxcnc-hal`] is recommended for user code.
//!
//! # Examples
//!
//! All functions exported from this crate are `unsafe`, hence each example is wrapped in a big
//! `unsafe` block for clarity.
//!
//! The LinuxCNC HAL requires a certain setup procedure to operate correctly. The basic program
//! structure should be roughly as follows
//!
//! 1. Call [`hal_init`] to create a new HAL component
//! 1. Register `SIGTERM` and `SIGINT` signals, likely with the [`signal_hook`] crate. LinuxCNC will
//! hang if these signals are not registered.
//! 1. Register pins with [`hal_pin_float_new`], [`hal_pin_u32_new`], etc
//! 1. Call [`hal_ready`] to signal to LinuxCNC that the component is ready
//! 1. Enter an infinite loop to continuously update input/output pin values and perform component
//! logic
//!
//! These examples can be loaded into LinuxCNC using a HAL file similar to this:
//!
//! ```text
//! loadusr -W /path/to/your/component/target/debug/comp_bin_name
//! net input-1 spindle.0.speed-out pins.input_1
//! ```
//!
//! ## Create an input pin
//!
//! This example creates a component called `pins` and registers an input pin to it that accepts a
//! floating point value using [`hal_pin_float_new`]. Each HAL pin requires some memory allocated to
//! store its value which is performed with [`hal_malloc`].
//!
//! The example can be loaded into LinuxCNC using a HAL file similar to this:
//!
//! **Note that there is no error handling in this example for brevity.**
//!
//! ```rust,no_run
//! use linuxcnc_hal_sys::*;
//! use signal_hook::iterator::Signals;
//! use std::ffi::CString;
//! use std::mem;
//! use std::thread;
//! use std::time::Duration;
//!
//! unsafe {
//!     let id = hal_init(CString::new("pins").unwrap().as_ptr() as *const i8);
//!
//!     println!("ID {}", id);
//!
//!     let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();
//!
//!     let storage = hal_malloc(mem::size_of::<f64>() as i64) as *mut *mut f64;
//!
//!     println!("Storage {:?}", storage);
//!
//!     let pin_name = CString::new("pins.input_1").unwrap();
//!
//!     let ret = hal_pin_float_new(
//!         pin_name.as_ptr() as *const i8,
//!         hal_pin_dir_t_HAL_IN,
//!         storage,
//!         id,
//!     );
//!
//!     println!("Pin init {}", ret);
//!
//!     let ret = hal_ready(id);
//!
//!     println!("Ready {}", ret);
//!
//!     while !signals.pending().any(|signal| match signal {
//!         signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
//!         _ => false,
//!     }) {
//!         println!("Input {:?}", **storage);
//!
//!         thread::sleep(Duration::from_millis(500));
//!     }
//! }
//! ```
//!
//! ## Error handling
//!
//! Errors are handled in this crate the same way as in the C code. Some consts are exported like
//! [`EINVAL`] and [`EPERM`] to allow matching of returned error codes.
//!
//! ```rust,no_run
//! use linuxcnc_hal_sys::*;
//! use signal_hook::iterator::Signals;
//! use std::ffi::CString;
//! use std::mem;
//! use std::thread;
//! use std::time::Duration;
//!
//! unsafe {
//!     let ret = hal_init(CString::new("pins").unwrap().as_ptr() as *const i8);
//!
//!     // Check that component was created successfully
//!     let component_id = match ret {
//!         x if x == -(EINVAL as i32) => panic!("Failed to initialise component"),
//!         x if x == -(ENOMEM as i32) => panic!("Not enough memory to initialise component"),
//!         id if id > 0 => id,
//!         code => unreachable!("Hit unreachable error code {}", code),
//!     };
//!
//!     println!("Component registered with ID {}", component_id);
//!
//!     let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();
//!
//!     let storage = hal_malloc(mem::size_of::<f64>() as i64) as *mut *mut f64;
//!
//!     if storage.is_null() {
//!         panic!("Failed to allocate storage");
//!     }
//!
//!     let pin_name = CString::new("pins.input_1").unwrap();
//!
//!     let ret = hal_pin_float_new(
//!         pin_name.as_ptr() as *const i8,
//!         hal_pin_dir_t_HAL_IN,
//!         storage,
//!         component_id,
//!     );
//!
//!     // Check that pin was registered successfully
//!     match ret {
//!         0 => println!("Pin registered successfully"),
//!         x if x == -(EINVAL as i32) => panic!("Failed to register pin"),
//!         x if x == -(EPERM as i32) => {
//!             panic!("HAL is locked. Register pins before calling hal_ready()`")
//!         }
//!         x if x == -(ENOMEM as i32) => panic!("Failed to register pin"),
//!         code => unreachable!("Hit unreachable error code {}", code),
//!     }
//!
//!     let ret = hal_ready(component_id);
//!
//!     // Check that component is ready
//!     match ret {
//!         0 => println!("Component is ready"),
//!         x if x == -(EINVAL as i32) => panic!("HAL component was not found or is already ready"),
//!         code => unreachable!("Hit unreachable error code {}", code),
//!     }
//!
//!     while !signals.pending().any(|signal| match signal {
//!         signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
//!         _ => false,
//!     }) {
//!         println!("Input {:?}", **storage);
//!
//!         thread::sleep(Duration::from_millis(500));
//!     }
//! }
//! ```
//! [`linuxcnc-hal`]: https://docs.rs/linuxcnc-hal
//! [`bindgen`]: https://docs.rs/bindgen
//! [`signal_hook`]: https://docs.rs/signal_hook

#![deny(intra_doc_link_resolution_failure)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod check_readme;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
