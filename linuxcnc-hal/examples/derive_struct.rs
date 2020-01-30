use linuxcnc_hal::{hal_pin::HalPinF64, HalComponentBuilder, HalData};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

#[linuxcnc_derive::hal]
struct Pins {
    //
}

fn main() -> Result<(), Box<dyn Error>> {
    let thing = Pins {};

    thing.test_fn();

    Ok(())
}
