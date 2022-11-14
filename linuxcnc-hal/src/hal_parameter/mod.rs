//! HAL parameters

#[macro_use]
mod macros;

mod parameter_trait;

use linuxcnc_hal_sys::{
    hal_param_bit_new, hal_param_dir_t_HAL_RO as HAL_RO, hal_param_dir_t_HAL_RW as HAL_RW,
    hal_param_float_new, hal_param_s32_new, hal_param_u32_new,
};
pub use parameter_trait::HalParameter;

/// Parameter write mode.
#[derive(Copy, Clone, Debug)]
pub enum ParameterPermissions {
    /// The parameter may only be read by LinuxCNC.
    ReadOnly = HAL_RO as isize,

    /// The parameter can be both read and written by LinuxCNC.
    ReadWrite = HAL_RW as isize,
}

/// A parameter that can be read and written by LinuxCNC
///
/// Supported parameter types are as follows
///
/// | Type              | Storage | Equivalent `linuxcnc_hal_sys` function |
/// | ------------------| ------- | -------------------------------------- |
/// | `Parameter<f64>`  | `f64`   | [`hal_param_float_new`]                  |
/// | `Parameter<u32>`  | `u32`   | [`hal_param_u32_new`]                    |
/// | `Parameter<i32>`  | `i32`   | [`hal_param_s32_new`]                    |
/// | `Parameter<bool>` | `bool`  | [`hal_param_bit_new`]                    |
///
/// # Examples
///
/// ## Create a parameter
///
/// This example creates an `Parameter` under `demo-component.named-parameter`.
///
/// ```rust,no_run
/// use linuxcnc_hal::{
///    error::ParameterRegisterError,
///    Parameter,
///    prelude::*,
///    HalComponent, RegisterResources, Resources,
/// };
/// use std::{
///    error::Error,
///    thread,
///    time::{Duration, Instant},
/// };
///
/// struct MyApp {
///    parameter: Parameter<f64>,
/// }
///
/// impl Resources for MyApp {
///    type RegisterError = ParameterRegisterError;
///
///    fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
///        Ok(MyApp {
///            parameter: comp.register_parameter("named-parameter")?,
///        })
///    }
/// }
///
/// fn main() -> Result<(), Box<dyn Error>> {
///    let comp: HalComponent<MyApp> = HalComponent::new("demo-component")?;
///
///    let MyApp { parameter } = comp.resources();
///
///    let start = Instant::now();
///
///     // Main control loop
///     while !comp.should_exit() {
///         parameter.set_value(123.45f64);
///         thread::sleep(Duration::from_millis(1000));
///     }
///
///    Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Parameter<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut S,
}

impl<S> Drop for Parameter<S> {
    fn drop(&mut self) {
        debug!("Drop Parameter {}", self.name);
    }
}

impl_param!(Parameter, f64, hal_param_float_new);
impl_param!(Parameter, u32, hal_param_u32_new);
impl_param!(Parameter, i32, hal_param_s32_new);
impl_param!(Parameter, bool, hal_param_bit_new);
