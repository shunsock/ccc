use domain::error::CccError;
use domain::value::Value;

pub fn collect_numbers(name: &str, elements: &[Value]) -> Result<Vec<f64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

pub fn collect_seconds(name: &str, elements: &[Value]) -> Result<Vec<i64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::DurationTime(s) => Ok(*s),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

pub fn collect_integers(name: &str, elements: &[Value]) -> Result<Vec<i64>, CccError> {
    elements
        .iter()
        .map(|e| match e {
            Value::Integer(n) => Ok(*n),
            _ => Err(CccError::eval(format!(
                "{name}: list elements must be the same type"
            ))),
        })
        .collect()
}

/// Fold numeric list elements with an accumulator, preserving int/float distinction.
pub fn fold_numbers(
    name: &str,
    elements: &[Value],
    int_identity: i64,
    float_identity: f64,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> Result<Value, CccError> {
    let mut has_float = false;
    let mut int_acc = int_identity;
    let mut float_acc = float_identity;

    for elem in elements {
        match elem {
            Value::Integer(n) => {
                int_acc = int_op(int_acc, *n);
                float_acc = float_op(float_acc, *n as f64);
            }
            Value::Float(n) => {
                has_float = true;
                float_acc = float_op(float_acc, *n);
            }
            _ => {
                return Err(CccError::eval(format!(
                    "{name}: list elements must be numbers"
                )));
            }
        }
    }

    if has_float {
        Ok(Value::Float(float_acc))
    } else {
        Ok(Value::Integer(int_acc))
    }
}
