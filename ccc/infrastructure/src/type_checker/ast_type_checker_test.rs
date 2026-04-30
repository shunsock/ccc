#[cfg(test)]
mod tests {
    use domain::ast::{AbstractSyntaxTree, BinaryOperation, CastTargetType, Expression, UnaryOperation};
    use domain::interface::type_checker::CccTypeChecker;

    use crate::type_checker::AstTypeChecker;

    fn check(expression: Expression) -> Result<(), domain::error::CccError> {
        let ast = AbstractSyntaxTree { expression };
        let checker = AstTypeChecker;
        checker.check(&ast)
    }

    // --- Literals always pass ---

    #[test]
    fn integer_literal_passes() {
        // Arrange & Act & Assert
        assert!(check(Expression::Integer(42)).is_ok());
    }

    #[test]
    fn float_literal_passes() {
        // Arrange & Act & Assert
        assert!(check(Expression::Float(3.14)).is_ok());
    }

    #[test]
    fn list_literal_passes() {
        // Arrange & Act & Assert
        assert!(check(Expression::List(vec![Expression::Integer(1)])).is_ok());
    }

    #[test]
    fn list_homogeneous_integers_passes() {
        // Arrange
        let expr = Expression::List(vec![
            Expression::Integer(1),
            Expression::Integer(2),
            Expression::Integer(3),
        ]);

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn list_homogeneous_floats_passes() {
        // Arrange
        let expr = Expression::List(vec![Expression::Float(1.0), Expression::Float(2.0)]);

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn list_homogeneous_durations_passes() {
        // Arrange
        let expr = Expression::List(vec![
            Expression::DurationTime {
                hours: 0,
                minutes: 10,
                seconds: 0,
            },
            Expression::DurationTime {
                hours: 0,
                minutes: 20,
                seconds: 0,
            },
        ]);

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn list_empty_passes() {
        // Arrange
        let expr = Expression::List(vec![]);

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn list_mixed_integer_float_is_error() {
        // Arrange
        let expr = Expression::List(vec![Expression::Integer(1), Expression::Float(2.0)]);

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn list_mixed_integer_duration_is_error() {
        // Arrange
        let expr = Expression::List(vec![
            Expression::Integer(1),
            Expression::DurationTime {
                hours: 0,
                minutes: 10,
                seconds: 0,
            },
        ]);

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn duration_literal_passes() {
        // Arrange & Act & Assert
        assert!(
            check(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            })
            .is_ok()
        );
    }

    #[test]
    fn datetime_literal_passes() {
        // Arrange & Act & Assert
        assert!(
            check(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            })
            .is_ok()
        );
    }

    // --- Type cast ---

    #[test]
    fn cast_integer_to_float_passes() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::Integer(3)),
            target_type: CastTargetType::Float,
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn cast_float_to_int_passes() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::Float(3.7)),
            target_type: CastTargetType::Integer,
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn cast_integer_to_int_passes() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::Integer(3)),
            target_type: CastTargetType::Integer,
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn cast_float_to_float_passes() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::Float(3.0)),
            target_type: CastTargetType::Float,
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn cast_duration_to_int_is_error() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::DurationTime {
                hours: 0,
                minutes: 10,
                seconds: 0,
            }),
            target_type: CastTargetType::Integer,
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn cast_datetime_to_int_is_error() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
            target_type: CastTargetType::Integer,
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn cast_list_to_int_is_error() {
        // Arrange
        let expr = Expression::TypeCast {
            operand: Box::new(Expression::List(vec![Expression::Integer(1)])),
            target_type: CastTargetType::Integer,
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    // --- Valid numeric binary operations ---

    #[test]
    fn integer_add_integer_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn float_multiply_integer_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::Float(1.5)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    // --- Valid time binary operations ---

    #[test]
    fn duration_add_duration_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::DurationTime {
                hours: 0,
                minutes: 30,
                seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn duration_multiply_integer_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::Integer(3)),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn integer_multiply_duration_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::Integer(3)),
            right: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn datetime_add_duration_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
            right: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn datetime_subtract_datetime_passes() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 2,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
            right: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    // --- Invalid binary operations ---

    #[test]
    fn datetime_add_datetime_is_error() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
            right: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 2,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn datetime_multiply_integer_is_error() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
            right: Box::new(Expression::Integer(2)),
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn duration_add_float_is_error() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::Float(1.5)),
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    // --- Unary operations ---

    #[test]
    fn negate_integer_passes() {
        // Arrange
        let expr = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Integer(5)),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn negate_duration_passes() {
        // Arrange
        let expr = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn negate_datetime_is_error() {
        // Arrange
        let expr = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }),
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    // --- Function argument validation ---

    #[test]
    fn sin_with_integer_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "sin".to_string(),
            arguments: vec![Expression::Float(3.14)],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn sin_with_duration_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "sin".to_string(),
            arguments: vec![Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn len_with_list_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "len".to_string(),
            arguments: vec![Expression::List(vec![Expression::Integer(1)])],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn len_with_integer_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "len".to_string(),
            arguments: vec![Expression::Integer(42)],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn datetime_constructor_with_integers_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "DateTime".to_string(),
            arguments: vec![
                Expression::Integer(2026),
                Expression::Integer(1),
                Expression::Integer(1),
                Expression::Integer(0),
                Expression::Integer(0),
                Expression::Integer(0),
            ],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn datetime_constructor_with_float_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "DateTime".to_string(),
            arguments: vec![
                Expression::Float(2026.0),
                Expression::Integer(1),
                Expression::Integer(1),
                Expression::Integer(0),
                Expression::Integer(0),
                Expression::Integer(0),
            ],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn timestamp_with_integer_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "Timestamp".to_string(),
            arguments: vec![Expression::Integer(1234567890)],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn timestamp_with_duration_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "Timestamp".to_string(),
            arguments: vec![Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn datetime_to_timestamp_with_datetime_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "datetime_to_timestamp".to_string(),
            arguments: vec![Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                offset_seconds: 0,
            }],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn datetime_to_timestamp_with_integer_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "datetime_to_timestamp".to_string(),
            arguments: vec![Expression::Integer(42)],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    // --- Time utility functions ---

    #[test]
    fn now_with_no_args_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "now".to_string(),
            arguments: vec![],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn now_with_args_is_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "now".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act & Assert
        assert!(check(expr).is_err());
    }

    #[test]
    fn today_with_no_args_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "today".to_string(),
            arguments: vec![],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    #[test]
    fn current_timestamp_with_no_args_passes() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "current_timestamp".to_string(),
            arguments: vec![],
        };

        // Act & Assert
        assert!(check(expr).is_ok());
    }

    // --- Unknown functions pass through ---

    #[test]
    fn unknown_function_is_type_error() {
        // Arrange
        let expr = Expression::FunctionCall {
            name: "unknown_func".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act
        let result = check(expr);

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("undefined function"),
            "expected 'undefined function' error, got: {err}"
        );
    }
}
