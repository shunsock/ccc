use domain::time::UtcOffset;
use domain::value::Value;

use crate::test_fixture::{add, call, datetime_value, duration_literal, duration_value, int, sub};

use super::eval;

// --- Timestamp constructors ---

#[test]
fn eval_timestamp_constructor_integer() {
    // Arrange
    let expression = call("Timestamp", vec![int(1234567890)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Timestamp(1234567890.0));
}

#[test]
fn eval_timestamp_constructor_float() {
    // Arrange
    use crate::test_fixture::float;
    let expression = call("Timestamp", vec![float(1234567890.5)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Timestamp(1234567890.5));
}

#[test]
fn eval_timestamp_constructor_wrong_arg_count() {
    // Arrange
    let expression = call("Timestamp", vec![]);

    // Act
    let result = eval(expression);

    // Assert
    assert!(result.is_err());
}

#[test]
fn eval_timestamp_constructor_rejects_nan() {
    // Arrange
    use crate::test_fixture::float;
    use domain::error::CccError;
    let expression = call("Timestamp", vec![float(f64::NAN)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("Timestamp: expected finite number, got NaN".to_string())
    );
}

#[test]
fn eval_timestamp_constructor_rejects_infinity() {
    // Arrange
    use crate::test_fixture::float;
    use domain::error::CccError;
    let expression = call("Timestamp", vec![float(f64::INFINITY)]);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("Timestamp: expected finite number, got inf".to_string())
    );
}

// --- Timestamp arithmetic ---

#[test]
fn eval_timestamp_add_duration() {
    // Arrange: Timestamp(0) + 1:00:00
    let expression = add(call("Timestamp", vec![int(0)]), duration_literal(1, 0, 0));

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), Value::Timestamp(3600.0));
}

#[test]
fn eval_timestamp_subtract_timestamp() {
    // Arrange: Timestamp(7200) - Timestamp(3600)
    let expression = sub(
        call("Timestamp", vec![int(7200)]),
        call("Timestamp", vec![int(3600)]),
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), duration_value(3600));
}

// --- Timestamp type casts ---

#[test]
fn cast_datetime_to_timestamp() {
    // Arrange: DateTime(2026,1,1,0,0,0) as timestamp
    use crate::test_fixture::cast;
    use domain::ast::CastTargetType;
    let expression = cast(
        call(
            "DateTime",
            vec![int(2026), int(1), int(1), int(0), int(0), int(0)],
        ),
        CastTargetType::Timestamp,
    );

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::Timestamp(ts) = result {
        assert_eq!(ts, 1_767_225_600.0);
    } else {
        panic!("expected Timestamp");
    }
}

#[test]
fn cast_timestamp_to_datetime() {
    // Arrange: Timestamp(0) as datetime
    use crate::test_fixture::cast;
    use domain::ast::CastTargetType;
    let expression = cast(call("Timestamp", vec![int(0)]), CastTargetType::DateTime);

    // Act
    let result = eval(expression).unwrap();

    // Assert
    assert_eq!(result, datetime_value(0, 0));
}

#[test]
fn eval_round_trip_datetime_timestamp() {
    // Arrange: (DateTime(2026,6,15,12,30,0) as timestamp) as datetime
    use crate::test_fixture::cast;
    use domain::ast::CastTargetType;
    let expression = cast(
        cast(
            call(
                "DateTime",
                vec![int(2026), int(6), int(15), int(12), int(30), int(0)],
            ),
            CastTargetType::Timestamp,
        ),
        CastTargetType::DateTime,
    );

    // Act
    let result = eval(expression).unwrap();

    // Assert
    if let Value::DateTime { epoch, offset } = result {
        assert_eq!(offset, UtcOffset::UTC);
        let (y, m, d, h, mi, s) = epoch.to_calendar();
        assert_eq!((y, m, d, h, mi, s), (2026, 6, 15, 12, 30, 0));
    } else {
        panic!("expected DateTime");
    }
}

#[test]
fn cast_nan_timestamp_to_datetime_is_error() {
    // Arrange: the constructor now rejects NaN first, so the pipeline still
    // cannot turn "not a number" into epoch 0 (the cast guard remains as
    // defense in depth)
    use crate::test_fixture::{cast, float};
    use domain::ast::CastTargetType;
    use domain::error::CccError;
    let expression = cast(
        call("Timestamp", vec![float(f64::NAN)]),
        CastTargetType::DateTime,
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("Timestamp: expected finite number, got NaN".to_string())
    );
}

#[test]
fn cast_infinite_timestamp_to_datetime_is_error() {
    // Arrange
    use crate::test_fixture::{cast, float};
    use domain::ast::CastTargetType;
    use domain::error::CccError;
    let expression = cast(
        call("Timestamp", vec![float(f64::INFINITY)]),
        CastTargetType::DateTime,
    );

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(
        result.unwrap_err(),
        CccError::eval("Timestamp: expected finite number, got inf".to_string())
    );
}
