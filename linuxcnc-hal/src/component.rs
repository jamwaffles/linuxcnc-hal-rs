use crate::{error::ComponentInitError, RegisterResources, Resources};
use linuxcnc_hal_sys::{hal_exit, hal_init, hal_ready, EINVAL, ENOMEM, HAL_NAME_LEN};
use signal_hook::iterator::Signals;
use std::ffi::CString;

/// HAL component
///
/// The main HAL component interface. See the [crate documentation](./index.html) for examples.
///
/// During registraton, all resource names are prefixed with the component name and a `.` full stop
/// character. For example, a component named `rust-comp` with a pin named `input-1` will show up in
/// LinuxCNC as a pin called `rust-comp.input-1`.
///
/// `HalComponent` has a custom `Drop` implementation which calls [`hal_exit`] (among other things)
/// when the variable holding the component goes out of scope. Due to this, the component should be
/// initialised in `main()` so it lives for the entire life of the program.
#[derive(Debug)]
pub struct HalComponent<R> {
    /// Component name
    name: &'static str,

    /// Component ID
    id: i32,

    /// Handles to Unix exit signals
    signals: Signals,

    /// Handle to resources (pins, signals, etc) used in the component
    ///
    /// This is an `Option` so that it can be `Drop`ped before the component itself is dropped.
    /// Resources references to shared memory in LinuxCNC's HAL must be freed before [`hal_exit`] is
    /// called.
    resources: Option<R>,
}

impl<R> HalComponent<R>
where
    R: Resources,
{
    /// Create a new HAL component
    ///
    /// `new` registers a new HAL component with LinuxCNC, registers the required UNIX signal
    /// handlers and allocates resources (pins, signals, etc) required by the component.
    pub fn new(name: &'static str) -> Result<Self, ComponentInitError> {
        let id = Self::create_component(name)?;

        let resources = R::register_resources(&RegisterResources { id, name })
            .map_err(|e| ComponentInitError::ResourceRegistration(e.into()))?;

        let signals = Self::register_signals()?;

        let comp = Self {
            name,
            id,
            resources: Some(resources),
            signals,
        };

        comp.ready()
    }

    /// Register signal handlers so component closes cleanly
    ///
    /// These are also required for the component to pass initialisation in LinuxCNC. If LinuxCNC
    /// hangs during starting waiting for the component to become ready, it might be due to signal
    /// handlers not being registered.
    fn register_signals() -> Result<Signals, ComponentInitError> {
        let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])
            .map_err(ComponentInitError::Signals)?;

        debug!("Signals registered");

        Ok(signals)
    }

    /// Create a HAL component
    ///
    /// # Errors
    ///
    /// * [`ComponentInitError::NameLength`] - If the component name is longer than [`HAL_NAME_LEN`]
    /// * [`ComponentInitError::InvalidName`] - If the component name cannot be converted to a
    ///   [`std::ffi::CString`]
    /// * [`ComponentInitError::Init`] - If the call to [`hal_init`] returned an [`EINVAL`] status
    /// * [`ComponentInitError::Memory`] - If there is not enough memory to allocate the component
    fn create_component(name: &'static str) -> Result<i32, ComponentInitError> {
        if name.len() > HAL_NAME_LEN as usize {
            error!(
                "Component name must be no longer than {} bytes",
                HAL_NAME_LEN
            );

            Err(ComponentInitError::NameLength)
        } else {
            let name_c = CString::new(name).map_err(|_| ComponentInitError::InvalidName)?;

            let id = unsafe { hal_init(name_c.as_ptr().cast()) };

            match id {
                x if x == -(EINVAL as i32) => Err(ComponentInitError::Init),
                x if x == -(ENOMEM as i32) => Err(ComponentInitError::Memory),
                id if id > 0 => {
                    debug!("Init component {} with ID {}", name, id);

                    Ok(id)
                }
                code => unreachable!("Hit unreachable error code {}", code),
            }
        }
    }

    /// Signal to the HAL that the component is ready
    fn ready(self) -> Result<Self, ComponentInitError> {
        let ret = unsafe { hal_ready(self.id) };

        match ret {
            x if x == -(EINVAL as i32) => Err(ComponentInitError::Ready),
            0 => {
                debug!("Component is ready");

                Ok(self)
            }
            ret => unreachable!("Unknown error status {} returned from hal_ready()", ret),
        }
    }

    /// Get the HAL-assigned ID for this component
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the component name
    pub fn name(&self) -> &str {
        self.name
    }

    /// Check whether the component was signalled to shut down
    pub fn should_exit(&self) -> bool {
        self.signals.pending().any(|signal| {
            matches!(
                signal,
                signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL
            )
        })
    }

    /// Get a reference to the component's resources
    pub fn resources(&self) -> &R {
        // NOTE: Unwrap is safe here due to `Some(resources)` in HalComponent::new
        &self.resources.as_ref().unwrap()
    }
}

impl<R> Drop for HalComponent<R> {
    /// Clean up resources, signals and HAL component
    fn drop(&mut self) {
        // Force resources to be dropped before close
        self.resources = None;

        debug!("Closing component ID {}, name {}", self.id, self.name);

        self.signals.close();

        unsafe {
            hal_exit(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::{ComponentInitError, PinRegisterError},
        RegisterResources,
    };

    #[derive(Debug)]
    struct EmptyResources {}
    impl Resources for EmptyResources {
        type RegisterError = PinRegisterError;

        fn register_resources(_comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
            Ok(Self {})
        }
    }

    #[test]
    fn name_too_long() -> Result<(), ComponentInitError> {
        let comp = HalComponent::<EmptyResources>::new(
            "name-thats-way-too-long-for-linuxcnc-to-handle-wow-this-is-ridiculous",
        );

        println!("{:?}", comp);

        match comp {
            Err(ComponentInitError::NameLength) => Ok(()),
            Err(e) => Err(e),
            Ok(_) => Err(ComponentInitError::Init),
        }
    }
}
