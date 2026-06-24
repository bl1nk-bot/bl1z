---
title: "รากของ Crate (Crate Root)"
description: "เอกสารอ้างอิงสำหรับการนำเข้าและการส่งออกใหม่ในระดับบนสุดที่เปิดเผยโดย bl1z"
---

รากของ crate ใน `src/lib.rs` คือหน้าสัมผัสหลักในการนำเข้าสำหรับโค้ดแอปพลิเคชัน โดยจะทำการส่งออกใหม่ (re-export) ชนิดข้อมูลและฟังก์ชันที่ใช้งานบ่อยที่สุด เพื่อให้คุณสามารถสร้างขั้นตอนการประมวลผลสูตรได้อย่างครบถ้วนโดยไม่ต้องเข้าถึงเส้นทางโมดูลย่อย เว้นแต่คุณต้องการรายละเอียดเชิงลึก เช่น `BuiltinFunction`, recovery parsing หรือเครื่องมือช่วยวิเคราะห์ประสิทธิภาพ

## เส้นทางการนำเข้า (Import Path)

```rust
use bl1z::{
    Context, EngineConfig, Expr, FormulaError, FunctionRegistry, RecoveryResult, Value,
    evaluate, evaluate_with_config, parse, parse_with_recovery, tokenize,
};
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

### `EngineConfig`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/config.rs`

```rust
pub struct EngineConfig
```

เก็บค่าจำกัดของ engine เช่น ความยาวสูตรสูงสุด ความลึกสูงสุด และ timeout แบบเลือกใช้ได้

### `RecoveryResult`

แหล่งที่มา: `src/lib.rs` ส่งออกใหม่จาก `src/parser.rs`

```rust
pub struct RecoveryResult
```

เก็บ AST ที่กู้คืนได้บางส่วนและรายการ parse errors จาก `parse_with_recovery`

## โมดูลสาธารณะ (Public Modules)

รากของ crate ยังเปิดเผยโมดูลเหล่านี้โดยตรง:

| โมดูล | เส้นทางการนำเข้า | วัตถุประสงค์ |
|--------|-------------|---------|
| `ast` | `bl1z::ast` | Enums ของ AST และตัวห่อหุ้มนิพจน์พร้อมข้อมูลตำแหน่ง |
| `builtins` | `bl1z::builtins` | การลงทะเบียนฟังก์ชันมาตรฐานและกลุ่มฟังก์ชันภายใน |
| `cache` | `bl1z::cache` | เครื่องมือช่วย cache สูตร |
| `config` | `bl1z::config` | การตั้งค่าข้อจำกัดของ engine |
| `context` | `bl1z::context` | การจัดเก็บตัวแปรในขณะรันไทม์ |
| `diagnostics` | `bl1z::diagnostics` | เครื่องมือช่วยจัดรูปแบบข้อผิดพลาด |
| `error` | `bl1z::error` | ชนิดของข้อผิดพลาดและฟังก์ชันสร้างข้อผิดพลาด |
| `eval` | `bl1z::eval` | จุดเริ่มต้นของ Evaluator |
| `functions` | `bl1z::functions` | ทะเบียนฟังก์ชันและรายการที่เรียกใช้ได้ |
| `lexer` | `bl1z::lexer` | โทเค็นและตัวตัดคำ (tokenizer) |
| `parser` | `bl1z::parser` | ตัววิเคราะห์ไวยากรณ์ (parser) และการสร้าง AST |
| `plugins` | `bl1z::plugins` | trait และ manager สำหรับปลั๊กอิน |
| `profiling` | `bl1z::profiling` | การวัดและวิเคราะห์ประสิทธิภาพ |
| `span` | `bl1z::span` | ชนิดข้อมูล `Position` และ `Span` |
| `value` | `bl1z::value` | ค่าต่างๆ ในขณะรันไทม์ |

## ตัวอย่าง (Example)

```rust
use bl1z::builtins;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

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
