/// Target type for `as` type cast expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum CastTargetType {
    Integer,
    Float,
    Timestamp,
    DateTime,
}

/// Binary operators.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

/// Unary operators.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
    Negate,
    Positive,
}

/// AST node representing an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    BinaryOperation {
        operator: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperation,
        operand: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    List(Vec<Expression>),
    /// Type cast expression: `<expr> as int` or `<expr> as float`.
    TypeCast {
        operand: Box<Expression>,
        target_type: CastTargetType,
    },
    /// Duration literal parsed from `HH:MM:SS` syntax.
    DurationTime {
        hours: i64,
        minutes: u8,
        seconds: u8,
    },
    /// DateTime literal parsed from `YYYY-MM-DDTHH:MM:SS` syntax.
    DateTime {
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        /// Timezone offset in seconds from UTC.
        offset_seconds: i32,
    },
}

/// Root of the abstract syntax tree produced by the parser.
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSyntaxTree {
    pub expression: Expression,
}
