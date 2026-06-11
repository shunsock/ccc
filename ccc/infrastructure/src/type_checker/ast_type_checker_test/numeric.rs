use crate::test_fixture::{add, float, int, mul};

use super::check;

// --- Literals always pass ---

#[test]
fn integer_literal_passes() {
    // Arrange & Act & Assert
    assert!(check(int(42)).is_ok());
}

#[test]
fn float_literal_passes() {
    // Arrange & Act & Assert
    assert!(check(float(2.5)).is_ok());
}

// --- Valid numeric binary operations ---

#[test]
fn integer_add_integer_passes() {
    // Arrange
    let expr = add(int(1), int(2));

    // Act & Assert
    assert!(check(expr).is_ok());
}

#[test]
fn float_multiply_integer_passes() {
    // Arrange
    let expr = mul(float(1.5), int(2));

    // Act & Assert
    assert!(check(expr).is_ok());
}
