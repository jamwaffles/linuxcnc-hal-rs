//! Create a component that does nothing except init and exit

use linuxcnc_hal::HalComponent;

fn main() {
    let mut comp = HalComponent::new("empty_component").expect("Could not create component");

    comp.ready().expect("Component could not be made ready");
}
