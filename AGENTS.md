# Agent Guide for comp

## Project Context
**Project**: Stack-based RPN interpreter written in Rust
**Tech Stack**: Rust (edition 2021), Cargo build system
**Architecture**: List processor for reverse-Polish notation, stack-based execution model

## Build & Test Commands
- **Build**: `cargo build --release` (outputs to `target/release/comp`)
- **Development build**: `cargo build` (faster, includes debug symbols)
- **Run all tests**: `./tests/tests.sh` (executes all .cm test files sequentially)
- **Run single test**: `comp -f ./tests/<testname>.cm` (e.g., `comp -f ./tests/maths.cm`)
- **Format code**: `cargo fmt` (uses rustfmt for standard Rust formatting)
- **Version check**: `comp version`

## Code Style Guidelines

### Imports & Organization
- Order: std library → external crates → local modules
- Group related imports, separate with blank lines
- Example: `use std::{env, fs};` then `use colored::ColoredString;` then `mod comp;`

### Naming Conventions
- **Variables/Functions**: `snake_case` (e.g., `evaluate_ops`, `file_contents`)
- **Types/Structs**: `PascalCase` (e.g., `Interpreter`, `BoxedClosure`)
- **Constants**: `SCREAMING_SNAKE_CASE` as static (e.g., `PERSISTENCE_FILE`, `RELEASE_STATE`)

### Error Handling
- Use explicit error messages with `eprintln!` and `cor::Theme` for colorized output
- Exit with appropriate `exitcode` constants (e.g., `exitcode::NOINPUT`, `exitcode::OSFILE`)
- Pattern: `match` for error handling, propagate with clear context

### Code Patterns
- Prefer `match` over `if-else` chains for control flow
- Constructors always named `new()` for structs
- Use HashMap for command dispatch pattern: `HashMap<String, fn(&mut Interpreter, &str)>`
- Comment architecture/design notes generously; use `//` inline, `/* */` for blocks

### Testing
- Tests are `.cm` files (comp language scripts) in `tests/` directory
- Test output uses stack result validation (e.g., `1252 ifeq` checks expected value)
- Tests include: `maths.cm`, `functions.cm`, `memory.cm`, `map.cm`, `fold.cm`, `scan.cm`

## Context Handoff Notes
When transitioning between tasks, preserve:
- Current implementation state and affected files
- Open questions or architectural decisions needed
- Test coverage status
- Performance or design constraints
