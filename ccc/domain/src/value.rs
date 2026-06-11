use crate::time::{DurationSeconds, EpochSeconds, UtcOffset};

/// Represents a computed value in the calculator.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    List(Vec<Value>),
    /// Duration as a signed time span.
    DurationTime(DurationSeconds),
    /// DateTime as a UTC instant with a display timezone offset.
    DateTime {
        epoch: EpochSeconds,
        offset: UtcOffset,
    },
    /// Unix timestamp in seconds. Stored as f64 to support sub-second precision.
    Timestamp(f64),
}

impl Value {
    /// Human-readable type name used in error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::List(_) => "list",
            Value::DurationTime(_) => "duration",
            Value::DateTime { .. } => "datetime",
            Value::Timestamp(_) => "timestamp",
        }
    }
}
