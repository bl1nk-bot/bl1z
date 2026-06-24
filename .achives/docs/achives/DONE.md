# ✅ DONE — Completed Work Log

## Phase 8: Access Chaining & Indexing
**Completed:** 2026-05-18

### What was done
- Added `Expr::PropertyAccess { object, field }` to AST
- Added `Expr::IndexAccess { object, index }` to AST
- Replaced string-concatenation dot hack in parser with `parse_postfix()` method
- Replaced `name.split('.')` hack in evaluator with proper `PropertyAccess`/`IndexAccess` handlers
- Error codes: `E307` (PropertyNotFound), `E308` (IndexOutOfBounds)
- 16 new integration tests covering property access, index access, chained access, error cases

### New syntax
- `{name: "Alice"}.name` → `"Alice"`
- `[10, 20, 30][1]` → `20`
- `{users: [{name: "A"}]}.users[0].name` → `"A"`
- `{a: {b: {c: 42}}}.a.b.c` → `42`
- `[10, 20, 30][1 + 1]` → `30`

### Files changed
- `src/ast.rs` — added `PropertyAccess`, `IndexAccess` variants
- `src/parser.rs` — added `parse_postfix()`, removed string-concat dot handling
- `src/eval.rs` — added `PropertyAccess`/`IndexAccess` evaluation, removed `split('.')` logic
- `src/error.rs` — added `#[derive(PartialEq)]` for test assertions
- `src/lib.rs` — extracted tests to `lib_tests.rs`
- `src/lib_tests.rs` — new file with 80+ renamed integration tests

---

## Phase 8.5: Context Scoping
**Completed:** 2026-05-18

### What was done
- Refactored `Context` to support parent chain via `Option<Rc<Context>>`
- Added `Context::with_parent()` constructor for child scopes
- `get()` walks up parent chain for variable resolution
- `set()` writes to current scope only (shadowing)
- Added `get_all()` for debugging — returns all visible variables
- Added `depth()` for introspection
- Changed internal storage from `HashMap` to `BTreeMap` for deterministic iteration
- 12 new unit tests covering shadowing, inheritance, multi-level scopes, clone isolation

### Scoping model
- Child inherits parent variables via `get()`
- Child can shadow parent via `set()` without mutating parent
- Clone copies current scope + parent reference (shared via `Rc`)
- Ready for Phase 9 lambda closures

### Files changed
- `src/context.rs` — full rewrite with parent chain scoping
- `Cargo.toml` — added `pretty_assertions` dev-dependency

---

## Test Infrastructure Improvements
**Completed:** 2026-05-18

### What was done
- Extracted 169 tests from inline `mod integration_tests` → `src/lib_tests.rs`
- Renamed all tests to `{subject}_{scenario}_{expected_outcome}` convention
- Added `eval_formula()` / `eval_with_ctx()` helper functions
- Added `#[derive(PartialEq)]` to `FormulaError` for whole-object comparison
- Added missing edge case tests (empty strings, null property access, map indexing, etc.)
- Fixed 4 pre-existing test failures (`E401` → `E501` error code mismatch)
- Fixed 1 snapshot test (`array_function_wrong_type`)

### Test count
- **Before:** 132 unit + 11 snapshot + 16 doc = 159 total
- **After:** 181 unit + 11 snapshot + 19 doc = 211 total
