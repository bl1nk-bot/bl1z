---
title: "Context And Functions"
description: "Reference Context, BuiltinFunction, FunctionRegistry, and builtins::register_all."
---

This page documents the runtime configuration API exposed through `src/context.rs`, `src/functions.rs`, and `src/builtins/mod.rs`.

## `formula_engine::Context`

Source: `src/context.rs`

### Type

```rust
pub struct Context
```

Stores variables in a private `HashMap<String, Value>`.

### `Context::new`

Import path: `formula_engine::Context::new`

```rust
pub fn new() -> Self
```

Creates an empty context.

### `Context::set`

```rust
pub fn set(&mut self, name: &str, value: Value)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `&str` | — | Variable name stored in the context |
| `value` | `Value` | — | Runtime value associated with the variable |

### `Context::get`

```rust
pub fn get(&self, name: &str) -> Option<&Value>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `&str` | — | Variable name to retrieve |

Example:

```rust
use bl1z::{Context, Value};

let mut ctx = Context::new();
ctx.set("enabled", Value::Bool(true));
assert_eq!(ctx.get("enabled"), Some(&Value::Bool(true)));
```

## `formula_engine::functions::BuiltinFunction`

Source: `src/functions.rs`

```rust
pub struct BuiltinFunction {
    pub name: String,
    pub arity: usize,
    pub call: fn(&[Value]) -> Result<Value, FormulaError>,
}
```

This is the unit of registration in `FunctionRegistry`.

Example:

```rust
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::Value;

fn double(args: &[Value]) -> Result<Value, FormulaError> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n * 2.0)),
        _ => Err(FormulaError::new(ErrorKind::TypeError, "E401", "Expected number", None)),
    }
}

let func = BuiltinFunction {
    name: "double".to_string(),
    arity: 1,
    call: double,
};
```

## `formula_engine::FunctionRegistry`

Source: `src/functions.rs`

### Type

```rust
pub struct FunctionRegistry
```

### `FunctionRegistry::new`

```rust
pub fn new() -> Self
```

Creates an empty registry.

### `FunctionRegistry::register`

```rust
pub fn register(&mut self, func: BuiltinFunction)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `func` | `BuiltinFunction` | — | Function entry to insert or replace |

### `FunctionRegistry::find`

```rust
pub fn find(&self, name: &str) -> Option<&BuiltinFunction>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `&str` | — | Function name to look up |

Example:

```rust
use formula_engine::{FunctionRegistry, builtins};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
assert!(registry.find("len").is_some());
```

## `formula_engine::builtins::register_all`

Source: `src/builtins/mod.rs`

Import path: `formula_engine::builtins::register_all`

```rust
pub fn register_all(registry: &mut FunctionRegistry)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `registry` | `&mut FunctionRegistry` | — | Target registry for all standard built-ins |

Registers built-ins from:

- `formula_engine::builtins::string`
- `formula_engine::builtins::math`
- `formula_engine::builtins::logic`
- `formula_engine::builtins::collection`
- `formula_engine::builtins::date`

## Common Pattern: Built-Ins Plus Custom Functions

```rust
use formula_engine::builtins;
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::{FunctionRegistry, Value};

fn is_even(args: &[Value]) -> Result<Value, FormulaError> {
    match args.first() {
        Some(Value::Number(n)) if n.fract() == 0.0 => Ok(Value::Bool((*n as i64) % 2 == 0)),
        _ => Err(FormulaError::new(ErrorKind::TypeError, "E401", "Expected integer", None)),
    }
}

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
registry.register(BuiltinFunction {
    name: "is_even".to_string(),
    arity: 1,
    call: is_even,
});
```

See [Built-Ins](/docs/api-reference/builtins) for the full standard catalog.
