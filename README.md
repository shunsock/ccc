# ccc: CalCulator Cli

A command-line calculator written in Rust. It supports arithmetic operations, built-in mathematical functions, list operations, and date/time calculations.

## Features

- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
- Built-in functions: `sqrt`, `sin`, `cos`, `log`, `abs`, `floor`, `ceil`, `round`, etc.
- List operations: `len`, `sum`, `prod`, `head`, `tail`
- Date/time arithmetic: `DateTime`, `DurationTime`, `Timestamp`
- Static type checking before evaluation
- Interactive REPL mode
- Pipe input support

## Installation

### Build from source

```bash
cargo build --release
```

## Usage

### Expression mode

```bash
ccc "2 + 3 * 4"
# 14
```

### REPL mode

```bash
ccc repl
ccc> sqrt(16) + 1
5
ccc> exit
```

### Pipe mode

```bash
echo "2 + 3" | ccc
# 5
```

```bash
echo "10" | ccc "+ 5"
# 15
```

## Development

The development environment is managed by [LazyNix](https://github.com/shunsock/lazynix). `lazynix.yaml` declares the toolchain and `flake.nix` / `flake.lock` are generated from it.

```bash
# Enter the development shell (rustc, cargo, clippy, rustfmt, rust-analyzer)
nix run github:shunsock/lazynix -- develop

# Run the full check suite (fmt, clippy, build, test)
nix run github:shunsock/lazynix -- test
```

Plain Nix also works since `flake.nix` is a standard flake:

```bash
nix develop
```

## Documentation

- [Getting Started](docs/getting_started.md)
- [Examples](docs/example.md)
- [Contributing](docs/contribute.md)

### Specification

- [System Architecture](docs/spec/system_architecture.md)
- [Type System](docs/spec/type_system.md)
- [AST](docs/spec/ast.md)
- [Interpreter](docs/spec/interpreter.md)
- [Type Checker](docs/spec/type_checker.md)

## License

Licensed under either of

- MIT License ([LICENSE](LICENSE))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
