use crate::test_fixture::{add, div, int, modulo, mul, sub};

use super::parse_expr;

// --- Addition and Subtraction ---

#[test]
fn parse_addition() {
    // Arrange
    let input = "1 + 2";
    let expected = add(int(1), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_subtraction() {
    // Arrange
    let input = "5 - 3";
    let expected = sub(int(5), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_chained_addition() {
    // Arrange: 1 + 2 + 3 = (1 + 2) + 3 (left-associative)
    let input = "1 + 2 + 3";
    let expected = add(add(int(1), int(2)), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

// --- Multiplication, Division, Modulo ---

#[test]
fn parse_multiplication() {
    // Arrange
    let input = "2 * 3";
    let expected = mul(int(2), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_division() {
    // Arrange
    let input = "10 / 2";
    let expected = div(int(10), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_modulo() {
    // Arrange
    let input = "7 % 3";
    let expected = modulo(int(7), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

// --- Whitespace handling ---

#[test]
fn parse_no_spaces() {
    // Arrange
    let input = "1+2";
    let expected = add(int(1), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_extra_spaces() {
    // Arrange
    let input = "  1  +  2  ";
    let expected = add(int(1), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_tabs() {
    // Arrange
    let input = "1\t+\t2";
    let expected = add(int(1), int(2));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}
