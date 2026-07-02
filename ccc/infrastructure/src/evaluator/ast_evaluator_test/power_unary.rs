use domain::error::CccError;
use domain::value::Value;

use crate::test_fixture::{float, int, neg, pos, pow};

use super::eval;

// --- Power ---

#[test]
fn power_integers() {
    // Arrange
    let expression = pow(int(2), int(10));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(1024));
}

#[test]
fn power_negative_exponent_promotes_to_float() {
    // Arrange
    let expression = pow(int(2), int(-1));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.5));
}

#[test]
fn power_float_base() {
    // Arrange
    let expression = pow(float(2.0), int(3));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(8.0));
}

#[test]
fn power_overflow_reports_error() {
    // Arrange: 2 ^ 64 exceeds i64::MAX; the message uses the canonical `^` symbol
    let expression = pow(int(2), int(64));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("integer overflow: 2 ^ 64".to_string())
    );
}

// --- Unary operators ---

#[test]
fn negate_integer() {
    // Arrange
    let expression = neg(int(5));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(-5));
}

#[test]
fn negate_float() {
    // Arrange
    let expression = neg(float(2.5));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(-2.5));
}

#[test]
fn negate_integer_overflow() {
    // Arrange
    let expression = neg(int(i64::MIN));

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
    let expression = pos(int(7));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(7));
}
