use domain::error::CccError;
use domain::value::Value;

use super::builtin_helpers::{expect_single_arg, to_f64, value_type_name};

pub fn unary_float_function(
    name: &str,
    arguments: &[Value],
    function: fn(f64) -> f64,
) -> Result<Value, CccError> {
    let arg = expect_single_arg(name, arguments)?;
    let n = to_f64(arg)?;
    Ok(Value::Float(function(n)))
}

pub fn unary_absolute(arguments: &[Value]) -> Result<Value, CccError> {
    let arg = expect_single_arg("abs", arguments)?;
    match arg {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(n) => Ok(Value::Float(n.abs())),
        _ => Err(CccError::eval(format!(
            "abs: expected number, got {}",
            value_type_name(arg)
        ))),
    }
}

/// `log(x)` — natural logarithm, `log(base, x)` — logarithm with arbitrary base.
pub fn log_function(arguments: &[Value]) -> Result<Value, CccError> {
    match arguments.len() {
        1 => {
            let x = to_f64(&arguments[0])?;
            Ok(Value::Float(x.ln()))
        }
        2 => {
            let base = to_f64(&arguments[0])?;
            let x = to_f64(&arguments[1])?;
            Ok(Value::Float(x.log(base)))
        }
        n => Err(CccError::eval(format!(
            "log expects 1 or 2 arguments, got {n}"
        ))),
    }
}

/// `ln(x)` — natural logarithm (alias for `log(x)`).
pub fn ln_function(arguments: &[Value]) -> Result<Value, CccError> {
    let arg = expect_single_arg("ln", arguments)?;
    let x = to_f64(arg)?;
    Ok(Value::Float(x.ln()))
}
