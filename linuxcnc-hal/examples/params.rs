//! Create a component called `rust-comp` that has one input and one output pin.
//!
//! Parameter names:
//!
//! * Read only `params.ro`
//! * Read write `params.rw`
//!
//! The component can be loaded and connected using something like the following `.hal` file:
//!
//! ```ini
//! loadusr -Wn rust-comp /path/to/hal/params
//! setp params.ro 1.1234
//! setp params.rw 321
//! ```

use linuxcnc_hal::{
    error::ParameterRegisterError, HalComponent, Parameter, RegisterResources, Resources,
};
use std::{error::Error, thread, time::Duration};

struct Comp {
    /// LinuxCNC can only read this parameter.
    ro: Parameter<f64>,

    /// LinuxCNC can read and set this parameter.
    rw: Parameter<u32>,
}

impl Resources for Comp {
    type RegisterError = ParameterRegisterError;

    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
        Ok(Comp {
            ro: comp.register_readonly_parameter("ro")?,
            rw: comp.register_parameter("rw")?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    rtapi_logger::init().ok();

    // Create a new HAL component called `rust-comp`
    let comp: HalComponent<Comp> = HalComponent::new("params")?;

    // Get a reference to the `Comp` struct
    let resources = comp.resources();

    // Main control loop
    while !comp.should_exit() {
        resources.ro.set_value(1.234)?;

        println!("RW: {:?}", resources.rw.value());

        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}
