# Type System

## Overview

ccc has a static type system. Types are checked before evaluation so that type errors are caught early without side effects.

There are two type representations:

- **`StaticType`** - Used during type checking (compile-time types)
- **`Value`** - Used during evaluation (runtime values)

## Types

### Integer

Signed 64-bit integer (`i64`).

```
42
-7
0
```

### Float

64-bit floating point (`f64`).

```
3.14
-0.5
```

### List

A sequence of values. Elements can be of any type.

```
[1, 2, 3]
[1.0, 2, 3.5]
[]
```

### DurationTime

A time duration stored as total seconds (signed `i64`). Represented in `HH:MM:SS` format.

```
1:30:00
0:05:30
```

Negative durations are displayed with a leading `-`:

```
-1:30:00
```

### DateTime

A point in time stored as UTC epoch seconds with a timezone offset for display. Represented in ISO 8601 format.

```
2025-12-25T15:30:00Z
2025-12-25T15:30:00+09:00
```

### Timestamp

A Unix timestamp stored as `f64` for sub-second precision.

```
Timestamp(1735000000)
```

### Unknown

An internal type used when the static type cannot be determined (e.g., `head([1, 2, 3])` returns an element whose type is unknown at check time). Unknown values pass through type checking without error.

## Binary Operation Type Rules

### Numeric Operations

All operators (`+`, `-`, `*`, `/`, `%`, `^`) are supported.

| Left | Right | Result |
|------|-------|--------|
| Integer | Integer | Integer |
| Integer | Float | Float |
| Float | Integer | Float |
| Float | Float | Float |

### DurationTime Operations

| Left | Operator | Right | Result |
|------|----------|-------|--------|
| DurationTime | `+` `-` | DurationTime | DurationTime |
| DurationTime | `*` `/` | Integer | DurationTime |
| Integer | `*` | DurationTime | DurationTime |

### DateTime Operations

| Left | Operator | Right | Result |
|------|----------|-------|--------|
| DateTime | `+` `-` | DurationTime | DateTime |
| DateTime | `-` | DateTime | DurationTime |

### Timestamp Operations

| Left | Operator | Right | Result |
|------|----------|-------|--------|
| Timestamp | `+` `-` | DurationTime | Timestamp |
| Timestamp | `-` | Timestamp | DurationTime |

### Unary Operations

Unary `+` and `-` are supported for:

- Integer
- Float
- DurationTime

Applying a unary operator to other types is a type error.

### Invalid Operations

Any combination not listed above results in a type error:

```
  "hello" + 1
  error: type check: unsupported operation: ...
```

## Function Type Rules

### Math Functions

All require a numeric argument (Integer or Float) and return Float.

| Function | Input | Output |
|----------|-------|--------|
| `sqrt`, `sin`, `cos`, `tan` | numeric | Float |
| `arcsin`, `arccos`, `arctan` | numeric | Float |
| `log`, `log2`, `log10` | numeric | Float |
| `floor`, `ceil`, `round` | numeric | Float |
| `abs` | numeric | same as input |

### List Functions

| Function | Input | Output |
|----------|-------|--------|
| `len` | List | Integer |
| `sum` | List | Unknown |
| `prod` | List | Unknown |
| `head` | List | Unknown |
| `tail` | List | Unknown |

`sum`, `prod`, `head`, `tail` return Unknown because element types are not tracked statically.

### Time Constructors

| Function | Arguments | Output |
|----------|-----------|--------|
| `DurationTime` | 3 Integers (h, m, s) or 4 Integers (d, h, m, s) | DurationTime |
| `DateTime` | 6 Integers (year, month, day, hour, minute, second) | DateTime |
| `Timestamp` | 1 numeric | Timestamp |

### Time Converters

| Function | Arguments | Output |
|----------|-----------|--------|
| `datetime_to_timestamp` | DateTime | Timestamp |
| `timestamp_to_datetime` | Timestamp | DateTime |
| `timestamp_to_datetime` | Timestamp, Integer (offset hours) | DateTime |

### Time Utilities

| Function | Arguments | Output |
|----------|-----------|--------|
| `now` | none | DateTime |
| `today` | none | DateTime |
| `current_timestamp` | none | Timestamp |

## Error Messages

Type errors include the operation and the types involved:

```
unsupported operation: datetime + integer
```

```
sqrt: expected numeric argument, got list
```

```
DurationTime expects 3 or 4 arguments, got 1
```
