use crate::test_fixture::{float, int};

use super::parse_expr;

// --- Integer and Float literals ---

#[test]
fn parse_integer() {
    // Arrange
    let input = "42";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, int(42));
}

#[test]
fn parse_zero() {
    // Arrange
    let input = "0";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, int(0));
}

#[test]
fn parse_float() {
    // Arrange
    let input = "2.5";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, float(2.5));
}

#[test]
fn parse_float_with_leading_digits() {
    // Arrange
    let input = "123.456";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, float(123.456));
}
