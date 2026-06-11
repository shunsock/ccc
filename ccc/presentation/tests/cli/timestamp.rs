use crate::ccc;
use predicates::prelude::*;

// --- Timestamp constructor ---

#[test]
fn evaluate_timestamp_integer() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(1234567890)").assert();

    // Assert
    result.success().stdout("1234567890\n");
}

#[test]
fn evaluate_timestamp_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(1234567890.5)").assert();

    // Assert
    result.success().stdout("1234567890.5\n");
}

// --- Timestamp type casts ---

#[test]
fn evaluate_timestamp_as_datetime() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(0) as datetime").assert();

    // Assert
    result.success().stdout("1970-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_as_timestamp() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z as timestamp").assert();

    // Assert
    result.success().stdout("1767225600\n");
}

#[test]
fn evaluate_integer_as_timestamp_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1 as timestamp").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("cannot cast"));
}
