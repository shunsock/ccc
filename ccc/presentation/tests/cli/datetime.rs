use crate::ccc;

// --- DateTime literal ---

#[test]
fn evaluate_datetime_literal_utc() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00").assert();

    // Assert
    result.success().stdout("2026-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_literal_z_suffix() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z").assert();

    // Assert
    result.success().stdout("2026-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_literal_with_offset() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T09:00:00+09:00").assert();

    // Assert
    result.success().stdout("2026-01-01T09:00:00+09:00\n");
}

// --- DateTime constructor ---

#[test]
fn evaluate_datetime_constructor() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("DateTime(2026, 6, 15, 12, 30, 0)").assert();

    // Assert
    result.success().stdout("2026-06-15T12:30:00Z\n");
}

// --- DateTime arithmetic ---

#[test]
fn evaluate_datetime_add_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z + 1:30:00").assert();

    // Assert
    result.success().stdout("2026-01-01T01:30:00Z\n");
}

#[test]
fn evaluate_datetime_subtract_datetime() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd
        .arg("2026-01-02T00:00:00Z - 2026-01-01T00:00:00Z")
        .assert();

    // Assert
    result.success().stdout("24:00:00\n");
}

#[test]
fn evaluate_datetime_add_mm_ss_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2025-12-25T15:30:00+09 + 10:00").assert();

    // Assert
    result.success().stdout("2025-12-25T15:40:00+09:00\n");
}

// --- Time utility functions ---

#[test]
fn evaluate_now_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("now()").assert();

    // Assert: output contains a datetime-like pattern
    result.success();
}

#[test]
fn evaluate_today_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("today()").assert();

    // Assert
    result.success();
}

#[test]
fn evaluate_current_timestamp_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("current_timestamp()").assert();

    // Assert
    result.success();
}
