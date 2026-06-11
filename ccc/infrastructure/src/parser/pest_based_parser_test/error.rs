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

// --- Trailing input rejection (issue #39) ---
// The grammar is anchored with SOI/EOI, so a valid prefix followed by
// invalid trailing content must fail instead of being silently truncated.

#[test]
fn parse_trailing_operator_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("1 +");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_valid_prefix_with_invalid_operator_sequence_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("1 + * 2");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_two_expressions_without_operator_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("1 2");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_unmatched_closing_paren_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("1 + 2)");

    // Assert
    assert!(result.is_err());
}
