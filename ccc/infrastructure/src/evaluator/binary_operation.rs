use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::value::Value;

use super::binary_datetime::{
    add_duration_to_datetime, add_duration_to_timestamp, subtract_datetimes, subtract_timestamps,
};
use super::binary_duration::{
    combine_durations, multiply_integer_by_duration, scale_duration_by_integer,
};
use super::binary_numeric::{evaluate_binary_float, evaluate_binary_integer};
use super::value_conversion::to_f64;

/// Dispatch a binary operation by the runtime types of both operands.
pub(super) fn evaluate_binary(
    operator: &BinaryOperation,
    left: &Value,
    right: &Value,
) -> Result<Value, CccError> {
    match (left, right) {
        // Numeric operations
        (Value::Integer(l), Value::Integer(r)) => evaluate_binary_integer(operator, *l, *r),
        (Value::Integer(_) | Value::Float(_), Value::Integer(_) | Value::Float(_)) => {
            evaluate_binary_float(operator, to_f64(left)?, to_f64(right)?)
        }

        // Duration combinations
        (Value::DurationTime(l), Value::DurationTime(r)) => combine_durations(operator, *l, *r),
        (Value::DurationTime(d), Value::Integer(n)) => scale_duration_by_integer(operator, *d, *n),
        (Value::Integer(n), Value::DurationTime(d)) => {
            multiply_integer_by_duration(operator, *n, *d)
        }

        // DateTime combinations
        (Value::DateTime { epoch, offset }, Value::DurationTime(d)) => {
            add_duration_to_datetime(operator, *epoch, *offset, *d)
        }
        (Value::DateTime { epoch: l, .. }, Value::DateTime { epoch: r, .. }) => {
            subtract_datetimes(operator, *l, *r)
        }

        // Timestamp combinations
        (Value::Timestamp(ts), Value::DurationTime(d)) => {
            add_duration_to_timestamp(operator, *ts, *d)
        }
        (Value::Timestamp(l), Value::Timestamp(r)) => subtract_timestamps(operator, *l, *r),

        _ => Err(unsupported_binary_op(
            operator,
            left.type_name(),
            right.type_name(),
        )),
    }
}

pub(super) fn unsupported_binary_op(
    operator: &BinaryOperation,
    left: &str,
    right: &str,
) -> CccError {
    CccError::eval(format!(
        "unsupported operation: {left} {} {right}",
        operator.symbol()
    ))
}
