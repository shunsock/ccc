use crate::test_fixture::{duration_literal, float, int, list};

use super::check;

// --- List literals ---

#[test]
fn list_literal_passes() {
    // Arrange & Act & Assert
    assert!(check(list(vec![int(1)])).is_ok());
}

#[test]
fn list_homogeneous_integers_passes() {
    // Arrange
    let expr = list(vec![int(1), int(2), int(3)]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn list_homogeneous_floats_passes() {
    // Arrange
    let expr = list(vec![float(1.0), float(2.0)]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn list_homogeneous_durations_passes() {
    // Arrange
    let expr = list(vec![duration_literal(0, 10, 0), duration_literal(0, 20, 0)]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn list_empty_passes() {
    // Arrange
    let expr = list(vec![]);

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn list_mixed_integer_float_is_error() {
    // Arrange
    let expr = list(vec![int(1), float(2.0)]);

    // Act & Assert
    assert!(check(expr).is_err());
}

#[test]
fn list_mixed_integer_duration_is_error() {
    // Arrange
    let expr = list(vec![int(1), duration_literal(0, 10, 0)]);

    // Act & Assert
    assert!(check(expr).is_err());
}
