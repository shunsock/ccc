mod cast;
mod duration_datetime;
mod function;
mod list;
mod numeric;

use domain::ast::AbstractSyntaxTree;
use domain::interface::type_checker::CccTypeChecker;

use crate::type_checker::AstTypeChecker;

pub(super) fn check(expression: domain::ast::Expression) -> Result<(), domain::error::CccError> {
    let ast = AbstractSyntaxTree { expression };
    let checker = AstTypeChecker;
    checker.check(&ast)
}
