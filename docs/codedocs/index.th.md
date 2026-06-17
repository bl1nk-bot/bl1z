---
title: "Getting Started"
description: "สร้าง, วิเคราะห์ และประมวลผลสูตรแบบ Notion ใน Rust ด้วยรันไทม์ขนาดเล็กที่ขยายความสามารถได้"
---

`formula_engine` เป็น Rust crate สำหรับการทำ tokenizing, parsing และประมวลผล (evaluating) นิพจน์สูตรแบบ Notion ที่รองรับตัวแปร, ฟังก์ชันพื้นฐาน, อาร์เรย์, แมป และตัวช่วยด้านวันที่

## The Problem

- ลอจิกของนิพจน์ (Expression logic) มักกระจัดกระจายอยู่ในบล็อก `match`, ส่วนของ SQL และโค้ดตรวจสอบความถูกต้องเฉพาะของแอปพลิเคชัน
- การสร้าง parser ขึ้นมาเองมักหมายถึงการต้องสร้างลำดับความสำคัญของตัวดำเนินการ (operator precedence), ข้อมูลตำแหน่งข้อผิดพลาด (error spans) และระบบประเภทข้อมูลรันไทม์ใหม่ทั้งหมด ก่อนที่จะเริ่มเขียนลอจิกของแอปพลิเคชันได้
- โค้ดของผลิตภัณฑ์มักต้องการสูตรที่ผู้ใช้กำหนดเองซึ่งสามารถอ้างอิงข้อมูลภายนอกได้ แต่การใช้วิธีแบบ `eval` โดยตรงนั้นไม่ปลอดภัยและควบคุมได้ยาก
- ทีมต้องการวิธีขยายสูตรด้วยฟังก์ชันเฉพาะทางโดเมนโดยไม่ต้องเขียน execution pipeline ใหม่

## The Solution

`formula_engine` แบ่งงานออกเป็นขั้นตอนที่ชัดเจนซึ่งถูกเปิดเผยโดย crate root ใน [`src/lib.rs`](https://github.com/bl1nk-bot/poe-sdk-rs/blob/main/src/lib.rs): `tokenize`, `parse` และ `evaluate` คุณส่งสตริงสูตรเข้าไปใน lexer, parse โทเค็นสตรีมให้เป็น `SpannedExpr` และประมวลผล AST กับ `Context` พร้อมกับ `FunctionRegistry` โดย registry เดียวกันนี้สามารถเก็บได้ทั้ง built-ins จาก [`src/builtins`](https://github.com/bl1nk-bot/poe-sdk-rs/tree/main/src/builtins) และฟังก์ชันของคุณเอง

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("score", Value::Number(87.5));
    ctx.set("bonus", Value::Number(12.5));

    let tokens = tokenize(r#"if(score + bonus >= 100, "pass", "review")"#)?;
    let ast = parse(&tokens)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    println!("{result:?}");
    Ok(())
}
```

Expected result:

```text
String("pass")
```

## Installation

<Tabs values={["crates.io", "git", "path", "workspace"]}>
<Tab value="crates.io">

```toml
[dependencies]
formula_engine = "0.1.0"
```

</Tab>
<Tab value="git">

```toml
[dependencies]
formula_engine = { git = "https://github.com/bl1nk-bot/poe-sdk-rs", branch = "main" }
```

</Tab>
<Tab value="path">

```toml
[dependencies]
formula_engine = { path = "../poe-sdk-rs" }
```

</Tab>
<Tab value="workspace">

```toml
[workspace.dependencies]
formula_engine = { path = "poe-sdk-rs" }
```

</Tab>
</Tabs>

โปรเจกต์นี้เป็น Rust crate ดังนั้นการติดตั้งจะทำผ่าน `Cargo.toml` แทนที่จะเป็นตัวจัดการแพ็กเกจ JavaScript

## Quick Start

ขั้นตอนที่สั้นที่สุดที่ใช้งานได้คือ:

1. สร้าง `FunctionRegistry`
2. ลงทะเบียน built-ins
3. สร้าง `Context` สำหรับตัวแปรภายนอก
4. เรียกใช้ `tokenize`, ตามด้วย `parse`, แล้วจึง `evaluate`

```rust
use formula_engine::builtins;
use formula_engine::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut ctx = Context::new();
    ctx.set("name", Value::String("Alice".to_string()));
    ctx.set("score", Value::Number(85.0));

    let formula = "if(score >= 80, upper(name), lower(name))";
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    let result = evaluate(&ast, &ctx, &registry)?;

    println!("formula: {formula}");
    println!("result: {result:?}");
    Ok(())
}
```

Expected output:

```text
formula: if(score >= 80, upper(name), lower(name))
result: String("ALICE")
```

หากคุณกำลังประมวลผลสูตรเดิมซ้ำๆ ให้เก็บ AST ที่ parse แล้วไว้และเรียกใช้ `evaluate` หลายๆ ครั้งด้วยบริบทที่แตกต่างกัน Parser และ Evaluator ถูกแยกออกจากกันด้วยเหตุผลนั้น

## Key Features

- Pipeline แบบแบ่งชั้น: lexer, parser, evaluator, diagnostics และ profiling helpers ถูกเปิดเผยเป็นโมดูลแยกกัน
- ค่ารันไทม์รองรับ `Number`, `String`, `Bool`, `Null`, `Array` และ `Map` ที่ซ้อนกันได้
- Built-ins ครอบคลุมการทำงานด้านสตริง, ตรรกะ, คอลเลกชัน และวันที่ และถูกลงทะเบียนอย่างชัดเจนด้วย `builtins::register_all`
- `Context` แก้ไขตัวแปรในขณะรันไทม์ รวมถึงการเข้าถึงแบบใช้จุดแยก (dot-separated access) เข้าไปยังโครงสร้าง `Value::Map` ที่ซ้อนกัน
- ข้อผิดพลาดจะเก็บข้อมูล `ErrorKind`, รหัสที่เสถียรเช่น `E401` และข้อมูล `Span` (ไม่บังคับ) สำหรับการวินิจฉัยที่มีรูปแบบชัดเจน
- crate นี้มีเครื่องมือเก็บสถิติน้ำหนักเบา (profiling utilities) สำหรับการวัดผลและวิเคราะห์ต้นทุนของสูตร

## Supported Environments

- Rust `1.70+` ตามเครื่องหมาย README ของโปรเจกต์
- Edition `2021` ตามที่ประกาศไว้ใน [`Cargo.toml`](https://github.com/bl1nk-bot/poe-sdk-rs/blob/main/Cargo.toml)
- สภาพแวดล้อมไลบรารีมาตรฐาน crate นี้ใช้ `std::collections::HashMap`, `std::time` และไลบรารีวันที่ `jiff`

## Where To Go Next

<Cards>
  <Card title="Architecture" href="/docs/architecture">ดูว่า lexer, parser, evaluator และ registries ทำงานร่วมกันอย่างไร</Card>
  <Card title="Core Concepts" href="/docs/execution-pipeline">ทำความเข้าใจ execution pipeline, ค่ารันไทม์, ฟังก์ชัน และโมเดลข้อผิดพลาด</Card>
  <Card title="API Reference" href="/docs/api-reference/crate-root">ข้ามไปที่เส้นทางการนำเข้า, ลายเซ็นฟังก์ชัน และพฤติกรรมที่อ้างอิงจากซอร์สโค้ด</Card>
</Cards>
