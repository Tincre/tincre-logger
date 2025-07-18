// src/logger.rs

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
//!     logger::info!("Server has started.");
//!     logger::warn!("Low disk space detected.");
//!     logger::error!("Failed to connect to database!");
//!     // To see debug messages, run with `RUST_LOG=debug`
//!     logger::debug!("User 'admin' logged in.");
//! }
//! ```

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

#[cfg(test)]
mod tests {
    // ... your tests ...
}
