use domain::error::CccError;
use domain::value::Value;

pub fn to_f64(value: &Value) -> Result<f64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Float(n) => Ok(*n),
        _ => Err(CccError::eval(format!(
            "expected number, got {}",
            value_type_name(value)
        ))),
    }
}

pub fn to_i64_strict(value: &Value, param_name: &str) -> Result<i64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n),
        _ => Err(CccError::eval(format!("{param_name}: expected integer"))),
    }
}

pub fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::List(_) => "list",
        Value::DurationTime(_) => "duration",
        Value::DateTime { .. } => "datetime",
        Value::Timestamp(_) => "timestamp",
    }
}
