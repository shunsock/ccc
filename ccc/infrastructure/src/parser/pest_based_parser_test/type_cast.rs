use domain::ast::CastTargetType;

use crate::test_fixture::{add, cast, float, int, mul};

use super::parse_expr;

// --- Type cast syntax ---

#[test]
fn parse_integer_as_float() {
    // Arrange
    let input = "3 as float";
    let expected = cast(int(3), CastTargetType::Float);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_float_as_int() {
    // Arrange
    let input = "3.7 as int";
    let expected = cast(float(3.7), CastTargetType::Integer);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_expression_result_as_int() {
    // Arrange: (1 + 2) as int
    let input = "(1 + 2) as int";
    let expected = cast(add(int(1), int(2)), CastTargetType::Integer);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_cast_in_binary_expression() {
    // Arrange: 2 * 3 as float + 1 → (2 * (3 as float)) + 1
    // `as` binds at postfix level (tighter than multiplicative)
    let input = "2 * 3 as float + 1";
    let expected = add(mul(int(2), cast(int(3), CastTargetType::Float)), int(1));

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_chained_cast() {
    // Arrange: 42 as float as int
    let input = "42 as float as int";
    let expected = cast(
        cast(int(42), CastTargetType::Float),
        CastTargetType::Integer,
    );

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}
