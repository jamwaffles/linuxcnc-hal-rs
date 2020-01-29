# Changelog

A safe abstraction used to create LinuxCNC HAL components in Rust.

<!-- next-header -->

## [Unreleased] - ReleaseDate

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

[unreleased]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.2...HEAD
[0.1.2]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.1...linuxcnc-hal-v0.1.2
[0.1.1]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.0...linuxcnc-hal-v0.1.1
