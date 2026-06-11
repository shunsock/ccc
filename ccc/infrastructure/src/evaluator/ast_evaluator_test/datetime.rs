use domain::time::UtcOffset;
use domain::value::Value;

use crate::test_fixture::{add, call, datetime_literal, duration_literal, int, sub};

use super::eval;

// --- DateTime literals ---

#[test]
fn eval_datetime_literal_utc() {
    // Arrange: 2026-01-01T00:00:00 (UTC)
    let expression = datetime_literal(2026, 1, 1, 0, 0, 0, 0);

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(offset, UtcOffset::UTC);
        // Verify round-trip
        let (y, m, d, h, mi, s) = epoch.to_calendar();
        assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 0, 0, 0));
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_datetime_literal_with_offset() {
    // Arrange: 2026-01-01T09:00:00+09:00
    let expression = datetime_literal(2026, 1, 1, 9, 0, 0, 9 * 3600);

    // Act
    let result = eval(expression).unwrap();

    // Assert: same UTC instant as 2026-01-01T00:00:00Z
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(offset, UtcOffset::from_seconds(9 * 3600).unwrap());
        let (y, m, d, h, mi, s) = epoch.to_calendar();
        assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 0, 0, 0));
    } else {
        panic!("expected DateTime");
    }
}

// --- DateTime constructors ---

#[test]
fn eval_datetime_constructor() {
    // Arrange: DateTime(2026, 6, 15, 12, 30, 0)
    let expression = call(
        "DateTime",
        vec![int(2026), int(6), int(15), int(12), int(30), int(0)],
    );

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(offset, UtcOffset::UTC); // constructor defaults to UTC
        let (y, m, d, h, mi, s) = epoch.to_calendar();
        assert_eq!((y, m, d, h, mi, s), (2026, 6, 15, 12, 30, 0));
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_datetime_constructor_wrong_arg_count() {
    // Arrange
    let expression = call("DateTime", vec![int(2026), int(1)]);

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

#[test]
fn eval_datetime_constructor_invalid_month() {
    // Arrange
    let expression = call(
        "DateTime",
        vec![int(2026), int(13), int(1), int(0), int(0), int(0)],
    );

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

// --- DateTime unary ---

#[test]
fn eval_negate_datetime_is_error() {
    // Arrange
    use crate::test_fixture::neg;
    let expression = neg(datetime_literal(2026, 1, 1, 0, 0, 0, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

// --- DateTime arithmetic ---

#[test]
fn eval_datetime_add_duration() {
    // Arrange: 2026-01-01T00:00:00Z + 1:30:00
    let expression = add(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        duration_literal(1, 30, 0),
    );

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(offset, UtcOffset::UTC);
        let (y, m, d, h, mi, s) = epoch.to_calendar();
        assert_eq!((y, m, d, h, mi, s), (2026, 1, 1, 1, 30, 0));
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_datetime_subtract_datetime() {
    // Arrange: 2026-01-02T00:00:00Z - 2026-01-01T00:00:00Z
    let expression = sub(
        datetime_literal(2026, 1, 2, 0, 0, 0, 0),
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
    );

    // Act
    let result = eval(expression);

    // Assert: 24 hours = 86400 seconds
    use crate::test_fixture::duration_value;
    assert_eq!(result.unwrap(), duration_value(86400));
}

#[test]
fn eval_datetime_preserves_timezone() {
    // Arrange: 2026-01-01T09:00:00+09:00 + 1:00:00
    let expression = add(
        datetime_literal(2026, 1, 1, 9, 0, 0, 9 * 3600),
        duration_literal(1, 0, 0),
    );

    // Act
    let result = eval(expression).unwrap();

    // Assert: offset preserved
    if let Value::DateTime { offset, .. } = result {
        assert_eq!(offset, UtcOffset::from_seconds(9 * 3600).unwrap());
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn eval_datetime_add_datetime_is_error() {
    // Arrange: DateTime + DateTime
    let expression = add(
        datetime_literal(2026, 1, 1, 0, 0, 0, 0),
        datetime_literal(2026, 1, 2, 0, 0, 0, 0),
    );

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}
