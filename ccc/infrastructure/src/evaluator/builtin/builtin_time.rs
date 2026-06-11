use domain::error::CccError;
use domain::time::{EpochSeconds, UtcOffset};
use domain::value::Value;

use super::builtin_helpers::expect_no_args;

fn current_epoch() -> Result<std::time::Duration, CccError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| CccError::eval(format!("system time error: {e}")))
}

pub fn now_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("now", arguments)?;
    let epoch = current_epoch()?;
    let epoch = EpochSeconds::from_seconds(epoch.as_secs() as i64)
        .ok_or_else(|| CccError::eval("system time out of range"))?;
    Ok(Value::DateTime {
        epoch,
        offset: UtcOffset::UTC,
    })
}

pub fn today_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("today", arguments)?;
    let epoch = current_epoch()?;
    let secs = epoch.as_secs() as i64;
    let day_seconds = secs - (secs % 86400);
    let epoch = EpochSeconds::from_seconds(day_seconds)
        .ok_or_else(|| CccError::eval("system time out of range"))?;
    Ok(Value::DateTime {
        epoch,
        offset: UtcOffset::UTC,
    })
}

pub fn current_timestamp_function(arguments: &[Value]) -> Result<Value, CccError> {
    expect_no_args("current_timestamp", arguments)?;
    let epoch = current_epoch()?;
    Ok(Value::Timestamp(epoch.as_secs_f64()))
}
