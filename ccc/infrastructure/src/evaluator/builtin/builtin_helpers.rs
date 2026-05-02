use domain::error::CccError;
use domain::value::Value;

// ---------------------------------------------------------------------------
// Argument extraction
// ---------------------------------------------------------------------------

pub fn expect_single_arg<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "{name} expects 1 argument, got {}",
            arguments.len()
        )));
    }
    Ok(&arguments[0])
}

pub fn expect_no_args(name: &str, arguments: &[Value]) -> Result<(), CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "{name} expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    Ok(())
}

pub fn expect_single_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    let arg = expect_single_arg(name, arguments)?;
    match arg {
        Value::List(elements) => Ok(elements.as_slice()),
        _ => Err(CccError::eval(format!("{name}: expected list"))),
    }
}

pub fn expect_nonempty_list<'a>(
    name: &str,
    arguments: &'a [Value],
) -> Result<&'a [Value], CccError> {
    let elements = expect_single_list(name, arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval(format!("{name}: empty list")));
    }
    Ok(elements)
}

// ---------------------------------------------------------------------------
// Value conversion
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// List element collection
// ---------------------------------------------------------------------------

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
