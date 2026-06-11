use domain::value::Value;

use crate::test_fixture::{call, int};

use super::eval;

// --- Time utility functions ---

#[test]
fn eval_now_returns_datetime() {
    // Arrange
    let expression = call("now", vec![]);

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert!(epoch.seconds() > 1_700_000_000); // after 2023
        assert!(offset.is_utc());
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_today_returns_datetime_at_midnight() {
    // Arrange
    let expression = call("today", vec![]);

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(epoch.seconds() % 86400, 0); // midnight
        assert!(offset.is_utc());
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_current_timestamp_returns_timestamp() {
    // Arrange
    let expression = call("current_timestamp", vec![]);

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
    let expression = call("now", vec![int(1)]);

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}
