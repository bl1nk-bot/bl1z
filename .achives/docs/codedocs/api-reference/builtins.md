---
title: "Built-Ins"
description: "Reference every built-in function exported by the builtins modules."
---

Built-ins are defined under `src/builtins` and returned as `BuiltinFunction` values. They are not active until you call `formula_engine::builtins::register_all(&mut registry)`.

## String Built-Ins

Import path: `formula_engine::builtins::string`

### Signatures

```rust
pub fn len() -> BuiltinFunction
pub fn upper() -> BuiltinFunction
pub fn lower() -> BuiltinFunction
pub fn contains() -> BuiltinFunction
pub fn starts_with() -> BuiltinFunction
pub fn ends_with() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `len` | `1` | `String` or `Array` | `Value::Number` | `src/builtins/string.rs` |
| `upper` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `lower` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `contains` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `starts_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `ends_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |

Example:

```rust
let result = formula_engine::evaluate(
    &formula_engine::parse(&formula_engine::tokenize("upper(\"hello\")").unwrap()).unwrap(),
    &formula_engine::Context::new(),
    &{
        let mut registry = formula_engine::FunctionRegistry::new();
        formula_engine::builtins::register_all(&mut registry);
        registry
    },
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"HELLO\")");
```

## Math Built-Ins

Import path: `formula_engine::builtins::math`

```rust
pub fn abs() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `abs` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |

## Logic Built-Ins

Import path: `formula_engine::builtins::logic`

```rust
pub fn if_fn() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `if` | `3` | `Bool`, any, any | branch value | `src/builtins/logic.rs` |

`if_fn()` registers a function named `if`.

## Collection Built-Ins

Import path: `formula_engine::builtins::collection`

```rust
pub fn sum() -> BuiltinFunction
pub fn avg() -> BuiltinFunction
pub fn min_arr() -> BuiltinFunction
pub fn max_arr() -> BuiltinFunction
pub fn join() -> BuiltinFunction
pub fn count() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `sum` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `avg` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `min` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `max` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `join` | `2` | `Array<String>`, `String` | `Value::String` | `src/builtins/collection.rs` |
| `count` | `1` | `Array<any>` | `Value::Number` | `src/builtins/collection.rs` |

Notes:

- `sum([])` returns `0`.
- `avg([])`, `min([])`, and `max([])` return `E504`.
- `count` is similar to `len` for arrays.

Example:

```rust
let source = "join([\"north\", \"south\"], \"/\")";
let mut registry = formula_engine::FunctionRegistry::new();
formula_engine::builtins::register_all(&mut registry);
let result = formula_engine::evaluate(
    &formula_engine::parse(&formula_engine::tokenize(source).unwrap()).unwrap(),
    &formula_engine::Context::new(),
    &registry,
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"north/south\")");
```

## Date Built-Ins

Import path: `formula_engine::builtins::date`

```rust
pub fn now() -> BuiltinFunction
pub fn date_add() -> BuiltinFunction
pub fn date() -> BuiltinFunction
pub fn year() -> BuiltinFunction
pub fn month() -> BuiltinFunction
pub fn day() -> BuiltinFunction
pub fn date_diff() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `now` | `0` | none | current timestamp string | `src/builtins/date.rs` |
| `date_add` | `2` | date string, number of days | date string | `src/builtins/date.rs` |
| `date` | `3` | year, month, day numbers | date string | `src/builtins/date.rs` |
| `year` | `1` | date or timestamp string | `Value::Number` | `src/builtins/date.rs` |
| `month` | `1` | date or timestamp string | `Value::Number` | `src/builtins/date.rs` |
| `day` | `1` | date or timestamp string | `Value::Number` | `src/builtins/date.rs` |
| `date_diff` | `3` | date string, date string, unit string | `Value::Number` | `src/builtins/date.rs` |

Important behavior:

- `date_add` parses a civil date and adds whole calendar days using `jiff`.
- `year`, `month`, and `day` accept either a date string or a timestamp string because `parse_to_timestamp` tries both.
- `date_diff` currently ignores the third `unit` argument and always returns day-based output.

## Combining Built-Ins

```rust
use formula_engine::builtins;
use formula_engine::{Context, FunctionRegistry, evaluate, parse, tokenize};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let source = "if(count([1,2,3]) == 3, month(date_add(\"2023-01-01\", 31)), 0)";
let result = evaluate(&parse(&tokenize(source).unwrap()).unwrap(), &Context::new(), &registry).unwrap();
assert_eq!(format!("{result:?}"), "Number(2.0)");
```

Use [Function System](/docs/function-registry) for extension patterns and [Context and Functions](/docs/api-reference/context-and-functions) for registry details.
