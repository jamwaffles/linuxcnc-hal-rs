# Changelog

A safe abstraction used to create LinuxCNC HAL components in Rust.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- #6 Added `BidirectionalPin` to allow an I/O pin to be registered on a component
- Added basic `struct.rs` example showing storage of pins in a struct

### Changed

- **(breaking)** Simplify pin creation, reducing number of custom types. Pins are now created like this:

  ```rust
  use linuxcnc_hal::{
      hal_pin::{InputPin, OutputPin},
      prelude::*,
      HalComponentBuilder,
  };

  let mut builder = HalComponentBuilder::new("pins")?;

  let input_1 = builder.register_pin::<InputPin<f64>>("input-1")?;
  let output_1 = builder.register_pin::<OutputPin<f64>>("output-1")?;
  ```

  Note the usage of `register_pin` as opposed to `register_input_pin` or `register_output_pin`.

## [0.1.3] - 2020-01-29

### Fixed

- Fixed usage of `hal_malloc()` to correctly allocate memory for pins. Many thanks to the users in [this thread](https://users.rust-lang.org/t/convert-mut-mut-f64-to-box-f64/37521) for the guidance.

## [0.1.2] - 2020-01-28

### Added

- None

### Changed

- None

### Fixed

- None

## [0.1.1] - 2020-01-27

### Added

- Added `log` crate support

### Changed

- Move `println!()`s to `log` crate for less noise

## 0.1.0 - 2020-01-27

Initial release

<!-- next-url -->

[unreleased]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.3...HEAD
[0.1.3]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.2...linuxcnc-hal-v0.1.3
[0.1.2]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.1...linuxcnc-hal-v0.1.2
[0.1.1]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.0...linuxcnc-hal-v0.1.1
