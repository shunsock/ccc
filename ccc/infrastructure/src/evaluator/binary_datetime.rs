use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::time::{DurationSeconds, EpochSeconds, UtcOffset};
use domain::value::Value;

use super::binary_operation::unsupported_binary_op;

/// DateTime ± DurationTime → DateTime (the display offset is preserved)
pub(super) fn add_duration_to_datetime(
    operator: &BinaryOperation,
    epoch: EpochSeconds,
    offset: UtcOffset,
    duration: DurationSeconds,
) -> Result<Value, CccError> {
    let moved = match operator {
        BinaryOperation::Add => epoch.checked_add(duration),
        BinaryOperation::Subtract => epoch.checked_sub(duration),
        _ => return Err(unsupported_binary_op(operator, "datetime", "duration")),
    };
    moved
        .map(|epoch| Value::DateTime { epoch, offset })
        .ok_or_else(|| CccError::eval("datetime out of range"))
}

/// DateTime - DateTime → DurationTime
pub(super) fn subtract_datetimes(
    operator: &BinaryOperation,
    left: EpochSeconds,
    right: EpochSeconds,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Subtract => Ok(Value::DurationTime(left.duration_since(right))),
        _ => Err(unsupported_binary_op(operator, "datetime", "datetime")),
    }
}

/// Timestamp ± DurationTime → Timestamp
pub(super) fn add_duration_to_timestamp(
    operator: &BinaryOperation,
    timestamp: f64,
    duration: DurationSeconds,
) -> Result<Value, CccError> {
    let delta = duration.seconds() as f64;
    match operator {
        BinaryOperation::Add => Ok(Value::Timestamp(timestamp + delta)),
        BinaryOperation::Subtract => Ok(Value::Timestamp(timestamp - delta)),
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
        BinaryOperation::Subtract => Ok(Value::DurationTime(DurationSeconds::from_seconds(
            (left - right) as i64,
        ))),
        _ => Err(unsupported_binary_op(operator, "timestamp", "timestamp")),
    }
}
