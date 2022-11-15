# Changelog

A safe abstraction used to create LinuxCNC HAL components in Rust.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0] - 2022-11-15

### Changed

- **(breaking)** Migrate to Rust edition 2021
- Change build process so components are dynamically linked when loaded, instead of statically
  linked.

## [0.2.0] - 2021-01-06

### Added

- #6 Added `BidirectionalPin` to allow an I/O pin to be registered on a component

### Changed

- **(breaking)** #10 The `HalComponentBuilder` is removed. Use `HalComponent::new()` directly
  instead.
- **(breaking)** #10 Pins must now be grouped together in a struct that implements the `Resources`.
  For example:

  ```rust
  struct Pins {
      input_1: InputPin<f64>,
      output_1: OutputPin<f64>,
  }

  impl Resources for Pins {
      type RegisterError = PinRegisterError;

      fn register_resources(comp: &RegisterResources) -> Result<Self, Self::RegisterError> {
          Ok(Pins {
              input_1: comp.register_pin::<InputPin<f64>>("input-1")?,
              output_1: comp.register_pin::<OutputPin<f64>>("output-1")?,
          })
      }
  }
  ```

- **(breaking)** #8 Change how `HalPin` types work. Instead of `InputPinF64`, `OutputPinBool`, etc,
  the `InputPin` and `OutputPin` structs are added. Usage is like this:

  ```rust
  struct Pins {
      input_1: InputPin<f64>,
      output_1: OutputPin<bool>,
  }
  ```

### Fixed

- #10 Fixed a soundness issue where pins were freed after the component exited

## [0.1.3] - 2020-01-29

### Fixed

- Fixed usage of `hal_malloc()` to correctly allocate memory for pins. Many thanks to the users in
  [this thread](https://users.rust-lang.org/t/convert-mut-mut-f64-to-box-f64/37521) for the
  guidance.

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
[unreleased]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.3.0...HEAD

[0.3.0]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.2.0...linuxcnc-hal-v0.3.0
[0.2.0]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.3...linuxcnc-hal-v0.2.0
[0.1.3]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.2...linuxcnc-hal-v0.1.3
[0.1.2]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.1...linuxcnc-hal-v0.1.2
[0.1.1]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.0...linuxcnc-hal-v0.1.1
