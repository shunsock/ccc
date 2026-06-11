use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::value::Value;

use super::binary_operation::unsupported_binary_op;

/// DateTime ± DurationTime → DateTime (the display offset is preserved)
pub(super) fn add_duration_to_datetime(
    operator: &BinaryOperation,
    epoch_seconds: i64,
    offset_seconds: i32,
    duration: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::DateTime {
            epoch_seconds: epoch_seconds + duration,
            offset_seconds,
        }),
        BinaryOperation::Subtract => Ok(Value::DateTime {
            epoch_seconds: epoch_seconds - duration,
            offset_seconds,
        }),
        _ => Err(unsupported_binary_op(operator, "datetime", "duration")),
    }
}

/// DateTime - DateTime → DurationTime
pub(super) fn subtract_datetimes(
    operator: &BinaryOperation,
    left_epoch: i64,
    right_epoch: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Subtract => Ok(Value::DurationTime(left_epoch - right_epoch)),
        _ => Err(unsupported_binary_op(operator, "datetime", "datetime")),
    }
}

/// Timestamp ± DurationTime → Timestamp
pub(super) fn add_duration_to_timestamp(
    operator: &BinaryOperation,
    timestamp: f64,
    duration: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::Timestamp(timestamp + duration as f64)),
        BinaryOperation::Subtract => Ok(Value::Timestamp(timestamp - duration as f64)),
        _ => Err(unsupported_binary_op(operator, "timestamp", "duration")),
    }
}

/// Timestamp - Timestamp → DurationTime
pub(super) fn subtract_timestamps(
    operator: &BinaryOperation,
    left: f64,
    right: f64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Subtract => Ok(Value::DurationTime((left - right) as i64)),
        _ => Err(unsupported_binary_op(operator, "timestamp", "timestamp")),
    }
}
