# Examples

## Basic Arithmetic

```bash
ccc "2 + 3"
# 5

ccc "10 - 4"
# 6

ccc "3 * 7"
# 21

ccc "10 / 3"
# 3.3333333333333335

ccc "10 % 3"
# 1

ccc "2 ^ 10"
# 1024
```

## Operator Precedence

```bash
ccc "2 + 3 * 4"
# 14

ccc "(2 + 3) * 4"
# 20

ccc "2 ^ 3 ^ 2"
# 512
```

Power is right-associative: `2 ^ 3 ^ 2` = `2 ^ (3 ^ 2)` = `2 ^ 9` = `512`.

## Unary Operators

```bash
ccc "-5 + 3"
# -2

ccc "--5"
# 5
```

## Math Functions

```bash
ccc "sqrt(16)"
# 4

ccc "abs(-42)"
# 42

ccc "sin(3.14159265)"
# 0.00000000358979...

ccc "log(1)"
# 0

ccc "log(2, 8)"
# 3

ccc "log(10, 100)"
# 2

ccc "ln(1)"
# 0

ccc "floor(3.7)"
# 3

ccc "ceil(3.2)"
# 4

ccc "round(3.5)"
# 4
```

## List Operations

```bash
ccc "len([1, 2, 3, 4, 5])"
# 5

ccc "sum([1, 2, 3, 4, 5])"
# 15

ccc "prod([1, 2, 3, 4, 5])"
# 120

ccc "head([10, 20, 30])"
# 10

ccc "tail([10, 20, 30])"
# [20, 30]
```

## Duration Arithmetic

```bash
ccc "1:30:00 + 0:45:00"
# 2:15:00

ccc "2:00:00 - 0:30:00"
# 1:30:00

ccc "1:00:00 * 3"
# 3:00:00

ccc "3:00:00 / 2"
# 1:30:00

ccc "-1:30:00"
# -1:30:00
```

## DateTime Arithmetic

```bash
ccc "2025-12-25T15:30:00Z + 1:00:00"
# 2025-12-25T16:30:00Z

ccc "2025-12-25T15:30:00Z - 2025-12-25T12:00:00Z"
# 3:30:00
```

### DateTime with Timezone

```bash
ccc "2025-12-25T15:30:00+09:00"
# 2025-12-25T15:30:00+09:00
```

## Type Cast (`as`)

```bash
ccc "3 as float"
# 3

ccc "3.7 as int"
# 3

ccc "2025-12-25T00:00:00Z as timestamp"
# 1766620800

ccc "Timestamp(1766620800) as datetime"
# 2025-12-25T00:00:00Z
```

## Time Utility Functions

```bash
ccc "now()"
# (current UTC datetime)

ccc "today()"
# (today at 00:00:00Z)

ccc "current_timestamp()"
# (current Unix timestamp)
```

## Pipe Input

Read a single expression:

```bash
echo "2 + 3" | ccc
# 5
```

Append an expression to piped input:

```bash
echo "10" | ccc "* 2"
# 20
```

Process multiple lines:

```bash
printf "1 + 1\n2 + 2\n3 + 3" | ccc
# 2
# 4
# 6
```

## REPL

```bash
ccc repl
ccc> 1 + 2
3
ccc> sqrt(16) + 1
5
ccc> [1, 2, 3]
[1, 2, 3]
ccc> sum([1, 2, 3, 4, 5])
15
ccc> exit
```

## Error Examples

### Parse Error

```bash
ccc "2 + + 3"
#   2 + + 3
#       ^
#   error: parse error: expected number, function call, or '('
```

### Type Error

```bash
ccc "1 + 1:30:00"
#   error: type check: unsupported operation: integer + duration
```

### Division by Zero

```bash
ccc "10 / 0"
#   error: eval error: division by zero: 10 / 0
```

### Empty List

```bash
ccc "head([])"
#   error: eval error: head: empty list
```
