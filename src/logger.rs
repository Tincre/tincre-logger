//! A simple, "zero-setup" logger that works out-of-the-box.
//!
//! This module provides a set of functions for logging messages at different
//! levels (`info`, `warn`, `error`, `debug`). It automatically initializes
//! on the first log call and respects the `RUST_LOG` environment variable.
//!
//! # Example
//!
//! ```
//! use tincre_logger::logger;
//!
//! fn main() {
//!     logger::info("Server has started.");
//!     logger::warn("Low disk space detected.");
//!     logger::error("Failed to connect to database!");
//!     // To see debug messages, run with `RUST_LOG=debug`
//!     logger::debug("User 'admin' logged in.");
//! }

use chrono::Utc;
use serde_json::Value;
use tracing::{debug, error, info, warn};

#[cfg_attr(coverage, coverage(off))]
#[inline(always)]
fn ensure_initialized() {
    #[cfg(not(test))]
    {
        use std::sync::Once;
        use tracing_subscriber::{prelude::*, EnvFilter};

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let env_filter =
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().with_ansi(true))
                .with(env_filter)
                .init();
        });
    }
}

// --- Public API Functions ---

/// Logs a message at the `INFO` level. This is an alias for `info()`.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
///
/// logger::log("This is an informational message.");
/// ```
pub fn log(message: &str) {
    ensure_initialized();
    info!(message);
}

/// Logs a message at the `INFO` level.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
///
/// logger::info("An informational message.");
/// ```
pub fn info(message: &str) {
    ensure_initialized();
    info!(message);
}

/// Logs a message at the `WARN` level.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
///
/// logger::warn("A warning message.");
/// ```
pub fn warn(message: &str) {
    ensure_initialized();
    warn!(message);
}

/// Logs a message at the `ERROR` level.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
///
/// logger::error("An error message.");
/// ```
pub fn error(message: &str) {
    ensure_initialized();
    error!(message);
}

/// Logs a message at the `DEBUG` level.
///
/// By default, debug messages are hidden. They can be enabled by setting
/// the `RUST_LOG` environment variable (e.g., `RUST_LOG=debug`).
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
///
/// // To see this message, run your application with `RUST_LOG=debug`
/// logger::debug("A verbose debug message for developers.");
/// ```
pub fn debug(message: &str) {
    ensure_initialized();
    debug!(message);
}

/// Logs a message at the `INFO` level with additional structured metadata.
///
/// This function accepts a message and a second parameter representing structured
/// data. The data is serialized and logged as part of the event. A UTC timestamp is
/// automatically injected.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
/// use serde_json::json;
///
/// logger::info_with("User signed in", json!({ "user_id": 42, "method": "oauth" }));
/// ```
pub fn info_with(message: &str, data: impl Into<Value>) {
    ensure_initialized();
    let timestamp = Utc::now().to_rfc3339();
    info!(%timestamp, message = %message, data = ?data.into());
}

/// Logs a message at the `WARN` level with additional structured metadata.
///
/// This function is useful for highlighting warnings while attaching extra
/// information, such as rate limit states or configuration drift. A UTC timestamp
/// is automatically injected.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
/// use serde_json::json;
///
/// logger::warn_with("Cache miss", json!({ "key": "homepage", "attempts": 2 }));
/// ```
pub fn warn_with(message: &str, data: impl Into<Value>) {
    ensure_initialized();
    let timestamp = Utc::now().to_rfc3339();
    warn!(%timestamp, message = %message, data = ?data.into());
}

/// Logs a message at the `ERROR` level with additional structured metadata.
///
/// This function is intended for errors that should be captured in monitoring
/// pipelines with relevant context, such as error codes or service names.
/// A UTC timestamp is automatically injected.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
/// use serde_json::json;
///
/// logger::error_with("Database write failed", json!({ "table": "users", "code": 500 }));
/// ```
pub fn error_with(message: &str, data: impl Into<Value>) {
    ensure_initialized();
    let timestamp = Utc::now().to_rfc3339();
    error!(%timestamp, message = %message, data = ?data.into());
}

/// Logs a message at the `DEBUG` level with additional structured metadata.
///
/// By default, debug messages are hidden. They can be enabled by setting
/// the `RUST_LOG` environment variable (e.g., `RUST_LOG=debug`). A UTC timestamp
/// is automatically injected.
///
/// # Example
///
/// ```
/// use tincre_logger::logger;
/// use serde_json::json;
///
/// // To see this message, run your application with `RUST_LOG=debug`
/// logger::debug_with("Loaded config", json!({ "env": "dev", "debug_mode": true }));
/// ```
pub fn debug_with(message: &str, data: impl Into<Value>) {
    ensure_initialized();
    let timestamp = Utc::now().to_rfc3339();
    debug!(%timestamp, message = %message, data = ?data.into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::{filter::LevelFilter, fmt, layer::SubscriberExt, registry};

    #[derive(Clone)]
    struct TestWriter {
        buf: Arc<Mutex<Vec<u8>>>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                buf: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_contents(&self) -> String {
            let mut buf = self.buf.lock().unwrap();
            let output = String::from_utf8(buf.clone()).expect("Logs should be valid UTF-8");
            buf.clear();
            output
        }
    }

    #[cfg_attr(coverage, coverage(off))]
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.lock().unwrap().write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.buf.lock().unwrap().flush()
        }
    }

    #[test]
    fn it_logs_all_levels() {
        let writer = TestWriter::new();
        let writer_clone = writer.clone();

        // Build a subscriber that captures all log levels for this test
        let subscriber = registry()
            .with(
                fmt::layer()
                    .with_writer(move || writer_clone.clone())
                    .with_ansi(false),
            )
            // Explicitly set the filter to capture all levels down to TRACE
            .with(LevelFilter::TRACE);

        tracing::subscriber::with_default(subscriber, || {
            log("hello world");
            info("this is info");
            warn("a warning message");
            error("an error message");
            debug("a debug message");
        });

        let output = writer.get_contents();

        assert!(output.contains("INFO") && output.contains("hello world"));
        assert!(output.contains("INFO") && output.contains("this is info"));
        assert!(output.contains("WARN") && output.contains("a warning message"));
        assert!(output.contains("ERROR") && output.contains("an error message"));
        assert!(output.contains("DEBUG") && output.contains("a debug message"));
    }

    #[test]
    fn it_logs_all_levels_with_data() {
        let writer = TestWriter::new();
        let writer_clone = writer.clone();

        let subscriber = registry()
            .with(
                fmt::layer()
                    .with_writer(move || writer_clone.clone())
                    .with_ansi(false),
            )
            .with(LevelFilter::TRACE);

        tracing::subscriber::with_default(subscriber, || {
            info_with("structured info", json!({ "k": "v" }));
            warn_with("structured warn", json!({ "warn_level": 2 }));
            error_with("structured error", json!({ "err": "boom" }));
            debug_with("structured debug", json!({ "flag": true }));
        });

        let output = writer.get_contents();

        assert!(output.contains("INFO") && output.contains("structured info"));
        assert!(output.contains("WARN") && output.contains("structured warn"));
        assert!(output.contains("ERROR") && output.contains("structured error"));
        assert!(output.contains("DEBUG") && output.contains("structured debug"));
        assert!(output.contains("timestamp")); // check for injected field
        assert!(output.contains('k') && output.contains('v'));
    }
}
