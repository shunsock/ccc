#[cfg(test)]
mod tests {
    use crate::error::{CccError, SourcePosition};

    #[test]
    fn scan_error_display() {
        // Arrange
        let err = CccError::scan("unexpected character");

        // Act
        let result = format!("{err}");

        // Assert
        assert_eq!(result, "scan error: unexpected character");
    }

    #[test]
    fn parse_error_display() {
        // Arrange
        let err = CccError::parse("unexpected token");

        // Act
        let result = format!("{err}");

        // Assert
        assert_eq!(result, "parse error: unexpected token");
    }

    #[test]
    fn parse_error_with_position() {
        // Arrange
        let err = CccError::parse_at("unexpected token", 5);

        // Act & Assert
        if let CccError::Parse { position, .. } = &err {
            assert_eq!(position, &Some(SourcePosition { column: 5 }));
        } else {
            panic!("Expected Parse variant");
        }
    }

    #[test]
    fn parse_error_without_position() {
        // Arrange
        let err = CccError::parse("unexpected token");

        // Act & Assert
        if let CccError::Parse { position, .. } = &err {
            assert_eq!(position, &None);
        } else {
            panic!("Expected Parse variant");
        }
    }

    #[test]
    fn eval_error_display() {
        // Arrange
        let err = CccError::eval("division by zero");

        // Act
        let result = format!("{err}");

        // Assert
        assert_eq!(result, "eval error: division by zero");
    }

    #[test]
    fn type_check_error_display() {
        // Arrange
        let err = CccError::type_check("argument count mismatch");

        // Act
        let result = format!("{err}");

        // Assert
        assert_eq!(result, "type error: argument count mismatch");
    }

    #[test]
    fn type_check_error_with_position() {
        // Arrange
        let err = CccError::type_check_at("undefined function", 3);

        // Act & Assert
        if let CccError::TypeCheck { position, .. } = &err {
            assert_eq!(position, &Some(SourcePosition { column: 3 }));
        } else {
            panic!("Expected TypeCheck variant");
        }
    }

    #[test]
    fn type_check_error_without_position() {
        // Arrange
        let err = CccError::type_check("undefined function");

        // Act & Assert
        if let CccError::TypeCheck { position, .. } = &err {
            assert_eq!(position, &None);
        } else {
            panic!("Expected TypeCheck variant");
        }
    }

    #[test]
    fn errors_are_distinct_variants() {
        // Arrange
        let scan = CccError::scan("x");
        let parse = CccError::parse("x");
        let eval = CccError::eval("x");
        let type_check = CccError::type_check("x");

        // Act & Assert
        assert_ne!(scan, parse);
        assert_ne!(parse, eval);
        assert_ne!(scan, eval);
        assert_ne!(type_check, scan);
        assert_ne!(type_check, parse);
        assert_ne!(type_check, eval);
    }

    #[test]
    fn same_variant_same_message_are_equal() {
        // Arrange
        let a = CccError::scan("err");
        let b = CccError::scan("err");

        // Act & Assert
        assert_eq!(a, b);
    }

    #[test]
    fn same_variant_different_message_are_not_equal() {
        // Arrange
        let a = CccError::scan("err1");
        let b = CccError::scan("err2");

        // Act & Assert
        assert_ne!(a, b);
    }

    #[test]
    fn error_implements_std_error() {
        // Arrange
        let err: Box<dyn std::error::Error> = Box::new(CccError::eval("test"));

        // Act
        let result = err.to_string();

        // Assert
        assert!(result.contains("eval error"));
    }

    #[test]
    fn debug_format_is_available() {
        // Arrange
        let err = CccError::parse("bad input");

        // Act
        let debug = format!("{err:?}");

        // Assert
        assert!(debug.contains("Parse"));
    }
}
