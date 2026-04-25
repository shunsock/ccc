use chrono::{Datelike, FixedOffset, NaiveDate, TimeZone, Timelike};

/// Returns the number of days in the given month (1-indexed) of the given year.
/// Returns `None` if the month is out of range or the year is out of chrono's range.
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    let year_i32 = i32::try_from(year).ok()?;
    if month == 12 {
        let start = NaiveDate::from_ymd_opt(year_i32, 12, 1)?;
        let end = NaiveDate::from_ymd_opt(year_i32 + 1, 1, 1)?;
        Some((end - start).num_days() as u8)
    } else {
        let start = NaiveDate::from_ymd_opt(year_i32, month as u32, 1)?;
        let end = NaiveDate::from_ymd_opt(year_i32, month as u32 + 1, 1)?;
        Some((end - start).num_days() as u8)
    }
}

/// Converts calendar components to epoch seconds (seconds since 1970-01-01T00:00:00 UTC).
/// Returns `None` if the date/time components are invalid.
pub fn calendar_to_epoch_seconds(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> Option<i64> {
    let year_i32 = i32::try_from(year).ok()?;
    let naive = NaiveDate::from_ymd_opt(year_i32, month as u32, day as u32)?.and_hms_opt(
        hour as u32,
        minute as u32,
        second as u32,
    )?;
    let utc = chrono::Utc.from_utc_datetime(&naive);
    Some(utc.timestamp())
}

/// Converts epoch seconds to calendar components (year, month, day, hour, minute, second).
pub fn epoch_seconds_to_calendar(epoch_seconds: i64) -> (i64, u8, u8, u8, u8, u8) {
    let dt = chrono::DateTime::from_timestamp(epoch_seconds, 0)
        .expect("epoch seconds out of chrono range");
    (
        dt.year() as i64,
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second() as u8,
    )
}

/// Represents a computed value in the calculator.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    List(Vec<Value>),
    /// Duration stored as total seconds (signed to support negative durations).
    DurationTime(i64),
    /// DateTime stored as UTC epoch seconds with a display timezone offset.
    DateTime {
        epoch_seconds: i64,
        offset_seconds: i32,
    },
    /// Unix timestamp in seconds. Stored as f64 to support sub-second precision.
    Timestamp(f64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::List(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{elem}")?;
                }
                write!(f, "]")
            }
            Value::DateTime {
                epoch_seconds,
                offset_seconds,
            } => {
                let offset =
                    FixedOffset::east_opt(*offset_seconds).expect("timezone offset out of range");
                let dt = chrono::DateTime::from_timestamp(*epoch_seconds, 0)
                    .expect("epoch seconds out of range")
                    .with_timezone(&offset);
                write!(
                    f,
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                    dt.year(),
                    dt.month(),
                    dt.day(),
                    dt.hour(),
                    dt.minute(),
                    dt.second()
                )?;
                if *offset_seconds == 0 {
                    write!(f, "Z")
                } else {
                    let sign = if *offset_seconds >= 0 { '+' } else { '-' };
                    let abs_offset = offset_seconds.unsigned_abs();
                    let offset_hours = abs_offset / 3600;
                    let offset_minutes = (abs_offset % 3600) / 60;
                    write!(f, "{sign}{offset_hours:02}:{offset_minutes:02}")
                }
            }
            Value::Timestamp(ts) => {
                if *ts == ts.trunc() {
                    // Display as integer when there's no fractional part
                    write!(f, "{}", *ts as i64)
                } else {
                    write!(f, "{ts}")
                }
            }
            Value::DurationTime(total_seconds) => {
                let negative = *total_seconds < 0;
                let abs_seconds = total_seconds.unsigned_abs();
                let hours = abs_seconds / 3600;
                let minutes = (abs_seconds % 3600) / 60;
                let seconds = abs_seconds % 60;
                if negative {
                    write!(f, "-{hours}:{minutes:02}:{seconds:02}")
                } else {
                    write!(f, "{hours}:{minutes:02}:{seconds:02}")
                }
            }
        }
    }
}
