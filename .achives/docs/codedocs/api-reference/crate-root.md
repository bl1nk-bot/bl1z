---
title: "Crate Root"
description: "Reference the top-level imports and re-exports exposed by formula_engine."
---

The crate root in `src/lib.rs` is the main import surface for application code. It re-exports the most common types and functions so you can build the full formula pipeline without reaching into module paths unless you want implementation details such as `BuiltinFunction` or profiling helpers.

## Import Path

```rust
use formula_engine::{Context, Expr, FormulaError, FunctionRegistry, Value, evaluate, parse, tokenize};
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

## Public Modules

The crate root also exposes these modules directly:

| Module | Import path | Purpose |
|--------|-------------|---------|
| `ast` | `formula_engine::ast` | AST enums and span-carrying expression wrappers |
| `builtins` | `formula_engine::builtins` | Standard function registration and grouped built-ins |
| `context` | `formula_engine::context` | Runtime variable storage |
| `diagnostics` | `formula_engine::diagnostics` | Error formatting helpers |
| `error` | `formula_engine::error` | Error types and constructors |
| `eval` | `formula_engine::eval` | Evaluator entry point |
| `functions` | `formula_engine::functions` | Function registry and callable entries |
| `lexer` | `formula_engine::lexer` | Tokens and tokenizer |
| `parser` | `formula_engine::parser` | Parser and AST construction |
| `profiling` | `formula_engine::profiling` | Performance measurement and analysis |
| `span` | `formula_engine::span` | `Position` and `Span` types |
| `value` | `formula_engine::value` | Runtime values |

## Example

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

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
