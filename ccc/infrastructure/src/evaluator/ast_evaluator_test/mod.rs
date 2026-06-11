mod arithmetic;
mod builtin_math;
mod datetime;
mod duration;
mod list;
mod literal;
mod power_unary;
mod time_function;
mod timestamp;
mod type_cast;

use domain::ast::AbstractSyntaxTree;
use domain::error::CccError;
use domain::value::Value;

use crate::evaluator::AstEvaluator;

pub(super) fn eval(expression: domain::ast::Expression) -> Result<Value, CccError> {
    let evaluator = AstEvaluator;
    let ast = AbstractSyntaxTree { expression };
    use domain::interface::evaluator::CccEvaluator;
    evaluator.evaluate(&ast)
}
