use domain::ast::CastTargetType;
use domain::error::CccError;
use domain::static_type::StaticType;

/// Determine the result type of a type cast, or error if unsupported.
pub(super) fn infer_cast_type(
    operand_type: &StaticType,
    target_type: &CastTargetType,
) -> Result<StaticType, CccError> {
    match (operand_type, target_type) {
        // Numeric casts
        (StaticType::Integer | StaticType::Float, CastTargetType::Integer) => {
            Ok(StaticType::Integer)
        }
        (StaticType::Integer | StaticType::Float, CastTargetType::Float) => Ok(StaticType::Float),
        // DateTime → Timestamp
        (StaticType::DateTime, CastTargetType::Timestamp) => Ok(StaticType::Timestamp),
        // Timestamp → DateTime
        (StaticType::Timestamp, CastTargetType::DateTime) => Ok(StaticType::DateTime),
        // Unknown passes through
        (StaticType::Unknown, target) => Ok(cast_result_type(target)),
        _ => Err(CccError::type_check(format!(
            "cannot cast {operand_type} to {}",
            cast_target_name(target_type)
        ))),
    }
}

fn cast_result_type(target: &CastTargetType) -> StaticType {
    match target {
        CastTargetType::Integer => StaticType::Integer,
        CastTargetType::Float => StaticType::Float,
        CastTargetType::Timestamp => StaticType::Timestamp,
        CastTargetType::DateTime => StaticType::DateTime,
    }
}

fn cast_target_name(target: &CastTargetType) -> &'static str {
    match target {
        CastTargetType::Integer => "int",
        CastTargetType::Float => "float",
        CastTargetType::Timestamp => "timestamp",
        CastTargetType::DateTime => "datetime",
    }
}
