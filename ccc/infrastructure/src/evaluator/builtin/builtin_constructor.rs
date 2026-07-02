use domain::error::CccError;
use domain::time::{DurationSeconds, EpochSeconds, UtcOffset};
use domain::value::Value;

use super::builtin_helpers::{expect_single_arg, to_f64, to_i64_strict};

pub fn duration_time_constructor(arguments: &[Value]) -> Result<Value, CccError> {
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

    DurationSeconds::from_components(days, hours, minutes, seconds)
        .map(Value::DurationTime)
        .ok_or_else(|| CccError::eval("DurationTime: duration out of range"))
}

pub fn datetime_constructor(arguments: &[Value]) -> Result<Value, CccError> {
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

    let epoch = EpochSeconds::from_calendar(
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
        epoch,
        offset: UtcOffset::UTC,
    })
}

pub fn timestamp_constructor(arguments: &[Value]) -> Result<Value, CccError> {
    let arg = expect_single_arg("Timestamp", arguments)?;
    let n = to_f64(arg)?;
    Ok(Value::Timestamp(n))
}
