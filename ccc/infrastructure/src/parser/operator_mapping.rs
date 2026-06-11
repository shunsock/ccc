use domain::ast::{BinaryOperation, UnaryOperation};

use super::pest_based_parser::Rule;

pub(super) fn to_additive_operation(pair: &pest::iterators::Pair<Rule>) -> BinaryOperation {
    match pair.as_str() {
        "+" => BinaryOperation::Add,
        "-" => BinaryOperation::Subtract,
        _ => BinaryOperation::Add,
    }
}

pub(super) fn to_multiplicative_operation(pair: &pest::iterators::Pair<Rule>) -> BinaryOperation {
    match pair.as_str() {
        "*" => BinaryOperation::Multiply,
        "/" => BinaryOperation::Divide,
        "%" => BinaryOperation::Modulo,
        _ => BinaryOperation::Multiply,
    }
}

pub(super) fn to_unary_operation(pair: &pest::iterators::Pair<Rule>) -> UnaryOperation {
    match pair.as_str() {
        "-" => UnaryOperation::Negate,
        _ => UnaryOperation::Positive,
    }
}
