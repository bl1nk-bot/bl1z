# bl1z

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)]()
[![Build Status](https://github.com/bl1nk-bot/poe-sdk-rs/workflows/CI/badge.svg)](https://github.com/bl1nk-bot/poe-sdk-rs/actions)
[![Documentation](https://docs.rs/bl1z/badge.svg)](https://docs.rs/bl1z)
[![Crates.io](https://img.shields.io/crates/v/bl1z.svg)](https://crates.io/crates/bl1z)
[![Downloads](https://img.shields.io/crates/d/bl1z.svg)](https://crates.io/crates/bl1z)
[![Code Coverage](https://codecov.io/gh/bl1nk-bot/poe-sdk-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/bl1nk-bot/poe-sdk-rs)

## 📖 ภาพรวม

**bl1z** คือไลบรารีสำหรับแยกส่วน (parse) และประมวลผล (evaluate) สูตรทางคณิตศาสตร์และตรรกะแบบ Notion-like ที่เขียนด้วยภาษา Rust ออกแบบมาให้มีความยืดหยุ่นสูง สามารถขยายฟังก์ชันและชนิดข้อมูลได้ง่ายผ่านระบบ registry

### ✨ คุณสมบัติหลัก

- **Lexer & Parser**: แปลงข้อความสูตรเป็น AST (Abstract Syntax Tree) พร้อมข้อมูลตำแหน่ง (span) สำหรับรายงานข้อผิดพลาดที่แม่นยำ
- **Evaluator**: ประเมินค่า AST โดยรองรับชนิดข้อมูลพื้นฐาน 6 ประเภท:
  - `Number` (ตัวเลข)
  - `String` (ข้อความ)
  - `Bool` (ค่าความจริง)
  - `Null` (ค่าว่าง)
  - `Array` (อาร์เรย์)
  - `Map` (แมป/พจนานุกรม)
- **Function System**: ระบบฟังก์ชันที่ขยายได้ รองรับ built-in functions และผู้ใช้สามารถเพิ่มฟังก์ชันใหม่ได้เอง
- **Context Support**: รองรับตัวแปรและการอ้างอิงค่าจากภายนอก
- **Error Reporting**: รายงานข้อผิดพลาดอย่างละเอียดพร้อมตำแหน่งบรรทัดและคอลัมน์

---

## 🚀 การติดตั้ง

เพิ่ม dependency นี้ในไฟล์ `Cargo.toml` ของคุณ:

```toml
[dependencies]
bl1z = "0.1.0"
```

หรือหากต้องการใช้จาก source code ในเครื่อง:

```toml
[dependencies]
bl1z = { path = "./path/to/bl1z" }
```

---

## 📖 วิธีใช้งาน

### ตัวอย่างพื้นฐาน

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry};
use bl1z::builtins;

// สร้าง registry พร้อมฟังก์ชันพื้นฐาน
let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

// Parse และ evaluate สูตร
let tokens = tokenize("1 + 2 * 3").unwrap();
let ast = parse(&tokens).unwrap();
let ctx = Context::new();
let result = evaluate(&ast, &ctx, &registry).unwrap();

println!("ผลลัพธ์: {:?}", result); // Number(7.0)
```

### ใช้กับฟังก์ชัน Built-in

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry};
use bl1z::builtins;

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

// ตัวอย่าง: ฟังก์ชัน if
let tokens = tokenize("if(true, \"ผ่าน\", \"ไม่ผ่าน\")").unwrap();
let ast = parse(&tokens).unwrap();
let ctx = Context::new();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::String("ผ่าน".to_string()));

// ตัวอย่าง: ฟังก์ชัน string
let tokens = tokenize("len(\"hello\")").unwrap();
let ast = parse(&tokens).unwrap();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::Number(5.0));
```

### ใช้กับตัวแปร (Context)

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry, Value};
use bl1z::builtins;

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let mut ctx = Context::new();
ctx.set_variable("score", Value::Number(85.0));
ctx.set_variable("name", Value::String("สมชาย".to_string()));

let tokens = tokenize("if(score > 50, name, \"ไม่มีใคร\")").unwrap();
let ast = parse(&tokens).unwrap();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::String("สมชาย".to_string()));
```

---

## 📋 ไวยากรณ์ที่รองรับ

### ตัวดำเนินการ (Operators)

| หมวดหมู่ | ตัวดำเนินการ | คำอธิบาย |
|----------|-------------|----------|
| คณิตศาสตร์ | `+`, `-`, `*`, `/` | บวก, ลบ, คูณ, หาร |
| เปรียบเทียบ | `<`, `>`, `<=`, `>=` | เปรียบเทียบค่า |
| เท่ากัน | `==`, `!=` | ตรวจสอบความเท่ากัน |
| ตรรกะ | `&&`, `\|\|`, `!` | AND, OR, NOT |
| ยูนารี | `-`, `!` | ลบเลข, นิเสธ |

### ชนิดข้อมูล (Data Types)

- **Number**: ตัวเลขทศนิยม เช่น `1`, `3.14`, `-5`
- **String**: ข้อความในเครื่องหมายคำพูด เช่น `"hello"`, `"สวัสดี"`
- **Bool**: ค่าความจริง `true` หรือ `false`
- **Null**: ค่าว่าง `null`
- **Array**: อาร์เรย์ เช่น `[1, 2, 3]`, `["a", "b"]`, `[[1, 2], [3, 4]]`
- **Map**: แมป เช่น `{name: "John", age: 30}`, `{key: value}`

### ฟังก์ชัน Built-in

#### ฟังก์ชันจัดการข้อความ (String)
| ฟังก์ชัน | คำอธิบาย | ตัวอย่าง |
|----------|----------|----------|
| `len(str)` | หาความยาวข้อความ | `len("hello")` → `5` |
| `upper(str)` | แปลงเป็นตัวพิมพ์ใหญ่ | `upper("abc")` → `"ABC"` |
| `lower(str)` | แปลงเป็นตัวพิมพ์เล็ก | `lower("ABC")` → `"abc"` |
| `contains(str, substr)` | ตรวจสอบว่ามีข้อความย่อยหรือไม่ | `contains("hello", "ell")` → `true` |
| `starts_with(str, prefix)` | ตรวจสอบขึ้นต้นด้วย | `starts_with("hello", "he")` → `true` |
| `ends_with(str, suffix)` | ตรวจสอบลงท้ายด้วย | `ends_with("hello", "lo")` → `true` |

#### ฟังก์ชันคณิตศาสตร์ (Math)
| ฟังก์ชัน | คำอธิบาย | ตัวอย่าง |
|----------|----------|----------|
| `abs(num)` | ค่าสัมบูรณ์ | `abs(-5)` → `5` |
| `min(a, b)` | ค่าน้อยที่สุดระหว่างสองค่า | `min(3, 1)` → `1` |
| `max(a, b)` | ค่ามากที่สุดระหว่างสองค่า | `max(3, 1)` → `3` |

#### ฟังก์ชันตรรกะ (Logic)
| ฟังก์ชัน | คำอธิบาย | ตัวอย่าง |
|----------|----------|----------|
| `if(condition, true_val, false_val)` | เงื่อนไข | `if(true, 1, 0)` → `1` |

#### ฟังก์ชันคอลเลกชัน (Collection)
| ฟังก์ชัน | คำอธิบาย | ตัวอย่าง |
|----------|----------|----------|
| `sum(arr)` | ผลรวมของอาร์เรย์ | `sum([1, 2, 3])` → `6` |
| `avg(arr)` | ค่าเฉลี่ยของอาร์เรย์ | `avg([1, 2, 3])` → `2` |
| `min(arr)` | ค่าน้อยที่สุดในอาร์เรย์ | `min([1, 2, 3])` → `1` |
| `max(arr)` | ค่ามากที่สุดในอาร์เรย์ | `max([1, 2, 3])` → `3` |
| `count(arr)` | นับจำนวนสมาชิก | `count([1, 2, 3])` → `3` |
| `join(arr, sep)` | ต่อข้อความในอาร์เรย์ | `join(["a","b"], ",")` → `"a,b"` |

#### ฟังก์ชันวันที่ (Date)
| ฟังก์ชัน | คำอธิบาย | ตัวอย่าง |
|----------|----------|----------|
| `now()` | วันที่และเวลาปัจจุบัน (ISO 8601) | `now()` → `"2023-12-01T12:00:00Z"` |
| `date_add(date, days)` | เพิ่มวันให้วันที่ | `date_add("2023-01-01", 5)` → `"2023-01-06T00:00:00Z"` |
| `date_diff(date1, date2)` | จำนวนวันระหว่างวันที่ | `date_diff("2023-01-05", "2023-01-01")` → `4` |
| `year(date)` | ปีจากวันที่ | `year("2023-05-15")` → `2023` |
| `month(date)` | เดือนจากวันที่ | `month("2023-05-15")` → `5` |
| `day(date)` | วันที่จากวันที่ | `day("2023-05-15")` → `15` |

---

## 🏗️ สถาปัตยกรรม

bl1z ใช้สถาปัตยกรรมแบบ layered architecture:

```
┌─────────────────┐
│   Input Layer   │  ← สูตร: "1 + 2 * if(x > 0, 3, 4)"
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Lexer       │  → Token Stream พร้อม Span
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Parser      │  → AST (Abstract Syntax Tree)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Evaluator    │  → Value (Number/String/Bool/Null)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Result      │
└─────────────────┘
```

### โครงสร้างโปรเจกต์

```
bl1z/
├── src/
│   ├── lib.rs           # จุดเข้าใช้งานหลักและ re-export
│   ├── lexer.rs         # Lexer: แปลง string → tokens
│   ├── parser.rs        # Parser: แปลง tokens → AST
│   ├── ast.rs           # นิยามโครงสร้าง AST
│   ├── eval.rs          # Evaluator: ประเมินค่า AST
│   ├── value.rs         # นิยามชนิดข้อมูล Value
│   ├── context.rs       # Context: จัดการตัวแปร
│   ├── functions.rs     # Function Registry
│   ├── error.rs         # นิยามข้อผิดพลาด
│   ├── span.rs          # ข้อมูลตำแหน่ง (บรรทัด/คอลัมน์)
│   ├── diagnostics.rs   # ระบบวินิจฉัยข้อผิดพลาด
│   └── builtins/        # ฟังก์ชันพื้นฐาน
│       ├── mod.rs
│       ├── string.rs    # ฟังก์ชันข้อความ
│       ├── math.rs      # ฟังก์ชันคณิตศาสตร์
│       ├── logic.rs     # ฟังก์ชันตรรกะ
│       ├── date.rs      # ฟังก์ชันวันที่
│       └── collection.rs # ฟังก์ชันคอลเลกชัน
├── docs/                # เอกสารประกอบ
├── Cargo.toml
├── LICENSE
├── README.md
├── SPEC.md              # ข้อมูลจำเพาะทางเทคนิค
└── PLAN.md              # แผนงานการพัฒนา
```

---

## 📊 สถานะการพัฒนา

### ✅ เสร็จสมบูรณ์ (V1)

- [x] Lexer: Tokenization พร้อม span tracking
- [x] Parser: Recursive descent parser พร้อม precedence handling
- [x] AST: โครงสร้างต้นไม้พร้อม span ทุกโหนด
- [x] Evaluator: รองรับ number, string, bool, null
- [x] Operators: คณิตศาสตร์, เปรียบเทียบ, equality, logic
- [x] Function System: Registry และ built-in functions 10+ ตัว
- [x] Context: ตัวแปรและการอ้างอิงค่า
- [x] Error Reporting: รายงานข้อผิดพลาดพร้อมตำแหน่ง
- [x] Documentation: Doc-tests และ integration tests

### ✅ เสร็จสมบูรณ์ (Phase 6 - Advanced Features)

- [x] Array syntax และ evaluation
- [x] Map/Dictionary support
- [x] Date/Time functions
- [x] Additional collection functions
- [ ] Performance optimization (Phase 7)

### 📋 แผนในอนาคต (Phase 7+)

- [ ] Static type checking
- [ ] Macro system
- [ ] User-defined functions
- [ ] Lazy evaluation
- [ ] Caching/Memoization

---

## 🧪 การทดสอบ

รันการทดสอบทั้งหมด:

```bash
cargo test
```

รันการทดสอบเฉพาะโมดูล:

```bash
cargo test lexer
cargo test parser
cargo test eval
```

รัน doc-tests:

```bash
cargo test --doc
```

---

## 📄 เอกสารเพิ่มเติม

- **[SPEC.md](SPEC.md)**: ข้อมูลจำเพาะทางเทคนิคและสถาปัตยกรรมโดยละเอียด
- **[PLAN.md](PLAN.md)**: แผนงานการพัฒนาแต่ละเฟส
- **[docs/PRD.md](docs/PRD.md)**: เอกสารความต้องการผลิตภัณฑ์
- **[docs/idea-extension-gemini.md](docs/idea-extension-gemini.md)**: ไอเดียการขยายระบบ
- **[docs/overview-extension-poe.md](docs/overview-extension-poe.md)**: ภาพรวมการขยายสำหรับ POE SDK

---

## 🤝 การมีส่วนร่วม

เราสนับสนุนการมีส่วนร่วมจากชุมชน! หากคุณพบปัญหาหรือมีข้อเสนอแนะ:

1. เปิด Issue เพื่อรายงานบั๊กหรือขอฟีเจอร์ใหม่
2. Fork โปรเจกต์และสร้าง Pull Request
3. เขียนทดสอบให้ครอบคลุมการเปลี่ยนแปลง
4. อัพเดตเอกสารหากมีการเปลี่ยนแปลง API

---

## 📝 ใบอนุญาต

โปรเจกต์นี้เผยแพร่ภายใต้ใบอนุญาต [MIT License](LICENSE)

---

## 🙏 ขอบคุณ

- สร้างด้วย ❤️ โดยใช้ [Rust](https://www.rust-lang.org/)
- ได้รับแรงบันดาลใจจาก Notion formula engine
- พัฒนาสำหรับ POE SDK ecosystem

---

## 📞 ติดต่อ

หากมีคำถามหรือต้องการความช่วยเหลือ กรุณาเปิด Issue ใน GitHub repository นี้