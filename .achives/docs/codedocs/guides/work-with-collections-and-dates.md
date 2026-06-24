---
title: "Work With Collections And Dates"
description: "Use arrays, maps, and date helpers to build realistic formulas over nested application data."
---

This guide focuses on the data-heavy parts of the runtime: arrays, maps, dot access, and date helpers. These features are where the crate becomes useful for scoring, reporting, and workflow rules instead of only arithmetic.

## Problem

You need formulas that read nested records, aggregate lists, and compare dates, for example when scoring a user, generating a label, or checking time-based rules.

## Solution

Build a `Context` with nested `Value::Map` and `Value::Array` values, then use collection functions such as `sum`, `avg`, `count`, and `join`, plus date helpers like `date_add`, `year`, `month`, and `day`. The implementation for these lives in `src/builtins/collection.rs`, `src/builtins/date.rs`, and the dot lookup logic in `src/eval.rs`.

<Steps>
<Step>
### Model nested data with `Value::Map`

```rust
use formula_engine::{Context, Value};
use std::collections::HashMap;

let mut profile = HashMap::new();
profile.insert("name".to_string(), Value::String("Alice".to_string()));
profile.insert("created_at".to_string(), Value::String("2023-06-15".to_string()));
profile.insert(
    "scores".to_string(),
    Value::Array(vec![
        Value::Number(85.0),
        Value::Number(90.0),
        Value::Number(87.5),
    ]),
);

let mut user = HashMap::new();
user.insert("profile".to_string(), Value::Map(profile));

let mut ctx = Context::new();
ctx.set("user", Value::Map(user));
```

</Step>
<Step>
### Evaluate collection formulas

```rust
let average_formula = "avg(user.profile.scores)";
let count_formula = "count(user.profile.scores)";
```

</Step>
<Step>
### Use date helpers on stored strings

```rust
let month_formula = "month(date_add(user.profile.created_at, 30))";
let label_formula = r#"if(avg(user.profile.scores) > 88, upper(user.profile.name), "review")"#;
```

</Step>
</Steps>

## Complete Example

```rust
use formula_engine::builtins;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut profile = HashMap::new();
    profile.insert("name".to_string(), Value::String("Alice".to_string()));
    profile.insert("created_at".to_string(), Value::String("2023-06-15".to_string()));
    profile.insert(
        "scores".to_string(),
        Value::Array(vec![
            Value::Number(85.0),
            Value::Number(90.0),
            Value::Number(87.5),
        ]),
    );

    let mut user = HashMap::new();
    user.insert("profile".to_string(), Value::Map(profile));

    let mut ctx = Context::new();
    ctx.set("user", Value::Map(user));

    for source in [
        "avg(user.profile.scores)",
        "count(user.profile.scores)",
        "month(date_add(user.profile.created_at, 30))",
        r#"if(avg(user.profile.scores) > 88, upper(user.profile.name), "review")"#,
    ] {
        let result = evaluate(&parse(&tokenize(source)?)?, &ctx, &registry)?;
        println!("{source} => {result:?}");
    }

    Ok(())
}
```

Example output:

```text
avg(user.profile.scores) => Number(87.5)
count(user.profile.scores) => Number(3.0)
month(date_add(user.profile.created_at, 30)) => Number(7.0)
if(avg(user.profile.scores) > 88, upper(user.profile.name), "review") => String("review")
```

## Notes From The Source

- `sum`, `avg`, `min`, `max`, and `count` require an array as their first argument.
- `join` requires an array of strings plus a separator string.
- `date_add` expects a date string plus a number of days.
- `date_diff` currently accepts three arguments, but the third unit argument is ignored in `src/builtins/date.rs`.

That last point matters if you are designing a public formula API. Even though examples may pass `"days"`, the implementation always computes a day-based difference by converting seconds to days.

Related pages: [Runtime Data Model](/docs/runtime-data-model), [Built-Ins API](/docs/api-reference/builtins), and [Function System](/docs/function-registry).
