use domain::error::CccError;

use crate::test_fixture::{add, call, div, duration_literal, duration_value, int, mul, neg, sub};

use super::eval;

// --- DurationTime literals ---

#[test]
fn eval_duration_time_literal() {
    // Arrange: 10:20:30
    let expression = duration_literal(10, 20, 30);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(10 * 3600 + 20 * 60 + 30));
}

#[test]
fn eval_duration_time_zero() {
    // Arrange
    let expression = duration_literal(0, 0, 0);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(0));
}

// --- DurationTime constructors ---

#[allow(clippy::identity_op)]
#[test]
fn eval_duration_time_constructor_3_args() {
    // Arrange: DurationTime(1, 30, 0) = 1:30:00
    let expression = call("DurationTime", vec![int(1), int(30), int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(1 * 3600 + 30 * 60));
}

#[allow(clippy::identity_op)]
#[test]
fn eval_duration_time_constructor_4_args_with_days() {
    // Arrange: DurationTime(1, 2, 30, 0) = 1 day + 2:30:00 = 26:30:00
    let expression = call("DurationTime", vec![int(1), int(2), int(30), int(0)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap(),
        duration_value(1 * 86400 + 2 * 3600 + 30 * 60)
    );
}

#[test]
fn eval_duration_time_constructor_wrong_arg_count() {
    // Arrange
    let expression = call("DurationTime", vec![int(1), int(2)]);

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

// --- DurationTime unary ---

#[test]
fn eval_negate_duration_time() {
    // Arrange: -1:00:00
    let expression = neg(duration_literal(1, 0, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(-3600));
}

// --- DurationTime arithmetic ---

#[test]
fn eval_duration_add_duration() {
    // Arrange: 1:00:00 + 0:30:00
    let expression = add(duration_literal(1, 0, 0), duration_literal(0, 30, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(5400));
}

#[test]
fn eval_duration_subtract_duration() {
    // Arrange: 2:00:00 - 0:30:00
    let expression = sub(duration_literal(2, 0, 0), duration_literal(0, 30, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(5400));
}

#[test]
fn eval_duration_negative_result() {
    // Arrange: 1:00:00 - 2:00:00
    let expression = sub(duration_literal(1, 0, 0), duration_literal(2, 0, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(-3600));
}

#[test]
fn eval_duration_multiply_integer() {
    // Arrange: 1:00:00 * 3
    let expression = mul(duration_literal(1, 0, 0), int(3));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(10800));
}

#[test]
fn eval_integer_multiply_duration() {
    // Arrange: 3 * 1:00:00
    let expression = mul(int(3), duration_literal(1, 0, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(10800));
}

#[test]
fn eval_duration_divide_integer() {
    // Arrange: 3:00:00 / 2
    let expression = div(duration_literal(3, 0, 0), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(5400));
}

#[test]
fn eval_duration_divide_zero_is_error() {
    // Arrange: 1:00:00 / 0
    let expression = div(duration_literal(1, 0, 0), int(0));

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

// --- Overflow ---

#[test]
fn eval_duration_literal_overflow_is_error() {
    // Arrange: 9999999999999999 hours exceed the i64 seconds range
    let expression = duration_literal(9_999_999_999_999_999, 0, 0);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("duration out of range".to_string())
    );
}

#[test]
fn eval_duration_constructor_overflow_is_error() {
    // Arrange: DurationTime(99999999999999999 days, 0, 0, 0)
    let expression = call(
        "DurationTime",
        vec![int(99_999_999_999_999_999), int(0), int(0), int(0)],
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("DurationTime: duration out of range".to_string())
    );
}

#[test]
fn eval_duration_add_overflow_is_error() {
    // Arrange: i64::MAX seconds is 2562047788015215:30:07; adding one second overflows
    let expression = add(
        duration_literal(2_562_047_788_015_215, 30, 7),
        duration_literal(0, 0, 1),
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("duration out of range".to_string())
    );
}

#[test]
fn eval_duration_multiply_overflow_is_error() {
    // Arrange
    let expression = mul(duration_literal(2_562_047_788_015_215, 0, 0), int(2));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("duration out of range".to_string())
    );
}
