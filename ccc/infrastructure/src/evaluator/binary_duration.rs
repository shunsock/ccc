use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::time::DurationSeconds;
use domain::value::Value;

use super::binary_operation::unsupported_binary_op;

fn duration_out_of_range() -> CccError {
    CccError::eval("duration out of range")
}

/// DurationTime ± DurationTime → DurationTime
pub(super) fn combine_durations(
    operator: &BinaryOperation,
    left: DurationSeconds,
    right: DurationSeconds,
) -> Result<Value, CccError> {
    let combined = match operator {
        BinaryOperation::Add => left.checked_add(right),
        BinaryOperation::Subtract => left.checked_sub(right),
        _ => return Err(unsupported_binary_op(operator, "duration", "duration")),
    };
    combined
        .map(Value::DurationTime)
        .ok_or_else(duration_out_of_range)
}

/// DurationTime */÷ Integer → DurationTime
pub(super) fn scale_duration_by_integer(
    operator: &BinaryOperation,
    duration: DurationSeconds,
    scalar: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Multiply => duration
            .checked_mul(scalar)
            .map(Value::DurationTime)
            .ok_or_else(duration_out_of_range),
        BinaryOperation::Divide => {
            if scalar == 0 {
                return Err(CccError::eval("division by zero"));
            }
            duration
                .checked_div(scalar)
                .map(Value::DurationTime)
                .ok_or_else(duration_out_of_range)
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
        BinaryOperation::Multiply => duration
            .checked_mul(scalar)
            .map(Value::DurationTime)
            .ok_or_else(duration_out_of_range),
        _ => Err(unsupported_binary_op(operator, "integer", "duration")),
    }
}
