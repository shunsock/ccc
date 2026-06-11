use domain::ast::Expression;

use crate::test_fixture::{add, call, int, list};

use super::parse_expr;

// --- Function calls ---

#[test]
fn parse_function_call_single_arg() {
    // Arrange
    let input = "sqrt(16)";
    let expected = call("sqrt", vec![int(16)]);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_function_call_with_expression_arg() {
    // Arrange
    let input = "sqrt(1 + 3)";
    let expected = call("sqrt", vec![add(int(1), int(3))]);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_nested_function_call() {
    // Arrange: sin(sqrt(9))
    let input = "sin(sqrt(9))";
    let expected = call("sin", vec![call("sqrt", vec![int(9)])]);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_function_call_no_args() {
    // Arrange
    let input = "pi()";
    let expected = call("pi", vec![]);

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
        assert_eq!(arguments[0], int(1));
        assert_eq!(arguments[1], int(2));
        assert_eq!(arguments[2], int(3));
    } else {
        panic!("Expected FunctionCall");
    }
}

// --- Method chain syntax ---

#[test]
fn parse_method_call_sum() {
    // Arrange: [1, 2, 3].sum() desugars to sum([1, 2, 3])
    let input = "[1, 2, 3].sum()";
    let expected = call("sum", vec![list(vec![int(1), int(2), int(3)])]);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_method_chain_multiple() {
    // Arrange: [1, 2, 3].tail().sum() desugars to sum(tail([1, 2, 3]))
    let input = "[1, 2, 3].tail().sum()";
    let expected = call(
        "sum",
        vec![call("tail", vec![list(vec![int(1), int(2), int(3)])])],
    );

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}
