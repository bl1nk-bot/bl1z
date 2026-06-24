---
title: "Built-Ins"
description: "Reference every built-in function exported by the builtins modules."
---

Built-ins are defined under `src/builtins` and returned as `BuiltinFunction` values. They are not active until you call `bl1z::builtins::register_all(&mut registry)`.

## String Built-Ins

Import path: `bl1z::builtins::string`

### Signatures

```rust
pub fn len() -> BuiltinFunction
pub fn upper() -> BuiltinFunction
pub fn lower() -> BuiltinFunction
pub fn contains() -> BuiltinFunction
pub fn starts_with() -> BuiltinFunction
pub fn ends_with() -> BuiltinFunction
pub fn trim() -> BuiltinFunction
pub fn trim_start() -> BuiltinFunction
pub fn trim_end() -> BuiltinFunction
pub fn split() -> BuiltinFunction
pub fn replace() -> BuiltinFunction
pub fn substring() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `len` | `1` | `String` or `Array` | `Value::Number` | `src/builtins/string.rs` |
| `upper` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `lower` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `contains` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `starts_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `ends_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `trim` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `trim_start` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `trim_end` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `split` | `2` | `String`, `String` | `Value::Array` | `src/builtins/string.rs` |
| `replace` | `3` | `String`, `String`, `String` | `Value::String` | `src/builtins/string.rs` |
| `substring` | `3` | `String`, `Number`, `Number` | `Value::String` | `src/builtins/string.rs` |

Example:

```rust
let result = bl1z::evaluate(
    &bl1z::parse(&bl1z::tokenize("upper(\"hello\")").unwrap()).unwrap(),
    &bl1z::Context::new(),
    &{
        let mut registry = bl1z::FunctionRegistry::new();
        bl1z::builtins::register_all(&mut registry);
        registry
    },
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"HELLO\")");
```

## Math Built-Ins

Import path: `bl1z::builtins::math`

```rust
pub fn abs() -> BuiltinFunction
pub fn pi() -> BuiltinFunction
pub fn round() -> BuiltinFunction
pub fn ceil() -> BuiltinFunction
pub fn floor() -> BuiltinFunction
pub fn sqrt() -> BuiltinFunction
pub fn pow() -> BuiltinFunction
pub fn sin() -> BuiltinFunction
pub fn cos() -> BuiltinFunction
pub fn tan() -> BuiltinFunction
pub fn random() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `abs` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `pi` | `0` | none | `Value::Number` | `src/builtins/math.rs` |
| `round` | `1..2` | `Number`, optional precision | `Value::Number` | `src/builtins/math.rs` |
| `ceil` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `floor` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `sqrt` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `pow` | `2` | `Number`, `Number` | `Value::Number` | `src/builtins/math.rs` |
| `sin` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `cos` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `tan` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `random` | `0` | none | `Value::Number` | `src/builtins/math.rs` |

## Logic Built-Ins

Import path: `bl1z::builtins::logic`

```rust
pub fn if_fn() -> BuiltinFunction
```

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `if` | `3` | `Bool`, any, any | branch value | `src/builtins/logic.rs` |

`if_fn()` registers a function named `if`.

## Collection Built-Ins

Import path: `bl1z::builtins::collection`

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

## Higher-Order And Sequence Helpers

Import path: `bl1z::builtins::higher_order`

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `map` | `2` | `Array`, `Lambda` | `Value::Array` | `src/builtins/higher_order.rs` |
| `filter` | `2` | `Array`, `Lambda` | `Value::Array` | `src/builtins/higher_order.rs` |
| `reduce` | `3` | `Array`, `Lambda`, initial value | any | `src/builtins/higher_order.rs` |
| `sort` | `1..2` | `Array`, optional key lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `sort_with` | `2` | `Array`, comparator lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `unique` | `1..2` | `Array`, optional key lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `group_by` | `2` | `Array`, key lambda | `Value::Map` | `src/builtins/higher_order.rs` |
| `set` | `1` | `Array` | `Value::Set` | `src/builtins/higher_order.rs` |
| `range` | `1..3` | numeric bounds | `Value::Range` | `src/builtins/higher_order.rs` |
| `range_to_array` | `1` | `Range` | `Value::Array` | `src/builtins/higher_order.rs` |

## Set Built-Ins

Import path: `bl1z::builtins::sets`

| Function name | Arity | Accepts | Returns | Source |
|---------------|-------|---------|---------|--------|
| `set_union` | `2` | `Set`-like values | `Value::Set` | `src/builtins/sets.rs` |
| `set_intersection` | `2` | `Set`-like values | `Value::Set` | `src/builtins/sets.rs` |
| `set_difference` | `2` | `Set`-like values | `Value::Set` | `src/builtins/sets.rs` |
| `set_in` | `2` | value, `Set`-like value | `Value::Bool` | `src/builtins/sets.rs` |

Example:

```rust
let source = "join([\"north\", \"south\"], \"/\")";
let mut registry = bl1z::FunctionRegistry::new();
bl1z::builtins::register_all(&mut registry);
let result = bl1z::evaluate(
    &bl1z::parse(&bl1z::tokenize(source).unwrap()).unwrap(),
    &bl1z::Context::new(),
    &registry,
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"north/south\")");
```

## Date Built-Ins

Import path: `bl1z::builtins::date`

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
| `now` | `0` | none | `Value::DateTime` | `src/builtins/date.rs` |
| `date_add` | `2` | `DateTime` or date string, number of days | `Value::DateTime` | `src/builtins/date.rs` |
| `date` | `3` | year, month, day numbers | `Value::DateTime` | `src/builtins/date.rs` |
| `year` | `1` | `DateTime` or date string | `Value::Number` | `src/builtins/date.rs` |
| `month` | `1` | `DateTime` or date string | `Value::Number` | `src/builtins/date.rs` |
| `day` | `1` | `DateTime` or date string | `Value::Number` | `src/builtins/date.rs` |
| `date_diff` | `3` | date-like, date-like, unit string | `Value::Number` | `src/builtins/date.rs` |

Important behavior:

- `date_add` and `date_diff` accept both native `Value::DateTime` and string input for backward compatibility.
- `date_diff` supports explicit units such as days, hours, minutes, months, and years.
- `now()` and `date()` now return native runtime date values instead of strings.

## Combining Built-Ins

```rust
use bl1z::builtins;
use bl1z::{Context, FunctionRegistry, evaluate, parse, tokenize};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let source = "if(count([1,2,3]) == 3, month(date_add(\"2023-01-01\", 31)), 0)";
let result = evaluate(&parse(&tokenize(source).unwrap()).unwrap(), &Context::new(), &registry).unwrap();
assert_eq!(format!("{result:?}"), "Number(2.0)");
```

Use [Function System](/docs/function-registry) for extension patterns and [Context and Functions](/docs/api-reference/context-and-functions) for registry details.
