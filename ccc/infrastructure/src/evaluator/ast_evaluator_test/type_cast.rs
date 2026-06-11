use domain::ast::CastTargetType;
use domain::value::Value;

use crate::test_fixture::{cast, float, int, neg};

use super::eval;

// --- Type cast ---

#[test]
fn cast_integer_to_float() {
    // Arrange: 3 as float → 3.0
    let expression = cast(int(3), CastTargetType::Float);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(3.0));
}

#[test]
fn cast_zero_to_float() {
    // Arrange: 0 as float → 0.0
    let expression = cast(int(0), CastTargetType::Float);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(0.0));
}

#[test]
fn cast_negative_integer_to_float() {
    // Arrange: -5 as float → -5.0
    let expression = cast(neg(int(5)), CastTargetType::Float);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Float(-5.0));
}

#[test]
fn cast_float_to_int_truncates() {
    // Arrange: 3.7 as int → 3
    let expression = cast(float(3.7), CastTargetType::Integer);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(3));
}

#[test]
fn cast_negative_float_to_int_truncates_toward_zero() {
    // Arrange: -2.9 as int → -2
    let expression = cast(neg(float(2.9)), CastTargetType::Integer);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(-2));
}

#[test]
fn cast_zero_float_to_int() {
    // Arrange: 0.0 as int → 0
    let expression = cast(float(0.0), CastTargetType::Integer);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(0));
}

#[test]
fn cast_integer_to_int_is_identity() {
    // Arrange: 3 as int → 3
    let expression = cast(int(3), CastTargetType::Integer);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Integer(3));
}
