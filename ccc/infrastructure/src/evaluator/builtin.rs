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
        "variance" => list_var(arguments),
        "max" => list_max(arguments),
        "min" => list_min(arguments),
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

fn to_f64(value: &Value) -> Result<f64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Float(n) => Ok(*n),
        Value::List(_) => Err(CccError::eval("expected number, got list")),
        Value::DurationTime(_) => Err(CccError::eval("expected number, got duration")),
        Value::DateTime { .. } => Err(CccError::eval("expected number, got datetime")),
        Value::Timestamp(_) => Err(CccError::eval("expected number, got timestamp")),
    }
}

fn to_i64_strict(value: &Value, param_name: &str) -> Result<i64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n),
        _ => Err(CccError::eval(format!("{param_name}: expected integer"))),
    }
}

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

fn unary_float_function(
    name: &str,
    arguments: &[Value],
    function: fn(f64) -> f64,
) -> Result<Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "{name} expects 1 argument, got {}",
            arguments.len()
        )));
    }
    let n = to_f64(&arguments[0])?;
    Ok(Value::Float(function(n)))
}

fn expect_single_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "{name} expects 1 argument, got {}",
            arguments.len()
        )));
    }
    match &arguments[0] {
        Value::List(elements) => Ok(elements),
        _ => Err(CccError::eval(format!("{name}: expected list"))),
    }
}

fn list_len(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("len", arguments)?;
    Ok(Value::Integer(elements.len() as i64))
}

fn sum_durations(elements: &[Value]) -> Result<Value, CccError> {
    let mut total_seconds: i64 = 0;
    for elem in elements {
        match elem {
            Value::DurationTime(s) => total_seconds += s,
            _ => return Err(CccError::eval("sum: list elements must be the same type")),
        }
    }
    Ok(Value::DurationTime(total_seconds))
}

fn sum_numbers(elements: &[Value]) -> Result<Value, CccError> {
    let mut has_float = false;
    let mut int_sum: i64 = 0;
    let mut float_sum: f64 = 0.0;
    for elem in elements {
        match elem {
            Value::Integer(n) => {
                int_sum += n;
                float_sum += *n as f64;
            }
            Value::Float(n) => {
                has_float = true;
                float_sum += n;
            }
            _ => return Err(CccError::eval("sum: list elements must be the same type")),
        }
    }
    if has_float {
        Ok(Value::Float(float_sum))
    } else {
        Ok(Value::Integer(int_sum))
    }
}

fn list_sum(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("sum", arguments)?;

    match elements.first() {
        None => Ok(Value::Integer(0)),
        Some(Value::DurationTime(_)) => sum_durations(elements),
        Some(Value::Integer(_) | Value::Float(_)) => sum_numbers(elements),
        _ => Err(CccError::eval("sum: unsupported element type")),
    }
}

fn expect_nonempty_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    let elements = expect_single_list(name, arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval(format!("{name}: empty list")));
    }
    Ok(elements)
}

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

fn list_mean(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("mean", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("mean", elements)?;
            let total: i64 = secs.iter().sum();
            Ok(Value::DurationTime(total / secs.len() as i64))
        }
        Some(Value::Integer(_) | Value::Float(_)) => {
            let nums = collect_numbers("mean", elements)?;
            let total: f64 = nums.iter().sum();
            Ok(Value::Float(total / nums.len() as f64))
        }
        _ => Err(CccError::eval("mean: unsupported element type")),
    }
}

fn list_var(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("variance", arguments)?;

    match elements.first() {
        Some(Value::Integer(_) | Value::Float(_)) => {
            let nums = collect_numbers("variance", elements)?;
            let n = nums.len() as f64;
            let mean = nums.iter().sum::<f64>() / n;
            let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
            Ok(Value::Float(variance))
        }
        _ => Err(CccError::eval("var: unsupported element type")),
    }
}

fn list_max(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("max", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("max", elements)?;
            Ok(Value::DurationTime(secs.into_iter().max().unwrap()))
        }
        Some(Value::Integer(_)) => {
            let ints = collect_integers("max", elements)?;
            Ok(Value::Integer(ints.into_iter().max().unwrap()))
        }
        Some(Value::Float(_)) => {
            let nums = collect_numbers("max", elements)?;
            Ok(Value::Float(nums.into_iter().reduce(f64::max).unwrap()))
        }
        _ => Err(CccError::eval("max: unsupported element type")),
    }
}

fn list_min(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("min", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let secs = collect_seconds("min", elements)?;
            Ok(Value::DurationTime(secs.into_iter().min().unwrap()))
        }
        Some(Value::Integer(_)) => {
            let ints = collect_integers("min", elements)?;
            Ok(Value::Integer(ints.into_iter().min().unwrap()))
        }
        Some(Value::Float(_)) => {
            let nums = collect_numbers("min", elements)?;
            Ok(Value::Float(nums.into_iter().reduce(f64::min).unwrap()))
        }
        _ => Err(CccError::eval("min: unsupported element type")),
    }
}

fn list_median(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_nonempty_list("median", arguments)?;

    match elements.first() {
        Some(Value::DurationTime(_)) => {
            let mut secs = collect_seconds("median", elements)?;
            secs.sort();
            let n = secs.len();
            if n % 2 == 1 {
                Ok(Value::DurationTime(secs[n / 2]))
            } else {
                Ok(Value::DurationTime((secs[n / 2 - 1] + secs[n / 2]) / 2))
            }
        }
        Some(Value::Integer(_) | Value::Float(_)) => {
            let mut nums = collect_numbers("median", elements)?;
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = nums.len();
            if n % 2 == 1 {
                Ok(Value::Float(nums[n / 2]))
            } else {
                Ok(Value::Float((nums[n / 2 - 1] + nums[n / 2]) / 2.0))
            }
        }
        _ => Err(CccError::eval("median: unsupported element type")),
    }
}

fn list_prod(arguments: &[Value]) -> Result<Value, CccError> {
    let elements = expect_single_list("prod", arguments)?;
    let mut has_float = false;
    let mut int_prod: i64 = 1;
    let mut float_prod: f64 = 1.0;

    for elem in elements {
        match elem {
            Value::Integer(n) => {
                int_prod *= n;
                float_prod *= *n as f64;
            }
            Value::Float(n) => {
                has_float = true;
                float_prod *= n;
            }
            Value::List(_) => return Err(CccError::eval("prod: list elements must be numbers")),
            Value::DurationTime(_) | Value::DateTime { .. } | Value::Timestamp(_) => {
                return Err(CccError::eval("prod: list elements must be numbers"));
            }
        }
    }

    if has_float {
        Ok(Value::Float(float_prod))
    } else {
        Ok(Value::Integer(int_prod))
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

fn unary_absolute(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "abs expects 1 argument, got {}",
            arguments.len()
        )));
    }
    match &arguments[0] {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(n) => Ok(Value::Float(n.abs())),
        Value::List(_) => Err(CccError::eval("abs: expected number, got list")),
        Value::DurationTime(_) => Err(CccError::eval("abs: expected number, got duration")),
        Value::DateTime { .. } => Err(CccError::eval("abs: expected number, got datetime")),
        Value::Timestamp(_) => Err(CccError::eval("abs: expected number, got timestamp")),
    }
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
        offset_seconds: 0, // Constructor defaults to UTC
    })
}

fn timestamp_constructor(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "Timestamp expects 1 argument, got {}",
            arguments.len()
        )));
    }
    match &arguments[0] {
        Value::Integer(n) => Ok(Value::Timestamp(*n as f64)),
        Value::Float(n) => Ok(Value::Timestamp(*n)),
        _ => Err(CccError::eval("Timestamp: expected integer or float")),
    }
}

fn now_function(arguments: &[Value]) -> Result<Value, CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "now expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| CccError::eval(format!("system time error: {e}")))?;
    Ok(Value::DateTime {
        epoch_seconds: epoch.as_secs() as i64,
        offset_seconds: 0,
    })
}

fn today_function(arguments: &[Value]) -> Result<Value, CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "today expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| CccError::eval(format!("system time error: {e}")))?;
    let secs = epoch.as_secs() as i64;
    // Truncate to day boundary
    let day_seconds = secs - (secs % 86400);
    Ok(Value::DateTime {
        epoch_seconds: day_seconds,
        offset_seconds: 0,
    })
}

fn current_timestamp_function(arguments: &[Value]) -> Result<Value, CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "current_timestamp expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| CccError::eval(format!("system time error: {e}")))?;
    Ok(Value::Timestamp(epoch.as_secs_f64()))
}
