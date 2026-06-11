use domain::ast::BinaryOperation;
use domain::error::CccError;
use domain::static_type::StaticType;

/// Determine the result type of a binary operation, or error if unsupported.
pub(super) fn infer_binary_type(
    operator: &BinaryOperation,
    left: &StaticType,
    right: &StaticType,
) -> Result<StaticType, CccError> {
    use BinaryOperation::*;
    use StaticType::*;

    match (left, operator, right) {
        // Numeric: all operators allowed
        (Integer, _, Integer) => Ok(Integer),
        (Integer, _, Float) | (Float, _, Integer) | (Float, _, Float) => Ok(Float),

        // DurationTime ± DurationTime → DurationTime
        (DurationTime, Add | Subtract, DurationTime) => Ok(DurationTime),
        // DurationTime * Integer, DurationTime / Integer → DurationTime
        (DurationTime, Multiply | Divide, Integer) => Ok(DurationTime),
        // Integer * DurationTime → DurationTime
        (Integer, Multiply, DurationTime) => Ok(DurationTime),

        // DateTime ± DurationTime → DateTime
        (DateTime, Add | Subtract, DurationTime) => Ok(DateTime),
        // DateTime - DateTime → DurationTime
        (DateTime, Subtract, DateTime) => Ok(DurationTime),

        // Timestamp ± DurationTime → Timestamp
        (Timestamp, Add | Subtract, DurationTime) => Ok(Timestamp),
        // Timestamp - Timestamp → DurationTime
        (Timestamp, Subtract, Timestamp) => Ok(DurationTime),

        // Unknown passes through (skeleton compatibility)
        (Unknown, _, _) | (_, _, Unknown) => Ok(Unknown),

        _ => Err(CccError::type_check(format!(
            "unsupported operation: {left} {} {right}",
            operator.symbol()
        ))),
    }
}
