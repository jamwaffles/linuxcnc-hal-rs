//! `rtapi-logger` is a logging driver for the [`log`](https://docs.rs/log) ecosystem.
//!
//! Other loggers which don't use the RTAPI logging machinery provided by LinuxCNC are relatively
//! slow and can unnecessarily block realtime tasks. `rtapi-logger` hooks into LinuxCNC's logging
//! machinery to prevent these problems, whilst also allowing the use of the convenient macros
//! provided by [`log`].
//!
//! Please note this crate is still somewhat experimental. For example, currently all messages are
//! logged at the `ERR` level provided by LinuxCNC.

use linuxcnc_hal_sys::{rtapi_get_msg_level, rtapi_print_msg};
use log::{Level, Metadata, Record};
use std::ffi::CString;

use log::{LevelFilter, SetLoggerError};

/// Log level.
///
/// Defined in the LinuxCNC source as `msg_level_t`.
///
/// Note that this currently does nothing; all messages are logged at `ERR` level.
#[derive(Debug, Copy, Clone)]
enum RtapiLogLevel {
    None = 0,
    Err = 1,
    Warn = 2,
    Info = 3,
    Dbg = 4,
    All = 5,
}

impl From<Level> for RtapiLogLevel {
    fn from(other: Level) -> Self {
        match other {
            Level::Error => Self::Err,
            Level::Warn => Self::Warn,
            Level::Info => Self::Info,
            Level::Debug => Self::Dbg,
            Level::Trace => Self::All,
        }
    }
}

static LOGGER: RtapiLogger = RtapiLogger;

pub fn init() -> Result<(), SetLoggerError> {
    let _rtapi_level = unsafe { rtapi_get_msg_level() };

    // log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    // Log everything by default
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

pub struct RtapiLogger;

impl log::Log for RtapiLogger {
    // FIXME: This should not just return true - performance will take a hit. LinuxCNC's logging
    // system does the filtering, so that works at least.
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Info
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let out = format!("{}\n", record.args());

            // FIXME: LinuxCNC seems to always use `Err` level regardless of DEBUG config value.
            // let level: RtapiLogLevel = record.level().into();
            let level = RtapiLogLevel::Err;

            if let Ok(f) = CString::new(out) {
                unsafe { rtapi_print_msg(level as u32, f.as_ptr()) };
            } else {
                let fail = CString::new("failed to build log message string").unwrap();

                unsafe { rtapi_print_msg(level as u32, fail.as_ptr()) }
            }
        }
    }

    fn flush(&self) {}
}
