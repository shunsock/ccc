use crate::test_fixture::{add, int, neg, pos};

use super::parse_expr;

// --- Unary operators ---

#[test]
fn parse_unary_negate() {
    // Arrange
    let input = "-5";
    let expected = neg(int(5));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_unary_positive() {
    // Arrange
    let input = "+5";
    let expected = pos(int(5));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_negate_in_expression() {
    // Arrange: -2 + 3
    let input = "-2 + 3";
    let expected = add(neg(int(2)), int(3));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_double_negate() {
    // Arrange: --2000 → UnaryOperation(-, UnaryOperation(-, 2000))
    let input = "- -2000";
    let expected = neg(neg(int(2000)));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_triple_negate() {
    // Arrange: - - -3 → Negate(Negate(Negate(3)))
    let input = "- - -3";
    let expected = neg(neg(neg(int(3))));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}
