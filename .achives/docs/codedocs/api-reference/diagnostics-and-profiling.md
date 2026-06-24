---
title: "Diagnostics, Errors, And Profiling"
description: "Reference structured errors, diagnostic formatting, and performance analysis helpers."
---

This page covers the support modules that sit around the parser and evaluator: `src/error.rs`, `src/diagnostics.rs`, and `src/profiling.rs`.

## `formula_engine::error`

### `ErrorKind`

```rust
pub enum ErrorKind {
    LexError,
    ParseError,
    EvalError,
    TypeError,
    FunctionError,
    ContextError,
}
```

### `FormulaError`

Import path: `formula_engine::FormulaError` or `formula_engine::error::FormulaError`

```rust
pub struct FormulaError {
    pub kind: ErrorKind,
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
}
```

### `FormulaError::new`

```rust
pub fn new(kind: ErrorKind, code: &str, message: &str, span: Option<Span>) -> Self
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `kind` | `ErrorKind` | — | Error category |
| `code` | `&str` | — | Stable code such as `E101` or `E401` |
| `message` | `&str` | — | Human-readable message |
| `span` | `Option<Span>` | — | Source location when available |

## `formula_engine::diagnostics`

### `format_error`

Import path: `formula_engine::diagnostics::format_error`

```rust
pub fn format_error(source: &str, error: &FormulaError) -> String
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `source` | `&str` | — | Original formula source |
| `error` | `&FormulaError` | — | Structured error to render |

Example:

```rust
use formula_engine::diagnostics::format_error;

let source = "a | b";
let err = formula_engine::tokenize(source).unwrap_err();
println!("{}", format_error(source, &err));
```

## `formula_engine::profiling`

### `PerformanceMetrics`

```rust
pub struct PerformanceMetrics {
    pub tokenize_time: Duration,
    pub parse_time: Duration,
    pub eval_time: Duration,
    pub total_time: Duration,
    pub iterations: usize,
}
```

### `profile_formula`

Import path: `formula_engine::profiling::profile_formula`

```rust
pub fn profile_formula(
    formula: &str,
    ctx: &Context,
    registry: &FunctionRegistry,
    iterations: usize,
) -> Result<PerformanceMetrics, FormulaError>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `formula` | `&str` | — | Formula to measure |
| `ctx` | `&Context` | — | Runtime context |
| `registry` | `&FunctionRegistry` | — | Function registry |
| `iterations` | `usize` | — | Number of measurement loops. **Must be > 0.** Passing `0` will cause a runtime panic (divide-by-zero). |

### `OptimizationSuggestions`

```rust
pub struct OptimizationSuggestions {
    pub suggestions: Vec<String>,
    pub complexity: FormulaComplexity,
}
```

### `FormulaComplexity`

```rust
pub enum FormulaComplexity {
    Simple,
    Moderate,
    Complex,
    High,
}
```

### `analyze_formula`

Import path: `formula_engine::profiling::analyze_formula`

```rust
pub fn analyze_formula(formula: &str) -> Result<OptimizationSuggestions, FormulaError>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `formula` | `&str` | — | Formula to inspect |

Implementation notes from `src/profiling.rs`:

- Large arrays over `20` elements raise complexity toward `Complex`.
- Arrays over `100` elements are labeled `High`.
- `sum`, `avg`, `min`, and `max` trigger caching suggestions when called with a single array argument.
- A function named `fibonacci` is treated as high complexity in the analyzer, which aligns with the recursive example in `examples/advanced.rs` (though this function is not a registered built-in).

## Example

```rust
use formula_engine::builtins;
use formula_engine::profiling::{analyze_formula, profile_formula};
use formula_engine::{Context, FunctionRegistry};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let metrics = profile_formula("sum([1,2,3,4,5])", &Context::new(), &registry, 100).unwrap();
let analysis = analyze_formula("sum([1,2,3,4,5])").unwrap();

println!("{metrics:?}");
println!("{analysis:?}");
```

When you need human-friendly debugging, pair this page with [Error Reporting](/docs/error-reporting).
