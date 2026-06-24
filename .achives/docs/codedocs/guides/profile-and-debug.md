---
title: "Profile And Debug"
description: "Measure formula cost and render better diagnostics with the profiling and diagnostics modules."
---

This guide covers the operational side of `formula_engine`: understanding why a formula failed and deciding whether a formula is too expensive to evaluate repeatedly.

## Problem

Once formulas are in production, the hard part is no longer just “does it run.” You need to know which phase failed, where the bad token sits in the original expression, and whether some formulas are doing more work than they should.

## Solution

Use `diagnostics::format_error` to render errors with carets, and use `profiling::profile_formula` plus `profiling::analyze_formula` to measure and inspect formulas. These helpers are defined in `src/diagnostics.rs` and `src/profiling.rs`, and they build on the public API instead of hidden internals.

<Steps>
<Step>
### Render syntax and runtime errors

```rust
use formula_engine::diagnostics::format_error;
use formula_engine::tokenize;

let source = "a & b";
let err = tokenize(source).unwrap_err();
println!("{}", format_error(source, &err));
```

</Step>
<Step>
### Measure average stage timings

```rust
use formula_engine::builtins;
use formula_engine::profiling::profile_formula;
use formula_engine::{Context, FunctionRegistry};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
let metrics = profile_formula("sum([1,2,3,4,5])", &Context::new(), &registry, 100)?;
println!("{metrics:?}");
```

</Step>
<Step>
### Analyze formulas before accepting them

```rust
use formula_engine::profiling::analyze_formula;

let analysis = analyze_formula("sum([1,2,3,4,5,6,7,8,9,10])")?;
println!("{analysis:?}");
```

</Step>
</Steps>

## Complete Example

```rust
use formula_engine::builtins;
use formula_engine::diagnostics::format_error;
use formula_engine::profiling::{analyze_formula, profile_formula};
use formula_engine::{Context, FunctionRegistry, tokenize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    let bad_source = "1 + \"oops\"";
    let err = (|| {
        let tokens = tokenize(bad_source)?;
        let ast = formula_engine::parse(&tokens)?;
        formula_engine::evaluate(&ast, &ctx, &registry)
    })()
    .unwrap_err();

    println!("{}", format_error(bad_source, &err));

    let metrics = profile_formula("sum([1,2,3,4,5])", &ctx, &registry, 50)?;
    println!("average total: {:?}", metrics.total_time);

    let analysis = analyze_formula("sum([1,2,3,4,5,6,7,8,9,10])")?;
    println!("complexity: {:?}", analysis.complexity);
    println!("suggestions: {:?}", analysis.suggestions);

    Ok(())
}
```

## Operational Advice

- Profile complete formulas before exposing them to hot request paths.
- If the same formula runs many times, cache the AST and only call `evaluate`.
- Use `analyze_formula` as a linting step in admin tooling, not just during manual debugging.
- When showing failures to users, prefer a combination of `kind`, `code`, and a formatted diagnostic instead of only the raw message.

Related pages: [Error Reporting](/docs/error-reporting) and [Diagnostics, Errors, and Profiling API](/docs/api-reference/diagnostics-and-profiling).
