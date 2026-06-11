use pest::Parser as PestParserTrait;
use pest_derive::Parser as PestDerive;

use domain::ast::{AbstractSyntaxTree, Expression};
use domain::error::CccError;
use domain::interface::parser::CccParser;

use super::atomic_expression::{build_atom, build_function_call, build_list, build_number};
use super::binary_expression::{build_binary_expression, build_right_associative_expression};
use super::datetime_literal::build_datetime_literal;
use super::duration_literal::build_duration_literal;
use super::error_message::to_parse_error;
use super::operator_mapping::{to_additive_operation, to_multiplicative_operation};
use super::postfix_expression::build_postfix;
use super::unary_expression::build_unary;

#[derive(PestDerive)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

/// CccParser implementation backed by pest PEG parser.
pub struct PestBasedParser;

impl CccParser for PestBasedParser {
    fn parse(&self, input: &str) -> Result<AbstractSyntaxTree, CccError> {
        // Rule::program is anchored with SOI/EOI, so input with a valid
        // prefix but invalid trailing content fails here instead of being
        // silently truncated (see issue #39).
        let pairs =
            ExpressionParser::parse(Rule::program, input).map_err(|e| to_parse_error(&e))?;

        let pair = pairs
            .into_iter()
            .next()
            .and_then(|program| program.into_inner().next())
            .ok_or_else(|| CccError::parse("empty input".to_string()))?;

        let expression = build_expression(pair)?;
        Ok(AbstractSyntaxTree { expression })
    }
}

/// Dispatch a parsed pair to the builder for its grammar rule.
pub(super) fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
    match pair.as_rule() {
        Rule::expression => build_binary_expression(pair, to_additive_operation),
        Rule::term => build_binary_expression(pair, to_multiplicative_operation),
        Rule::power => build_right_associative_expression(pair),
        Rule::unary => build_unary(pair),
        Rule::postfix => build_postfix(pair),
        Rule::atom => build_atom(pair),
        Rule::number => build_number(pair),
        Rule::function_call => build_function_call(pair),
        Rule::datetime_literal => build_datetime_literal(pair),
        Rule::duration_literal => build_duration_literal(pair),
        Rule::list => build_list(pair),
        _ => Err(CccError::parse(format!(
            "unexpected rule: {:?}",
            pair.as_rule()
        ))),
    }
}
