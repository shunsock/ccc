use crate::ast::AbstractSyntaxTree;
use crate::error::CccError;
use crate::value::Value;

/// Interface for AST evaluation.
pub trait CccEvaluator {
    fn evaluate(&self, ast: &AbstractSyntaxTree) -> Result<Value, CccError>;
}
