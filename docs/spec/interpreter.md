# Interpreter

## Overview

The interpreter (`AstEvaluator`) takes a type-checked `AbstractSyntaxTree` and recursively evaluates it to produce a `Value`. It implements the `CccEvaluator` trait defined in the domain layer.

## Evaluation Pipeline

```
AbstractSyntaxTree
  │
  ▼
evaluate_expression(expression)
  │
  ├── Integer(n) → Value::Integer(n)
  ├── Float(n) → Value::Float(n)
  ├── List([...]) → evaluate each element → Value::List([...])
  ├── DurationTime{h,m,s} → Value::DurationTime(total_seconds)
  ├── DateTime{...} → Value::DateTime{epoch_seconds, offset_seconds}
  ├── UnaryOperation → evaluate operand → apply operator
  ├── BinaryOperation → evaluate both sides → apply operator
  └── FunctionCall → evaluate arguments → call_builtin(name, args)
```

## Numeric Evaluation Rules

### Integer Arithmetic

| Expression | Result | Notes |
|-----------|--------|-------|
| `3 + 2` | `Integer(5)` | |
| `10 / 2` | `Integer(5)` | Exact division stays Integer |
| `10 / 3` | `Float(3.333...)` | Remainder promotes to Float |
| `2 ^ 3` | `Integer(8)` | Non-negative exponent |
| `2 ^ -1` | `Float(0.5)` | Negative exponent promotes to Float |

### Mixed Numeric

When one operand is Float, the result is always Float:

| Expression | Result |
|-----------|--------|
| `1 + 2.0` | `Float(3.0)` |
| `3.0 * 2` | `Float(6.0)` |

### Division by Zero

Returns an evaluation error:

```
10 / 0   → error: division by zero: 10 / 0
10 % 0   → error: modulo by zero: 10 % 0
```

## Unary Operations

| Operation | Input | Result |
|-----------|-------|--------|
| `-n` | Integer | `Integer(-n)` (overflow checked) |
| `-n` | Float | `Float(-n)` |
| `-d` | DurationTime | `DurationTime(-d)` |
| `+v` | any | returns `v` unchanged |

Negating a List, DateTime, or Timestamp is an error.

## Built-in Functions

### Math Functions

All accept a numeric argument (Integer or Float) and return Float.

| Function | Description |
|----------|-------------|
| `sqrt(n)` | Square root |
| `abs(n)` | Absolute value (preserves input type) |
| `sin(n)` | Sine |
| `cos(n)` | Cosine |
| `tan(n)` | Tangent |
| `arcsin(n)` | Arcsine |
| `arccos(n)` | Arccosine |
| `arctan(n)` | Arctangent |
| `log(n)` | Natural logarithm |
| `log2(n)` | Base-2 logarithm |
| `log10(n)` | Base-10 logarithm |
| `floor(n)` | Floor |
| `ceil(n)` | Ceiling |
| `round(n)` | Round to nearest integer |

`abs` is special: it returns Integer for Integer input, Float for Float input.

### List Functions

| Function | Description | Error Condition |
|----------|-------------|-----------------|
| `len(list)` | Number of elements | |
| `sum(list)` | Sum of numeric elements | Non-numeric elements |
| `prod(list)` | Product of numeric elements | Non-numeric elements |
| `head(list)` | First element | Empty list |
| `tail(list)` | All elements except the first | Empty list |

`sum` and `prod` return Integer if all elements are Integer, otherwise Float.

### Time Constructors

| Function | Arguments | Description |
|----------|-----------|-------------|
| `DurationTime(h, m, s)` | 3 Integers | Duration from hours, minutes, seconds |
| `DurationTime(d, h, m, s)` | 4 Integers | Duration from days, hours, minutes, seconds |
| `DateTime(y, mo, d, h, mi, s)` | 6 Integers | DateTime in UTC |
| `Timestamp(n)` | 1 numeric | Unix timestamp |

### Time Converters

| Function | Arguments | Description |
|----------|-----------|-------------|
| `datetime_to_timestamp(dt)` | DateTime | Convert to Unix timestamp |
| `timestamp_to_datetime(ts)` | Timestamp | Convert to DateTime (UTC) |
| `timestamp_to_datetime(ts, offset)` | Timestamp, Integer | Convert with timezone offset (hours) |

### Time Utilities

| Function | Description |
|----------|-------------|
| `now()` | Current UTC datetime |
| `today()` | Today at 00:00:00 UTC |
| `current_timestamp()` | Current Unix timestamp with sub-second precision |

## DateTime Evaluation

DateTime literals store time internally as UTC epoch seconds. A timezone offset is preserved for display but the underlying value is always UTC.

```
2025-12-25T15:30:00+09:00
```

This is stored as:

- `epoch_seconds`: UTC equivalent (15:30 local - 9 hours offset)
- `offset_seconds`: 32400 (9 * 3600)

## Error Handling

The evaluator produces `CccError::Eval` for runtime errors:

- Division/modulo by zero
- Integer negation overflow
- Invalid datetime components
- Unknown function names
- Argument count mismatches
- Type mismatches in function arguments (e.g., `sum` with non-numeric elements)
- Empty list for `head`/`tail`
