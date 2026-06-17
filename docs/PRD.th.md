# เอกสารความต้องการทางเทคนิคของ bl1z (bl1z Technical Requirements Document)

เอกสารฉบับนี้อธิบายรายละเอียดข้อมูลจำเพาะทางเทคนิคสำหรับไลบรารีการคำนวณ bl1z

## ภาพรวมและเป้าหมาย (Overview and Goals)

> ระบบ bl1z คือไลบรารีการคำนวณภายใน (คล้ายกับสูตรใน Notion) ซึ่งมีความสามารถหลัก 3 ประการ:

1. **การแยกส่วน (Parse)** – แปลงข้อความสูตรเป็นรูปแบบ AST
2. **การประเมินค่า (Evaluate)** – คำนวณ AST และคืนค่าผลลัพธ์
3. **การขยาย (Extend)** – เพิ่มชนิดข้อมูล, ฟังก์ชัน และบริบทใหม่ได้ง่าย โดยส่งผลกระทบต่อโค้ดส่วนอื่นให้น้อยที่สุด

## ขอบเขตของระบบ (System Scope)

### อยู่ในขอบเขต (V1)

- นิพจน์ทางคณิตศาสตร์ (บวก, ลบ, คูณ, หาร)
- การเปรียบเทียบ (>, <, >=, <=, ==, !=)
- ตรรกะ (AND, OR, NOT)
- การจัดการข้อความพื้นฐาน (การเชื่อมข้อความ)
- การเรียกใช้ฟังก์ชัน
- การอ้างอิงตัวแปร/บริบท
- การรายงานข้อผิดพลาด (การตัดคำ, การวิเคราะห์ไวยากรณ์, การประเมินค่า)
- ฟังก์ชันภายในที่สามารถลงทะเบียนเพิ่มได้

### นอกเหนือขอบเขต (V1)

- คอมไพเลอร์เต็มรูปแบบ
- การปรับแต่งประสิทธิภาพขั้นสูง
- ระบบชนิดข้อมูลสถิตที่ซับซ้อนอย่างเต็มที่
- ฟังก์ชันที่ผู้ใช้กำหนดเอง
- การประมวลผลแบบอะซิงโครนัส
- การรันในแซนด์บ็อกซ์

### สถาปัตยกรรมระดับสูง (High-Level Architecture)

#### สถาปัตยกรรมระบบแบบแบ่งชั้น (Layered System Architecture)

| ชั้น (Layer) | ชื่อ (Name) | ความรับผิดชอบ (Responsibility) |
|-------|------|----------------|
| 1 | ข้อมูลนำเข้า (Input) | รับข้อความสูตร เช่น `if(score > 50, "pass", "fail")` |
| 2 | การวิเคราะห์คำศัพท์ (Lexer) | แปลงข้อความ → ลำดับโทเค็น (Token Stream) |
| 3 | การวิเคราะห์ไวยากรณ์ (Parser) | แปลงโทเค็น → โครงสร้าง AST (Abstract Syntax Tree) |
| 4 | ความหมายและการตรวจสอบ (Semantic / Validation) | ตรวจสอบความถูกต้องเบื้องต้น (การมีอยู่ของฟังก์ชัน, จำนวนพารามิเตอร์, ชนิดข้อมูล) |
| 5 | การประเมินค่า (Evaluator) | ประมวลผล AST และสร้างค่าลัพธ์ (Value) |
| 6 | บริบท (Context) | จัดเก็บข้อมูลรันไทม์ (ตัวแปร, ข้อมูลเรคคอร์ด, สภาพแวดล้อม) |
| 7 | ทะเบียนฟังก์ชัน (Function Registry) | จัดเก็บฟังก์ชันภายใน (Built-in functions) |
| 8 | ระบบข้อผิดพลาด (Error System) | จัดการข้อผิดพลาดพร้อมข้อมูลตำแหน่ง (Span) |

## ข้อมูลจำเพาะของส่วนประกอบ (Component Specification)

### 1. AST (ast.rs)

- โครงสร้างข้อมูลกลางที่ไม่มีลอจิกการคำนวณ
- ประกอบด้วย `enum Expr` (Literal, UnaryExpr, BinaryExpr, FunctionCall, VariableRef, Grouping)
- แยก `BinaryOp` และ `UnaryOp` ออกจากกัน

### 2. การวิเคราะห์คำศัพท์ (Lexer - lexer.rs)

- รับข้อความและสร้างโทเค็นทีละรายการ
- โทเค็นประกอบด้วย `TokenKind` (Identifier, Number, String, LParen, RParen, Comma, Operator, Keyword)
- เก็บข้อมูลตำแหน่ง (บรรทัด/คอลัมน์) สำหรับ `Span`
- ข้ามช่องว่าง (Whitespace), จัดการค่าคงที่ (Literals), แยกตัวดำเนินการ

### 3. การวิเคราะห์ไวยากรณ์ (Parser - parser.rs)

- ใช้งานแบบ recursive descent พร้อมตารางลำดับความสำคัญและกฎการจัดกลุ่ม
- ลำดับความสำคัญของตัวดำเนินการ:
  1. วงเล็บ `()`
  2. ยูนารี `-`, `!`
  3. `*`, `/`
  4. `+`, `-`
  5. การเปรียบเทียบ (`<`, `>`, `<=`, `>=`)
  6. ความเท่ากัน (`==`, `!=`)
  7. ตรรกะ AND
  8. ตรรกะ OR
- รองรับการเรียกใช้ฟังก์ชันและการอ้างอิงตัวแปร
- รายงานข้อผิดพลาดทางไวยากรณ์พร้อมตำแหน่ง
- ระบบกู้คืนข้อผิดพลาดเบื้องต้น (ทางเลือก)

### 4. คุณค่า (Value - value.rs)

- ชนิดข้อมูลที่ส่งคืนโดย Evaluator
- `Enum Value`:
  - `Number(f64)`
  - `String(String)`
  - `Bool(bool)`
  - `Null`
  - `Array(Vec<Value>)`
  - `Map(HashMap<String, Value>)`
  - `DateTime(jiff::Timestamp)`
  - `Duration(jiff::Span)`
  - `Set(HashSet<Value>)`
  - `Range { start, end, step }`
- ใช้ `f64` สำหรับตัวเลข; `jiff` สำหรับวันที่/เวลา

### 5. บริบท (Context - context.rs)

- จัดเก็บตัวแปรในขณะรันไทม์ใน `HashMap<String, Value>`
- ฟังก์ชัน `get(name) -> Option<Value>` และ `set(name, value)`
- ใช้สำหรับการระบุตัวตน เช่น `score`, `user.name` (รองรับการค้นหาแบบลำดับชั้นในภายหลัง)

### 6. ทะเบียนฟังก์ชัน (Function Registry - functions.rs)

- จัดเก็บฟังก์ชันภายในใน `HashMap<String, BuiltinFunction>`
- `BuiltinFunction` มีชื่อ, จำนวนพารามิเตอร์ (arity) และเมธอด `call(args: &[Value]) -> Value`
- ตรวจสอบจำนวนพารามิเตอร์ก่อนการเรียกใช้
- เพิ่มฟังก์ชันใหม่ได้โดยไม่ต้องแก้ไข Evaluator

### 7. การประเมินค่า (Evaluator - eval.rs)

- ฟังก์ชัน `eval(expr: &Expr, ctx: &Context, functions: &FunctionRegistry) -> Result<Value, EvalError>`
- ทำงานแบบ Recursive: ตรวจสอบรูปแบบ `Expr` และประมวลผลตามชนิด
- คืนค่าข้อผิดพลาดสำหรับการดำเนินการที่ไม่ถูกต้อง (เช่น การหารด้วยศูนย์, ชนิดข้อมูลไม่ตรงกัน)

### 8. ระบบข้อผิดพลาด (Error System - error.rs, span.rs, diagnostics.rs)

- ทุกข้อผิดพลาดประกอบด้วย รหัส, ข้อความ, ตำแหน่ง และหมวดหมู่
- หมวดหมู่: LexError, ParseError, EvalError, TypeError, FunctionError, ContextError
- `span.rs` จัดเก็บ `Span { start: Position, end: Position }` สำหรับการติดตามตำแหน่ง
- `diagnostics.rs` ช่วยจัดรูปแบบข้อความแสดงข้อผิดพลาดพร้อมบริบท

### 9. ฟังก์ชันภายใน (Built-in Functions - builtins/)

จัดกลุ่มเป็นโมดูล:

- `string.rs`: len, upper, lower, contains, starts_with, ends_with, replace
- `math.rs`: abs, round, min, max, sqrt
- `logic.rs`: if, and, or, not
- `date.rs`: now, date_add, date_diff (เพิ่มภายหลัง)

#### ฟังก์ชันพื้นฐานสำหรับ V1

| ฟังก์ชัน | คำอธิบาย | จำนวนพารามิเตอร์ |
|----------|-------------|-----------------|
| `if(cond, a, b)` | ถ้า cond เป็นจริง คืนค่า a, มิเช่นนั้นคืนค่า b | 3 |
| `len(text)` | ความยาวข้อความ | 1 |
| `upper(text)` | แปลงเป็นตัวพิมพ์ใหญ่ | 1 |
| `lower(text)` | แปลงเป็นตัวพิมพ์เล็ก | 1 |

## ข้อมูลจำเพาะของไวยากรณ์ (Syntax Specification - Basic Grammar)

```
expression → logical_or
logical_or → logical_and ('||' logical_and)*
logical_and → equality ('&&' equality)*
equality → comparison (('==' | '!=') comparison)*
comparison → term (('<' | '>' | '<=' | '>=') term)*
term → factor (('+' | '-') factor)*
factor → unary (('*' | '/') unary)*
unary → ('-' | '!')? primary
primary → NUMBER | STRING | IDENTIFIER | '(' expression ')' | function_call
function_call → IDENTIFIER '(' expression (',' expression)* ')'
```

## ข้อมูลจำเพาะของข้อผิดพลาด (Error Specification)

ทุกข้อผิดพลาดต้องมี:

- **รหัส (code)** (เช่น E001)
- **ข้อความ (message)** (คำอธิบาย)
- **ตำแหน่ง (span)** (ตำแหน่งเริ่มต้น-สิ้นสุดในข้อความ)
- **หมวดหมู่ (category)** (กลุ่มของข้อผิดพลาด)

### ตัวอย่างรหัสข้อผิดพลาด:

- E001 – UnexpectedToken (พบโทเค็นที่ไม่คาดคิด)
- E002 – UnknownIdentifier (ไม่รู้จักตัวระบุ)
- E003 – UnsupportedOperator (ไม่รองรับตัวดำเนินการ)
- E004 – DivisionByZero (การหารด้วยศูนย์)
- E005 – ArgumentCountMismatch (จำนวนพารามิเตอร์ไม่ถูกต้อง)
- E006 – TypeMismatch (ชนิดข้อมูลไม่ตรงกัน)

## โครงสร้างไฟล์ Rust ที่แนะนำ

```
src/
  lib.rs
  lexer.rs
  parser.rs
  ast.rs
  value.rs
  eval.rs
  context.rs
  functions.rs
  error.rs
  span.rs
  diagnostics.rs
  builtins/
    mod.rs
    string.rs
    math.rs
    logic.rs
    date.rs
```

## แผนงานการพัฒนาแบบแบ่งเฟส (Phased Development Roadmap)

### เฟส 0: การออกแบบและกำหนดขอบเขต
- กำหนดกรณีการใช้งาน, ไวยากรณ์, ชนิดข้อมูล, ตัวดำเนินการ, ฟังก์ชันภายใน และรูปแบบข้อผิดพลาด
- จัดทำเอกสารข้อมูลจำเพาะ V1

### เฟส 1: Lexer + AST + Parser
- สร้าง tokenizer และ parser
- วิเคราะห์สูตร `1 + 2 * 3` ให้เป็น AST ที่ถูกต้อง

### เฟส 2: การประเมินค่าพื้นฐาน (Basic Evaluator)
- ติดตั้ง Value และ evaluator
- ประมวลผลคณิตศาสตร์, การเปรียบเทียบ และตรรกะ

### เฟส 3: ระบบฟังก์ชัน
- สร้างทะเบียนฟังก์ชัน, เรียกใช้ `if`, `len`, `upper`

### เฟส 4: การค้นหาตัวแปร (Context Resolution)
- รองรับตัวแปรจากบริบท

### เฟส 5: การจัดการและการตรวจสอบชนิดข้อมูล (ภายหลัง)
- ตรวจสอบชนิดข้อมูลก่อนการประเมินค่าเพื่อให้ได้ข้อความแสดงข้อผิดพลาดที่ชัดเจนขึ้น

### เฟส 6: ฟีเจอร์ขั้นสูง (Array, DateTime, Object, Chaining)

### เฟส 7: คุณภาพและเครื่องมือ (การทดสอบ, เอกสาร, benchmarks)

## ข้อกำหนดที่ไม่เกี่ยวกับฟังก์ชันการทำงาน (Non-Functional Requirements - NFR)

- **ประสิทธิภาพ (Performance)**: ตอบสนองต่อโทเค็นจำนวนมากภายในหน่วยมิลลิวินาที
- **การบำรุงรักษา (Maintainability)**: เพิ่มฟังก์ชันใหม่ได้โดยไม่ต้องแก้ไขส่วนหลักของ evaluator
- **การรายงานข้อผิดพลาด (Error Reporting)**: ระบุตำแหน่งที่ถูกต้องแม่นยำ
- **ความเสถียร (Stability)**: ต้องไม่เกิดอาการ panic จากข้อมูลนำเข้าที่ผิดปกติ (ใช้งาน `Result` ในทุกจุด)
