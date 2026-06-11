use domain::error::CccError;
use domain::value::Value;

use super::builtin_helpers::{
    collect_integers, collect_numbers, collect_seconds, expect_nonempty_list,
};

pub fn list_extremum(
    name: &str,
    arguments: &[Value],
    int_reduce: fn(i64, i64) -> i64,
    float_reduce: fn(f64, f64) -> f64,
    sec_reduce: fn(i64, i64) -> i64,
) -> Result<Value, CccError> {
    let elements = expect_nonempty_list(name, arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds(name, elements)?;
            Ok(Value::DurationTime(
                secs.into_iter().reduce(sec_reduce).unwrap(),
            ))
        }
        Some(Value::Integer(_)) => {
            let ints = collect_integers(name, elements)?;
            Ok(Value::Integer(ints.into_iter().reduce(int_reduce).unwrap()))
        }
        Some(Value::Float(_)) => {
            let nums = collect_numbers(name, elements)?;
            Ok(Value::Float(nums.into_iter().reduce(float_reduce).unwrap()))
        }
        _ => Err(CccError::eval(format!("{name}: unsupported element type"))),
    }
}
