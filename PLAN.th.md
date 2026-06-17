# แผนงานแบบเฟส – Session 2 (Advanced Features)

สถานะ: **V1 เสร็จสมบูรณ์** → Session 2 (Phase 8, 9, 10, 11, 12, 13 ✅ | Phase 14 🚧)

---

## Session 2 Overview

**เป้าหมายหลัก:** ขยาย engine ให้เป็น platform สำหรับ formula computation ที่ทรงพลัง
- Access chaining (`obj.prop`, `arr[0]`)
- Lambda & higher-order functions (`map`, `filter`, `reduce`)
- User-defined functions (`fn`)
- Native DateTime/Duration (ผ่าน `jiff`)
- Plugin SDK foundation
- Serialization & caching
- Performance optimizations

**Timeline:** ~14 สัปดาห์ (7 phases)

---

## Phase 8: Access Chaining & Indexing ✅

**Priority:** 🔴 สูงสุด (ทุก use case ต้องการ)

**งาน:**
- [x] เพิ่ม `PropertyAccess` และ `IndexAccess` ใน AST
- [x] Lexer: เพิ่ม token `Dot` สำหรับ `.`
- [x] Parser: สร้าง method `parse_postfix` เพื่อรองรับ chain `expr '.' IDENT` และ `expr '[' expr ']'` (left-associative)
- [x] Evaluator:
  - [x] `PropertyAccess`: evaluate object, ถ้าเป็น `Map` ให้ lookup property, ถ้าไม่พบแจ้ง `PropertyNotFound`
  - [x] `IndexAccess`: evaluate object และ index, ถ้า object เป็น `Array` และ index เป็น `Number` ให้เข้าถึง element, ตรวจสอบ bounds
- [x] Error: `PropertyNotFound`, `IndexOutOfBounds`
- [x] Tests: nested objects, mixed chain, error cases

**Files:** `ast.rs`, `lexer.rs`, `parser.rs`, `eval.rs`, `error.rs`

---

## Phase 9: Lambda & Higher-Order Functions ✅

**Priority:** 🔴 สูงสุด (หัวใจ functional)

**งาน:**
- [x] `LambdaExpr` ใน AST: `params: Vec<String>`, `body: Box<SpannedExpr>`
- [x] Lexer: token `Arrow` (`=>`)
- [x] Parser: `'(' params ')' '=>' expression` (lambda เป็น expression)
- [x] Evaluation:
  - [x] สร้าง closure struct `Lambda` ที่เก็บ params, body, และ environment (copy ของ context ปัจจุบัน)
  - [x] เมื่อถูกเรียกผ่าน `map`/`filter`/`reduce` ให้ bind arguments เข้ากับ params แล้ว evaluate body
- [x] Built-in functions: `map`, `filter`, `reduce`, `sort`, `group_by`, `unique` (รับ lambda เป็น argument)
- [x] Tests: lambda ทุก arity, nested lambda, higher-order กับ array เปล่า, closure จับตัวแปร

**Files:** `ast.rs`, `lexer.rs`, `parser.rs`, `eval.rs`, `builtins/functional.rs`

---

## Phase 10: User-Defined Functions ✅

**Priority:** 🟡 รองจาก Lambda

**งาน:**
- [x] Syntax: `fn name(params) = expression`
- [x] Parser: `FunctionDef` ใน AST
- [x] Context: เก็บ `HashMap<String, UserFunction>`
- [x] Evaluation: เมื่อเจอ `FunctionCall` ที่ชื่อตรงกับ UDF ให้ bind arguments เข้ากับ params แล้ว evaluate body
- [x] Recursion limit (configurable) เพื่อป้องกัน stack overflow
- [x] Tests: factorial, mutual recursion, edge cases (recursion limit)

**Files:** `functions.rs`, `context.rs`, `parser.rs`, `eval.rs`

---

## Phase 11: Advanced Data Types (jiff) ✅

**Priority:** 🟡 (จำเป็นสำหรับ date/time จริงจัง)

**งาน:**
- เพิ่ม `Value::DateTime(Timestamp)`, `Value::Duration(Span)`, `Set(BTreeSet<Value>)`, `Range { start, end }`
- Parser literals:
  - `@2023-01-01` → `DateTime`
  - `1h30m` → `Duration`
  - `1..10` → `Range`
  - `{1,2,3}` → `Set`
- ปรับ Date functions ให้ทำงานบน native `DateTime` โดยตรง (ภายในยังมีเวอร์ชันที่รับ string เพื่อ backward compatibility)
- Conversion functions: `to_datetime(str)`, `to_duration(str)`
- Arithmetic: `DateTime + Duration`, `DateTime - DateTime` (ได้ Duration)
- Tests: arithmetic กับ duration, comparison กับ datetime, set operations

**Files:** `value.rs`, `parser.rs`, `eval.rs`, `builtins/date.rs`

---

## Phase 12: Serialization & Caching ✅

**Priority:** 🟢 (production ready)

**งาน:**
- Serde derive บน `Expr`, `Value`, `Context` (behind feature gate `serialization`)
- `FormulaCache` – key-value store (`String` → `SpannedExpr`) สำหรับ parsed formulas
- `eval_cached(formula, ctx, registry)` – parse once, eval many
- Context snapshot & restore
- Tests: roundtrip JSON, cache hit/miss

**Files:** `serialization.rs`, `cache.rs`, `lib.rs`

---

## Phase 13: Plugin SDK Foundation ✅

**Priority:** 🟢 (เปิด extensibility)

**งาน:**
- [x] `trait Plugin` และ `PluginManager` (ตาม SPEC)
- [x] `FunctionRegistry::import_plugin(&mut self, plugin: &dyn Plugin)` (หมายเหตุ: ใช้ `merge_functions` บน `PluginManager` เพื่อดึงฟังก์ชันเข้าสู่ registry)
- [x] Plugin conflict resolution (name collision → error)
- [x] Tests: register plugin, call plugin function
- **ไม่อยู่ใน scope:** WASM, sandbox, dynamic loading

**Files:** `plugins.rs`, `functions.rs`, `lib.rs`

---

## Phase 14: Performance & Optimization 🚧

**Priority:** 🟢 (หลัง feature ครบ)

**งาน:**
- Constant folding pass เมื่อ parse เสร็จ (evaluate constant sub-expressions)
- Vectorized `map`/`filter` สำหรับ array ขนาดใหญ่ (ใช้ rayon หรือ iterator)
- Benchmark suite ด้วย `criterion`
- Profile guided optimization points
- Docs: performance best practices

**Files:** `optimizer.rs`, `profiling.rs`, `benches/`

---

## Timeline (ประมาณการ)

| Phase | หัวข้อ | ระยะเวลา |
|-------|--------|----------|
| 8 | Access Chaining | 2 สัปดาห์ |
| 9 | Lambda & Higher-Order | 3 สัปดาห์ |
| 10 | User-Defined Functions | 2 สัปดาห์ |
| 11 | Advanced Data Types | 2 สัปดาห์ |
| 12 | Serialization & Caching | 1.5 สัปดาห์ |
| 13 | Plugin SDK | 1.5 สัปดาห์ |
| 14 | Performance & Optimization | 2 สัปดาห์ |
| **รวม** | | ~14 สัปดาห์ |

---

## Success Criteria (Session 2)

- ✅ ผู้ใช้สามารถเขียน `user.name`, `arr[0]`, `items[0].price` ได้
- ✅ Lambda `(x) => x * 2` ทำงานร่วมกับ `map`, `filter`, `reduce`
- ✅ `fn factorial(n) = ...` ใช้งานได้จริง (recursion, มี limit)
- ✅ DateTime/Duration ทำงาน native ผ่าน `jiff` โดยไม่มี C dependency
- ✅ Formula สามารถ cache แล้ว eval ซ้ำได้เร็วขึ้น
- ✅ Plugin SDK มี trait และ manager ให้ third-party เขียนส่วนขยาย
- ✅ CI/CD ยังเขียว, `cargo test`, `fmt`, `clippy` ผ่าน
