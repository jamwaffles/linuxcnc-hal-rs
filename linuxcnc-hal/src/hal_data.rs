//! HAL data trait

/// HAL data trait
///
/// Implemented automatically by the `linuxcnc_derive::Hal` proc macro or manually, this trait
/// allows a struct to be used to store multiple HAL resources in the same item.
pub trait HalData {
    /// DELETE ME
    fn test_fn(&self);
}
