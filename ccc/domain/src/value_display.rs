use chrono::{Datelike, FixedOffset, Timelike};

use crate::time::{DurationSeconds, EpochSeconds, UtcOffset};
use crate::value::Value;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::List(elements) => format_list(f, elements),
            Value::DateTime { epoch, offset } => format_datetime(f, *epoch, *offset),
            Value::Timestamp(ts) => format_timestamp(f, *ts),
            Value::DurationTime(span) => format_duration(f, *span),
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
    epoch: EpochSeconds,
    offset: UtcOffset,
) -> std::fmt::Result {
    // Both expect calls are infallible: the value objects validate at construction.
    let fixed = FixedOffset::east_opt(offset.seconds()).expect("UtcOffset guarantees range");
    let dt = chrono::DateTime::from_timestamp(epoch.seconds(), 0)
        .expect("EpochSeconds guarantees range")
        .with_timezone(&fixed);
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
    if offset.is_utc() {
        write!(f, "Z")
    } else {
        let sign = if offset.seconds() >= 0 { '+' } else { '-' };
        let abs_offset = offset.seconds().unsigned_abs();
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

fn format_duration(f: &mut std::fmt::Formatter<'_>, span: DurationSeconds) -> std::fmt::Result {
    let negative = span.seconds() < 0;
    let abs_seconds = span.seconds().unsigned_abs();
    let hours = abs_seconds / 3600;
    let minutes = (abs_seconds % 3600) / 60;
    let seconds = abs_seconds % 60;
    if negative {
        write!(f, "-{hours}:{minutes:02}:{seconds:02}")
    } else {
        write!(f, "{hours}:{minutes:02}:{seconds:02}")
    }
}
