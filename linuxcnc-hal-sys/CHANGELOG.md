# Changelog

Unsafe Rust bindings to LinuxCNC's HAL module.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0] - 2022-11-15

### Changed

- **(breaking)** Migrate to Rust edition 2021
- Change build process so components are dynamically linked when loaded, instead of statically
  linked.

## [0.2.0] - 2021-01-06

### Changed

- **(breaking)** The LinuxCNC source code must now be provided using the
  `LINUXCNC_SRC=path/to/linuxcnc/src/git` env var.

## [0.1.7] - 2020-01-29

### Added

- Added a `struct.rs` example showing how to allocate memory for a struct of pins

## [0.1.6] - 2020-01-28

### Changed

- Pin LinuxCNC source code to version 2.7.15
- Add examples to `examples/` folder and docs header

## 0.1.5 - 2020-01-27

### Added

- Initial release with `bindgen`-generated items

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-sys-v0.3.0...HEAD

[0.3.0]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-sys-v0.2.0...linuxcnc-hal-sys-v0.3.0
[0.2.0]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-sys-v0.1.7...linuxcnc-hal-sys-v0.2.0
[0.1.7]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.6...linuxcnc-hal-sys-v0.1.7
[0.1.6]:
  https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.5...linuxcnc-hal-v0.1.6
