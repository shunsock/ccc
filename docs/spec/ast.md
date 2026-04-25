# AST (Abstract Syntax Tree)

## Overview

The parser converts an input string into an `AbstractSyntaxTree`, which is a tree of `Expression` nodes. The AST is then passed to the type checker and evaluator.

```rust
pub struct AbstractSyntaxTree {
    pub expression: Expression,
}
```

## Expression Nodes

### Integer

An integer literal.

```rust
Expression::Integer(i64)
```

Example: `42` → `Expression::Integer(42)`

### Float

A floating-point literal.

```rust
Expression::Float(f64)
```

Example: `3.14` → `Expression::Float(3.14)`

### BinaryOperation

An operation with two operands.

```rust
Expression::BinaryOperation {
    operator: BinaryOperation,
    left: Box<Expression>,
    right: Box<Expression>,
}
```

Operators:

| Variant | Symbol | Precedence | Associativity |
|---------|--------|------------|---------------|
| Add | `+` | Low | Left |
| Subtract | `-` | Low | Left |
| Multiply | `*` | Medium | Left |
| Divide | `/` | Medium | Left |
| Modulo | `%` | Medium | Left |
| Power | `^` | High | Right |

Example: `2 + 3 * 4` →

```
BinaryOperation(Add)
├── Integer(2)
└── BinaryOperation(Multiply)
    ├── Integer(3)
    └── Integer(4)
```

Example: `2 ^ 3 ^ 2` (right-associative) →

```
BinaryOperation(Power)
├── Integer(2)
└── BinaryOperation(Power)
    ├── Integer(3)
    └── Integer(2)
```

### UnaryOperation

A prefix operation with one operand.

```rust
Expression::UnaryOperation {
    operator: UnaryOperation,
    operand: Box<Expression>,
}
```

Operators:

| Variant | Symbol |
|---------|--------|
| Negate | `-` |
| Positive | `+` |

Example: `-5` → `UnaryOperation(Negate, Integer(5))`

### FunctionCall

A named function invocation with arguments.

```rust
Expression::FunctionCall {
    name: String,
    arguments: Vec<Expression>,
}
```

Example: `sqrt(16)` → `FunctionCall { name: "sqrt", arguments: [Integer(16)] }`

### List

A sequence of expressions enclosed in brackets.

```rust
Expression::List(Vec<Expression>)
```

Example: `[1, 2, 3]` → `List([Integer(1), Integer(2), Integer(3)])`

### DurationTime

A time duration literal in `HH:MM:SS` format.

```rust
Expression::DurationTime {
    hours: i64,
    minutes: u8,
    seconds: u8,
}
```

Example: `1:30:00` → `DurationTime { hours: 1, minutes: 30, seconds: 0 }`

### DateTime

A date-time literal in ISO 8601 format.

```rust
Expression::DateTime {
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    offset_seconds: i32,
}
```

Example: `2025-12-25T15:30:00+09:00` → `DateTime { year: 2025, month: 12, day: 25, hour: 15, minute: 30, second: 0, offset_seconds: 32400 }`

## Grammar

The parser uses PEG (Parsing Expression Grammar) via [pest](https://pest.rs/).

### Precedence (lowest to highest)

1. **expression** - `term ((+ | -) term)*`
2. **term** - `power ((* | / | %) power)*`
3. **power** - `unary (^ unary)*` (right-associative)
4. **unary** - `(+ | -)? atom`
5. **atom** - function call, datetime literal, duration literal, number, list, or parenthesized expression

### Literal Formats

| Type | Format | Example |
|------|--------|---------|
| Integer | `[0-9]+` | `42` |
| Float | `[0-9]+.[0-9]+` | `3.14` |
| Duration | `[0-9]+:[0-9]{2}:[0-9]{2}` | `1:30:00` |
| DateTime | `YYYY-MM-DDTHH:MM:SS[tz]` | `2025-12-25T15:30:00Z` |

DateTime timezone formats: `Z`, `+09:00`, `-05`, `+09`

### Whitespace

Spaces and tabs are ignored between tokens.

### Identifiers

Function names: `[a-zA-Z_][a-zA-Z0-9_]*`
