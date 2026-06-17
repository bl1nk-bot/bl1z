# ✅ DETAILED COMPLETED TASKS ARCHIVE

This file serves as the permanent record of all completed phases and tasks for the bl1z V2.

## Phase 8: Access Chaining & Indexing — DONE

Duration: 2 weeks
- [x] **8.1** Created `Expr::PropertyAccess { object: Box<SpannedExpr>, field: String }` in AST
- [x] **8.2** Created `Expr::IndexAccess { object: Box<SpannedExpr>, index: Box<SpannedExpr> }` in AST
- [x] **8.3** Refactor parser: Replace string concatenation with recursive `parse_property_access()` / `parse_index_access()`
- [x] **8.4** Refactor evaluator: Replace `name.split('.')` with recursive `eval_property_access()` / `eval_index_access()`
- [x] **8.5** Added errors: `PropertyNotFound` (E207), `IndexOutOfBounds` (E208), `NotIndexable` (E401)
- [x] **8.6** Support nested: `a.b[0].c.d`, `a["key"].nested`
- [x] **8.7** Unit tests: property access on Map/Context, index access on Array/String, chained access, error cases
- [x] **8.8** Integration tests: `user.profile.scores[0]`, `config["db"].host`

## Phase 8.5: Context Scoping — DONE

Duration: 1 week
- [x] **8.5.1** Refactor `Context`: Added `parent: Option<Box<Context>>` field
- [x] **8.5.2** Implement `Context::with_parent(parent)` constructor
- [x] **8.5.3** Implement `Context::get()` → Search in current scope first → then parent chain
- [x] **8.5.4** Implement `Context::set()` → Set in current scope only (shadowing)
- [x] **8.5.5** Implement `Context::get_all()` for debug — show full chain
- [x] **8.5.6** Test: variable shadowing, inheritance, multi-level scope
- [x] **8.5.7** Test: nested scopes do not affect parent variables (immutable parent)

## Phase 9: Lambda & Higher-Order Functions — DONE

Duration: 3 weeks
- [x] **9.1** Added `TokenKind::Arrow` (`=>`) in lexer
- [x] **9.2** Created `Expr::Lambda { params: Vec<String>, body: Box<SpannedExpr> }` in AST
- [x] **9.3** Parser: `parse_lambda()` — syntax: `(x, y) => x + y`
- [x] **9.4** Created `Value::Lambda { params, body, captured_scope }` or `Value::Closure { ... }`
- [x] **9.5** Evaluator: `eval_lambda()` — Create closure capturing scope
- [x] **9.6** Evaluator: `eval_call()` — Apply closure with arguments
- [x] **9.7** Implement builtin: `map(array, lambda)`, `filter(array, lambda)`, `reduce(array, lambda, initial)`
- [x] **9.8** Implement builtin: `sort(array)`, `sort(array, comparator_lambda)`
- [x] **9.9** Implement builtin: `unique(array)`, `group_by(array, lambda)`
- [x] **9.10** Test: closure captures variable, pipeline (map → filter → reduce)
- [x] **9.11** Test: nested lambdas, recursive lambdas (optional)

## Phase 9.5: BuiltinFunction Trait Refactor — DONE

Duration: 1 week
- [x] **9.5.1** Changed `BuiltinFunction::call` to `Function` trait
- [x] **9.5.2** `FunctionRegistry::register(name, Box<dyn Function>)` — Support stateful functions
- [x] **9.5.3** Refactored all builtins: `struct SumFunction`, `struct AvgFunction`, etc. implement `Function`
- [x] **9.5.4** Updated all tests to use new trait
- [x] **9.5.5** Test: custom function with captured state (counter, cache)

## Phase 10: User-Defined Functions — DONE

Duration: 1 week
- [x] **10.1** syntax: `def greet(name) = "Hello, " + name`
- [x] **10.2** Parser created `Expr::FunctionDef { name, params, body }`
- [x] **10.3** Evaluator: Register function in scope
- [x] **10.4** Support recursive user-defined functions
- [x] **10.5** Test: define → call, multiple definitions, shadowing builtins

## Phase 10.5: Missing Math + String Builtins — DONE

Duration: 2 weeks
- [x] **10.5.1** Math: `pi()`, `round(n, d)`, `ceil()`, `floor()`, `sqrt()`, `pow(b, e)`
- [x] **10.5.2** Trigonometry: `sin()`, `cos()`, `tan()`
- [x] **10.5.3** Random: `random()` → 0-1
- [x] **10.5.4** String: `trim()`, `trim_start()`, `trim_end()`
- [x] **10.5.5** String Manipulation: `split(s, sep)`, `replace(s, old, new)`, `substring(s, start, len)`

## Phase 11: Advanced Data Types (jiff) — PARTIAL

Duration: 2 weeks
- [x] **11.1** Added `Value::DateTime(jiff::Timestamp)` and `Value::Duration(jiff::SignedDuration)`
- [x] **11.2** Added `Value::Set(HashSet<Value>)` and `Value::Range { start, end, step }`
- [x] **11.5** Added `@` operator for DateTime literals: `@2024-01-01`
- [x] **11.6** Set operations: `set_union`, `set_intersection`, `set_difference`, `set_in`
- [x] **11.7** Range operations: `range(start, end)`, `range_to_array(range)`
- [x] **11.8** Full test suite for sets, ranges, and DateTime parsing/display

## Phase 12: Serialization & Caching — DONE

Duration: 1.5 weeks
- [x] **12.1** Implemented `serde` for `Value` and `Expr` (feature gated)
- [x] **12.2** Added `serialization` feature in `Cargo.toml`
- [x] **12.3** Created `FormulaCache` (LRU) for caching parsed ASTs
- [x] **12.4** Added `Context::to_json()` and `Context::from_json()`
- [x] **12.5** Round-trip serialization integration tests

## Phase 13: Plugin SDK Foundation — DONE

Duration: 1.5 weeks
- [x] **13.1** `pub trait Plugin: Send + Sync { fn register(&self, registry: &mut FunctionRegistry); }`
- [x] **13.2** `PluginManager` — load/unload plugins
- [x] **13.3** Plugin example: `MathPlugin`, `StringPlugin`, `GeoPlugin`
- [x] **13.4** Documentation: how to write custom plugin
- [x] **13.5** Test: plugin isolation, error propagation
