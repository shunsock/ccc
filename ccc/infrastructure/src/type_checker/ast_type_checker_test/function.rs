use crate::test_fixture::{call, duration_literal, float, int, list};

use super::check;

// --- Function argument validation ---

#[test]
fn sin_with_integer_passes() {
    // Arrange
    let expr = call("sin", vec![float(2.5)]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn sin_with_duration_is_error() {
    // Arrange
    let expr = call("sin", vec![duration_literal(1, 0, 0)]);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn len_with_list_passes() {
    // Arrange
    let expr = call("len", vec![list(vec![int(1)])]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn len_with_integer_is_error() {
    // Arrange
    let expr = call("len", vec![int(42)]);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn datetime_constructor_with_integers_passes() {
    // Arrange
    let expr = call(
        "DateTime",
        vec![int(2026), int(1), int(1), int(0), int(0), int(0)],
    );

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn datetime_constructor_with_float_is_error() {
    // Arrange
    let expr = call(
        "DateTime",
        vec![float(2026.0), int(1), int(1), int(0), int(0), int(0)],
    );

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn timestamp_with_integer_passes() {
    // Arrange
    let expr = call("Timestamp", vec![int(1234567890)]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn timestamp_with_duration_is_error() {
    // Arrange
    let expr = call("Timestamp", vec![duration_literal(1, 0, 0)]);

    // Act & Assert
    assert!(check(expr).is_err());
}

// --- Time utility functions ---

#[test]
fn now_with_no_args_passes() {
    // Arrange
    let expr = call("now", vec![]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn now_with_args_is_error() {
    // Arrange
    let expr = call("now", vec![int(1)]);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn today_with_no_args_passes() {
    // Arrange
    let expr = call("today", vec![]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn current_timestamp_with_no_args_passes() {
    // Arrange
    let expr = call("current_timestamp", vec![]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

// --- Unknown functions pass through ---

#[test]
fn unknown_function_is_type_error() {
    // Arrange
    let expr = call("unknown_func", vec![int(1)]);

    // Act
    let result = check(expr);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("undefined function"),
        "expected 'undefined function' error, got: {err}"
    );
}
