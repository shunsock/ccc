use domain::error::CccError;
use domain::value::Value;

use crate::test_fixture::{add, call, float, int, list};

use super::eval;

// --- Math built-in functions ---

#[test]
fn sqrt_integer() {
    // Arrange
    let expression = call("sqrt", vec![int(16)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(4.0));
}

#[test]
fn abs_positive_integer() {
    // Arrange
    let expression = call("abs", vec![int(5)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(5));
}

#[test]
fn abs_negative_integer() {
    // Arrange
    let expression = call("abs", vec![int(-5)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(5));
}

#[test]
fn abs_negative_float() {
    // Arrange
    let expression = call("abs", vec![float(-2.5)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(2.5));
}

#[test]
fn sin_zero() {
    // Arrange
    let expression = call("sin", vec![int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn cos_zero() {
    // Arrange
    let expression = call("cos", vec![int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(1.0));
}

#[test]
fn tan_zero() {
    // Arrange
    let expression = call("tan", vec![int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn arcsin_zero() {
    // Arrange
    let expression = call("arcsin", vec![int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn arccos_one() {
    // Arrange
    let expression = call("arccos", vec![int(1)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn arctan_zero() {
    // Arrange
    let expression = call("arctan", vec![int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn log_natural() {
    // Arrange: log(1) = 0
    let expression = call("log", vec![int(1)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn log_with_base() {
    // Arrange: log(2, 8) = 3
    let expression = call("log", vec![int(2), int(8)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(3.0));
}

#[test]
fn log_base10() {
    // Arrange: log(10, 100) = 2
    let expression = call("log", vec![int(10), int(100)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(2.0));
}

#[test]
fn ln_natural() {
    // Arrange: ln(1) = 0
    let expression = call("ln", vec![int(1)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn floor_float() {
    // Arrange
    let expression = call("floor", vec![float(3.7)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(3.0));
}

#[test]
fn ceil_float() {
    // Arrange
    let expression = call("ceil", vec![float(3.2)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(4.0));
}

#[test]
fn round_float() {
    // Arrange
    let expression = call("round", vec![float(3.5)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(4.0));
}

// --- Function error cases ---

#[test]
fn unknown_function() {
    // Arrange
    let expression = call("unknown", vec![int(1)]);

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
    let expression = call("sqrt", vec![int(4), int(9)]);

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
    let expression = call("abs", vec![]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("abs expects 1 argument, got 0".to_string())
    );
}

// --- Function with expression argument ---

#[test]
fn function_with_expression_argument() {
    // Arrange: sqrt(1 + 3) = sqrt(4) = 2.0
    let expression = call("sqrt", vec![add(int(1), int(3))]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(2.0));
}

// --- List aggregate functions ---

#[test]
fn cast_expression_result_to_int() {
    // Arrange: E([1, 2, 3]) as int → mean([1,2,3]) = 2.0 → 2
    use crate::test_fixture::cast;
    use domain::ast::CastTargetType;
    let expression = cast(
        call("mean", vec![list(vec![int(1), int(2), int(3)])]),
        CastTargetType::Integer,
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(2));
}
