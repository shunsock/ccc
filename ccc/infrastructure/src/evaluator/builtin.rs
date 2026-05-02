use std::ops::{Add, Mul};

use domain::error::CccError;
use domain::value::Value;

/// Dispatch a builtin function call.
pub fn call_builtin(name: &str, arguments: &[Value]) -> Result<Value, CccError> {
    match name {
        "sqrt" => unary_float_function(name, arguments, f64::sqrt),
        "abs" => unary_absolute(arguments),
        "sin" => unary_float_function(name, arguments, f64::sin),
        "cos" => unary_float_function(name, arguments, f64::cos),
        "tan" => unary_float_function(name, arguments, f64::tan),
        "arcsin" => unary_float_function(name, arguments, f64::asin),
        "arccos" => unary_float_function(name, arguments, f64::acos),
        "arctan" => unary_float_function(name, arguments, f64::atan),
        "log" => unary_float_function(name, arguments, f64::ln),
        "log2" => unary_float_function(name, arguments, f64::log2),
        "log10" => unary_float_function(name, arguments, f64::log10),
        "floor" => unary_float_function(name, arguments, f64::floor),
        "ceil" => unary_float_function(name, arguments, f64::ceil),
        "round" => unary_float_function(name, arguments, f64::round),
        "len" => list_len(arguments),
        "sum" => list_sum(arguments),
        "prod" => list_prod(arguments),
        "mean" => list_mean(arguments),
        "variance" => list_variance(arguments),
        "max" => list_extremum("max", arguments, i64::max, f64::max, i64::max),
        "min" => list_extremum("min", arguments, i64::min, f64::min, i64::min),
        "median" => list_median(arguments),
        "head" => list_head(arguments),
        "tail" => list_tail(arguments),
        "DurationTime" => duration_time_constructor(arguments),
        "DateTime" => datetime_constructor(arguments),
        "Timestamp" => timestamp_constructor(arguments),
        "now" => now_function(arguments),
        "today" => today_function(arguments),
        "current_timestamp" => current_timestamp_function(arguments),
        _ => Err(CccError::eval(format!("unknown function: {name}"))),
    }
}

// ---------------------------------------------------------------------------
// Helpers: argument extraction
// ---------------------------------------------------------------------------

fn expect_single_arg<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "{name} expects 1 argument, got {}",
            arguments.len()
        )));
    }
    Ok(&arguments[0])
}

fn expect_no_args(name: &str, arguments: &[Value]) -> Result<(), CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "{name} expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    Ok(())
}

fn expect_single_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    let arg = expect_single_arg(name, arguments)?;
    match arg {
        Value::List(elements) => Ok(elements.as_slice()),
        _ => Err(CccError::eval(format!("{name}: expected list"))),
    }
}

fn expect_nonempty_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    let elements = expect_single_list(name, arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval(format!("{name}: empty list")));
    }
    Ok(elements)
}

// ---------------------------------------------------------------------------
// Helpers: value conversion
// ---------------------------------------------------------------------------

fn to_f64(value: &Value) -> Result<f64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Float(n) => Ok(*n),
        _ => Err(CccError::eval(format!(
            "expected number, got {}",
            value_type_name(value)
        ))),
    }
}

fn to_i64_strict(value: &Value, param_name: &str) -> Result<i64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n),
        _ => Err(CccError::eval(format!("{param_name}: expected integer"))),
    }
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::List(_) => "list",
        Value::DurationTime(_) => "duration",
        Value::DateTime { .. } => "datetime",
        Value::Timestamp(_) => "timestamp",
    }
}

// ---------------------------------------------------------------------------
// Helpers: list element collection
// ---------------------------------------------------------------------------

fn collect_numbers(name: &str, elements: &[Value]) -> Result<Vec<f64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

fn collect_seconds(name: &str, elements: &[Value]) -> Result<Vec<i64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::DurationTime(s) => Ok(*s),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

fn collect_integers(name: &str, elements: &[Value]) -> Result<Vec<i64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::Integer(n) => Ok(*n),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

/// Fold numeric list elements with an accumulator, preserving int/float distinction.
fn fold_numbers(
    name: &str,
    elements: &[Value],
    int_identity: i64,
    float_identity: f64,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> Result<Value, CccError> {
    let mut has_float = false;
    let mut int_acc = int_identity;
    let mut float_acc = float_identity;

    for elem in elements {
        match elem {
            Value::Integer(n) => {
                int_acc = int_op(int_acc, *n);
                float_acc = float_op(float_acc, *n as f64);
            }
            Value::Float(n) => {
                has_float = true;
                float_acc = float_op(float_acc, *n);
            }
            _ => {
                return Err(CccError::eval(format!(
                    "{name}: list elements must be numbers"
                )));
            }
        }
    }

    if has_float {
        Ok(Value::Float(float_acc))
    } else {
        Ok(Value::Integer(int_acc))
    }
}

// ---------------------------------------------------------------------------
// Helpers: time
// ---------------------------------------------------------------------------

fn current_epoch() -> Result<std::time::Duration, CccError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| CccError::eval(format!("system time error: {e}")))
}

// ---------------------------------------------------------------------------
// Math functions
// ---------------------------------------------------------------------------

fn unary_float_function(
    name: &str,
    arguments: &[Value],
    function: fn(f64) -> f64,
) -> Result<Value, CccError> {
    let arg = expect_single_arg(name, arguments)?;
    let n = to_f64(arg)?;
    Ok(Value::Float(function(n)))
}

fn unary_absolute(arguments: &[Value]) -> Result<Value, CccError> {
    let arg = expect_single_arg("abs", arguments)?;
    match arg {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(n) => Ok(Value::Float(n.abs())),
        _ => Err(CccError::eval(format!(
            "abs: expected number, got {}",
            value_type_name(arg)
        ))),
    }
}

// ---------------------------------------------------------------------------
// List functions
// ---------------------------------------------------------------------------

fn list_len(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("len", arguments)?;
    Ok(Value::Integer(elements.len() as i64))
}

fn list_sum(arguments: &[Value]) -> Result<Value, CccError> {
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

fn list_prod(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("prod", arguments)?;
    fold_numbers("prod", elements, 1, 1.0, i64::wrapping_mul, f64::mul)
}

fn list_mean(arguments: &[Value]) -> Result<Value, CccError> {
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

fn list_variance(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("variance", arguments)?;
    let nums = collect_numbers("variance", elements)?;
    let n = nums.len() as f64;
    let mean = nums.iter().sum::<f64>() / n;
    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    Ok(Value::Float(variance))
}

/// Compute max or min over a list, dispatching by element type.
fn list_extremum(
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

fn list_median(arguments: &[Value]) -> Result<Value, CccError> {
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

fn list_head(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("head", arguments)?;
    elements
        .first()
        .cloned()
        .ok_or_else(|| CccError::eval("head: empty list"))
}

fn list_tail(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("tail", arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval("tail: empty list"));
    }
    Ok(Value::List(elements[1..].to_vec()))
}

// ---------------------------------------------------------------------------
// Constructors
// ---------------------------------------------------------------------------

fn duration_time_constructor(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.len() < 3 || arguments.len() > 4 {
        return Err(CccError::eval(format!(
            "DurationTime expects 3 or 4 arguments (day, hour, minute, second) or (hour, minute, second), got {}",
            arguments.len()
        )));
    }

    let (days, hours, minutes, seconds) = if arguments.len() == 4 {
        (
            to_i64_strict(&arguments[0], "day")?,
            to_i64_strict(&arguments[1], "hour")?,
            to_i64_strict(&arguments[2], "minute")?,
            to_i64_strict(&arguments[3], "second")?,
        )
    } else {
        (
            0,
            to_i64_strict(&arguments[0], "hour")?,
            to_i64_strict(&arguments[1], "minute")?,
            to_i64_strict(&arguments[2], "second")?,
        )
    };

    let total_seconds = days * 86400 + hours * 3600 + minutes * 60 + seconds;
    Ok(Value::DurationTime(total_seconds))
}

fn datetime_constructor(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.len() != 6 {
        return Err(CccError::eval(format!(
            "DateTime expects 6 arguments (year, month, day, hour, minute, second), got {}",
            arguments.len()
        )));
    }

    let year = to_i64_strict(&arguments[0], "year")?;
    let month = to_i64_strict(&arguments[1], "month")?;
    let day = to_i64_strict(&arguments[2], "day")?;
    let hour = to_i64_strict(&arguments[3], "hour")?;
    let minute = to_i64_strict(&arguments[4], "minute")?;
    let second = to_i64_strict(&arguments[5], "second")?;

    let epoch_seconds = domain::value::calendar_to_epoch_seconds(
        year,
        month as u8,
        day as u8,
        hour as u8,
        minute as u8,
        second as u8,
    )
    .ok_or_else(|| {
        CccError::eval(format!(
            "DateTime: invalid date/time components ({year}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02})"
        ))
    })?;

    Ok(Value::DateTime {
        epoch_seconds,
        offset_seconds: 0,
    })
}

fn timestamp_constructor(arguments: &[Value]) -> Result<Value, CccError> {
    let arg = expect_single_arg("Timestamp", arguments)?;
    let n = to_f64(arg)?;
    Ok(Value::Timestamp(n))
}

// ---------------------------------------------------------------------------
// Time utility functions
// ---------------------------------------------------------------------------

fn now_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("now", arguments)?;
    let epoch = current_epoch()?;
    Ok(Value::DateTime {
        epoch_seconds: epoch.as_secs() as i64,
        offset_seconds: 0,
    })
}

fn today_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("today", arguments)?;
    let epoch = current_epoch()?;
    let secs = epoch.as_secs() as i64;
    let day_seconds = secs - (secs % 86400);
    Ok(Value::DateTime {
        epoch_seconds: day_seconds,
        offset_seconds: 0,
    })
}

fn current_timestamp_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("current_timestamp", arguments)?;
    let epoch = current_epoch()?;
    Ok(Value::Timestamp(epoch.as_secs_f64()))
}
