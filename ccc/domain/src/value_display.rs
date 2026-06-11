use chrono::{Datelike, FixedOffset, Timelike};

use crate::value::Value;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::List(elements) => format_list(f, elements),
            Value::DateTime {
                epoch_seconds,
                offset_seconds,
            } => format_datetime(f, *epoch_seconds, *offset_seconds),
            Value::Timestamp(ts) => format_timestamp(f, *ts),
            Value::DurationTime(total_seconds) => format_duration(f, *total_seconds),
        }
    }
}

fn format_list(f: &mut std::fmt::Formatter<'_>, elements: &[Value]) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in elements.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{elem}")?;
    }
    write!(f, "]")
}

fn format_datetime(
    f: &mut std::fmt::Formatter<'_>,
    epoch_seconds: i64,
    offset_seconds: i32,
) -> std::fmt::Result {
    let offset = FixedOffset::east_opt(offset_seconds).expect("timezone offset out of range");
    let dt = chrono::DateTime::from_timestamp(epoch_seconds, 0)
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
    if offset_seconds == 0 {
        write!(f, "Z")
    } else {
        let sign = if offset_seconds >= 0 { '+' } else { '-' };
        let abs_offset = offset_seconds.unsigned_abs();
        let offset_hours = abs_offset / 3600;
        let offset_minutes = (abs_offset % 3600) / 60;
        write!(f, "{sign}{offset_hours:02}:{offset_minutes:02}")
    }
}

fn format_timestamp(f: &mut std::fmt::Formatter<'_>, ts: f64) -> std::fmt::Result {
    if ts == ts.trunc() {
        // Display as integer when there's no fractional part
        write!(f, "{}", ts as i64)
    } else {
        write!(f, "{ts}")
    }
}

fn format_duration(f: &mut std::fmt::Formatter<'_>, total_seconds: i64) -> std::fmt::Result {
    let negative = total_seconds < 0;
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
