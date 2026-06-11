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
