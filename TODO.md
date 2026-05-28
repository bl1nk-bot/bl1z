# 📋 Active TODO Checklist — bl1z V2

## 🟢 Phase 10.5: Missing Math + String Builtins 🆕 (2 สัปดาห์) 🔄 IN PROGRESS

> ทำให้ฟีเจอร์ครบตาม SPEC.md

**Math:**
- [x] `pi()` → 3.14159...
- [x] `round(n)`, `ceil(n)`, `floor(n)`
- [x] `sqrt(n)`, `pow(base, exp)`, `abs(n)` ✅ (มีแล้ว)
- [x] `sin(n)`, `cos(n)`, `tan(n)` (ใช้ `libm` หรือ pure Rust implementation)
- [x] `random()` → random float 0-1

**String:**
- [x] `trim(s)`, `trim_start(s)`, `trim_end(s)`
- [x] `split(s, delimiter)` → Array of Strings
- [x] `replace(s, from, to)`
- [x] `substring(s, start, length)`

## 🟢 Phase 11: Advanced Data Types (2 สัปดาห์) ✅ DONE

> ⚠️ Refactor date builtins จาก string wrapping → native DateTime/Duration

- [x] **11.1** เพิ่ม `Value::DateTime(jiff::Timestamp)` และ `Value::Duration(jiff::SignedDuration)`
- [x] **11.2** เพิ่ม `Value::Set(HashSet<Value>)` และ `Value::Range { start, end, step }`
- [ ] **11.3** Refactor date builtins: `now()` → return `Value::DateTime`, `date()` → parse → `Value::DateTime`
- [ ] **11.4** Refactor `date_add()`, `date_diff()` → operate บน native types
- [x] **11.5** เพิ่ม @ operator: `@2024-01-01` → DateTime literal
- [ ] **11.6** Set operations: `union`, `intersection`, `difference`, `in`
- [ ] **11.7** Range operations: `1..10`, iteration, `contains`
- [ ] **11.8** Test: type coercion rules, display formatting

## 🟢 Phase 12: Serialization & Caching (1.5 สัปดาห์)

- [ ] **12.1** `#[derive(Serialize, Deserialize)]` on `Value`, `Expr` (behind `serde` feature gate)
- [ ] **12.2** Feature gate: `serialization` ใน Cargo.toml
- [ ] **12.3** `Evaluator::evaluate_cached()` — LRU cache สำหรับ expressions ที่ใช้ซ้ำ
- [ ] **12.4** `Context::to_json()` / `Context::from_json()` — serialize/deserialize variable store
- [ ] **12.5** Test: round-trip serialization, cache hit/miss

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

---
**Note:** For the history of completed tasks, see [docs/DONE_DETAILED.md](./docs/DONE_DETAILED.md).
