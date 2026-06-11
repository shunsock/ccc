use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::value::Value;

pub(super) fn evaluate_binary_integer(
    operator: &BinaryOperation,
    left: i64,
    right: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::Integer(left + right)),
        BinaryOperation::Subtract => Ok(Value::Integer(left - right)),
        BinaryOperation::Multiply => Ok(Value::Integer(left * right)),
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
        BinaryOperation::Power => {
            if right >= 0 && right <= u32::MAX as i64 {
                Ok(Value::Integer(left.pow(right as u32)))
            } else {
                Ok(Value::Float((left as f64).powf(right as f64)))
            }
        }
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
