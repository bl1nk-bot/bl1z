---
title: "Crate Root"
description: "Reference the top-level imports and re-exports exposed by bl1z."
---

The crate root in `src/lib.rs` is the main import surface for application code. It re-exports the most common types and functions so you can build the full formula pipeline without reaching into module paths unless you want implementation details such as `BuiltinFunction`, recovery parsing, or profiling helpers.

## Import Path

```rust
use bl1z::{
    Context, EngineConfig, Expr, FormulaError, FunctionRegistry, RecoveryResult, Value,
    evaluate, evaluate_with_config, parse, parse_with_recovery, tokenize,
};
```

## Re-Exported Items

### `tokenize`

Source: `src/lib.rs` re-exporting `src/lexer.rs`

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, FormulaError>
```

Tokenizes a formula string into `Token` values with spans.

### `parse`

Source: `src/lib.rs` re-exporting `src/parser.rs`

```rust
pub fn parse(tokens: &[Token]) -> Result<SpannedExpr, FormulaError>
```

Builds a `SpannedExpr` AST from a token slice.

### `evaluate`

Source: `src/lib.rs` re-exporting `src/eval.rs`

```rust
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError>
```

Evaluates an AST using runtime variables and a function registry.

### `evaluate_with_config`

Source: `src/lib.rs` re-exporting `src/eval.rs`

```rust
pub fn evaluate_with_config(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
    config: &EngineConfig,
) -> Result<Value, FormulaError>
```

Evaluates an AST while enforcing engine limits such as maximum depth or timeout.

### `Context`

Source: `src/lib.rs` re-exporting `src/context.rs`

```rust
pub struct Context
```

Runtime variable store for formulas.

### `FunctionRegistry`

Source: `src/lib.rs` re-exporting `src/functions.rs`

```rust
pub struct FunctionRegistry
```

Registry used to store and look up callable functions by name.

### `Value`

Source: `src/lib.rs` re-exporting `src/value.rs`

```rust
pub enum Value
```

Runtime result type for evaluated formulas.

### `Expr`

Source: `src/lib.rs` re-exporting `src/ast.rs`

```rust
pub enum Expr
```

AST node type wrapped by `SpannedExpr`.

### `FormulaError`

Source: `src/lib.rs` re-exporting `src/error.rs`

```rust
pub struct FormulaError
```

Structured error type used across all phases.

### `EngineConfig`

Source: `src/lib.rs` re-exporting `src/config.rs`

```rust
pub struct EngineConfig
```

Holds configurable limits such as maximum formula length, evaluation depth, and optional timeout.

### `RecoveryResult`

Source: `src/lib.rs` re-exporting `src/parser.rs`

```rust
pub struct RecoveryResult
```

Contains a partially recovered AST plus the parse errors collected by `parse_with_recovery`.

## Public Modules

The crate root also exposes these modules directly:

| Module | Import path | Purpose |
|--------|-------------|---------|
| `ast` | `bl1z::ast` | AST enums and span-carrying expression wrappers |
| `builtins` | `bl1z::builtins` | Standard function registration and grouped built-ins |
| `cache` | `bl1z::cache` | Formula caching utilities |
| `config` | `bl1z::config` | Engine limit configuration |
| `context` | `bl1z::context` | Runtime variable storage |
| `diagnostics` | `bl1z::diagnostics` | Error formatting helpers |
| `error` | `bl1z::error` | Error types and constructors |
| `eval` | `bl1z::eval` | Evaluator entry point |
| `functions` | `bl1z::functions` | Function registry and callable entries |
| `lexer` | `bl1z::lexer` | Tokens and tokenizer |
| `parser` | `bl1z::parser` | Parser and AST construction |
| `plugins` | `bl1z::plugins` | Plugin trait and plugin manager |
| `profiling` | `bl1z::profiling` | Performance measurement and analysis |
| `span` | `bl1z::span` | `Position` and `Span` types |
| `value` | `bl1z::value` | Runtime values |

## Example

```rust
use bl1z::builtins;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("score", Value::Number(42.0));

    let ast = parse(&tokenize("score + 8")?)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    assert_eq!(format!("{result:?}"), "Number(50.0)");
    Ok(())
}
```

Use the next pages for the complete signatures and field definitions behind these re-exports.
