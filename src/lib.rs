//! # Kaf
//!
//! `kaf` is a [`log`](https://docs.rs/log) implementation that enables the user to filter logs based on entry metadata.
//! For example:
//! ```
//! kaf::with_filter(
//!     Box::new(|target, _level| {
//!         (target == "mycrate" || target.starts_with("mycrate::"))
//!     }),
//!     log::LevelFilter::Info,
//! );
//! ```

use ansi_term::{ANSIGenericString, Color::*, Style};
use log::{Level, LevelFilter};
use std::{fmt::Display, io::Write};

/// A filter. Called for every log entry that is equal or below the global log level.
type Filter = Box<dyn Fn(String, Level) -> bool + Send + Sync>;

/// The logger.
pub struct Logger {
    filter: Filter,
}

/// Initializes logging with a global max level of `LevelFilter::Info`.
///
/// # Examples
/// ```
/// kaf::init();
/// ```
pub fn init() {
    with_level(LevelFilter::Info);
}

/// Initializes logging with a global max level.
///
/// # Examples
/// ```
/// kaf::with_level(log::LevelFilter::Info);
/// ```
pub fn with_level(level: LevelFilter) {
    with_filter(Box::new(|_, _| true), level);
}

/// Initializes logging with a `Filter` implementation.
///
/// * `filter` - The filter to determine if an entry should be logged.
/// * `max_level` - The global maximum log level, `filter` will not be called for levels that are higher.
///
/// # Examples
/// ```
/// kaf::with_filter(
///    Box::new(|target, _level| target == "crate::module"),
///        log::LevelFilter::Info,
///    );
/// ```
pub fn with_filter(filter: Filter, max_level: LevelFilter) {
    log::set_max_level(max_level);
    log::set_boxed_logger(Box::new(Logger { filter })).expect("Could not set logger");
}

/// Initializes logging from a `&str`. Helpful with external configuration.
///
/// # Examples
/// ```
/// kaf::from_str("INFO");
/// ```
pub fn from_str(level: &str) {
    use std::str::FromStr;
    with_level(LevelFilter::from_str(level).unwrap());
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
            && (*self.filter)(metadata.target().to_string(), metadata.level())
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();

            let (pre, args): (ANSIGenericString<'_, str>, Box<dyn Display>) = match record.level() {
                log::Level::Error => (
                    Red.bold().paint(record.target()),
                    Box::new(Style::new().bold().paint(record.args().to_string())),
                ),
                log::Level::Warn => (
                    Yellow.bold().paint(record.target()),
                    Box::new(record.args()),
                ),
                log::Level::Trace | log::Level::Debug => (
                    Purple.bold().paint(record.target()),
                    Box::new(record.args()),
                ),
                log::Level::Info => (Cyan.bold().paint(record.target()), Box::new(record.args())),
            };

            writeln!(handle, "{} {}", pre, args).unwrap();
        }
    }

    fn flush(&self) {}
}
