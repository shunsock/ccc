use crate::ccc;
use predicates::prelude::*;

// --- Numeric type casts ---

#[test]
fn evaluate_integer_as_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3 as float").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_zero_as_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0 as float").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3.7 as int").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_negative_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "-2.9 as int"]).assert();

    // Assert
    result.success().stdout("-2\n");
}

#[test]
fn evaluate_zero_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0.0 as int").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_integer_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3 as int").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_mean_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([1, 2, 3]) as int").assert();

    // Assert
    result.success().stdout("2\n");
}

// --- Invalid type casts ---

#[test]
fn evaluate_duration_as_int_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0:10:00 as int").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("cannot cast"));
}
