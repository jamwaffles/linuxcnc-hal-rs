# Changelog

Unsafe Rust bindings to LinuxCNC's HAL module.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- **(breaking)** The LinuxCNC source code must now be provided using the `LINUXCNC_SRC=path/to/linuxcnc/src/git` env var.

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

[unreleased]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-sys-v0.1.7...HEAD
[0.1.7]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.6...linuxcnc-hal-sys-v0.1.7
[0.1.6]: https://github.com/jamwaffles/linuxcnc-hal-rs/compare/linuxcnc-hal-v0.1.5...linuxcnc-hal-v0.1.6
