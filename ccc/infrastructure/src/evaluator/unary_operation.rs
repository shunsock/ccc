use domain::ast::UnaryOperation;
use domain::error::CccError;
use domain::value::Value;

pub(super) fn evaluate_unary(operator: &UnaryOperation, value: &Value) -> Result<Value, CccError> {
    match (operator, value) {
        (UnaryOperation::Negate, Value::Integer(n)) => n
            .checked_neg()
            .map(Value::Integer)
            .ok_or_else(|| CccError::eval("integer negation overflow".to_string())),
        (UnaryOperation::Negate, Value::Float(n)) => Ok(Value::Float(-n)),
        (UnaryOperation::Negate, Value::DurationTime(s)) => Ok(Value::DurationTime(-*s)),
        (UnaryOperation::Negate, v) => {
            Err(CccError::eval(format!("cannot negate a {}", v.type_name())))
        }
        (UnaryOperation::Positive, v) => Ok(v.clone()),
    }
}
