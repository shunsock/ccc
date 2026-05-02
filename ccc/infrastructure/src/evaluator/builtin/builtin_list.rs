use std::ops::{Add, Mul};

use domain::error::CccError;
use domain::value::Value;

use super::builtin_helpers::{
    collect_integers, collect_numbers, collect_seconds, expect_nonempty_list, expect_single_list,
    fold_numbers,
};

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
            Ok(Value::DurationTime(secs.iter().sum()))
        }
        _ => fold_numbers("sum", elements, 0, 0.0, i64::wrapping_add, f64::add),
    }
}

pub fn list_prod(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("prod", arguments)?;
    fold_numbers("prod", elements, 1, 1.0, i64::wrapping_mul, f64::mul)
}

pub fn list_mean(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("mean", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("mean", elements)?;
            let total: i64 = secs.iter().sum();
            Ok(Value::DurationTime(total / secs.len() as i64))
        }
        _ => {
            let nums = collect_numbers("mean", elements)?;
            let total: f64 = nums.iter().sum();
            Ok(Value::Float(total / nums.len() as f64))
        }
    }
}

pub fn list_variance(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("variance", arguments)?;
    let nums = collect_numbers("variance", elements)?;
    let n = nums.len() as f64;
    let mean = nums.iter().sum::<f64>() / n;
    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    Ok(Value::Float(variance))
}

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

pub fn list_median(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("median", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let mut secs = collect_seconds("median", elements)?;
            secs.sort();
            Ok(Value::DurationTime(median_sorted_i64(&secs)))
        }
        _ => {
            let mut nums = collect_numbers("median", elements)?;
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Value::Float(median_sorted_f64(&nums)))
        }
    }
}

fn median_sorted_f64(nums: &[f64]) -> f64 {
    let n = nums.len();
    if n % 2 == 1 {
        nums[n / 2]
    } else {
        (nums[n / 2 - 1] + nums[n / 2]) / 2.0
    }
}

fn median_sorted_i64(secs: &[i64]) -> i64 {
    let n = secs.len();
    if n % 2 == 1 {
        secs[n / 2]
    } else {
        (secs[n / 2 - 1] + secs[n / 2]) / 2
    }
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
