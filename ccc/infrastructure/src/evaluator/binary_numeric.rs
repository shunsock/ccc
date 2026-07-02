use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::value::Value;

// Wrapping would silently return a wrong value (e.g. i64::MAX + 1 → i64::MIN),
// so every integer operation that can overflow reports an eval error instead,
// mirroring the checked unary negation and datetime range checks.
fn integer_overflow_error(left: i64, operator_symbol: &str, right: i64) -> CccError {
    CccError::eval(format!(
        "integer overflow: {left} {operator_symbol} {right}"
    ))
}

fn checked_integer(
    checked_result: Option<i64>,
    left: i64,
    operator_symbol: &str,
    right: i64,
) -> Result<Value, CccError> {
    checked_result
        .map(Value::Integer)
        .ok_or_else(|| integer_overflow_error(left, operator_symbol, right))
}

fn evaluate_integer_power(left: i64, right: i64, symbol: &str) -> Result<Value, CccError> {
    if right >= 0 && right <= u32::MAX as i64 {
        checked_integer(left.checked_pow(right as u32), left, symbol, right)
    } else {
        Ok(Value::Float((left as f64).powf(right as f64)))
    }
}

pub(super) fn evaluate_binary_integer(
    operator: &BinaryOperation,
    left: i64,
    right: i64,
) -> Result<Value, CccError> {
    // operator.symbol() keeps overflow messages consistent with type-check errors.
    let symbol = operator.symbol();
    match operator {
        BinaryOperation::Add => checked_integer(left.checked_add(right), left, symbol, right),
        BinaryOperation::Subtract => checked_integer(left.checked_sub(right), left, symbol, right),
        BinaryOperation::Multiply => checked_integer(left.checked_mul(right), left, symbol, right),
        BinaryOperation::Divide => {
            if right == 0 {
                return Err(CccError::eval(format!(
                    "division by zero: {left} / {right}"
                )));
            }
            // Keep integer division exact; fall back to float when it would truncate
            if left % right == 0 {
                Ok(Value::Integer(left / right))
            } else {
                Ok(Value::Float(left as f64 / right as f64))
            }
        }
        BinaryOperation::Modulo => {
            if right == 0 {
                return Err(CccError::eval(format!("modulo by zero: {left} % {right}")));
            }
            Ok(Value::Integer(left % right))
        }
        BinaryOperation::Power => evaluate_integer_power(left, right, symbol),
    }
}

pub(super) fn evaluate_binary_float(
    operator: &BinaryOperation,
    left: f64,
    right: f64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::Float(left + right)),
        BinaryOperation::Subtract => Ok(Value::Float(left - right)),
        BinaryOperation::Multiply => Ok(Value::Float(left * right)),
        BinaryOperation::Divide => {
            if right == 0.0 {
                return Err(CccError::eval(format!(
                    "division by zero: {left} / {right}"
                )));
            }
            Ok(Value::Float(left / right))
        }
        BinaryOperation::Modulo => {
            if right == 0.0 {
                return Err(CccError::eval(format!("modulo by zero: {left} % {right}")));
            }
            Ok(Value::Float(left % right))
        }
        BinaryOperation::Power => Ok(Value::Float(left.powf(right))),
    }
}
