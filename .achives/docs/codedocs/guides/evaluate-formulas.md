---
title: "Evaluate Formulas"
description: "Parse and evaluate formulas safely in an application using the standard pipeline."
---

This guide shows the standard production flow for taking a formula string, validating it, and executing it against application data. It is the right starting point when formulas come from configuration, a user interface, or a database column.

## Problem

You need to evaluate expressions such as pricing rules, score thresholds, or feature flags, but you do not want to mix parsing, data access, and error handling throughout your business code.

## Solution

Use the explicit pipeline from `formula_engine`: create a registry, register built-ins, prepare a `Context`, tokenize the source, parse the tokens, and evaluate the AST. This aligns with the crate’s implementation in `src/lib.rs` and gives you clean hooks for validation and diagnostics.

<Steps>
<Step>
### Add the crate and set up the registry

```toml
[dependencies]
formula_engine = "0.1.0"
```

```rust
use formula_engine::builtins;
use formula_engine::FunctionRegistry;

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
```

</Step>
<Step>
### Build a runtime context from application data

```rust
use formula_engine::{Context, Value};

let mut ctx = Context::new();
ctx.set("subtotal", Value::Number(180.0));
ctx.set("vip", Value::Bool(true));
ctx.set("customer_name", Value::String("Alice".to_string()));
```

</Step>
<Step>
### Tokenize, parse, and evaluate the formula

```rust
use formula_engine::{evaluate, parse, tokenize};

let source = r#"if(vip && subtotal > 100, upper(customer_name), "guest")"#;
let tokens = tokenize(source)?;
let ast = parse(&tokens)?;
let result = evaluate(&ast, &ctx, &registry)?;
```

</Step>
<Step>
### Handle errors in one place

```rust
use formula_engine::diagnostics::format_error;

match tokenize(source)
    .and_then(|tokens| parse(&tokens))
    .and_then(|ast| evaluate(&ast, &ctx, &registry))
{
    Ok(value) => println!("Result: {value:?}"),
    Err(err) => println!("{}", format_error(source, &err)),
}
```

</Step>
</Steps>

## Complete Example

```rust
use formula_engine::builtins;
use formula_engine::diagnostics::format_error;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("subtotal", Value::Number(180.0));
    ctx.set("vip", Value::Bool(true));
    ctx.set("customer_name", Value::String("Alice".to_string()));

    let source = r#"if(vip && subtotal > 100, upper(customer_name), "guest")"#;

    let result = (|| {
        let tokens = tokenize(source)?;
        let ast = parse(&tokens)?;
        evaluate(&ast, &ctx, &registry)
    })();

    match result {
        Ok(value) => println!("Result: {value:?}"),
        Err(err) => println!("{}", format_error(source, &err)),
    }
}
```

Expected output:

```text
Result: String("ALICE")
```

## Production Notes

- Keep validation and execution separate if formulas are stored. Parse them when they are created, not only when they are used.
- Reuse parsed ASTs for repeated evaluation of the same expression.
- Treat `ErrorKind` and `code` as the stable contract for UI validation or API responses.
- Build the `Context` from clean, formula-friendly values. Nested `Value::Map` structures are the easiest way to support `user.profile.score`-style formulas.

Related pages: [Execution Pipeline](/docs/execution-pipeline), [Runtime Data Model](/docs/runtime-data-model), and [Crate Root API](/docs/api-reference/crate-root).
