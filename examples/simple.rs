// examples/simple.rs

use tincre_logger::logger;

fn main() {
    logger::log("hello from the example");
    logger::warn("this is a warning");
    logger::error("this is an error");
    logger::debug("this is a debug message");
}
