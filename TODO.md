# 📋 TODO Checklist — V2 (Session 2) Full Plan

## 🔴 Phase 8: Access Chaining & Indexing (2 สัปดาห์)

> ⚠️ **ต้องทำก่อน**: นี่คือ enabler ของทุก Phase หลังจากนี้

- [ ] **8.1** สร้าง `Expr::PropertyAccess { object: Box<SpannedExpr>, field: String }` ใน AST
- [ ] **8.2** สร้าง `Expr::IndexAccess { object: Box<SpannedExpr>, index: Box<SpannedExpr> }` ใน AST
- [ ] **8.3** Refactor parser: แทนที่ string concatenation (`name = format!("{}.{}", ...)`) ด้วย recursive `parse_property_access()` / `parse_index_access()`
- [ ] **8.4** Refactor evaluator: แทนที่ `name.split('.')` ด้วย recursive `eval_property_access()` / `eval_index_access()`
- [ ] **8.5** เพิ่ม error: `PropertyNotFound` (E207), `IndexOutOfBounds` (E208), `NotIndexable` (E401)
- [ ] **8.6** รองรับ nested: `a.b[0].c.d`, `a["key"].nested`
- [ ] **8.7** Unit tests: property access on Map/Context, index access on Array/String, chained access, error cases
- [ ] **8.8** Integration tests: `user.profile.scores[0]`, `config["db"].host`

## 🔴 Phase 8.5: Context Scoping 🆕 (1 สัปดาห์)

> ⚠️ **ต้องทำก่อน Phase 9** — Lambda ต้องการ closure environment

- [ ] **8.5.1** Refactor `Context`: เพิ่ม `parent: Option<Box<Context>>` field
- [ ] **8.5.2** Implement `Context::with_parent(parent)` constructor
- [ ] **8.5.3** Implement `Context::get()` → ค้นหาใน current scope ก่อน → ไล่ขึ้น parent chain
- [ ] **8.5.4** Implement `Context::set()` → set ใน current scope เท่านั้น (shadowing)
- [ ] **8.5.5** Implement `Context::get_all()` สำหรับ debug — แสดงทั้ง chain
- [ ] **8.5.6** Test: variable shadowing, inheritance, multi-level scope
- [ ] **8.5.7** Test: nested scopes ไม่กระทบ variable ของ parent (immutable parent)

## 🟡 Phase 9: Lambda & Higher-Order Functions (3 สัปดาห์)

> ⚠️ **Depends on**: Phase 8 (PropertyAccess), Phase 8.5 (Context Scoping), BuiltinFunction refactor

- [ ] **9.1** เพิ่ม `TokenKind::Arrow` (`=>`) ใน lexer
- [ ] **9.2** สร้าง `Expr::Lambda { params: Vec<String>, body: Box<SpannedExpr> }` ใน AST
- [ ] **9.3** Parser: `parse_lambda()` — syntax: `(x, y) => x + y`
- [ ] **9.4** สร้าง `Value::Lambda { params, body, captured_scope }` หรือ `Value::Closure { ... }`
- [ ] **9.5** Evaluator: `eval_lambda()` — สร้าง closure จับ scope
- [ ] **9.6** Evaluator: `eval_call()` — apply closure ด้วย arguments
- [ ] **9.7** Implement builtin: `map(array, lambda)`, `filter(array, lambda)`, `reduce(array, lambda, initial)`
- [ ] **9.8** Implement builtin: `sort(array)`, `sort(array, comparator_lambda)`
- [ ] **9.9** Implement builtin: `unique(array)`, `group_by(array, lambda)`
- [ ] **9.10** Test: closure captures variable, pipeline (map → filter → reduce)
- [ ] **9.11** Test: nested lambdas, recursive lambdas (optional)

## 🟡 Phase 9.5: BuiltinFunction Trait Refactor 🆕 (1 สัปดาห์)

> ⚠️ **ต้องทำก่อน หรือทำคู่กับ Phase 9** — `fn` pointer จับ state ไม่ได้

- [ ] **9.5.1** เปลี่ยน `BuiltinFunction::call` จาก `fn(&[Value]) -> Result<Value, FormulaError>` เป็น trait:
```rust
pub trait Function: Send + Sync {
    fn call(&self, args: &[Value]) -> Result<Value, FormulaError>;
    fn name(&self) -> &str;
}
```
- [ ] **9.5.2** `FunctionRegistry::register(name, Box<dyn Function>)` — รองรับ stateful functions
- [ ] **9.5.3** Refactor builtins ทั้งหมด: `struct SumFunction`, `struct AvgFunction`, etc. implements `Function`
- [ ] **9.5.4** อัพเดท tests ทั้งหมดให้ใช้ trait ใหม่
- [ ] **9.5.5** Test: custom function with captured state (counter, cache)

## 🟡 Phase 10: User-Defined Functions (1 สัปดาห์)

> Depends on: Phase 9 (Lambda closure), Phase 9.5 (Function trait)

- [ ] **10.1** syntax: `def greet(name) = "Hello, " + name`
- [ ] **10.2** Parser สร้าง `Expr::FunctionDef { name, params, body }`
- [ ] **10.3** Evaluator: register function ใน scope
- [ ] **10.4** รองรับ recursive user-defined functions
- [ ] **10.5** Test: define → call, multiple definitions, shadowing builtins

## 🟢 Phase 10.5: Missing Math + String Builtins 🆕 (2 สัปดาห์)

> ทำให้ฟีเจอร์ครบตาม SPEC.md

**Math:**
- [ ] `pi()` → 3.14159...
- [ ] `round(n)`, `ceil(n)`, `floor(n)`
- [ ] `sqrt(n)`, `pow(base, exp)`, `abs(n)` ✅ (มีแล้ว)
- [ ] `sin(n)`, `cos(n)`, `tan(n)` (ใช้ `libm` หรือ pure Rust implementation)
- [ ] `random()` → random float 0-1

**String:**
- [ ] `trim(s)`, `trim_start(s)`, `trim_end(s)`
- [ ] `split(s, delimiter)` → Array of Strings
- [ ] `replace(s, from, to)`
- [ ] `substring(s, start, length)`

## 🟢 Phase 11: Advanced Data Types (2 สัปดาห์)

> ⚠️ Refactor date builtins จาก string wrapping → native DateTime/Duration

- [ ] **11.1** เพิ่ม `Value::DateTime(jiff::Timestamp)` และ `Value::Duration(jiff::SignedDuration)`
- [ ] **11.2** เพิ่ม `Value::Set(HashSet<Value>)` และ `Value::Range { start, end, step }`
- [ ] **11.3** Refactor date builtins: `now()` → return `Value::DateTime`, `date()` → parse → `Value::DateTime`
- [ ] **11.4** Refactor `date_add()`, `date_diff()` → operate บน native types
- [ ] **11.5** เพิ่ม `@` operator: `@2024-01-01` → DateTime literal
- [ ] **11.6** Set operations: `union`, `intersection`, `difference`, `in`
- [ ] **11.7** Range operations: `1..10`, iteration, `contains`
- [ ] **11.8** Test: type coercion rules, display formatting

## 🟢 Phase 12: Serialization & Caching (1.5 สัปดาห์)

- [ ] **12.1** `#[derive(Serialize, Deserialize)]` on `Value`, `Expr` (behind `serde` feature gate)
- [ ] **12.2** Feature gate: `serialization` ใน Cargo.toml
- [ ] **12.3** `Evaluator::evaluate_cached()` — LRU cache สำหรับ expressions ที่ใช้ซ้ำ
- [ ] **12.4** `Context::to_json()` / `Context::from_json()` — serialize/deserialize variable store
- [ ] **12.5** Test: round-trip serialization, cache hit/miss

## 🔵 Phase 13: Plugin SDK Foundation (1.5 สัปดาห์)

- [ ] **13.1** `pub trait Plugin: Send + Sync { fn register(&self, registry: &mut FunctionRegistry); }`
- [ ] **13.2** `PluginManager` — load/unload plugins
- [ ] **13.3** Plugin example: `MathPlugin`, `StringPlugin`, `GeoPlugin`
- [ ] **13.4** Documentation: how to write custom plugin
- [ ] **13.5** Test: plugin isolation, error propagation

## 🔵 Phase 14: Performance & Optimization (2 สัปดาห์)

- [ ] **14.1** Constant folding optimization pass: `1 + 2` → `3` at parse/compile time
- [ ] **14.2** AST optimization: `if(true, X, Y)` → `X`, `if(false, X, Y)` → `Y`
- [ ] **14.3** Add criterion benchmarks: comparison with V1 baseline
- [ ] **14.4** Memoization for pure functions (no side effects)
- [ ] **14.5** `#[bench]` สำหรับทุก builtin function
- [ ] **14.6** Profile-guided optimization documentation

## 🟣 Phase 15: Error Recovery + Security Limits 🆕 (1 สัปดาห์)

- [ ] **15.1** `parse_with_recovery()` — collect all errors instead of fail-fast
- [ ] **15.2** Error recovery strategies: skip to next delimiter, insert missing token
- [ ] **15.3** `EngineConfig { max_formula_length, max_depth, max_time }`
- [ ] **15.4** `Evaluator::with_config(config)` — enforce limits
- [ ] **15.5** Test: formula too long, recursion depth exceeded, timeout
