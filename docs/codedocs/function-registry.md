---
title: "Function System"
description: "Use FunctionRegistry and built-ins to extend formulas with domain-specific behavior."
---

Functions are the main extension mechanism in `formula_engine`. Instead of hard-coding every operation into the evaluator, the crate stores callable entries in `FunctionRegistry` from `src/functions.rs`, and built-ins are just pre-registered `BuiltinFunction` values returned from modules in `src/builtins`.

## What This Concept Is

A `BuiltinFunction` bundles three things:

- `name: String`
- `arity: usize`
- `call: fn(&[Value]) -> Result<Value, FormulaError>`

`FunctionRegistry` stores those entries by name in a `HashMap`, and `builtins::register_all` populates the registry with the standard library of string, logic, collection, math, and date helpers.

## Why It Exists

This design keeps the evaluator small and makes the formula language application-specific without forking the parser. If your product needs `clamp`, `tier_name`, or `is_premium_user`, you register them the same way the crate registers `len`, `sum`, or `date_add`. The AST only needs to represent “function call by name,” which keeps `Expr::FunctionCall` stable even as the available functions change.

## How It Works Internally

The evaluator branch for `Expr::FunctionCall` in `src/eval.rs` does four things in order:

1. Finds the function in `FunctionRegistry`.
2. Returns `E502` if it is missing.
3. Checks exact arity and returns `E503` on mismatch.
4. Evaluates every argument and invokes the stored function pointer.

That fourth step is important. Functions are eager because the evaluator resolves arguments before calling the function implementation. The `if` helper in `src/builtins/logic.rs` therefore acts like a normal eager function, not a special lazy control-flow form.

```mermaid
flowchart TD
  A[Expr::FunctionCall] --> B[FunctionRegistry::find]
  B -->|missing| C[E502 FunctionError]
  B -->|found| D[Check arity]
  D -->|wrong| E[E503 FunctionError]
  D -->|ok| F[Evaluate all args]
  F --> G[call fn(&[Value])]
  G --> H[Value or FormulaError]
```

## How It Relates To Other Concepts

The [Execution Pipeline](/docs/execution-pipeline) reaches the function system during evaluation. The [Runtime Data Model](/docs/runtime-data-model) defines the `Value` arguments functions receive. The [Error Reporting](/docs/error-reporting) model defines what functions should return when argument validation fails.

## Basic Usage: Register All Built-Ins

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let ast = parse(&tokenize("join([\"a\", \"b\", \"c\"], \"-\")")?)?;
    let result = evaluate(&ast, &Context::new(), &registry)?;

    assert_eq!(format!("{result:?}"), "String(\"a-b-c\")");
    Ok(())
}
```

## Advanced Usage: Add A Custom Function

```rust
use formula_engine::builtins;
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn clamp(args: &[Value]) -> Result<Value, FormulaError> {
    match (args.first(), args.get(1), args.get(2)) {
        (Some(Value::Number(value)), Some(Value::Number(min)), Some(Value::Number(max))) => {
            Ok(Value::Number(value.max(*min).min(*max)))
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "clamp expects value, min, and max as numbers",
            None,
        )),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    registry.register(BuiltinFunction {
        name: "clamp".to_string(),
        arity: 3,
        call: clamp,
    });

    let ast = parse(&tokenize("clamp(125, 0, 100)")?)?;
    let result = evaluate(&ast, &Context::new(), &registry)?;

    assert_eq!(format!("{result:?}"), "Number(100.0)");
    Ok(())
}
```

<Callout type="warn">Function arguments are always fully evaluated before your function runs. Do not register functions that rely on lazy semantics, and do not expect `if()` to protect an invalid branch from evaluation. Also note that `arity` is exact; there is no support for optional or variadic parameters in `FunctionRegistry`.</Callout>

## Trade-Offs

<Accordions>
<Accordion title="Why functions use plain function pointers instead of closures or trait objects">
`BuiltinFunction` stores `call: fn(&[Value]) -> Result<Value, FormulaError>`, which makes registration straightforward and keeps the runtime representation small. Plain function pointers are easy to copy, easy to store in `HashMap`, and avoid lifetime or heap-allocation complexity. The trade-off is that registered functions cannot capture external state directly the way closures can. If you need configuration-dependent behavior, build the function from immutable global data, or expose data through `Context` so the function can read it from its arguments instead of its environment.
</Accordion>
<Accordion title="Why built-ins are opt-in instead of globally available">
The crate could have shipped a pre-populated global registry, but `src/functions.rs` and `src/builtins/mod.rs` intentionally keep registration explicit. That makes startup predictable and keeps tests or host applications in control of which functions exist. The trade-off is a small amount of boilerplate at the call site, because nearly every application begins with `let mut registry = FunctionRegistry::new(); builtins::register_all(&mut registry);`. In exchange, your application can remove built-ins it does not want to expose or layer custom functions on top without hidden global state.
</Accordion>
</Accordions>

The full built-in catalog and signatures are documented in [Built-Ins](/docs/api-reference/builtins).
