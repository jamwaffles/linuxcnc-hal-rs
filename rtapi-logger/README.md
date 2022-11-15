# RTAPI logger for Rust components in LinuxCNC

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/rtapi-logger.svg)](https://crates.io/crates/rtapi-logger)
[![Docs.rs](https://docs.rs/rtapi-logger/badge.svg)](https://docs.rs/rtapi-logger)
[![Liberapay](https://img.shields.io/badge/donate-liberapay-yellow.svg)](https://liberapay.com/jamwaffles)

Please consider [becoming a sponsor](https://github.com/sponsors/jamwaffles/) so I may continue to maintain this crate in my spare time!

# [Documentation](https://docs.rs/rtapi-logger)

`rtapi-logger` is a logging driver for the [`log`] ecosystem ecosystem.

Other loggers which don't use the RTAPI logging machinery provided by LinuxCNC are relatively
slow and can unnecessarily block realtime tasks. `rtapi-logger` hooks into LinuxCNC's logging
machinery to prevent these problems, whilst also allowing the use of the convenient macros
provided by [`log`].

Please note this crate is still somewhat experimental. For example, currently all messages are
logged at the `ERR` level provided by LinuxCNC.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[`log`]: https://docs.rs/log
