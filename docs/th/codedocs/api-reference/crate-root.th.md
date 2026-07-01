---
title: "รากของ Crate (Crate Root)"
description: "เอกสารอ้างอิงสำหรับการนำเข้าและการส่งออกใหม่ในระดับบนสุดที่เปิดเผยโดย formula_engine"
---

รากของ crate ใน `src/lib.rs` คือหน้าสัมผัสหลักในการนำเข้าสำหรับโค้ดแอปพลิเคชัน โดยจะทำการส่งออกใหม่ (re-export) ชนิดข้อมูลและฟังก์ชันที่ใช้งานบ่อยที่สุด เพื่อให้คุณสามารถสร้างขั้นตอนการประมวลผลสูตรได้อย่างครบถ้วนโดยไม่ต้องเข้าถึงเส้นทางโมดูลย่อย เว้นแต่คุณต้องการรายละเอียดเชิงลึก เช่น `BuiltinFunction` หรือเครื่องมือช่วยวิเคราะห์ประสิทธิภาพ

## เส้นทางการนำเข้า (Import Path)

```rust
use formula_engine::{Context, Expr, FormulaError, FunctionRegistry, Value, evaluate, parse, tokenize};
```

## รายการที่ถูกส่งออกใหม่ (Re-Exported Items)

### `tokenize`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/lexer.rs`

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, FormulaError>
```

ทำหน้าที่ตัดคำ (tokenize) ข้อความสูตรให้เป็นค่า `Token` พร้อมข้อมูลตำแหน่ง (span)

### `parse`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/parser.rs`

```rust
pub fn parse(tokens: &[Token]) -> Result<SpannedExpr, FormulaError>
```

สร้างโครงสร้าง AST แบบ `SpannedExpr` จากลำดับโทเค็น

### `evaluate`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/eval.rs`

```rust
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError>
```

ประเมินค่า AST โดยใช้ตัวแปรในขณะรันไทม์และทะเบียนฟังก์ชัน

### `Context`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/context.rs`

```rust
pub struct Context
```

ที่เก็บตัวแปรในขณะรันไทม์สำหรับสูตร

### `FunctionRegistry`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/functions.rs`

```rust
pub struct FunctionRegistry
```

ทะเบียนที่ใช้สำหรับจัดเก็บและค้นหาฟังก์ชันที่สามารถเรียกใช้งานได้ตามชื่อ

### `Value`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/value.rs`

```rust
pub enum Value
```

ชนิดข้อมูลผลลัพธ์ในขณะรันไทม์สำหรับสูตรที่ได้รับการประเมินแล้ว

### `Expr`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/ast.rs`

```rust
pub enum Expr
```

ชนิดของโหนด AST ที่ถูกห่อหุ้มโดย `SpannedExpr`

### `FormulaError`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/error.rs`

```rust
pub struct FormulaError
```

ชนิดข้อมูลข้อผิดพลาดแบบโครงสร้างที่ใช้งานในทุกขั้นตอน

## โมดูลสาธารณะ (Public Modules)

รากของ crate ยังเปิดเผยโมดูลเหล่านี้โดยตรง:

| โมดูล | เส้นทางการนำเข้า | วัตถุประสงค์ |
|--------|-------------|---------|
| `ast` | `formula_engine::ast` | Enums ของ AST และตัวห่อหุ้มนิพจน์พร้อมข้อมูลตำแหน่ง |
| `builtins` | `formula_engine::builtins` | การลงทะเบียนฟังก์ชันมาตรฐานและกลุ่มฟังก์ชันภายใน |
| `context` | `formula_engine::context` | การจัดเก็บตัวแปรในขณะรันไทม์ |
| `diagnostics` | `formula_engine::diagnostics` | เครื่องมือช่วยจัดรูปแบบข้อผิดพลาด |
| `error` | `formula_engine::error` | ชนิดของข้อผิดพลาดและฟังก์ชันสร้างข้อผิดพลาด |
| `eval` | `formula_engine::eval` | จุดเริ่มต้นของ Evaluator |
| `functions` | `formula_engine::functions` | ทะเบียนฟังก์ชันและรายการที่เรียกใช้ได้ |
| `lexer` | `formula_engine::lexer` | โทเค็นและตัวตัดคำ (tokenizer) |
| `parser` | `formula_engine::parser` | ตัววิเคราะห์ไวยากรณ์ (parser) และการสร้าง AST |
| `profiling` | `formula_engine::profiling` | การวัดและวิเคราะห์ประสิทธิภาพ |
| `span` | `formula_engine::span` | ชนิดข้อมูล `Position` และ `Span` |
| `value` | `formula_engine::value` | ค่าต่างๆ ในขณะรันไทม์ |

## ตัวอย่าง (Example)

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("score", Value::Number(42.0));

    let ast = parse(&tokenize("score + 8")?)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    assert_eq!(format!("{result:?}"), "Number(50.0)");
    Ok(())
}
```

ดูรายละเอียดโครงสร้างฟังก์ชันและคำนิยามของฟิลด์ต่างๆ ในหน้าถัดไป
