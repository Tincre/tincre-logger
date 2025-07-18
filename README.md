# tincre-logger: A logging wrapper for Rust projects

[![Crates.io](https://img.shields.io/crates/v/tincre-logger)](https://crates.io/crates/tincre-logger)
[![Build Status](https://github.com/Tincre/tincre-logger/actions/workflows/tests.yml/badge.svg)](https://github.com/Tincre/tincre-logger/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/Tincre/tincre-logger/branch/main/graph/badge.svg)](https://codecov.io/gh/Tincre/tincre-logger)
[![Docs.rs](https://docs.rs/tincre-logger/badge.svg)](https://docs.rs/tincre-logger)
[![License](https://img.shields.io/crates/l/tincre-logger)](https://crates.io/crates/tincre-logger)

tincre-logger is a simple, "zero-setup" logging library for Rust that works out-of-the-box with a familiar API.

It's designed to be efficient and easy to use, providing automatic initialization on the first log call.

Here are the logging functions included in this library:

- **`log()` / `info()`** for informational messages.
- **`warn()`** for warnings.
- **`error()`** for errors.
- **`debug()`** for verbose debugging messages.

This library leverages `tracing` to provide structured, high-performance logging with minimal boilerplate.

## Features

- **Zero-Setup Logging**: Automatically initializes on the first log call. No manual `init()` function is required in your application's `main`.
- **Simple, Familiar API**: Provides `log()`, `info()`, `warn()`, `error()`, and `debug()` functions in a flat `logger` module.
- **Environment Configuration**: Respects the standard `RUST_LOG` environment variable to control log levels (e.g., `RUST_LOG=debug`).
- **Colored Console Output**: Prints colorful, easy-to-read logs to the console by default.
- **Fully Tested**: Includes a full suite of unit and integration tests to ensure reliability and correctness.

## Installation

Add `tincre-logger` to your `Cargo.toml`:

```toml
[dependencies]
tincre-logger = "0.1.0" # Replace with the latest version
```

## Usage

```rust
use tincre_logger::logger;

fn main() {
    logger::info("Application starting up.");
    logger::warn("Configuration file not found, using defaults.");

    let user_id = "abc-123";
    // Debug logs are hidden by default.
    logger::debug(&format!("Processing data for user: {}", user_id));

    logger::error("Failed to connect to the database!");
}
```

To see the debug-level logs, run your application with the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug cargo run
```

## License

This project is licensed under the MIT License.
