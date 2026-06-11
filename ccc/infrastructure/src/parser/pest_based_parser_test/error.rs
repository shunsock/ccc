use domain::ast::Expression;
use domain::interface::parser::CccParser;

use crate::parser::PestBasedParser;

// --- Error cases ---

#[test]
fn parse_empty_string_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_invalid_input_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("+ +");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_unclosed_paren_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("(1 + 2");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_trailing_operator_ignores_trailing() {
    // Arrange: Pest grammar matches "1" as expression; trailing "+" is not consumed.
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("1 +");

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().expression, Expression::Integer(1));
}
