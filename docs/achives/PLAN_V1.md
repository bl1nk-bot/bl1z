# แผนงานแบบเฟส (อัปเดตจากสถานะจริง ณ commit 6d2529a)

สถานะโดยรวม: **V1 + Phase 6 เสร็จสมบูรณ์** (arrays, maps, date functions + collection functions) พร้อม test และ doc-tests เฟส 7 (Quality & Tooling) มีแล้วบางส่วน

---

## Phase 0: Design & Scope Lock ✅ เสร็จสมบูรณ์
**เป้าหมาย:** กำหนดสิ่งที่จะสร้างให้ชัดก่อนเริ่มเขียนจริง

**งานที่ทำแล้ว:**
- นิยาม use case หลัก: bl1z สำหรับ POE SDK
- นิยาม syntax และ grammar (ดู PRD ภาษาไทย)
- กำหนด value types V1: Number, String, Bool, Null
- กำหนด operator set: + - * / ! - (unary) && || == != < > <= >=
- กำหนด built-in functions ขั้นต่ำ: if, len, upper, lower
- กำหนด error format: code, message, span, category (LexError, ParseError, EvalError, TypeError, FunctionError, ContextError)
- ได้เอกสาร spec v1

**Milestone:** ✅ เอกสาร spec v1, grammar, รายการฟังก์ชัน, type model

---

## Phase 1: Lexer + AST + Parser ✅ เสร็จสมบูรณ์
**เป้าหมาย:** แปลงสูตร string เป็น AST

**งานที่ทำแล้ว:**
- implement tokenization (`lexer.rs`) ใช้ `Chars` iterator (zero-cost)
  - รองรับ Identifier, Number, String, วงเล็บ, comma, operators, true/false, null
  - สร้าง Span (บรรทัด/คอลัมน์) สำหรับทุก token
  - จัดการ escape ใน string (`\"`, `\\`, `\n`, `\t`)
  - error เมื่อพบอักขระไม่รู้จัก
- implement AST enums (`ast.rs`)
  - `Expr` แยก Literal, Variable, UnaryExpr, BinaryExpr, FunctionCall, Grouping
  - มี `SpannedExpr` ห่อหุ้ม `Expr` พร้อม `Span`
  - `BinaryOp` และ `UnaryOp` แยกชัดเจน
- implement parser (`parser.rs`) ด้วย recursive descent
  - ใช้ generic function `parse_left_associative_binary` ลดโค้ด duplication
  - จัดการ precedence ถูกต้อง: unary → factor → term → comparison → equality → AND → OR
  - รองรับ function call (nested ได้), variable reference, parentheses
  - พื้นที่ error พร้อม span

**ไฟล์ที่เกี่ยวข้อง:** `src/lexer.rs`, `src/ast.rs`, `src/parser.rs`, `src/span.rs`  
**Milestone ที่ผ่านแล้ว:**
- parse `1 + 2 * 3` → ได้ AST ที่มีการคูณก่อนบวก
- parse `(1 + 2) * 3`
- parse `"hello"`
- parse `foo`
- parse `sum(1, 2)`

---

## Phase 2: Basic Evaluator ✅ เสร็จสมบูรณ์
**เป้าหมาย:** รัน AST แล้วได้ผลลัพธ์

**งานที่ทำแล้ว:**
- implement `Value` enum (`value.rs`) พร้อม Number, String, Bool, Null
- implement evaluator (`eval.rs`)
  - recursive evaluation รับ `SpannedExpr`, `Context`, `FunctionRegistry`
  - รองรับ arithmetic: +, -, *, / (ตรวจสอบ division by zero)
  - รองรับ string concatenation
  - รองรับ comparison: <, >, <=, >= (เฉพาะ Number)
  - รองรับ equality: ==, != (ทุก type)
  - รองรับ boolean logic: &&, || (เฉพาะ Bool)
  - รองรับ unary ops: - (เฉพาะ Number), ! (เฉพาะ Bool)
  - type validation ทุก operation -> `TypeError` พร้อม span
  - null handling: Null compare ได้, operation กับ Null ให้ error ชัดเจน

**ไฟล์ที่เกี่ยวข้อง:** `src/value.rs`, `src/eval.rs`  
**Milestone ที่ผ่านแล้ว:**
- `1 + 2 * 3 = 7`
- `"a" + "b"` = `"ab"`
- comparison และ logic ทำงาน

---

## Phase 3: Function System ✅ เสร็จสมบูรณ์ (เกินแผน)
**เป้าหมาย:** เรียกฟังก์ชัน built-in ได้

**งานที่ทำแล้ว:**
- implement `FunctionRegistry` (`functions.rs`) พร้อม `HashMap<String, BuiltinFunction>`
- `BuiltinFunction` มี `name`, `arity`, `call` (คืน `Result<Value, FormulaError>`)
- implement built-in functions (10 ตัว):
  - `string.rs`: `len`, `upper`, `lower`, `contains`, `starts_with`, `ends_with`
  - `math.rs`: `abs`, `min`, `max`
  - `logic.rs`: `if`
- ตรวจสอบจำนวน argument ก่อนเรียก -> `ArgumentCountMismatch`
- เพิ่มฟังก์ชันใหม่ได้โดยไม่ต้องแก้ evaluator

**ไฟล์ที่เกี่ยวข้อง:** `src/functions.rs`, `src/builtins/mod.rs`, `src/builtins/string.rs`, `src/builtins/math.rs`, `src/builtins/logic.rs`  
**Milestone ที่ผ่านแล้ว:**
- `if(true, 1, 0)`
- `len("abc")`
- `upper("abc")`
- `contains("hello", "ell")`
- `min(3, 1)`

---

## Phase 4: Context Resolution ✅ เสร็จสมบูรณ์
**เป้าหมาย:** รองรับตัวแปรและข้อมูลภายนอก

**งานที่ทำแล้ว:**
- implement `Context` (`context.rs`) ด้วย `HashMap<String, Value>`
- `get(name)` คืน `Option<Value>`, `set(name, value)`
- evaluator resolve ตัวแปรผ่าน context (ด้วย span error หากไม่พบ)

**ไฟล์ที่เกี่ยวข้อง:** `src/context.rs`  
**Milestone ที่ผ่านแล้ว:**
- `score + 10` (เมื่อ context มี score = 40 => 50)
- `if(done, "yes", "no")` (done เป็น Bool จาก context)
- (ยังไม่ทำ `user.name` แบบ dot lookup – เป็น Phase 6)

---

## Phase 5: Type Handling & Validation 🟡 มีพื้นฐานแล้ว (ใช้งานได้จริง)
**เป้าหมาย:** ลด runtime error และเพิ่มความแม่นยำ

**งานที่ทำแล้ว:**
- type validation ใน evaluator (ทุก binary/unary op, function call) ใช้ pattern match แล้วสร้าง `TypeError` พร้อมข้อความภาษาไทย
- ข้อความ error ชัดเจน เช่น "การบวกใช้ได้กับ number+number หรือ string+string เท่านั้น"
- built-in functions ตรวจสอบ type argument ก่อนทำงาน
- วินิจฉัยดีขึ้น: `diagnostics.rs` แสดง source snippet พร้อม caret ชี้ตำแหน่ง
- ยังไม่มี type inference; เป็นการตรวจสอบ ณ runtime

**Milestone ที่ผ่าน:**
- error เมื่อเอา string ไปบวก number
- error เมื่อเรียกฟังก์ชันผิดชนิด argument
- diagnostics แสดงบรรทัดที่ผิด

**คงเหลือสำหรับ Phase 5 เต็ม:** static type checking (ก่อน evaluate) – เลื่อนไป Phase 7/อนาคต

---

## Phase 6: Advanced Features ✅ เสร็จสมบูรณ์
**เป้าหมาย:** ให้ระบบใช้งานจริงได้มากขึ้น

**งานที่ทำแล้ว:**
- array literal syntax `[ ... ]`, evaluation, nested arrays ✅
- map literal syntax `{ key: value }`, evaluation ✅
- collection functions: `sum`, `avg`, `min_arr`, `max_arr`, `join`, `count` สำหรับ arrays ✅
- date functions: `now`, `date_add`, `date_diff`, `year`, `month`, `day` ใช้ chrono crate ✅
- access chaining (เช่น `user.name`) – เลื่อนไป Phase 7 (ยังไม่ทำ)
- caching, serialization – เลื่อนไป Phase 7 (ยังไม่ทำ)
- ฟังก์ชันเพิ่มเติม `map`, `filter` – เลื่อนไป Phase 7 (ยังไม่ทำ)

---

## Phase 7: Quality & Tooling ✅ เสร็จสมบูรณ์
**เป้าหมาย:** ทำให้ library พร้อมใช้งานในโปรดักชัน

**งานที่ทำแล้ว:**
- unit tests 30+ cases ครอบคลุม lexer, parser, evaluator, functions, errors, integration
- doc-tests ใน `lib.rs` 2 ตัวอย่าง
- diagnostics แบบ snippet
- ใช้ `cargo test` รันผ่านทั้งหมด
- benchmark (การประมวลผล 100+ โหนด) ด้วย criterion ✅
- snapshot tests สำหรับ error formatting ด้วย insta ✅
- API docs (rustdoc) ทุก public item ✅
- CI/CD (GitHub Actions) รัน test, clippy, rustfmt ✅
- examples การใช้งานจริง (basic.rs, advanced.rs) ✅
- performance profiling และ optimization suggestions ✅
- proper Cargo.toml metadata และ README badges ✅

---

# V1 Scope Status
**V1 ควรมี:** number, string, bool, null, arithmetic, comparison, boolean logic, function call, variable/context lookup, syntax/runtime errors, span/diagnostics พื้นฐาน → **ทั้งหมดบรรลุแล้ว**

**V1 ยังไม่ควรมี:** user-defined functions, async, full static typing, plugin system, module system, optimizer ขั้นสูง → **ไม่มีในโค้ดปัจจุบัน** (เป็นไปตามแผน)

---

# สรุป roadmap ที่เป็นจริง (หลัง Phase 6)

- **Sprint 1 (Phase 0–1):** grammar, AST, lexer, parse arithmetic → เสร็จ
- **Sprint 2 (Phase 2):** parser precedence, evaluator, basic errors → เสร็จ
- **Sprint 3 (Phase 3–4):** functions, context, tests → เสร็จ (เกินแผนเรื่องจำนวนฟังก์ชัน)
- **Sprint 4 (Phase 5 บางส่วน + diagnostics + tests):** type checking, diagnostics, docs → เสร็จบางส่วน
- **Sprint 5 (Phase 6):** arrays, maps, date functions, collection functions → เสร็จ
- **Sprint 6 (Phase 7):** quality & tooling, benchmarks, CI/CD, docs, examples → เสร็จ
- **ถัดไป:** Session 2 - Phase 8+ (advanced features, access chaining, map/filter, user-defined functions, etc.)

---

# หลักสถาปัตยกรรมที่ยึดไว้ (คงเดิม)
- AST ต้องไม่ผูกกับ evaluation ✅
- Evaluator ต้องรับ Value/Context อย่างเดียว ✅
- Functions ต้องแยกออกจาก core ✅
- Error ต้องมี span เสมอ ✅
