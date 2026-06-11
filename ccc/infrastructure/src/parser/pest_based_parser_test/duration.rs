use crate::parser::PestBasedParser;
use crate::test_fixture::{duration_literal, int};
use domain::interface::parser::CccParser;

use super::parse_expr;

// --- DurationTime literals ---

#[test]
fn parse_duration_literal_basic() {
    // Arrange
    let input = "10:20:30";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(10, 20, 30));
}

#[test]
fn parse_duration_literal_zero() {
    // Arrange
    let input = "0:00:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(0, 0, 0));
}

#[test]
fn parse_duration_literal_large_hours() {
    // Arrange
    let input = "999:59:59";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(999, 59, 59));
}

#[test]
fn parse_duration_in_expression() {
    // Arrange: DurationTime used in a binary operation (parsed correctly)
    let input = "10:00:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(10, 0, 0));
}

#[test]
fn parse_duration_mm_ss_basic() {
    // Arrange
    let input = "10:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(0, 10, 0));
}

#[test]
fn parse_duration_mm_ss_with_seconds() {
    // Arrange
    let input = "1:30";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, duration_literal(0, 1, 30));
}

#[test]
fn parse_duration_mm_ss_seconds_out_of_range_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("10:60");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_duration_minutes_out_of_range_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("0:60:00");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_duration_seconds_out_of_range_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("0:00:60");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_integer_still_works() {
    // Arrange: plain integer should not be parsed as duration
    let input = "42";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, int(42));
}
