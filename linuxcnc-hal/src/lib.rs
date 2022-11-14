//! A safe, high-level interface to LinuxCNC's HAL (Hardware Abstraction Layer) module.
//!
//! For lower level, unsafe use, see the
//! [`linuxcnc-hal-sys`](https://crates.io/crates/linuxcnc-hal-sys) crate.
//!
//! # Development setup
//!
//! [`bindgen`](https://github.com/rust-lang/rust-bindgen) must be set up correctly. Follow the
//! [requirements section of its docs](https://rust-lang.github.io/rust-bindgen/requirements.html).
//!
//! To run and debug any HAL components, the LinuxCNC simulator can be set up. There's a guide
//! [here](https://wapl.es/cnc/2020/01/25/linuxcnc-simulator-build-linux-mint.html) for Linux Mint
//! (and other Debian derivatives).
//!
//! # Project setup
//!
//! This crate depends on the `linuxcnc-hal-sys` crate which requires the `LINUXCNC_SRC` environment
//! variable to be set to correctly generate the C bindings. The value must be the absolute path to
//! the root of the LinuxCNC source code.
//!
//! **The version of the LinuxCNC sources must match the LinuxCNC version used in the machine
//! control.**
//!
//! ```bash
//! # Clone LinuxCNC source code into linuxcnc/
//! git clone https://github.com/LinuxCNC/linuxcnc.git
//!
//! # Check out a specific version tag. This may also be a commit, but must match the version in use by the machine control.
//! cd linuxcnc && git checkout v2.8.0 && cd ..
//!
//! # Create your component lib
//! cargo new --lib my_comp
//!
//! cd my_comp
//!
//! # Add LinuxCNC HAL bindings as a Cargo dependency with cargo-edit
//! cargo add linuxcnc-hal
//!
//! LINUXCNC_SRC=/path/to/linuxcnc/source/code cargo build
//! ```
//!
//! If LinuxCNC is configured to run in place, `liblinuxcnchal.so.0` may not be found on startup. To
//! fix, try setting the library path with e.g. `export LD_LIBRARY_PATH=~/Repositories/linuxcnc/lib`
//!
//! # Examples
//!
//! ## Create a component with input and output
//!
//! This example creates a component called `"pins"` with a single input (`"input-1"`) and output
//! pin (`"output-1"`). It enters an infinite loop which updates the value of `output-1` every
//! second. LinuxCNC convention dictates that component and pin names should be `dash-cased`.
//!
//! This example can be loaded into LinuxCNC with a `.hal` file that looks similar to this:
//!
//! ```hal
//! loadusr -W /path/to/your/component/target/debug/comp_bin_name
//! net input-1 spindle.0.speed-out pins.input-1
//! net output-1 pins.output-1
//! ```
//!
//! Pins and other resources are registered using the [`Resources`] trait. This example creates a
//! `Pins` struct which holds the two pins. [`HalComponent::new`] handles component creation,
//! resources (pin, signal, etc) initialisation and UNIX signal handler registration.
//!
//! ```rust,no_run
//! use linuxcnc_hal::{
//!     error::PinRegisterError,
//!     hal_pin::{InputPin, OutputPin},
//!     prelude::*,
//!     HalComponent, RegisterResources, Resources,
//! };
//! use std::{
//!     error::Error,
//!     thread,
//!     time::{Duration, Instant},
//! };
//!
//! struct Pins {
//!     input_1: InputPin<f64>,
//!     output_1: OutputPin<f64>,
//! }
//!
//! impl Resources for Pins {
//!     type RegisterError = PinRegisterError;
//!
//!     fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
//!         Ok(Pins {
//!             input_1: comp.register_pin::<InputPin<f64>>("input-1")?,
//!             output_1: comp.register_pin::<OutputPin<f64>>("output-1")?,
//!         })
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     rtapi_logger::init();
//!
//!     // Create a new HAL component called `rust-comp`
//!     let comp: HalComponent<Pins> = HalComponent::new("rust-comp")?;
//!
//!     // Get a reference to the `Pins` struct
//!     let pins = comp.resources();
//!
//!     let start = Instant::now();
//!
//!     // Main control loop
//!     while !comp.should_exit() {
//!         let time = start.elapsed().as_secs() as i32;
//!
//!         // Set output pin to elapsed seconds since component started
//!         pins.output_1.set_value(time.into())?;
//!
//!         // Print the current value of the input pin
//!         println!("Input: {:?}", pins.input_1.value());
//!
//!         // Sleep for 1000ms. This should be a lower time if the component needs to update more
//!         // frequently.
//!         thread::sleep(Duration::from_millis(1000));
//!     }
//!
//!     // The custom implementation of `Drop` for `HalComponent` ensures that `hal_exit()` is called
//!     // at this point. Registered signal handlers are also deregistered.
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate log;

mod component;
pub mod error;
mod hal_parameter;
pub mod hal_pin;
pub mod prelude;

use hal_parameter::ParameterPermissions;

pub use crate::component::HalComponent;
pub use crate::hal_parameter::Parameter;
use crate::{
    error::{ParameterRegisterError, PinRegisterError, ResourcesError},
    hal_parameter::HalParameter,
    hal_pin::HalPin,
};

/// Resources for a component
pub trait Resources: Sized {
    /// The type of error to return if a resource registration failed
    ///
    /// This must be convertable into a [`ResourcesError`].
    type RegisterError: Into<ResourcesError>;

    /// Register resources against a component
    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError>;
}

/// Component metadata used when registering resources
pub struct RegisterResources {
    /// Component name
    name: &'static str,

    /// Component ID
    id: i32,
}

impl RegisterResources {
    /// Register a pin with this component.
    ///
    /// The pin name will be prefixed with the component name
    pub fn register_pin<P>(&self, pin_name: &'static str) -> Result<P, PinRegisterError>
    where
        P: HalPin,
    {
        let full_name = format!("{}.{}", self.name, pin_name);

        let pin = P::register(&full_name, self.id)?;

        Ok(pin)
    }

    /// Register a read/write parameter with this component.
    ///
    /// The parameter name will be prefixed with the component name.
    ///
    /// To register a pin that LinuxCNC cannot write to, call [`RegisterResources::register_readonly_parameter`].
    pub fn register_parameter<P>(
        &self,
        parameter_name: &'static str,
    ) -> Result<P, ParameterRegisterError>
    where
        P: HalParameter,
    {
        let full_name = format!("{}.{}", self.name, parameter_name);

        let parameter = P::register(&full_name, self.id, ParameterPermissions::ReadWrite)?;

        Ok(parameter)
    }

    /// Register a read only parameter with this component.
    ///
    /// The parameter name will be prefixed with the component name
    pub fn register_readonly_parameter<P>(
        &self,
        parameter_name: &'static str,
    ) -> Result<P, ParameterRegisterError>
    where
        P: HalParameter,
    {
        let full_name = format!("{}.{}", self.name, parameter_name,);

        let parameter = P::register(&full_name, self.id, ParameterPermissions::ReadOnly)?;

        Ok(parameter)
    }
}
