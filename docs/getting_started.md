# Getting Started

## Installation

### With Nix (recommended)

```bash
nix profile install github:shunsock/ccc
```

### Build from Source

You need one of the following:

- [Rust](https://www.rust-lang.org/) (Edition 2024)
- [Nix](https://nixos.org/) (with flakes enabled)

#### With Cargo

```bash
cargo build --release
```

The binary is created at `target/release/ccc`.

#### With Nix

```bash
nix build
```

The binary is created at `result/bin/ccc`.

## Development Setup

If you have Nix installed, the development shell provides all necessary tools:

```bash
nix develop
```

This includes `rustc`, `cargo`, `rust-analyzer`, `rustfmt`, and `clippy`.

## Basic Usage

### Evaluate an expression

Pass a mathematical expression as an argument:

```bash
ccc "2 + 3 * 4"
# 14
```

### Interactive REPL

Start an interactive session with the `repl` subcommand:

```bash
ccc repl
ccc> 1 + 2
3
ccc> sqrt(16)
4
ccc> exit
```

Type `exit` or `quit` to leave the REPL.

### Pipe input

Read expressions from stdin, one per line:

```bash
echo "10 / 3" | ccc
# 3.3333333333333335
```

You can also append an expression to each line of piped input:

```bash
echo "10" | ccc "+ 5"
# 15
```

## What's Next

- [Examples](example.md) - More usage examples
- [Type System](spec/type_system.md) - Supported types and rules
- [System Architecture](spec/system_architecture.md) - How ccc is built internally
- [Contributing](contribute.md) - How to contribute
