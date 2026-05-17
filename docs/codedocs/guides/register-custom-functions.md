---
title: "Register Custom Functions"
description: "Extend formula_engine with application-specific functions using FunctionRegistry."
---

This guide covers the most important customization point in the crate: adding your own functions. The built-ins cover generic text, logic, collections, and dates, but real applications usually need functions that encode product rules or domain knowledge.

## Problem

You want formulas to call operations that are not part of the standard built-in set, such as `clamp`, `tier_name`, or `percent_of`, and you need them to behave like first-class formula functions.

## Solution

Implement a function with the signature required by `BuiltinFunction`, register it in `FunctionRegistry`, and evaluate formulas with that registry. The source example in `examples/advanced.rs` follows this exact pattern for `fibonacci`, `power`, `is_even`, and `clamp`.

<Steps>
<Step>
### Write a function that accepts `&[Value]`

```rust
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::Value;

fn percent_of(args: &[Value]) -> Result<Value, FormulaError> {
    match (args.first(), args.get(1)) {
        (Some(Value::Number(value)), Some(Value::Number(percent))) => {
            Ok(Value::Number(value * percent / 100.0))
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "percent_of expects two numbers",
            None,
        )),
    }
}
```

</Step>
<Step>
### Register it with an exact name and arity

```rust
use formula_engine::functions::BuiltinFunction;
use formula_engine::FunctionRegistry;

let mut registry = FunctionRegistry::new();
registry.register(BuiltinFunction {
    name: "percent_of".to_string(),
    arity: 2,
    call: percent_of,
});
```

</Step>
<Step>
### Combine built-ins and custom functions

```rust
use formula_engine::builtins;

builtins::register_all(&mut registry);
```

</Step>
<Step>
### Evaluate formulas against the custom registry

```rust
use formula_engine::{evaluate, parse, tokenize, Context};

let ast = parse(&tokenize("percent_of(250, 15)")?)?;
let result = evaluate(&ast, &Context::new(), &registry)?;
```

</Step>
</Steps>

## Complete Example

```rust
use formula_engine::builtins;
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn percent_of(args: &[Value]) -> Result<Value, FormulaError> {
    match (args.first(), args.get(1)) {
        (Some(Value::Number(value)), Some(Value::Number(percent))) => {
            Ok(Value::Number(value * percent / 100.0))
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "percent_of expects two numbers",
            None,
        )),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    registry.register(BuiltinFunction {
        name: "percent_of".to_string(),
        arity: 2,
        call: percent_of,
    });

    let ast = parse(&tokenize("percent_of(250, 15) + 2")?)?;
    let result = evaluate(&ast, &Context::new(), &registry)?;

    assert_eq!(format!("{result:?}"), "Number(39.5)");
    Ok(())
}
```

## Real-World Pattern

A common pattern is to keep formulas declarative and move heavy business logic into functions. For example, instead of storing a long formula for loyalty tier thresholds, you can expose a single `tier_name(score)` function and keep the tier logic in Rust where it is testable and versioned.

## Things To Watch

- `arity` is strict. If you want optional behavior, register multiple function names or encode options into a map/string argument.
- Your function receives already-evaluated arguments. It cannot inspect raw AST nodes.
- Return `FormulaError` values with consistent `kind` and `code` so the host application can interpret failures.

Related pages: [Function System](/docs/function-registry) and [Context and Functions API](/docs/api-reference/context-and-functions).
