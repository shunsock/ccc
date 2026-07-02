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
        Value::Float(n) => float_to_i64(*n),
        other => Err(CccError::eval(format!(
            "expected number, got {}",
            other.type_name()
        ))),
    }
}

// A bare `as` cast would silently turn NaN into 0 and saturate infinities and
// out-of-range floats, so reject any float without an exact in-range truncation.
fn float_to_i64(n: f64) -> Result<i64, CccError> {
    if !n.is_finite() {
        return Err(CccError::eval(format!("cannot cast {n} to int")));
    }
    let truncated = n.trunc();
    // i64::MAX as f64 rounds up to 2^63 exactly, so `<` also excludes 2^63 itself.
    if truncated >= i64::MIN as f64 && truncated < i64::MAX as f64 {
        Ok(truncated as i64)
    } else {
        Err(CccError::eval(format!("float out of int range: {n}")))
    }
}
