use domain::ast::CastTargetType;

use crate::test_fixture::{call, cast, datetime_literal, duration_literal, float, int, list};

use super::check;

// --- Type cast ---

#[test]
fn cast_integer_to_float_passes() {
    // Arrange
    let expr = cast(int(3), CastTargetType::Float);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_float_to_int_passes() {
    // Arrange
    let expr = cast(float(3.7), CastTargetType::Integer);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_integer_to_int_passes() {
    // Arrange
    let expr = cast(int(3), CastTargetType::Integer);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_float_to_float_passes() {
    // Arrange
    let expr = cast(float(3.0), CastTargetType::Float);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_duration_to_int_is_error() {
    // Arrange
    let expr = cast(duration_literal(0, 10, 0), CastTargetType::Integer);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn cast_datetime_to_int_is_error() {
    // Arrange
    let expr = cast(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        CastTargetType::Integer,
    );

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn cast_list_to_int_is_error() {
    // Arrange
    let expr = cast(list(vec![int(1)]), CastTargetType::Integer);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn cast_datetime_to_timestamp_passes() {
    // Arrange
    let expr = cast(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        CastTargetType::Timestamp,
    );

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_timestamp_to_datetime_passes() {
    // Arrange
    let expr = cast(call("Timestamp", vec![int(0)]), CastTargetType::DateTime);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn cast_integer_to_timestamp_is_error() {
    // Arrange
    let expr = cast(int(42), CastTargetType::Timestamp);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn cast_integer_to_datetime_is_error() {
    // Arrange
    let expr = cast(int(42), CastTargetType::DateTime);

    // Act & Assert
    assert!(check(expr).is_err());
}
