use std::ops::{Add, Mul};

use domain::error::CccError;
use domain::time::DurationSeconds;
use domain::value::Value;

use super::builtin_helpers::{collect_seconds, expect_single_list, fold_numbers};

fn sum_seconds_checked(name: &str, secs: &[i64]) -> Result<i64, CccError> {
    secs.iter()
        .try_fold(0i64, |acc, s| acc.checked_add(*s))
        .ok_or_else(|| CccError::eval(format!("{name}: duration overflow")))
}

pub fn list_len(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("len", arguments)?;
    Ok(Value::Integer(elements.len() as i64))
}

pub fn list_sum(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("sum", arguments)?;

    match elements.first() {
        None => Ok(Value::Integer(0)),
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("sum", elements)?;
            Ok(Value::DurationTime(DurationSeconds::from_seconds(
                sum_seconds_checked("sum", &secs)?,
            )))
        }
        _ => fold_numbers("sum", elements, 0, 0.0, i64::checked_add, f64::add),
    }
}

pub fn list_prod(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("prod", arguments)?;
    fold_numbers("prod", elements, 1, 1.0, i64::checked_mul, f64::mul)
}

pub fn list_head(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("head", arguments)?;
    elements
        .first()
        .cloned()
        .ok_or_else(|| CccError::eval("head: empty list"))
}

pub fn list_tail(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("tail", arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval("tail: empty list"));
    }
    Ok(Value::List(elements[1..].to_vec()))
}
