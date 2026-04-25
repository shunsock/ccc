use domain::error::CccError;
use domain::interface::evaluator::CccEvaluator;
use domain::interface::parser::CccParser;
use domain::interface::type_checker::CccTypeChecker;
use domain::value::Value;

/// Usecase: calculate a math expression (parse -> type_check -> evaluate).
pub struct CalculateMathExpressionUsecase<P, T, E> {
    parser: P,
    type_checker: T,
    evaluator: E,
}

impl<P, T, E> CalculateMathExpressionUsecase<P, T, E>
where
    P: CccParser,
    T: CccTypeChecker,
    E: CccEvaluator,
{
    pub fn new(parser: P, type_checker: T, evaluator: E) -> Self {
        Self {
            parser,
            type_checker,
            evaluator,
        }
    }

    pub fn execute(&self, input: &str) -> Result<Value, CccError> {
        let ast = self.parser.parse(input)?;
        self.type_checker.check(&ast)?;
        self.evaluator.evaluate(&ast)
    }
}
