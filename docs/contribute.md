# Contributing

## Development Environment

### With Nix (recommended)

```bash
nix develop
```

This provides `rustc`, `cargo`, `rust-analyzer`, `rustfmt`, and `clippy`.

### Without Nix

Install [Rust](https://www.rust-lang.org/) (Edition 2024) manually.

## Project Structure

```
ccc/
├── domain/          Core types, traits, errors
├── infrastructure/  Parser, TypeChecker, Evaluator implementations
├── usecase/         Application logic and orchestration
└── presentation/    CLI interface and entry point
```

Dependencies flow inward: presentation → usecase → domain ← infrastructure. See [System Architecture](spec/system_architecture.md) for details.

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

Tests are located alongside the source code as `*_test.rs` files, plus integration tests in `ccc/presentation/tests/`.

## Lint and Format

```bash
cargo fmt
cargo clippy
```

## Adding a New Built-in Function

Function names must follow the [Naming Convention](spec/naming_convention.md). In short: regular functions use **snake_case**, constructors use **PascalCase**.

1. Add the function name to the match in `ccc/infrastructure/src/evaluator/builtin.rs`
2. Add type checking rules in `ccc/infrastructure/src/type_checker/ast_type_checker.rs`
3. Add tests for both the evaluator and type checker
4. Document the function in `docs/spec/interpreter.md` and `docs/spec/type_system.md`

## Adding a New Syntax

1. Update the PEG grammar in `ccc/infrastructure/src/parser/grammar.pest`
2. Add the AST node variant in `ccc/domain/src/ast.rs`
3. Handle the new node in:
   - Parser: `ccc/infrastructure/src/parser/pest_based_parser.rs`
   - Type checker: `ccc/infrastructure/src/type_checker/ast_type_checker.rs`
   - Evaluator: `ccc/infrastructure/src/evaluator/ast_evaluator.rs`
4. Add the display format in `ccc/domain/src/value.rs` if a new Value variant is needed
5. Add tests at each layer

## Pull Requests

- Create one PR per issue
- Include tests for new functionality
- Ensure `cargo test` and `cargo clippy` pass before submitting
