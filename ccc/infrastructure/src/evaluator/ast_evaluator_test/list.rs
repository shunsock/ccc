use domain::error::CccError;
use domain::value::Value;

use crate::test_fixture::{add, call, int, list, mul};

use super::eval;

// --- List evaluation ---

#[test]
fn eval_empty_list() {
    // Arrange
    let expression = list(vec![]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::List(vec![]));
}

#[test]
fn eval_list_with_integers() {
    // Arrange
    let expression = list(vec![int(1), int(2), int(3)]);

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
    let expression = list(vec![add(int(1), int(2)), mul(int(3), int(4))]);

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
    let expression = list(vec![list(vec![int(1)]), list(vec![int(2)])]);

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

// --- Aggregate overflow ---

#[test]
fn eval_sum_overflow_is_error() {
    // Arrange: sum([i64::MAX, 1])
    let expression = call("sum", vec![list(vec![int(i64::MAX), int(1)])]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("sum: integer overflow".to_string())
    );
}

#[test]
fn eval_prod_overflow_is_error() {
    // Arrange: prod([i64::MAX, 2])
    let expression = call("prod", vec![list(vec![int(i64::MAX), int(2)])]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("prod: integer overflow".to_string())
    );
}

#[test]
fn eval_variance_of_durations_reports_unsupported_elements() {
    // Arrange: elements are homogeneous, so the error must name the unsupported
    // type instead of claiming a type mismatch
    let expression = call(
        "variance",
        vec![list(vec![
            crate::test_fixture::duration_literal(0, 1, 30),
            crate::test_fixture::duration_literal(0, 0, 30),
        ])],
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("variance: expected numeric list elements, got duration".to_string())
    );
}
