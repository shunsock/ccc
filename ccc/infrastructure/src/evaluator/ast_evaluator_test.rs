#[cfg(test)]
mod tests {
    use domain::ast::{AbstractSyntaxTree, BinaryOperation, CastTargetType, Expression, UnaryOperation};
    use domain::error::CccError;
    use domain::interface::evaluator::CccEvaluator;
    use domain::value::Value;

    use crate::evaluator::AstEvaluator;

    fn eval(expression: Expression) -> Result<Value, CccError> {
        let evaluator = AstEvaluator;
        let ast = AbstractSyntaxTree { expression };
        evaluator.evaluate(&ast)
    }

    // --- Literals ---

    #[test]
    fn eval_integer() {
        // Arrange
        let expression = Expression::Integer(42);

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(42));
    }

    #[test]
    fn eval_float() {
        // Arrange
        let expression = Expression::Float(3.14);

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.14));
    }

    // --- Addition ---

    #[test]
    fn add_integers() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(3));
    }

    #[test]
    fn add_integer_and_float() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Float(2.5)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.5));
    }

    #[test]
    fn add_floats() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Float(1.1)),
            right: Box::new(Expression::Float(2.2)),
        };

        // Act
        let result = eval(expression);

        // Assert
        if let Value::Float(n) = result.unwrap() {
            assert!((n - 3.3).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    // --- Subtraction ---

    #[test]
    fn subtract_integers() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::Integer(5)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(2));
    }

    // --- Multiplication ---

    #[test]
    fn multiply_integers() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::Integer(3)),
            right: Box::new(Expression::Integer(4)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(12));
    }

    // --- Division ---

    #[test]
    fn divide_integers_exact() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::Integer(10)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(5));
    }

    #[test]
    fn divide_integers_with_remainder_promotes_to_float() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::Integer(7)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.5));
    }

    #[test]
    fn divide_by_zero_integer() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(0)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("division by zero: 1 / 0".to_string())
        );
    }

    #[test]
    fn divide_by_zero_float() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::Float(1.0)),
            right: Box::new(Expression::Float(0.0)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("division by zero: 1 / 0".to_string())
        );
    }

    // --- Modulo ---

    #[test]
    fn modulo_integers() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Modulo,
            left: Box::new(Expression::Integer(7)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(1));
    }

    #[test]
    fn modulo_by_zero() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Modulo,
            left: Box::new(Expression::Integer(7)),
            right: Box::new(Expression::Integer(0)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("modulo by zero: 7 % 0".to_string())
        );
    }

    #[test]
    fn modulo_floats() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Modulo,
            left: Box::new(Expression::Float(7.5)),
            right: Box::new(Expression::Float(2.0)),
        };

        // Act
        let result = eval(expression);

        // Assert
        if let Value::Float(n) = result.unwrap() {
            assert!((n - 1.5).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    // --- Power ---

    #[test]
    fn power_integers() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::Integer(10)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(1024));
    }

    #[test]
    fn power_negative_exponent_promotes_to_float() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Integer(2)),
            right: Box::new(Expression::Integer(-1)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.5));
    }

    #[test]
    fn power_float_base() {
        // Arrange
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Power,
            left: Box::new(Expression::Float(2.0)),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(8.0));
    }

    // --- Unary operators ---

    #[test]
    fn negate_integer() {
        // Arrange
        let expression = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Integer(5)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(-5));
    }

    #[test]
    fn negate_float() {
        // Arrange
        let expression = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Float(2.5)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(-2.5));
    }

    #[test]
    fn negate_integer_overflow() {
        // Arrange
        let expression = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Integer(i64::MIN)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("integer negation overflow".to_string())
        );
    }

    #[test]
    fn positive_is_identity() {
        // Arrange
        let expression = Expression::UnaryOperation {
            operator: UnaryOperation::Positive,
            operand: Box::new(Expression::Integer(7)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(7));
    }

    // --- Type cast ---

    #[test]
    fn cast_integer_to_float() {
        // Arrange: 3 as float → 3.0
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::Integer(3)),
            target_type: CastTargetType::Float,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.0));
    }

    #[test]
    fn cast_zero_to_float() {
        // Arrange: 0 as float → 0.0
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::Integer(0)),
            target_type: CastTargetType::Float,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn cast_negative_integer_to_float() {
        // Arrange: -5 as float → -5.0
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::UnaryOperation {
                operator: UnaryOperation::Negate,
                operand: Box::new(Expression::Integer(5)),
            }),
            target_type: CastTargetType::Float,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(-5.0));
    }

    #[test]
    fn cast_float_to_int_truncates() {
        // Arrange: 3.7 as int → 3
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::Float(3.7)),
            target_type: CastTargetType::Integer,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(3));
    }

    #[test]
    fn cast_negative_float_to_int_truncates_toward_zero() {
        // Arrange: -2.9 as int → -2
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::UnaryOperation {
                operator: UnaryOperation::Negate,
                operand: Box::new(Expression::Float(2.9)),
            }),
            target_type: CastTargetType::Integer,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(-2));
    }

    #[test]
    fn cast_zero_float_to_int() {
        // Arrange: 0.0 as int → 0
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::Float(0.0)),
            target_type: CastTargetType::Integer,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(0));
    }

    #[test]
    fn cast_integer_to_int_is_identity() {
        // Arrange: 3 as int → 3
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::Integer(3)),
            target_type: CastTargetType::Integer,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(3));
    }

    #[test]
    fn cast_expression_result_to_int() {
        // Arrange: E([1, 2, 3]) as int → mean([1,2,3]) = 2.0 → 2
        let expression = Expression::TypeCast {
            operand: Box::new(Expression::FunctionCall {
                name: "mean".to_string(),
                arguments: vec![Expression::List(vec![
                    Expression::Integer(1),
                    Expression::Integer(2),
                    Expression::Integer(3),
                ])],
            }),
            target_type: CastTargetType::Integer,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(2));
    }

    // --- Builtin functions ---

    #[test]
    fn sqrt_integer() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::Integer(16)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(4.0));
    }

    #[test]
    fn abs_positive_integer() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "abs".to_string(),
            arguments: vec![Expression::Integer(5)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(5));
    }

    #[test]
    fn abs_negative_integer() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "abs".to_string(),
            arguments: vec![Expression::Integer(-5)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(5));
    }

    #[test]
    fn abs_negative_float() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "abs".to_string(),
            arguments: vec![Expression::Float(-3.14)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.14));
    }

    #[test]
    fn sin_zero() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "sin".to_string(),
            arguments: vec![Expression::Integer(0)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn cos_zero() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "cos".to_string(),
            arguments: vec![Expression::Integer(0)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(1.0));
    }

    #[test]
    fn tan_zero() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "tan".to_string(),
            arguments: vec![Expression::Integer(0)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn arcsin_zero() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "arcsin".to_string(),
            arguments: vec![Expression::Integer(0)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn arccos_one() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "arccos".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn arctan_zero() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "arctan".to_string(),
            arguments: vec![Expression::Integer(0)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn log_one() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "log".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(0.0));
    }

    #[test]
    fn log2_eight() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "log2".to_string(),
            arguments: vec![Expression::Integer(8)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.0));
    }

    #[test]
    fn log10_hundred() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "log10".to_string(),
            arguments: vec![Expression::Integer(100)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(2.0));
    }

    #[test]
    fn floor_float() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "floor".to_string(),
            arguments: vec![Expression::Float(3.7)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(3.0));
    }

    #[test]
    fn ceil_float() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "ceil".to_string(),
            arguments: vec![Expression::Float(3.2)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(4.0));
    }

    #[test]
    fn round_float() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "round".to_string(),
            arguments: vec![Expression::Float(3.5)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(4.0));
    }

    // --- Function error cases ---

    #[test]
    fn unknown_function() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "unknown".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("unknown function: unknown".to_string())
        );
    }

    #[test]
    fn sqrt_wrong_argument_count() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::Integer(4), Expression::Integer(9)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("sqrt expects 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn abs_no_arguments() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "abs".to_string(),
            arguments: vec![],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap_err(),
            CccError::eval("abs expects 1 argument, got 0".to_string())
        );
    }

    // --- Nested expressions ---

    #[test]
    fn nested_arithmetic() {
        // Arrange: (1 + 2) * (3 + 4) = 21
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(2)),
            }),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(3)),
                right: Box::new(Expression::Integer(4)),
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Integer(21));
    }

    #[test]
    fn function_with_expression_argument() {
        // Arrange: sqrt(1 + 3) = sqrt(4) = 2.0
        let expression = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(3)),
            }],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Float(2.0));
    }

    // --- List evaluation ---

    #[test]
    fn eval_empty_list() {
        // Arrange
        let expression = Expression::List(vec![]);

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::List(vec![]));
    }

    #[test]
    fn eval_list_with_integers() {
        // Arrange
        let expression = Expression::List(vec![
            Expression::Integer(1),
            Expression::Integer(2),
            Expression::Integer(3),
        ]);

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap(),
            Value::List(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ])
        );
    }

    #[test]
    fn eval_list_with_expressions() {
        // Arrange: [1+2, 3*4]
        let expression = Expression::List(vec![
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
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap(),
            Value::List(vec![Value::Integer(3), Value::Integer(12)])
        );
    }

    #[test]
    fn eval_nested_list() {
        // Arrange
        let expression = Expression::List(vec![
            Expression::List(vec![Expression::Integer(1)]),
            Expression::List(vec![Expression::Integer(2)]),
        ]);

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap(),
            Value::List(vec![
                Value::List(vec![Value::Integer(1)]),
                Value::List(vec![Value::Integer(2)]),
            ])
        );
    }

    // --- DurationTime ---

    #[test]
    fn eval_duration_time_literal() {
        // Arrange: 10:20:30
        let expression = Expression::DurationTime {
            hours: 10,
            minutes: 20,
            seconds: 30,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap(),
            Value::DurationTime(10 * 3600 + 20 * 60 + 30)
        );
    }

    #[test]
    fn eval_duration_time_zero() {
        // Arrange
        let expression = Expression::DurationTime {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(0));
    }

    #[test]
    fn eval_duration_time_constructor_3_args() {
        // Arrange: DurationTime(1, 30, 0) = 1:30:00
        let expression = Expression::FunctionCall {
            name: "DurationTime".to_string(),
            arguments: vec![
                Expression::Integer(1),
                Expression::Integer(30),
                Expression::Integer(0),
            ],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(1 * 3600 + 30 * 60));
    }

    #[test]
    fn eval_duration_time_constructor_4_args_with_days() {
        // Arrange: DurationTime(1, 2, 30, 0) = 1 day + 2:30:00 = 26:30:00
        let expression = Expression::FunctionCall {
            name: "DurationTime".to_string(),
            arguments: vec![
                Expression::Integer(1),
                Expression::Integer(2),
                Expression::Integer(30),
                Expression::Integer(0),
            ],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(
            result.unwrap(),
            Value::DurationTime(1 * 86400 + 2 * 3600 + 30 * 60)
        );
    }

    #[test]
    fn eval_duration_time_constructor_wrong_arg_count() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "DurationTime".to_string(),
            arguments: vec![Expression::Integer(1), Expression::Integer(2)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_negate_duration_time() {
        // Arrange: -1:00:00
        let expression = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(-3600));
    }

    // --- DateTime ---

    #[test]
    fn eval_datetime_literal_utc() {
        // Arrange: 2026-01-01T00:00:00 (UTC)
        let expression = Expression::DateTime {
            year: 2026,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            offset_seconds: 0,
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(offset_seconds, 0);
            // Verify round-trip
            let (y, m, d, h, mi, s) = domain::value::epoch_seconds_to_calendar(epoch_seconds);
            assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 0, 0, 0));
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_datetime_literal_with_offset() {
        // Arrange: 2026-01-01T09:00:00+09:00
        let expression = Expression::DateTime {
            year: 2026,
            month: 1,
            day: 1,
            hour: 9,
            minute: 0,
            second: 0,
            offset_seconds: 9 * 3600,
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert: same UTC instant as 2026-01-01T00:00:00Z
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(offset_seconds, 9 * 3600);
            let (y, m, d, h, mi, s) = domain::value::epoch_seconds_to_calendar(epoch_seconds);
            assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 0, 0, 0));
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_datetime_constructor() {
        // Arrange: DateTime(2026, 6, 15, 12, 30, 0)
        let expression = Expression::FunctionCall {
            name: "DateTime".to_string(),
            arguments: vec![
                Expression::Integer(2026),
                Expression::Integer(6),
                Expression::Integer(15),
                Expression::Integer(12),
                Expression::Integer(30),
                Expression::Integer(0),
            ],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(offset_seconds, 0); // constructor defaults to UTC
            let (y, m, d, h, mi, s) = domain::value::epoch_seconds_to_calendar(epoch_seconds);
            assert_eq!((y, m, d, h, mi, s), (2026, 6, 15, 12, 30, 0));
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_datetime_constructor_wrong_arg_count() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "DateTime".to_string(),
            arguments: vec![Expression::Integer(2026), Expression::Integer(1)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_datetime_constructor_invalid_month() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "DateTime".to_string(),
            arguments: vec![
                Expression::Integer(2026),
                Expression::Integer(13),
                Expression::Integer(1),
                Expression::Integer(0),
                Expression::Integer(0),
                Expression::Integer(0),
            ],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_negate_datetime_is_error() {
        // Arrange
        let expression = Expression::UnaryOperation {
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

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    // --- Timestamp ---

    #[test]
    fn eval_timestamp_constructor_integer() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "Timestamp".to_string(),
            arguments: vec![Expression::Integer(1234567890)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Timestamp(1234567890.0));
    }

    #[test]
    fn eval_timestamp_constructor_float() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "Timestamp".to_string(),
            arguments: vec![Expression::Float(1234567890.5)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Timestamp(1234567890.5));
    }

    #[test]
    fn eval_timestamp_constructor_wrong_arg_count() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "Timestamp".to_string(),
            arguments: vec![],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_to_timestamp_from_datetime() {
        // Arrange: to_timestamp(DateTime(2026,1,1,0,0,0))
        let expression = Expression::FunctionCall {
            name: "datetime_to_timestamp".to_string(),
            arguments: vec![Expression::FunctionCall {
                name: "DateTime".to_string(),
                arguments: vec![
                    Expression::Integer(2026),
                    Expression::Integer(1),
                    Expression::Integer(1),
                    Expression::Integer(0),
                    Expression::Integer(0),
                    Expression::Integer(0),
                ],
            }],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::Timestamp(ts) = result {
            assert_eq!(ts, 1_767_225_600.0);
        } else {
            panic!("expected Timestamp");
        }
    }

    #[test]
    fn eval_to_datetime_from_timestamp() {
        // Arrange: to_datetime(Timestamp(0))
        let expression = Expression::FunctionCall {
            name: "timestamp_to_datetime".to_string(),
            arguments: vec![Expression::FunctionCall {
                name: "Timestamp".to_string(),
                arguments: vec![Expression::Integer(0)],
            }],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        assert_eq!(
            result,
            Value::DateTime {
                epoch_seconds: 0,
                offset_seconds: 0,
            }
        );
    }

    #[test]
    fn eval_to_datetime_with_timezone_offset() {
        // Arrange: to_datetime(Timestamp(0), 9)
        let expression = Expression::FunctionCall {
            name: "timestamp_to_datetime".to_string(),
            arguments: vec![
                Expression::FunctionCall {
                    name: "Timestamp".to_string(),
                    arguments: vec![Expression::Integer(0)],
                },
                Expression::Integer(9),
            ],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        assert_eq!(
            result,
            Value::DateTime {
                epoch_seconds: 0,
                offset_seconds: 9 * 3600,
            }
        );
    }

    // --- Time arithmetic ---

    #[test]
    fn eval_duration_add_duration() {
        // Arrange: 1:00:00 + 0:30:00
        let expression = Expression::BinaryOperation {
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

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(5400));
    }

    #[test]
    fn eval_duration_subtract_duration() {
        // Arrange: 2:00:00 - 0:30:00
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::DurationTime {
                hours: 2,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::DurationTime {
                hours: 0,
                minutes: 30,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(5400));
    }

    #[test]
    fn eval_duration_negative_result() {
        // Arrange: 1:00:00 - 2:00:00
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::DurationTime {
                hours: 2,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(-3600));
    }

    #[test]
    fn eval_duration_multiply_integer() {
        // Arrange: 1:00:00 * 3
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(10800));
    }

    #[test]
    fn eval_integer_multiply_duration() {
        // Arrange: 3 * 1:00:00
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::Integer(3)),
            right: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(10800));
    }

    #[test]
    fn eval_duration_divide_integer() {
        // Arrange: 3:00:00 / 2
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::DurationTime {
                hours: 3,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(5400));
    }

    #[test]
    fn eval_datetime_add_duration() {
        // Arrange: 2026-01-01T00:00:00Z + 1:30:00
        let expression = Expression::BinaryOperation {
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
                minutes: 30,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(offset_seconds, 0);
            let (y, m, d, h, mi, s) = domain::value::epoch_seconds_to_calendar(epoch_seconds);
            assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 1, 30, 0));
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_datetime_subtract_datetime() {
        // Arrange: 2026-01-02T00:00:00Z - 2026-01-01T00:00:00Z
        let expression = Expression::BinaryOperation {
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

        // Act
        let result = eval(expression);

        // Assert: 24 hours = 86400 seconds
        assert_eq!(result.unwrap(), Value::DurationTime(86400));
    }

    #[test]
    fn eval_datetime_preserves_timezone() {
        // Arrange: 2026-01-01T09:00:00+09:00 + 1:00:00
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 9,
                minute: 0,
                second: 0,
                offset_seconds: 9 * 3600,
            }),
            right: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert: offset preserved
        if let Value::DateTime { offset_seconds, .. } = result {
            assert_eq!(offset_seconds, 9 * 3600);
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_timestamp_add_duration() {
        // Arrange: Timestamp(0) + 1:00:00
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::FunctionCall {
                name: "Timestamp".to_string(),
                arguments: vec![Expression::Integer(0)],
            }),
            right: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::Timestamp(3600.0));
    }

    #[test]
    fn eval_timestamp_subtract_timestamp() {
        // Arrange: Timestamp(7200) - Timestamp(3600)
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Subtract,
            left: Box::new(Expression::FunctionCall {
                name: "Timestamp".to_string(),
                arguments: vec![Expression::Integer(7200)],
            }),
            right: Box::new(Expression::FunctionCall {
                name: "Timestamp".to_string(),
                arguments: vec![Expression::Integer(3600)],
            }),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert_eq!(result.unwrap(), Value::DurationTime(3600));
    }

    #[test]
    fn eval_datetime_add_datetime_is_error() {
        // Arrange: DateTime + DateTime
        let expression = Expression::BinaryOperation {
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

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_duration_divide_zero_is_error() {
        // Arrange: 1:00:00 / 0
        let expression = Expression::BinaryOperation {
            operator: BinaryOperation::Divide,
            left: Box::new(Expression::DurationTime {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }),
            right: Box::new(Expression::Integer(0)),
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn eval_round_trip_datetime_timestamp() {
        // Arrange: to_datetime(to_timestamp(DateTime(2026,6,15,12,30,0)))
        let expression = Expression::FunctionCall {
            name: "timestamp_to_datetime".to_string(),
            arguments: vec![Expression::FunctionCall {
                name: "datetime_to_timestamp".to_string(),
                arguments: vec![Expression::FunctionCall {
                    name: "DateTime".to_string(),
                    arguments: vec![
                        Expression::Integer(2026),
                        Expression::Integer(6),
                        Expression::Integer(15),
                        Expression::Integer(12),
                        Expression::Integer(30),
                        Expression::Integer(0),
                    ],
                }],
            }],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(offset_seconds, 0);
            let (y, m, d, h, mi, s) = domain::value::epoch_seconds_to_calendar(epoch_seconds);
            assert_eq!((y, m, d, h, mi, s), (2026, 6, 15, 12, 30, 0));
        } else {
            panic!("expected DateTime");
        }
    }

    // --- Time utility functions ---

    #[test]
    fn eval_now_returns_datetime() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "now".to_string(),
            arguments: vec![],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert!(epoch_seconds > 1_700_000_000); // after 2023
            assert_eq!(offset_seconds, 0); // UTC
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_today_returns_datetime_at_midnight() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "today".to_string(),
            arguments: vec![],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::DateTime {
            epoch_seconds,
            offset_seconds,
        } = result
        {
            assert_eq!(epoch_seconds % 86400, 0); // midnight
            assert_eq!(offset_seconds, 0);
        } else {
            panic!("expected DateTime");
        }
    }

    #[test]
    fn eval_current_timestamp_returns_timestamp() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "current_timestamp".to_string(),
            arguments: vec![],
        };

        // Act
        let result = eval(expression).unwrap();

        // Assert
        if let Value::Timestamp(ts) = result {
            assert!(ts > 1_700_000_000.0);
        } else {
            panic!("expected Timestamp");
        }
    }

    #[test]
    fn eval_now_with_args_is_error() {
        // Arrange
        let expression = Expression::FunctionCall {
            name: "now".to_string(),
            arguments: vec![Expression::Integer(1)],
        };

        // Act
        let result = eval(expression);

        // Assert
        assert!(result.is_err());
    }
}
