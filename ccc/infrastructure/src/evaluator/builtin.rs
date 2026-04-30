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
        "head" => list_head(arguments),
        "tail" => list_tail(arguments),
        "DurationTime" => duration_time_constructor(arguments),
        "DateTime" => datetime_constructor(arguments),
        "Timestamp" => timestamp_constructor(arguments),
        "datetime_to_timestamp" => datetime_to_timestamp(arguments),
        "timestamp_to_datetime" => timestamp_to_datetime(arguments),
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

fn datetime_to_timestamp(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "datetime_to_timestamp expects 1 argument, got {}",
            arguments.len()
        )));
    }
    match &arguments[0] {
        Value::DateTime { epoch_seconds, .. } => Ok(Value::Timestamp(*epoch_seconds as f64)),
        _ => Err(CccError::eval("datetime_to_timestamp: expected datetime")),
    }
}

fn timestamp_to_datetime(arguments: &[Value]) -> Result<Value, CccError> {
    if arguments.is_empty() || arguments.len() > 2 {
        return Err(CccError::eval(format!(
            "timestamp_to_datetime expects 1-2 arguments, got {}",
            arguments.len()
        )));
    }
    let ts = match &arguments[0] {
        Value::Timestamp(n) => *n,
        _ => return Err(CccError::eval("timestamp_to_datetime: expected timestamp")),
    };
    let offset_seconds = if arguments.len() == 2 {
        match &arguments[1] {
            Value::Integer(hours) => (*hours as i32) * 3600,
            _ => {
                return Err(CccError::eval(
                    "timestamp_to_datetime: timezone offset must be integer",
                ));
            }
        }
    } else {
        0
    };
    Ok(Value::DateTime {
        epoch_seconds: ts as i64,
        offset_seconds,
    })
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
