# แผนงานแบบเฟส — bl1z (Session 2 + 3)

สถานะ: **V2 เสร็จสมบูรณ์ (Phase 8–15 ✅)** → **Session 3: Phase 16 ✅ (v0.2.16)**

> ไฟล์เดียวสำหรับแผนทั้งหมด: เฟส, งาน, สถานะ, timeline, roadmap ต่อ
> (SPEC.md = สถาปัตยกรรมเท่านั้น — ไม่มีแผนซ้ำที่นี่)
> (TODO.md = บันทึกงานที่เสร็จแล้ว — รักษาไว้หลัง task แต่ละตัว)

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

**Timeline:** ~20 สัปดาห์ (Session 2 + 3)

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

## Phase 10.5: Missing Math + String Builtins ✅

**Priority:** 🟡 (ตาม SPEC.md)

**งาน:**
- [x] Math: `pi()`, `round(n)`, `ceil(n)`, `floor(n)`, `sqrt(n)`, `pow(base, exp)`, `abs(n)`, `sin/cos/tan`, `random()`
- [x] String: `trim(s)`, `trim_start(s)`, `trim_end(s)`, `split(s, delimiter)`, `replace(s, from, to)`, `substring(s, start, length)`
- [x] Tests: ครอบคลุมทุกฟังก์ชันใหม่

**Files:** `builtins/math.rs`, `builtins/string.rs`

---

## Phase 11: Advanced Data Types (jiff) ✅

**Priority:** 🟡 (จำเป็นสำหรับ date/time จริงจัง)

**งาน:**
- [x] 11.1 เพิ่ม `Value::DateTime(jiff::Timestamp)` และ `Value::Duration(jiff::Span)`
- [x] 11.2 เพิ่ม `Value::Set(BTreeSet<Value>)` และ `Value::Range { start, end, step }`
- [x] 11.3 Refactor date builtins: `now()`, `date()` → return `Value::DateTime`
- [x] 11.4 Refactor `date_add()`, `date_diff()` → operate บน native types
- [x] 11.5 เพิ่ม @ operator: `@2024-01-01` → DateTime literal
- [x] 11.6 Set operations: `set_union`, `set_intersection`, `set_difference`, `set_in`
- [x] 11.7 Range operations: `range(1, 10)`, `range_to_array()`
- [x] 11.8 Conversion: `to_datetime(str)`, `to_duration(str)`; Arithmetic: `DateTime + Duration`, `DateTime - DateTime`
- [x] 11.9 Test: type coercion rules, display formatting for advanced types

**Files:** `value.rs`, `parser.rs`, `eval.rs`, `builtins/date.rs`

---

## Phase 12: Serialization & Caching ✅

**Priority:** 🟢 (production ready)

**งาน:**
- [x] 12.1 Serde derive บน `Expr`, `Value`, `Context` (behind feature gate `serialization`)
- [x] 12.2 Feature gate: `serialization` ใน Cargo.toml
- [x] 12.3 `FormulaCache` – LRU cache สำหรับ parsed formulas
- [x] 12.4 `Context::to_json()` / `Context::from_json()` — serialize/restore variable store
- [x] 12.5 Test: roundtrip JSON, cache hit/miss

**Files:** `serialization.rs`, `cache.rs`, `lib.rs`

---

## Phase 13: Plugin SDK Foundation ✅

**Priority:** 🟢 (เปิด extensibility)

**งาน:**
- [x] `trait Plugin` และ `PluginManager` (ตาม SPEC)
- [x] `FunctionRegistry::import_plugin(&mut self, plugin: &dyn Plugin)` (ใช้ `merge_functions` บน `PluginManager`)
- [x] Plugin conflict resolution (name collision → error)
- [x] Tests: register plugin, call plugin function
- **ไม่อยู่ใน scope:** WASM, sandbox, dynamic loading

**Files:** `plugins.rs`, `functions.rs`, `lib.rs`

---

## Phase 14: Performance & Optimization ✅

**Priority:** 🟢 (หลัง feature ครบ)

**งาน:**
- [x] 14.1 Constant folding pass เมื่อ parse เสร็จ
- [x] 14.2 AST optimization: algebraic identities (x+0, x*1, x*0, --x)
- [x] 14.3 Criterion benchmarks: 11 benchmarks ครอบคลุมทุกหมวด
- [x] 14.4 Memoization/registry passing สำหรับ higher-order functions
- [x] 14.5 Benchmark ทุก builtin category
- [x] 14.6 Profile guided optimization docs

**Files:** `optimizer.rs`, `profiling.rs`, `benches/`

---

## Phase 15: Error Recovery + Security Limits ✅

**Priority:** 🟢 (production readiness)

**งาน:**
- [x] 15.1 `parse_with_recovery()` — collect all errors แทน fail-fast
- [x] 15.2 Error recovery strategies: skip ไป semicolon ถัดไป, parse ต่อ
- [x] 15.3 `EngineConfig { max_formula_length, max_depth, max_time_ms }`
- [x] 15.4 `Evaluator::with_config(config)` — enforce limits (E901)
- [x] 15.5 Test: formula ยาวเกิน, recursion ลึกเกิน, timeout

**Files:** `parser.rs`, `config.rs`, `eval.rs`, `error.rs`

---

## Phase 16: Plugin Ecosystem (CLI + JSON plugins) ✅

**Priority:** 🟢 (เปิด extensibility จริง — v0.2.16)

**งาน:**
- [x] 16.1 CLI binary: `bl1z eval|repl|functions|plugins`, exit codes 0/1/2 (cargo-style)
- [x] 16.2 JSON plugin manifest: `load_json_plugin` / `JsonPlugin`, engine version pinning (E804)
- [x] 16.3 Script runner: spawn `<runner> <script> <fn>`, args JSON stdin → result JSON stdout
- [x] 16.4 Plugin store: install/link/uninstall/list/enable/disable/reload/debug/fmt/fix, `state.json`, `BL1Z_PLUGINS_DIR`
- [x] 16.5 Auto-load ปลั๊กอินที่ enabled ใน eval/repl (`enabled_plugin_paths`)
- [x] 16.6 IDL: `proto/bl1z_plugin.proto` → `tools/gen_schema.py` → 3 schemas (manifest/store/protocol)
- [x] 16.7 ตัวอย่างปลั๊กอิน: math_extra, string_utils, obsidian_like (Python scripts)
- [x] 16.8 ลบโค้ดซ้ำ: value_main.rs, value_pr26.rs, higher_order_original.rs, acp.yaml

**Files:** `main.rs`, `plugins.rs`, `plugins_cmd.rs`, `proto/`, `tools/gen_schema.py`

---

## Timeline (ประมาณการ)

| Phase | หัวข้อ | ระยะเวลา |
|-------|--------|----------|
| 8 | Access Chaining | 2 สัปดาห์ |
| 9 | Lambda & Higher-Order | 3 สัปดาห์ |
| 10 | User-Defined Functions | 2 สัปดาห์ |
| 10.5 | Math + String Builtins | 2 สัปดาห์ |
| 11 | Advanced Data Types | 2 สัปดาห์ |
| 12 | Serialization & Caching | 1.5 สัปดาห์ |
| 13 | Plugin SDK | 1.5 สัปดาห์ |
| 14 | Performance & Optimization | 2 สัปดาห์ |
| 15 | Error Recovery + Security Limits | 1 สัปดาห์ |
| 16 | Plugin Ecosystem (CLI + JSON) | 3 สัปดาห์ |
| **รวม** | | ~20 สัปดาห์ |

---

## Success Criteria (Session 2)

- ✅ ผู้ใช้สามารถเขียน `user.name`, `arr[0]`, `items[0].price` ได้
- ✅ Lambda `(x) => x * 2` ทำงานร่วมกับ `map`, `filter`, `reduce`
- ✅ `fn factorial(n) = ...` ใช้งานได้จริง (recursion, มี limit)
- ✅ DateTime/Duration ทำงาน native ผ่าน `jiff` โดยไม่มี C dependency
- ✅ Formula สามารถ cache แล้ว eval ซ้ำได้เร็วขึ้น
- ✅ Plugin SDK มี trait และ manager ให้ third-party เขียนส่วนขยาย
- ✅ CI/CD ยังเขียว, `cargo test`, `fmt`, `clippy` ผ่าน

## Success Criteria (Session 3 – Phase 16)

- ✅ `bl1z` CLI binary ใช้งานได้ (eval/repl/functions/plugins, exit codes 0/1/2)
- ✅ ปลั๊กอิน JSON ลงทะเบียนฟังก์ชันได้จริงผ่าน script runner (python3/node/shell)
- ✅ Plugin store จัดการ install/enable/disable พร้อม auto-load ใน eval/repl
- ✅ schema.json ทั้ง 3 (manifest/store/protocol) gen จาก proto — ไม่มีมือแก้

---

## Roadmap ต่อ (Session 3+) — ยังไม่จัดตาราง

> ตาม `.bump-version.json`: `"Phase 16-20": "0.2.x to 0.2.20"` — หมายเลขเฟส/
> เวอร์ชันของรายการด้านล่างยังไม่ยืนยันจนกว่าจะถูกจัดตารางจริง (เดิม Phase 16
> เคยเป็น JIT แต่ถูกแทนที่ด้วย plugin ecosystem — v0.2.16)

- **JIT/Cranelift compilation** — Lowered IR → Cranelift IR → regalloc2 → machine code (x86-64/ARM64/WASM)
- **WebAssembly plugin sandbox** — Wasmtime fuel consumption
- **Language Server Protocol (LSP)** — tower-lsp: completion, hover, publishDiagnostics, semanticTokens, definition, signatureHelp
- **User-defined types** — `type Person { name: string, age: number }`
- **Pattern matching** — `match x { n if n > 100 => "big", ... }`
