# Type Checker

## Overview

The type checker (`AstTypeChecker`) validates that operations in the AST are type-safe before evaluation. It implements the `CccTypeChecker` trait defined in the domain layer.

Type checking is performed after parsing and before evaluation. If the type checker finds an error, evaluation is skipped entirely.

## How It Works

The type checker recursively traverses the AST and infers the `StaticType` of each expression node. If an operation involves incompatible types, it returns `CccError::TypeCheck`.

```
AbstractSyntaxTree
  │
  ▼
infer_type(expression) → Result<StaticType, CccError>
```

### Leaf Nodes

| Expression | Inferred Type |
|-----------|---------------|
| `Integer(n)` | Integer |
| `Float(n)` | Float |
| `List([...])` | List |
| `DurationTime{...}` | DurationTime |
| `DateTime{...}` | DateTime |

### Unary Operations

The operand must be Integer, Float, or DurationTime:

```
-42        → Integer    (ok)
-3.14      → Float      (ok)
-1:30:00   → DurationTime (ok)
-[1,2,3]   → error: cannot apply unary operator to list
```

### Binary Operations

The type checker uses the same rules described in the [Type System](type_system.md#binary-operation-type-rules) specification. Incompatible combinations produce an error:

```
1 + 1:30:00   → error: unsupported operation: integer + duration
```

### Function Calls

The type checker validates:

1. **Argument count** - Each function expects a specific number of arguments
2. **Argument types** - Each argument must match the expected type

Nested expressions in arguments are also type-checked.

## Function Validation Rules

### Math Functions

`sqrt`, `sin`, `cos`, `tan`, `arcsin`, `arccos`, `arctan`, `ln`, `floor`, `ceil`, `round`:

- Exactly 1 argument
- Argument must be numeric (Integer, Float, or Unknown)

`log`:

- 1 or 2 arguments
- All arguments must be numeric
- 1 arg: natural logarithm, 2 args: logarithm with base (first arg)

`abs`:

- Exactly 1 argument
- Argument must be numeric

### List Functions

`len`, `sum`, `prod`, `head`, `tail`:

- Exactly 1 argument
- Argument must be List

### Time Constructors

`DurationTime`:

- 3 or 4 arguments
- All arguments must be Integer

`DateTime`:

- Exactly 6 arguments
- All arguments must be Integer

`Timestamp`:

- Exactly 1 argument
- Argument must be numeric

### Type Cast (`as`)

The `as` operator validates the following conversions:

- Integer/Float → `int` or `float` (numeric casts)
- DateTime → `timestamp`
- Timestamp → `datetime`

Other combinations produce a type error:

```
1 as timestamp   → error: cannot cast integer to timestamp
```

### Time Utilities

`now`, `today`:

- Exactly 0 arguments
- Returns DateTime

`current_timestamp`:

- Exactly 0 arguments
- Returns Timestamp

## Unknown Type

The `Unknown` type acts as a wildcard during type checking:

- Binary operations with Unknown on either side pass through as Unknown
- Functions that return Unknown (e.g., `head`, `tail`, `sum`, `prod`) allow any subsequent operation
- Unknown function names pass through as Unknown (the evaluator catches them at runtime)

This allows operations like `head([1, 2, 3]) + 1` to pass type checking, deferring the actual type validation to the evaluator.

## Error Messages

Type errors include context about the operation and types:

```
unsupported operation: datetime + integer
```

```
cannot apply unary operator to list
```

```
sqrt: expected numeric argument, got list
```

```
DurationTime expects 3 or 4 arguments, got 1
```

```
DateTime: argument 2 expected integer, got float
```
