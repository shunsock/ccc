# Naming Convention

## Overview

This document defines the naming rules for built-in functions. All new functions must follow these rules.

## Rules

### 1. Regular Functions: snake_case

Functions that compute values use **snake_case**.

- Single-word names are plain lowercase: `sqrt`, `abs`, `sin`, `sum`, `len`, `now`, `today`
- Multi-word names use underscores: `current_timestamp`, `log2`, `log10`

### 2. Constructors: PascalCase

Functions that create a typed value use **PascalCase**, matching the type name they construct.

| Constructor | Type |
|-------------|------|
| `DurationTime(...)` | duration |
| `DateTime(...)` | datetime |
| `Timestamp(...)` | timestamp |

This visual distinction makes it clear that the call produces a specific domain type, not a derived numeric result.

### 3. Operators and Keywords: lowercase

Operators and keywords embedded in the grammar use **lowercase**.

- Type cast keyword: `as`
- Cast target types: `int`, `float`, `timestamp`, `datetime`

## Naming Guidelines

### Prefer full words over abbreviations

Use descriptive names that read naturally. Abbreviations are acceptable only when they are universally understood in the domain.

| Preferred | Avoid |
|-----------|-------|
| `mean` | `E` |
| `variance` | `V` |
| `median` | `med` |
| `arcsin` | `asin` |

Exception: `sqrt`, `abs`, `sin`, `cos`, `tan`, `log`, `len`, `prod`, `max`, `min` are standard abbreviations widely understood in math and programming.

### Avoid prefixes for categories

Do not prefix function names with their category.

| Preferred | Avoid |
|-----------|-------|
| `sum` | `list_sum` |
| `mean` | `stat_mean` |

### Use `as` for type conversion, not functions

Type conversions use the `as` operator, not conversion functions.

| Preferred | Avoid |
|-----------|-------|
| `x as int` | `to_int(x)` |
| `dt as timestamp` | `datetime_to_timestamp(dt)` |

## Reference: Complete Function List

### Regular Functions (snake_case)

`sqrt`, `abs`, `sin`, `cos`, `tan`, `arcsin`, `arccos`, `arctan`, `log`, `log2`, `log10`, `floor`, `ceil`, `round`, `mean`, `variance`, `max`, `min`, `median`, `len`, `sum`, `prod`, `head`, `tail`, `now`, `today`, `current_timestamp`

### Constructors (PascalCase)

`DurationTime`, `DateTime`, `Timestamp`
