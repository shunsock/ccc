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
