use domain::error::CccError;
use domain::time::DurationSeconds;
use domain::value::Value;

use super::builtin_helpers::{collect_numbers, collect_seconds, expect_nonempty_list};

pub fn list_mean(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("mean", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("mean", elements)?;
            let total = secs
                .iter()
                .try_fold(0i64, |acc, s| acc.checked_add(*s))
                .ok_or_else(|| CccError::eval("mean: duration overflow"))?;
            Ok(Value::DurationTime(DurationSeconds::from_seconds(
                total / secs.len() as i64,
            )))
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

pub fn list_median(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("median", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let mut secs = collect_seconds("median", elements)?;
            secs.sort();
            Ok(Value::DurationTime(DurationSeconds::from_seconds(
                median_sorted_i64(&secs),
            )))
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
        let low = secs[n / 2 - 1];
        let high = secs[n / 2];
        // low + (high - low) / 2 cannot overflow because low <= high after sorting,
        // unlike (low + high) / 2 whose intermediate sum can leave the i64 range.
        low + (high - low) / 2
    }
}
