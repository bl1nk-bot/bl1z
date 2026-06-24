---
title: "Getting Started"
description: "Build, parse, and evaluate Notion-like formulas in Rust with a small extensible runtime."
---

`formula_engine` is a Rust crate for tokenizing, parsing, and evaluating Notion-like formula expressions with variables, built-in functions, arrays, maps, and date helpers.

## The Problem

- Expression logic often gets scattered across ad-hoc `match` blocks, SQL fragments, and application-specific validation code.
- Rolling a parser from scratch usually means rebuilding operator precedence, error spans, and a runtime type system before application logic can even start.
- Product code frequently needs user-defined formulas that can reference external data, but direct `eval`-style approaches are unsafe and hard to constrain.
- Teams need a way to extend formulas with custom domain functions without rewriting the execution pipeline.

## The Solution

`formula_engine` splits the job into clear stages exposed by the crate root in [`src/lib.rs`](https://github.com/bl1nk-bot/poe-sdk-rs/blob/main/src/lib.rs): `tokenize`, `parse`, and `evaluate`. You pass a formula string into the lexer, parse the token stream into a `SpannedExpr`, and evaluate the AST against a `Context` plus a `FunctionRegistry`. The same registry can hold built-ins from [`src/builtins`](https://github.com/bl1nk-bot/poe-sdk-rs/tree/main/src/builtins) and your own functions.

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("score", Value::Number(87.5));
    ctx.set("bonus", Value::Number(12.5));

    let tokens = tokenize(r#"if(score + bonus >= 100, "pass", "review")"#)?;
    let ast = parse(&tokens)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    println!("{result:?}");
    Ok(())
}
```

Expected result:

```text
String("pass")
```

## Installation

<Tabs values={["crates.io", "git", "path", "workspace"]}>
<Tab value="crates.io">

```toml
[dependencies]
formula_engine = "0.1.0"
```

</Tab>
<Tab value="git">

```toml
[dependencies]
formula_engine = { git = "https://github.com/bl1nk-bot/poe-sdk-rs", branch = "main" }
```

</Tab>
<Tab value="path">

```toml
[dependencies]
formula_engine = { path = "../poe-sdk-rs" }
```

</Tab>
<Tab value="workspace">

```toml
[workspace.dependencies]
formula_engine = { path = "poe-sdk-rs" }
```

</Tab>
</Tabs>

This project is a Rust crate, so installation happens through `Cargo.toml` rather than JavaScript package managers.

## Quick Start

The minimum viable flow is:

1. Create a `FunctionRegistry`.
2. Register built-ins.
3. Create a `Context` for external variables.
4. `tokenize`, then `parse`, then `evaluate`.

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("name", Value::String("Alice".to_string()));
    ctx.set("score", Value::Number(85.0));

    let formula = "if(score >= 80, upper(name), lower(name))";
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    println!("formula: {formula}");
    println!("result: {result:?}");
    Ok(())
}
```

Expected output:

```text
formula: if(score >= 80, upper(name), lower(name))
result: String("ALICE")
```

If you are evaluating the same formula repeatedly, keep the parsed AST and call `evaluate` multiple times with different contexts. The parser and evaluator are separated for exactly that reason.

## Key Features

- Layered pipeline: lexer, parser, evaluator, diagnostics, and profiling helpers are exposed as separate modules.
- Runtime values support `Number`, `String`, `Bool`, `Null`, `Array`, and nested `Map`.
- Built-ins cover string, logic, collection, and date operations and are registered explicitly with `builtins::register_all`.
- `Context` resolves variables at runtime, including dot-separated access into nested `Value::Map` structures.
- Errors carry an `ErrorKind`, a stable code like `E401`, and optional `Span` information for formatted diagnostics.
- The crate includes lightweight profiling utilities for measuring and analyzing formula cost.

## Supported Environments

- Rust `1.70+` from the project README badge.
- Edition `2021`, as declared in [`Cargo.toml`](https://github.com/bl1nk-bot/poe-sdk-rs/blob/main/Cargo.toml).
- Standard library environments. The crate uses `std::collections::HashMap`, `std::time`, and the `jiff` date library.

## Where To Go Next

<Cards>
  <Card title="Architecture" href="/docs/architecture">See how the lexer, parser, evaluator, and registries fit together.</Card>
  <Card title="Core Concepts" href="/docs/execution-pipeline">Understand the execution pipeline, runtime values, functions, and error model.</Card>
  <Card title="API Reference" href="/docs/api-reference/crate-root">Jump straight to import paths, signatures, and source-backed behavior.</Card>
</Cards>
