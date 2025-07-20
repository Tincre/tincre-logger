// examples/simple.rs
use serde_json::json;
use tincre_logger::logger;

fn main() {
    // Simple log messages
    logger::log("hello from the example");
    logger::warn("this is a warning");
    logger::error("this is an error");
    logger::debug("this is a debug message");

    // Structured log messages with additional metadata
    logger::info_with(
        "user signed in",
        json!({ "user_id": 42, "method": "oauth" }),
    );
    logger::warn_with("cache miss", json!({ "key": "homepage", "misses": 3 }));
    logger::error_with(
        "db failure",
        json!({ "code": 500, "query": "SELECT * FROM users" }),
    );
    logger::debug_with(
        "loaded config",
        json!({ "env": "local", "debug_mode": true }),
    );
}
