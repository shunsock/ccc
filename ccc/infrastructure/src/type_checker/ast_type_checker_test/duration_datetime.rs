use crate::test_fixture::{add, datetime_literal, duration_literal, float, int, mul, neg, sub};

use super::check;

// --- Literals always pass ---

#[test]
fn duration_literal_passes() {
    // Arrange & Act & Assert
    assert!(check(duration_literal(1, 0, 0)).is_ok());
}

#[test]
fn datetime_literal_passes() {
    // Arrange & Act & Assert
    assert!(check(datetime_literal(2026, 1, 1, 0, 0, 0, 0)).is_ok());
}

// --- Valid time binary operations ---

#[test]
fn duration_add_duration_passes() {
    // Arrange
    let expr = add(duration_literal(1, 0, 0), duration_literal(0, 30, 0));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn duration_multiply_integer_passes() {
    // Arrange
    let expr = mul(duration_literal(1, 0, 0), int(3));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn integer_multiply_duration_passes() {
    // Arrange
    let expr = mul(int(3), duration_literal(1, 0, 0));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn datetime_add_duration_passes() {
    // Arrange
    let expr = add(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        duration_literal(1, 0, 0),
    );

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn datetime_subtract_datetime_passes() {
    // Arrange
    let expr = sub(
        datetime_literal(2026, 1, 2, 0, 0, 0, 0),
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
    );

    // Act & Assert
    assert!(check(expr).is_ok());
}

// --- Invalid binary operations ---

#[test]
fn datetime_add_datetime_is_error() {
    // Arrange
    let expr = add(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        datetime_literal(2026, 1, 2, 0, 0, 0, 0),
    );

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn datetime_multiply_integer_is_error() {
    // Arrange
    let expr = mul(datetime_literal(2026, 1, 1, 0, 0, 0, 0), int(2));

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn duration_add_float_is_error() {
    // Arrange
    let expr = add(duration_literal(1, 0, 0), float(1.5));

    // Act & Assert
    assert!(check(expr).is_err());
}

// --- Unary operations ---

#[test]
fn negate_integer_passes() {
    // Arrange
    let expr = neg(int(5));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn negate_duration_passes() {
    // Arrange
    let expr = neg(duration_literal(1, 0, 0));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn negate_datetime_is_error() {
    // Arrange
    let expr = neg(datetime_literal(2026, 1, 1, 0, 0, 0, 0));

    // Act & Assert
    assert!(check(expr).is_err());
}
