---
title: "การวินิจฉัย, ข้อผิดพลาด และการวิเคราะห์ประสิทธิภาพ (Diagnostics, Errors, And Profiling)"
description: "เอกสารอ้างอิงสำหรับข้อผิดพลาดแบบโครงสร้าง, การจัดรูปแบบการวินิจฉัย และเครื่องมือช่วยวิเคราะห์ประสิทธิภาพ"
---

หน้านี้ครอบคลุมโมดูลสนับสนุนที่ทำงานควบคู่กับตัววิเคราะห์ไวยากรณ์และตัวประเมินค่า: `src/error.rs`, `src/diagnostics.rs` และ `src/profiling.rs`

## `bl1z::error`

### `ErrorKind`

```rust
pub enum ErrorKind {
    LexError,
    ParseError,
    EvalError,
    TypeError,
    FunctionError,
    ContextError,
}
```

### `FormulaError`

เส้นทางการนำเข้า (Import path): `bl1z::FormulaError` หรือ `bl1z::error::FormulaError`

```rust
pub struct FormulaError {
    pub kind: ErrorKind,
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
}
```

### `FormulaError::new`

```rust
pub fn new(kind: ErrorKind, code: &str, message: &str, span: Option<Span>) -> Self
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `kind` | `ErrorKind` | — | หมวดหมู่ของข้อผิดพลาด |
| `code` | `&str` | — | รหัสข้อผิดพลาดที่คงที่ เช่น `E101` หรือ `E401` |
| `message` | `&str` | — | ข้อความอธิบายที่มนุษย์อ่านเข้าใจ |
| `span` | `Option<Span>` | — | ตำแหน่งในซอร์สโค้ด (หากมี) |

## `bl1z::diagnostics`

### `format_error`

เส้นทางการนำเข้า (Import path): `bl1z::diagnostics::format_error`

```rust
pub fn format_error(source: &str, error: &FormulaError) -> String
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `source` | `&str` | — | ซอร์สโค้ดของสูตรต้นฉบับ |
| `error` | `&FormulaError` | — | ข้อผิดพลาดแบบโครงสร้างที่ต้องการแสดงผล |

ตัวอย่าง:

```rust
use bl1z::diagnostics::format_error;

let source = "a | b";
let err = bl1z::tokenize(source).unwrap_err();
println!("{}", format_error(source, &err));
```

## `bl1z::profiling`

### `PerformanceMetrics`

```rust
pub struct PerformanceMetrics {
    pub tokenize_time: Duration,
    pub parse_time: Duration,
    pub eval_time: Duration,
    pub total_time: Duration,
    pub iterations: usize,
}
```

### `profile_formula`

เส้นทางการนำเข้า (Import path): `bl1z::profiling::profile_formula`

```rust
pub fn profile_formula(
    formula: &str,
    ctx: &Context,
    registry: &FunctionRegistry,
    iterations: usize,
) -> Result<PerformanceMetrics, FormulaError>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `formula` | `&str` | — | สูตรที่ต้องการวัดประสิทธิภาพ |
| `ctx` | `&Context` | — | บริบทรันไทม์ |
| `registry` | `&FunctionRegistry` | — | ทะเบียนฟังก์ชัน |
| `iterations` | `usize` | — | จำนวนรอบในการวัดผล **ต้องมีค่ามากกว่า 0** หากส่งค่า `0` จะทำให้เกิด runtime panic (หารด้วยศูนย์) |

### `OptimizationSuggestions`

```rust
pub struct OptimizationSuggestions {
    pub suggestions: Vec<String>,
    pub complexity: FormulaComplexity,
}
```

### `FormulaComplexity`

```rust
pub enum FormulaComplexity {
    Simple,
    Moderate,
    Complex,
    High,
}
```

### `analyze_formula`

เส้นทางการนำเข้า (Import path): `bl1z::profiling::analyze_formula`

```rust
pub fn analyze_formula(formula: &str) -> Result<OptimizationSuggestions, FormulaError>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `formula` | `&str` | — | สูตรที่ต้องการตรวจสอบ |

หมายเหตุการติดตั้งจาก `src/profiling.rs`:

- อาร์เรย์ขนาดใหญ่ที่มีสมาชิกเกิน `20` ตัวจะเพิ่มระดับความซับซ้อนไปสู่ `Complex`
- อาร์เรย์ที่มีสมาชิกเกิน `100` ตัวจะถูกจัดระดับเป็น `High`
- ฟังก์ชัน `sum`, `avg`, `min` และ `max` จะกระตุ้นคำแนะนำเรื่องการทำแคชเมื่อถูกเรียกใช้ด้วยอาร์กิวเมนต์อาร์เรย์ตัวเดียว
- ฟังก์ชันชื่อ `fibonacci` จะถูกปฏิบัติเสมือนมีความซับซ้อนสูงในตัววิเคราะห์ ซึ่งสอดคล้องกับตัวอย่างแบบ recursive ใน `examples/advanced.rs` (แม้ว่าฟังก์ชันนี้จะไม่ได้เป็นฟังก์ชันภายในที่ลงทะเบียนไว้ก็ตาม)

## ตัวอย่าง (Example)

```rust
use bl1z::builtins;
use bl1z::profiling::{analyze_formula, profile_formula};
use bl1z::{Context, FunctionRegistry};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let metrics = profile_formula("sum([1,2,3,4,5])", &Context::new(), &registry, 100).unwrap();
let analysis = analyze_formula("sum([1,2,3,4,5])").unwrap();

println!("{metrics:?}");
println!("{analysis:?}");
```

เมื่อคุณต้องการการทำ debugging ที่เป็นมิตรต่อผู้ใช้งาน ให้ใช้งานหน้านี้ควบคู่กับ [การรายงานข้อผิดพลาด (Error Reporting)](/docs/error-reporting)
