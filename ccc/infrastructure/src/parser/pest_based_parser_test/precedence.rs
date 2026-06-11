use crate::test_fixture::{add, int, mul, pow};

use super::parse_expr;

// --- Operator precedence ---

#[test]
fn parse_precedence_add_mul() {
    // Arrange: 1 + 2 * 3 = 1 + (2 * 3)
    let input = "1 + 2 * 3";
    let expected = add(int(1), mul(int(2), int(3)));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_precedence_mul_add() {
    // Arrange: 2 * 3 + 1 = (2 * 3) + 1
    let input = "2 * 3 + 1";
    let expected = add(mul(int(2), int(3)), int(1));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

// --- Power (right-associative) ---

#[test]
fn parse_power() {
    // Arrange
    let input = "2 ^ 3";
    let expected = pow(int(2), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_double_star_power() {
    // Arrange
    let input = "2 ** 3";
    let expected = pow(int(2), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_double_star_right_associative() {
    // Arrange: 2**3**2 = 2**(3**2), NOT (2**3)**2
    let input = "2 ** 3 ** 2";
    let expected = pow(int(2), pow(int(3), int(2)));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_power_right_associative() {
    // Arrange: 2^3^2 = 2^(3^2), NOT (2^3)^2
    let input = "2 ^ 3 ^ 2";
    let expected = pow(int(2), pow(int(3), int(2)));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

// --- Parentheses ---

#[test]
fn parse_parentheses() {
    // Arrange: (1 + 2) * 3
    let input = "(1 + 2) * 3";
    let expected = mul(add(int(1), int(2)), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_nested_parentheses() {
    // Arrange: ((1 + 2))
    let input = "((1 + 2))";
    let expected = add(int(1), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}
