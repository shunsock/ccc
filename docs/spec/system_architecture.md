# System Architecture

## Overview

ccc follows Clean Architecture with four layers. Each layer has a single responsibility, and dependencies flow inward (presentation → usecase → domain ← infrastructure).

```
┌─────────────────────────────────┐
│         Presentation            │  CLI interface (clap)
├─────────────────────────────────┤
│           Usecase               │  Application logic
├─────────────────────────────────┤
│           Domain                │  Core types, traits, errors
├─────────────────────────────────┤
│        Infrastructure           │  Parser, TypeChecker, Evaluator
└─────────────────────────────────┘
```

## Layers

### Domain (`ccc/domain`)

The core of the system. Defines types and interfaces that other layers depend on.

- **`ast.rs`** - `Expression`, `BinaryOperation`, `UnaryOperation`, `AbstractSyntaxTree`
- **`static_type.rs`** - `StaticType` enum for type checking
- **`value.rs`** - `Value` enum for runtime evaluation results
- **`error.rs`** - `CccError` and `SourcePosition`
- **`interface/`** - Trait definitions: `Parser`, `TypeChecker`, `Evaluator`

Domain has no dependencies on other layers.

### Infrastructure (`ccc/infrastructure`)

Implements the interfaces defined in Domain.

- **`parser/`** - `PestBasedParser` using PEG grammar (`grammar.pest`)
- **`type_checker/`** - `AstTypeChecker` for static type validation
- **`evaluator/`** - `AstEvaluator` and `builtin` functions

Infrastructure depends on Domain.

### Usecase (`ccc/usecase`)

Orchestrates the processing pipeline and application-level logic.

- **`calculate_math_expression.rs`** - Main pipeline: parse → type check → evaluate
- **`interactive_repl.rs`** - REPL session management
- **`evaluate_piped_input.rs`** - Line-by-line stdin processing
- **`format_error.rs`** - Error display with caret indicators

Usecase depends on Domain (via trait interfaces). It does not depend on Infrastructure.

### Presentation (`ccc/presentation`)

Entry point and user-facing interface.

- **`main.rs`** - Wires up all layers and runs the application
- **`cli.rs`** - CLI argument definitions using clap
- **`input_mode.rs`** - Determines input mode (Expression, REPL, Pipe, PipeWithArgs)

Presentation depends on Usecase, Infrastructure, and Domain.

## Dependency Direction

```
Presentation ──→ Usecase ──→ Domain
     │                         ↑
     └──→ Infrastructure ──────┘
```

- **Usecase** depends only on **Domain** traits, not on concrete implementations.
- **Presentation** injects concrete Infrastructure implementations into Usecase.
- **Domain** depends on nothing. It is the innermost layer.

## Data Flow

```
Input (string)
  │
  ▼
Parser.parse()          → AbstractSyntaxTree
  │
  ▼
TypeChecker.check()     → Ok(()) or CccError::TypeCheck
  │
  ▼
Evaluator.evaluate()    → Value or CccError::Eval
  │
  ▼
Output (display Value)
```

This pipeline is encapsulated in `CalculateMathExpressionUsecase.execute()`.

## Workspace Structure

The project is organized as a Cargo workspace:

```
Cargo.toml                    (workspace root)
├── ccc/domain/Cargo.toml
├── ccc/infrastructure/Cargo.toml
├── ccc/usecase/Cargo.toml
└── ccc/presentation/Cargo.toml
```

Each layer is a separate crate, enforcing dependency boundaries at compile time.
