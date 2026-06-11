//! Expression / Value builders shared by unit tests.
//!
//! Tests assemble ASTs through these helpers instead of writing
//! `Expression::BinaryOperation { .. }` with `Box::new` noise, so the
//! Arrange section reads like the expression under test:
//! `add(int(1), mul(int(2), int(3)))` is `1 + 2 * 3`.

use domain::ast::{BinaryOperation, CastTargetType, Expression, UnaryOperation};
use domain::time::{DurationSeconds, EpochSeconds, UtcOffset};
use domain::value::Value;

// --- Expression builders: literals ---

pub(crate) fn int(n: i64) -> Expression {
    Expression::Integer(n)
}

pub(crate) fn float(n: f64) -> Expression {
    Expression::Float(n)
}

pub(crate) fn list(elements: Vec<Expression>) -> Expression {
    Expression::List(elements)
}

pub(crate) fn duration_literal(hours: i64, minutes: u8, seconds: u8) -> Expression {
    Expression::DurationTime {
        hours,
        minutes,
        seconds,
    }
}

/// DateTime literal node. `offset_seconds` must be within ±24h.
#[allow(clippy::too_many_arguments)]
pub(crate) fn datetime_literal(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    offset_seconds: i32,
) -> Expression {
    Expression::DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        offset: UtcOffset::from_seconds(offset_seconds).expect("fixture offset within range"),
    }
}

// --- Expression builders: operations ---

pub(crate) fn binary(operator: BinaryOperation, left: Expression, right: Expression) -> Expression {
    Expression::BinaryOperation {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub(crate) fn add(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Add, left, right)
}

pub(crate) fn sub(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Subtract, left, right)
}

pub(crate) fn mul(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Multiply, left, right)
}

pub(crate) fn div(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Divide, left, right)
}

pub(crate) fn modulo(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Modulo, left, right)
}

pub(crate) fn pow(left: Expression, right: Expression) -> Expression {
    binary(BinaryOperation::Power, left, right)
}

pub(crate) fn neg(operand: Expression) -> Expression {
    Expression::UnaryOperation {
        operator: UnaryOperation::Negate,
        operand: Box::new(operand),
    }
}

pub(crate) fn pos(operand: Expression) -> Expression {
    Expression::UnaryOperation {
        operator: UnaryOperation::Positive,
        operand: Box::new(operand),
    }
}

pub(crate) fn call(name: &str, arguments: Vec<Expression>) -> Expression {
    Expression::FunctionCall {
        name: name.to_string(),
        arguments,
    }
}

pub(crate) fn cast(operand: Expression, target_type: CastTargetType) -> Expression {
    Expression::TypeCast {
        operand: Box::new(operand),
        target_type,
    }
}

// --- Value builders ---

pub(crate) fn duration_value(seconds: i64) -> Value {
    Value::DurationTime(DurationSeconds::from_seconds(seconds))
}

/// DateTime value. Both arguments must satisfy the value-object invariants.
pub(crate) fn datetime_value(epoch_seconds: i64, offset_seconds: i32) -> Value {
    Value::DateTime {
        epoch: EpochSeconds::from_seconds(epoch_seconds).expect("fixture epoch within range"),
        offset: UtcOffset::from_seconds(offset_seconds).expect("fixture offset within range"),
    }
}
