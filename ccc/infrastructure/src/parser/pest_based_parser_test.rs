#[cfg(test)]
mod tests {
    use domain::ast::{BinaryOperation, Expression, UnaryOperation};
    use domain::interface::parser::CccParser;

    use crate::parser::PestBasedParser;

    fn parse_expr(input: &str) -> Expression {
        let parser = PestBasedParser;
        parser.parse(input).unwrap().expression
    }

    // --- Integer and Float literals ---

    #[test]
    fn parse_integer() {
        // Arrange
        let input = "42";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::Integer(42));
    }

    #[test]
    fn parse_zero() {
        // Arrange
        let input = "0";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::Integer(0));
    }

    #[test]
    fn parse_float() {
        // Arrange
        let input = "3.14";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::Float(3.14));
    }

    #[test]
    fn parse_float_with_leading_digits() {
        // Arrange
        let input = "123.456";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::Float(123.456));
    }

    // --- Addition and Subtraction ---

    #[test]
    fn parse_addition() {
        // Arrange
        let input = "1 + 2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_subtraction() {
        // Arrange
        let input = "5 - 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::Integer(5)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_chained_addition() {
        // Arrange: 1 + 2 + 3 = (1 + 2) + 3 (left-associative)
        let input = "1 + 2 + 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(2)),
            }),
            right: Box::new(Expression::Integer(3)),
        };

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
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_division() {
        // Arrange
        let input = "10 / 2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::Integer(10)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_modulo() {
        // Arrange
        let input = "7 % 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Modulo,
            left: Box::new(Expression::Integer(7)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    // --- Operator precedence ---

    #[test]
    fn parse_precedence_add_mul() {
        // Arrange: 1 + 2 * 3 = 1 + (2 * 3)
        let input = "1 + 2 * 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Multiply,
                left: Box::new(Expression::Integer(2)),
                right: Box::new(Expression::Integer(3)),
            }),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_precedence_mul_add() {
        // Arrange: 2 * 3 + 1 = (2 * 3) + 1
        let input = "2 * 3 + 1";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Multiply,
                left: Box::new(Expression::Integer(2)),
                right: Box::new(Expression::Integer(3)),
            }),
            right: Box::new(Expression::Integer(1)),
        };

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
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_double_star_power() {
        // Arrange
        let input = "2 ** 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_double_star_right_associative() {
        // Arrange: 2**3**2 = 2**(3**2), NOT (2**3)**2
        let input = "2 ** 3 ** 2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Power,
                left: Box::new(Expression::Integer(3)),
                right: Box::new(Expression::Integer(2)),
            }),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_power_right_associative() {
        // Arrange: 2^3^2 = 2^(3^2), NOT (2^3)^2
        let input = "2 ^ 3 ^ 2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Power,
                left: Box::new(Expression::Integer(3)),
                right: Box::new(Expression::Integer(2)),
            }),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    // --- Unary operators ---

    #[test]
    fn parse_unary_negate() {
        // Arrange
        let input = "-5";
        let expected = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Integer(5)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_unary_positive() {
        // Arrange
        let input = "+5";
        let expected = Expression::UnaryOperation {
            operator: UnaryOperation::Positive,
            operand: Box::new(Expression::Integer(5)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_negate_in_expression() {
        // Arrange: -2 + 3
        let input = "-2 + 3";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::UnaryOperation {
                operator: UnaryOperation::Negate,
                operand: Box::new(Expression::Integer(2)),
            }),
            right: Box::new(Expression::Integer(3)),
        };

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
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(2)),
            }),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_nested_parentheses() {
        // Arrange: ((1 + 2))
        let input = "((1 + 2))";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    // --- Function calls ---

    #[test]
    fn parse_function_call_single_arg() {
        // Arrange
        let input = "sqrt(16)";
        let expected = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::Integer(16)],
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_function_call_with_expression_arg() {
        // Arrange
        let input = "sqrt(1 + 3)";
        let expected = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(3)),
            }],
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_nested_function_call() {
        // Arrange: sin(sqrt(9))
        let input = "sin(sqrt(9))";
        let expected = Expression::FunctionCall {
            name: "sin".to_string(),
            arguments: vec![Expression::FunctionCall {
                name: "sqrt".to_string(),
                arguments: vec![Expression::Integer(9)],
            }],
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_function_call_no_args() {
        // Arrange
        let input = "pi()";
        let expected = Expression::FunctionCall {
            name: "pi".to_string(),
            arguments: vec![],
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_function_call_multiple_args() {
        // Arrange
        let input = "foo(1, 2, 3)";

        // Act
        let result = parse_expr(input);

        // Assert
        if let Expression::FunctionCall { name, arguments } = result {
            assert_eq!(name, "foo");
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[0], Expression::Integer(1));
            assert_eq!(arguments[1], Expression::Integer(2));
            assert_eq!(arguments[2], Expression::Integer(3));
        } else {
            panic!("Expected FunctionCall");
        }
    }

    // --- Whitespace handling ---

    #[test]
    fn parse_no_spaces() {
        // Arrange
        let input = "1+2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_extra_spaces() {
        // Arrange
        let input = "  1  +  2  ";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_tabs() {
        // Arrange
        let input = "1\t+\t2";
        let expected = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

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

    // --- List literals ---

    #[test]
    fn parse_empty_list() {
        // Arrange
        let input = "[]";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::List(vec![]));
    }

    #[test]
    fn parse_list_single_element() {
        // Arrange
        let input = "[1]";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, Expression::List(vec![Expression::Integer(1)]));
    }

    #[test]
    fn parse_list_multiple_elements() {
        // Arrange
        let input = "[1, 2, 3]";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::List(vec![
                Expression::Integer(1),
                Expression::Integer(2),
                Expression::Integer(3),
            ])
        );
    }

    #[test]
    fn parse_list_with_expressions() {
        // Arrange
        let input = "[1+2, 3*4]";
        let expected = Expression::List(vec![
            Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(2)),
            },
            Expression::BinaryOperation {
                operator: BinaryOperation::Multiply,
                left: Box::new(Expression::Integer(3)),
                right: Box::new(Expression::Integer(4)),
            },
        ]);

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_nested_list() {
        // Arrange
        let input = "[[1, 2], [3]]";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::List(vec![
                Expression::List(vec![Expression::Integer(1), Expression::Integer(2)]),
                Expression::List(vec![Expression::Integer(3)]),
            ])
        );
    }

    #[test]
    fn parse_list_with_floats() {
        // Arrange
        let input = "[1.5, 2.5]";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::List(vec![Expression::Float(1.5), Expression::Float(2.5)])
        );
    }

    // --- DurationTime literals ---

    #[test]
    fn parse_duration_literal_basic() {
        // Arrange
        let input = "10:20:30";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 10,
                minutes: 20,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_duration_literal_zero() {
        // Arrange
        let input = "0:00:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn parse_duration_literal_large_hours() {
        // Arrange
        let input = "999:59:59";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 999,
                minutes: 59,
                seconds: 59,
            }
        );
    }

    #[test]
    fn parse_duration_in_expression() {
        // Arrange: DurationTime used in a binary operation (parsed correctly)
        let input = "10:00:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 10,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn parse_duration_mm_ss_basic() {
        // Arrange
        let input = "10:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 0,
                minutes: 10,
                seconds: 0,
            }
        );
    }

    #[test]
    fn parse_duration_mm_ss_with_seconds() {
        // Arrange
        let input = "1:30";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DurationTime {
                hours: 0,
                minutes: 1,
                seconds: 30,
            }
        );
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
        assert_eq!(result, Expression::Integer(42));
    }

    // --- DateTime literals ---

    #[test]
    fn parse_datetime_utc_no_suffix() {
        // Arrange
        let input = "2026-01-01T00:00:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }
        );
    }

    #[test]
    fn parse_datetime_utc_z_suffix() {
        // Arrange
        let input = "2026-01-01T00:00:00Z";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }
        );
    }

    #[test]
    fn parse_datetime_positive_offset_with_minutes() {
        // Arrange
        let input = "2026-01-01T09:00:00+09:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 9,
                minute: 0,
                second: 0,
                offset_seconds: 9 * 3600,
            }
        );
    }

    #[test]
    fn parse_datetime_negative_offset() {
        // Arrange
        let input = "2026-01-01T00:00:00-05:00";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: -5 * 3600,
            }
        );
    }

    #[test]
    fn parse_datetime_offset_hours_only() {
        // Arrange
        let input = "2026-01-01T09:00:00+09";

        // Act
        let result = parse_expr(input);

        // Assert
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 9,
                minute: 0,
                second: 0,
                offset_seconds: 9 * 3600,
            }
        );
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
        assert_eq!(
            result,
            Expression::DateTime {
                year: 2024,
                month: 2,
                day: 29,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }
        );
    }
}
