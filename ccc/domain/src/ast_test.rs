#[cfg(test)]
mod tests {
    use crate::ast::{AbstractSyntaxTree, BinaryOperation, Expression, UnaryOperation};

    #[test]
    fn integer_expression() {
        // Arrange
        let expected = Expression::Integer(42);

        // Act
        let expr = Expression::Integer(42);

        // Assert
        assert_eq!(expr, expected);
    }

    #[test]
    fn float_expression() {
        // Arrange
        let expected = Expression::Float(3.14);

        // Act
        let expr = Expression::Float(3.14);

        // Assert
        assert_eq!(expr, expected);
    }

    #[test]
    fn binary_operation_expression() {
        // Arrange
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        };

        // Act
        let cloned = expr.clone();

        // Assert
        assert_eq!(cloned, expr);
    }

    #[test]
    fn nested_binary_operation() {
        // Arrange: (1 + 2) * 3
        let expr = Expression::BinaryOperation {
            operator: BinaryOperation::Multiply,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperation::Add,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(2)),
            }),
            right: Box::new(Expression::Integer(3)),
        };

        // Act
        let cloned = expr.clone();

        // Assert
        assert_eq!(cloned, expr);
    }

    #[test]
    fn unary_negate_expression() {
        // Arrange
        let expr = Expression::UnaryOperation {
            operator: UnaryOperation::Negate,
            operand: Box::new(Expression::Integer(5)),
        };

        // Act
        let cloned = expr.clone();

        // Assert
        assert_eq!(cloned, expr);
    }

    #[test]
    fn unary_positive_expression() {
        // Arrange
        let expr = Expression::UnaryOperation {
            operator: UnaryOperation::Positive,
            operand: Box::new(Expression::Float(2.5)),
        };

        // Act
        let cloned = expr.clone();

        // Assert
        assert_eq!(cloned, expr);
    }

    #[test]
    fn function_call_expression() {
        // Arrange
        let expected_name = "sqrt";
        let expected_arg_count = 1;

        // Act
        let expr = Expression::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![Expression::Integer(16)],
        };

        // Assert
        if let Expression::FunctionCall { name, arguments } = &expr {
            assert_eq!(name, expected_name);
            assert_eq!(arguments.len(), expected_arg_count);
        } else {
            panic!("Expected FunctionCall");
        }
    }

    #[test]
    fn function_call_with_no_arguments() {
        // Arrange & Act
        let expr = Expression::FunctionCall {
            name: "pi".to_string(),
            arguments: vec![],
        };

        // Assert
        if let Expression::FunctionCall { arguments, .. } = &expr {
            assert!(arguments.is_empty());
        } else {
            panic!("Expected FunctionCall");
        }
    }

    #[test]
    fn abstract_syntax_tree_wraps_expression() {
        // Arrange
        let expected_expression = Expression::Integer(99);

        // Act
        let ast = AbstractSyntaxTree {
            expression: Expression::Integer(99),
        };

        // Assert
        assert_eq!(ast.expression, expected_expression);
        assert_eq!(ast.clone(), ast);
    }

    #[test]
    fn all_binary_operations_are_distinct() {
        // Arrange
        let ops = vec![
            BinaryOperation::Add,
            BinaryOperation::Subtract,
            BinaryOperation::Multiply,
            BinaryOperation::Divide,
            BinaryOperation::Modulo,
            BinaryOperation::Power,
        ];

        // Act & Assert
        for (i, a) in ops.iter().enumerate() {
            for (j, b) in ops.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn unary_operations_are_distinct() {
        // Arrange
        let negate = UnaryOperation::Negate;
        let positive = UnaryOperation::Positive;

        // Act & Assert
        assert_ne!(negate, positive);
    }

    #[test]
    fn debug_format_is_available() {
        // Arrange
        let expr = Expression::Integer(1);

        // Act
        let debug = format!("{expr:?}");

        // Assert
        assert!(debug.contains("Integer"));
    }
}
