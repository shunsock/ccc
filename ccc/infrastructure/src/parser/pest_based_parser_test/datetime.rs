use crate::parser::PestBasedParser;
use crate::test_fixture::datetime_literal;
use domain::interface::parser::CccParser;

use super::parse_expr;

// --- DateTime literals ---

#[test]
fn parse_datetime_utc_no_suffix() {
    // Arrange
    let input = "2026-01-01T00:00:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2026, 1, 1, 0, 0, 0, 0));
}

#[test]
fn parse_datetime_utc_z_suffix() {
    // Arrange
    let input = "2026-01-01T00:00:00Z";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2026, 1, 1, 0, 0, 0, 0));
}

#[test]
fn parse_datetime_positive_offset_with_minutes() {
    // Arrange
    let input = "2026-01-01T09:00:00+09:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2026, 1, 1, 9, 0, 0, 9 * 3600));
}

#[test]
fn parse_datetime_negative_offset() {
    // Arrange
    let input = "2026-01-01T00:00:00-05:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2026, 1, 1, 0, 0, 0, -5 * 3600));
}

#[test]
fn parse_datetime_offset_hours_only() {
    // Arrange
    let input = "2026-01-01T09:00:00+09";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2026, 1, 1, 9, 0, 0, 9 * 3600));
}

#[test]
fn parse_datetime_invalid_month_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("2026-13-01T00:00:00");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_datetime_invalid_day_feb_29_non_leap_is_error() {
    // Arrange
    let parser = PestBasedParser;

    // Act
    let result = parser.parse("2025-02-29T00:00:00");

    // Assert
    assert!(result.is_err());
}

#[test]
fn parse_datetime_leap_year_feb_29_is_ok() {
    // Arrange
    let input = "2024-02-29T00:00:00";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, datetime_literal(2024, 2, 29, 0, 0, 0, 0));
}
