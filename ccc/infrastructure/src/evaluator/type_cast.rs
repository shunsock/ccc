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
            // NaN would silently become epoch 0 through `as i64`; reject it upfront.
            // Infinities saturate and then fail the EpochSeconds range check below.
            Value::Timestamp(ts) if ts.is_nan() => {
                Err(CccError::eval("cannot cast NaN timestamp to datetime"))
            }
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
