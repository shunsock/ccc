#[cfg(test)]
mod tests {
    use domain::error::CccError;

    use crate::format_error::format_error_with_caret;

    #[test]
    fn parse_error_with_position_shows_caret() {
        // Arrange
        let input = "2 + + 3";
        let error = CccError::parse_at("expected number", 5);

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(result.contains("2 + + 3"));
        assert!(result.contains("    ^"));
        assert!(result.contains("error: parse error: expected number"));
    }

    #[test]
    fn parse_error_at_column_1_shows_caret_at_start() {
        // Arrange
        let input = ")";
        let error = CccError::parse_at("expected number", 1);

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(result.contains(")"));
        assert!(result.contains("^"));
        assert!(result.contains("error: parse error: expected number"));
    }

    #[test]
    fn parse_error_without_position_shows_message_only() {
        // Arrange
        let input = "bad input";
        let error = CccError::parse("unexpected token");

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(!result.contains("bad input"));
        assert!(!result.contains("^"));
        assert!(result.contains("error: parse error: unexpected token"));
    }

    #[test]
    fn eval_error_shows_message_only() {
        // Arrange
        let input = "1/0";
        let error = CccError::eval("division by zero: 1 / 0");

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(!result.contains("^"));
        assert!(result.contains("error: eval error: division by zero: 1 / 0"));
    }

    #[test]
    fn type_check_error_with_position_shows_caret() {
        // Arrange
        let input = "sqrt(1, 2)";
        let error = CccError::type_check_at("sqrt expects 1 argument, got 2", 1);

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(result.contains("sqrt(1, 2)"));
        assert!(result.contains("^"));
        assert!(result.contains("error: type error: sqrt expects 1 argument, got 2"));
    }

    #[test]
    fn type_check_error_without_position_shows_message_only() {
        // Arrange
        let input = "unknown_func(1)";
        let error = CccError::type_check("undefined function: unknown_func");

        // Act
        let result = format_error_with_caret(input, &error);

        // Assert
        assert!(!result.contains("^"));
        assert!(result.contains("error: type error: undefined function: unknown_func"));
    }
}
