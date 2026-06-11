use domain::ast::CastTargetType;
use domain::error::CccError;
use domain::time::{EpochSeconds, UtcOffset};
use domain::value::Value;

use super::value_conversion::{to_f64, to_i64};

pub(super) fn evaluate_type_cast(
    value: &Value,
    target_type: &CastTargetType,
) -> Result<Value, CccError> {
    match target_type {
        CastTargetType::Integer => {
            let n = to_i64(value)?;
            Ok(Value::Integer(n))
        }
        CastTargetType::Float => {
            let n = to_f64(value)?;
            Ok(Value::Float(n))
        }
        CastTargetType::Timestamp => match value {
            Value::DateTime { epoch, .. } => Ok(Value::Timestamp(epoch.seconds() as f64)),
            _ => Err(CccError::eval(format!(
                "cannot cast {} to timestamp",
                value.type_name()
            ))),
        },
        CastTargetType::DateTime => match value {
            Value::Timestamp(ts) => EpochSeconds::from_seconds(*ts as i64)
                .map(|epoch| Value::DateTime {
                    epoch,
                    offset: UtcOffset::UTC,
                })
                .ok_or_else(|| CccError::eval("timestamp out of datetime range")),
            _ => Err(CccError::eval(format!(
                "cannot cast {} to datetime",
                value.type_name()
            ))),
        },
    }
}
