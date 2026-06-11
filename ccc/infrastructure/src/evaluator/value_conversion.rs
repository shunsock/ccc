use domain::error::CccError;
use domain::value::Value;

pub(super) fn to_f64(value: &Value) -> Result<f64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Float(n) => Ok(*n),
        other => Err(CccError::eval(format!(
            "expected number, got {}",
            other.type_name()
        ))),
    }
}

pub(super) fn to_i64(value: &Value) -> Result<i64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n),
        Value::Float(n) => Ok(*n as i64),
        other => Err(CccError::eval(format!(
            "expected number, got {}",
            other.type_name()
        ))),
    }
}
