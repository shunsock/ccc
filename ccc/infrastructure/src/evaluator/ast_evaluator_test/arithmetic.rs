use domain::error::CccError;
use domain::value::Value;

use crate::test_fixture::{add, div, float, int, modulo, mul, sub};

use super::eval;

// --- Addition ---

#[test]
fn add_integers() {
    // Arrange
    let expression = add(int(1), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(3));
}

#[test]
fn add_integer_and_float() {
    // Arrange
    let expression = add(int(1), float(2.5));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(3.5));
}

#[test]
fn add_floats() {
    // Arrange
    let expression = add(float(1.1), float(2.2));

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
    let expression = sub(int(5), int(3));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(2));
}

// --- Multiplication ---

#[test]
fn multiply_integers() {
    // Arrange
    let expression = mul(int(3), int(4));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(12));
}

// --- Division ---

#[test]
fn divide_integers_exact() {
    // Arrange
    let expression = div(int(10), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(5));
}

#[test]
fn divide_integers_with_remainder_promotes_to_float() {
    // Arrange
    let expression = div(int(7), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(3.5));
}

#[test]
fn divide_by_zero_integer() {
    // Arrange
    let expression = div(int(1), int(0));

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
    let expression = div(float(1.0), float(0.0));

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
    let expression = modulo(int(7), int(3));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(1));
}

#[test]
fn modulo_by_zero() {
    // Arrange
    let expression = modulo(int(7), int(0));

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
    let expression = modulo(float(7.5), float(2.0));

    // Act
    let result = eval(expression);

    // Assert
    if let Value::Float(n) = result.unwrap() {
        assert!((n - 1.5).abs() < 1e-10);
    } else {
        panic!("Expected Float");
    }
}

// --- Integer overflow ---

#[test]
fn add_overflow_reports_error() {
    // Arrange
    let expression = add(int(i64::MAX), int(1));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval(format!("integer overflow: {} + 1", i64::MAX))
    );
}

#[test]
fn subtract_overflow_reports_error() {
    // Arrange
    let expression = sub(int(i64::MIN), int(1));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval(format!("integer overflow: {} - 1", i64::MIN))
    );
}

#[test]
fn multiply_overflow_reports_error() {
    // Arrange
    let expression = mul(int(i64::MAX), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval(format!("integer overflow: {} * 2", i64::MAX))
    );
}

// --- Nested expressions ---

#[test]
fn nested_arithmetic() {
    // Arrange: (1 + 2) * (3 + 4) = 21
    let expression = mul(add(int(1), int(2)), add(int(3), int(4)));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(21));
}
