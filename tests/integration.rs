use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_example_output_default_level() {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--example", "simple", "--quiet"]);

    cmd.assert()
        .stdout(predicate::str::contains("hello from the example"))
        .stdout(predicate::str::contains("this is a warning"))
        .stdout(predicate::str::contains("this is an error"))
        .stdout(predicate::str::contains("user signed in"))
        .stdout(predicate::str::contains("cache miss"))
        .stdout(predicate::str::contains("db failure"))
        .stdout(predicate::str::contains("loaded config").not()) // debug only
        .stdout(predicate::str::contains("this is a debug message").not())
        .success();
}

#[test]
fn test_example_output_debug_level() {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--example", "simple", "--quiet"]);
    cmd.env("RUST_LOG", "debug");

    cmd.assert()
        .stdout(predicate::str::contains("this is a debug message"))
        .stdout(predicate::str::contains("loaded config"))
        .success();
}

#[test]
fn test_output_contains_color_codes() {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--example", "simple", "--quiet"]);

    cmd.assert()
        .stdout(predicate::str::contains("\x1b[32m")) // Green
        .stdout(predicate::str::contains("\x1b[33m")) // Yellow
        .stdout(predicate::str::contains("\x1b[31m")) // Red
        .success();
}
