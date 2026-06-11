use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::time::DurationSeconds;
use domain::value::Value;

use super::binary_operation::unsupported_binary_op;

/// DurationTime ± DurationTime → DurationTime
pub(super) fn combine_durations(
    operator: &BinaryOperation,
    left: DurationSeconds,
    right: DurationSeconds,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::DurationTime(left + right)),
        BinaryOperation::Subtract => Ok(Value::DurationTime(left - right)),
        _ => Err(unsupported_binary_op(operator, "duration", "duration")),
    }
}

/// DurationTime */÷ Integer → DurationTime
pub(super) fn scale_duration_by_integer(
    operator: &BinaryOperation,
    duration: DurationSeconds,
    scalar: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Multiply => Ok(Value::DurationTime(duration * scalar)),
        BinaryOperation::Divide => {
            if scalar == 0 {
                return Err(CccError::eval("division by zero"));
            }
            Ok(Value::DurationTime(duration / scalar))
        }
        _ => Err(unsupported_binary_op(operator, "duration", "integer")),
    }
}

/// Integer * DurationTime → DurationTime
pub(super) fn multiply_integer_by_duration(
    operator: &BinaryOperation,
    scalar: i64,
    duration: DurationSeconds,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Multiply => Ok(Value::DurationTime(duration * scalar)),
        _ => Err(unsupported_binary_op(operator, "integer", "duration")),
    }
}
