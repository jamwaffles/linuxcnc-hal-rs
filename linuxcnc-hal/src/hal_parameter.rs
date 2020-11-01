//! HAL component parameters.

use crate::hal_resource::{HalResource, ResourceRead, ResourceWrite};
use linuxcnc_hal_sys::{
    hal_param_bit_new, hal_param_dir_t_HAL_RO as HAL_RO, hal_param_dir_t_HAL_RW as HAL_RW,
    hal_param_float_new, hal_param_s32_new, hal_param_u32_new,
};

/// HAL parameter marker trait.
pub trait HalParameter: HalResource {}

/// Parameter direction.
pub enum ParameterDirection {
    /// Readonly parameter.
    Readonly = HAL_RO as isize,

    /// Read/write parameter.
    ReadWrite = HAL_RW as isize,
}

/// Readonly parameter.
#[derive(Debug)]
pub struct ReadonlyParameter<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut *mut S,
}

impl<S> Drop for ReadonlyParameter<S> {
    fn drop(&mut self) {
        debug!("Drop ReadonlyParameter {}", self.name);
    }
}

/// Read/write parameter.
#[derive(Debug)]
pub struct ReadWriteParameter<S> {
    pub(crate) name: String,
    pub(crate) storage: *mut *mut S,
}

impl<S> Drop for ReadWriteParameter<S> {
    fn drop(&mut self) {
        debug!("Drop ReadWriteParameter {}", self.name);
    }
}

impl_param!(
    ReadonlyParameter,
    f64,
    hal_param_float_new,
    ParameterDirection::Readonly
);
impl_param!(
    ReadonlyParameter,
    u32,
    hal_param_u32_new,
    ParameterDirection::Readonly
);
impl_param!(
    ReadonlyParameter,
    i32,
    hal_param_s32_new,
    ParameterDirection::Readonly
);
impl_param!(
    ReadonlyParameter,
    bool,
    hal_param_bit_new,
    ParameterDirection::Readonly
);

impl_param!(
    ReadWriteParameter,
    f64,
    hal_param_float_new,
    ParameterDirection::ReadWrite
);
impl_param!(
    ReadWriteParameter,
    u32,
    hal_param_u32_new,
    ParameterDirection::ReadWrite
);
impl_param!(
    ReadWriteParameter,
    i32,
    hal_param_s32_new,
    ParameterDirection::ReadWrite
);
impl_param!(
    ReadWriteParameter,
    bool,
    hal_param_bit_new,
    ParameterDirection::ReadWrite
);

impl ResourceRead for ReadonlyParameter<f64> {}
impl ResourceRead for ReadonlyParameter<u32> {}
impl ResourceRead for ReadonlyParameter<i32> {}
impl ResourceRead for ReadonlyParameter<bool> {}

impl ResourceRead for ReadWriteParameter<f64> {}
impl ResourceRead for ReadWriteParameter<u32> {}
impl ResourceRead for ReadWriteParameter<i32> {}
impl ResourceRead for ReadWriteParameter<bool> {}
impl ResourceWrite for ReadWriteParameter<f64> {}
impl ResourceWrite for ReadWriteParameter<u32> {}
impl ResourceWrite for ReadWriteParameter<i32> {}
impl ResourceWrite for ReadWriteParameter<bool> {}
