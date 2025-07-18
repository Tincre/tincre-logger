// src/logger.rs

use tracing::{debug, error, info, warn};

/// This helper function will be called by our public API.
/// In tests, this function does nothing to avoid conflicts.
/// In production, it initializes the logger on the first call.
#[cfg_attr(coverage, coverage(off))]
#[inline(always)]
fn ensure_initialized() {
    #[cfg(not(test))]
    {
        use std::sync::Once;
        use tracing_subscriber::{prelude::*, EnvFilter};

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let env_filter = EnvFilter::try_from_default_env()
                // If RUST_LOG is not set, default to showing `info` level logs.
                .unwrap_or_else(|_| EnvFilter::new("info"));

            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().with_ansi(true))
                .with(env_filter)
                .init();
        });
    }
}
// --- Public API Functions ---

/// Logs a message at the info level.
pub fn log(message: &str) {
    ensure_initialized();
    info!(message);
}

/// Logs a message at the info level.
pub fn info(message: &str) {
    ensure_initialized();
    info!(message);
}

/// Logs a message at the warn level.
pub fn warn(message: &str) {
    ensure_initialized();
    warn!(message);
}

/// Logs a message at the error level.
pub fn error(message: &str) {
    ensure_initialized();
    error!(message);
}

/// Logs a message at the debug level.
pub fn debug(message: &str) {
    ensure_initialized();
    debug!(message);
}

#[cfg(test)]
mod tests {
    use super::*;
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

        // Run the test code with our temporary subscriber
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
}
