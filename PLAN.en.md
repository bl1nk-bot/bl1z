# Phased Roadmap – Session 2 (Advanced Features)

Status: **V1 Complete** → Session 2 (Phase 8, 9, 10, 11, 12, 13 ✅ | Phase 14 🚧)

---

## Session 2 Overview

**Main Goal:** Expand the engine into a powerful platform for formula computation.
- Access chaining (`obj.prop`, `arr[0]`)
- Lambda & higher-order functions (`map`, `filter`, `reduce`)
- User-defined functions (`fn`)
- Native DateTime/Duration (via `jiff`)
- Plugin SDK foundation
- Serialization & caching
- Performance optimizations

**Timeline:** ~14 weeks (7 phases)

---

## Phase 8: Access Chaining & Indexing ✅

**Priority:** 🔴 Highest (Required for every use case)

**Tasks:**
- [x] Add `PropertyAccess` and `IndexAccess` to AST
- [x] Lexer: Add `Dot` token for `.`
- [x] Parser: Create `parse_postfix` method to support chains `expr '.' IDENT` and `expr '[' expr ']'` (left-associative)
- [x] Evaluator:
  - [x] `PropertyAccess`: Evaluate object; if it's a `Map`, lookup property; if not found, report `PropertyNotFound`
  - [x] `IndexAccess`: Evaluate object and index; if object is `Array` and index is `Number`, access element; check bounds
- [x] Error: `PropertyNotFound`, `IndexOutOfBounds`
- [x] Tests: nested objects, mixed chain, error cases

**Files:** `ast.rs`, `lexer.rs`, `parser.rs`, `eval.rs`, `error.rs`

---

## Phase 9: Lambda & Higher-Order Functions ✅

**Priority:** 🔴 Highest (Functional core)

**Tasks:**
- [x] `LambdaExpr` in AST: `params: Vec<String>`, `body: Box<SpannedExpr>`
- [x] Lexer: `Arrow` token (`=>`)
- [x] Parser: `'(' params ')' '=>' expression` (lambda as an expression)
- [x] Evaluation:
  - [x] Create closure struct `Lambda` storing params, body, and environment (copy of current context)
  - [x] When called via `map`/`filter`/`reduce`, bind arguments to params and evaluate body
- [x] Built-in functions: `map`, `filter`, `reduce`, `sort`, `group_by`, `unique` (accept lambda as argument)
- [x] Tests: lambda of all arities, nested lambda, higher-order with empty array, variable capturing closure

**Files:** `ast.rs`, `lexer.rs`, `parser.rs`, `eval.rs`, `builtins/functional.rs`

---

## Phase 10: User-Defined Functions ✅

**Priority:** 🟡 Secondary to Lambda

**Tasks:**
- [x] Syntax: `fn name(params) = expression`
- [x] Parser: `FunctionDef` in AST
- [x] Context: Store `HashMap<String, UserFunction>`
- [x] Evaluation: When encountering `FunctionCall` matching a UDF, bind arguments to params and evaluate body
- [x] Recursion limit (configurable) to prevent stack overflow
- [x] Tests: factorial, mutual recursion, edge cases (recursion limit)

**Files:** `functions.rs`, `context.rs`, `parser.rs`, `eval.rs`

---

## Phase 11: Advanced Data Types (jiff) ✅

**Priority:** 🟡 (Essential for serious date/time support)

**Tasks:**
- Add `Value::DateTime(Timestamp)`, `Value::Duration(Span)`, `Set(BTreeSet<Value>)`, `Range { start, end }`
- Parser literals:
  - `@2023-01-01` → `DateTime`
  - `1h30m` → `Duration`
  - `1..10` → `Range`
  - `{1,2,3}` → `Set`
- Refactor Date functions to work directly on native `DateTime` (internally string version remains for backward compatibility)
- Conversion functions: `to_datetime(str)`, `to_duration(str)`
- Arithmetic: `DateTime + Duration`, `DateTime - DateTime` (result is Duration)
- Tests: duration arithmetic, datetime comparison, set operations

**Files:** `value.rs`, `parser.rs`, `eval.rs`, `builtins/date.rs`

---

## Phase 12: Serialization & Caching ✅

**Priority:** 🟢 (Production ready)

**Tasks:**
- Serde derive on `Expr`, `Value`, `Context` (behind `serialization` feature gate)
- `FormulaCache` – key-value store (`String` → `SpannedExpr`) for parsed formulas
- `eval_cached(formula, ctx, registry)` – parse once, eval many
- Context snapshot & restore
- Tests: roundtrip JSON, cache hit/miss

**Files:** `serialization.rs`, `cache.rs`, `lib.rs`

---

## Phase 13: Plugin SDK Foundation ✅

**Priority:** 🟢 (Enables extensibility)

**Tasks:**
- [x] `trait Plugin` and `PluginManager` (as per SPEC)
- [x] `FunctionRegistry::import_plugin(&mut self, plugin: &dyn Plugin)` (Note: use `merge_functions` on `PluginManager` to pull functions into registry)
- [x] Plugin conflict resolution (name collision → error)
- [x] Tests: register plugin, call plugin function
- **Out of scope:** WASM, sandbox, dynamic loading

**Files:** `plugins.rs`, `functions.rs`, `lib.rs`

---

## Phase 14: Performance & Optimization 🚧

**Priority:** 🟢 (Post-feature completion)

**Tasks:**
- Constant folding pass after parsing (evaluate constant sub-expressions)
- Vectorized `map`/`filter` for large arrays (using rayon or iterator)
- Benchmark suite with `criterion`
- Profile guided optimization points
- Docs: performance best practices

**Files:** `optimizer.rs`, `profiling.rs`, `benches/`

---

## Timeline (Estimated)

| Phase | Topic | Duration |
|-------|--------|----------|
| 8 | Access Chaining | 2 weeks |
| 9 | Lambda & Higher-Order | 3 weeks |
| 10 | User-Defined Functions | 2 weeks |
| 11 | Advanced Data Types | 2 weeks |
| 12 | Serialization & Caching | 1.5 weeks |
| 13 | Plugin SDK | 1.5 weeks |
| 14 | Performance & Optimization | 2 weeks |
| **Total** | | ~14 weeks |

---

## Success Criteria (Session 2)

- ✅ Users can write `user.name`, `arr[0]`, `items[0].price`
- ✅ Lambda `(x) => x * 2` works with `map`, `filter`, `reduce`
- ✅ `fn factorial(n) = ...` works (recursive, with limit)
- ✅ DateTime/Duration works natively via `jiff` without C dependencies
- ✅ Formulas can be cached and re-evaluated faster
- ✅ Plugin SDK provides trait and manager for third-party extensions
- ✅ CI/CD green, `cargo test`, `fmt`, `clippy` passed
