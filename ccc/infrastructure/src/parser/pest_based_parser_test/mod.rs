mod arithmetic;
mod datetime;
mod duration;
mod error;
mod function_call;
mod list;
mod number;
mod precedence;
mod type_cast;
mod unary_postfix;

use domain::interface::parser::CccParser;

use crate::parser::PestBasedParser;

pub(super) fn parse_expr(input: &str) -> domain::ast::Expression {
    let parser = PestBasedParser;
    parser.parse(input).unwrap().expression
}
