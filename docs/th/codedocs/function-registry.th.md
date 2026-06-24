---
title: "Function System"
description: "ใช้ FunctionRegistry และ built-ins เพื่อขยายสูตรด้วยพฤติกรรมเฉพาะของโดเมน"
---

ฟังก์ชันคือกลไกหลักในการขยายความสามารถใน `bl1z` แทนที่จะเขียนโค้ดการทำงานทุกอย่างลงใน evaluator โดยตรง crate นี้จะจัดเก็บรายการที่สามารถเรียกใช้งานได้ใน `FunctionRegistry` จาก `src/functions.rs` และ built-ins เป็นเพียงค่า `BuiltinFunction` ที่ลงทะเบียนไว้ล่วงหน้าซึ่งส่งกลับมาจากโมดูลต่างๆ ใน `src/builtins`

## What This Concept Is

`BuiltinFunction` จะรวมสามสิ่งเข้าด้วยกัน:

- `name: String`
- `arity: usize`
- `call: fn(&[Value]) -> Result<Value, FormulaError>`

`FunctionRegistry` จะเก็บรายการเหล่านั้นตามชื่อใน `HashMap` และ `builtins::register_all` จะเติมข้อมูลใน registry ด้วยไลบรารีมาตรฐานของสตริง, ตรรกะ, คอลเลกชัน, คณิตศาสตร์ และตัวช่วยด้านวันที่

## Why It Exists

การออกแบบนี้ช่วยให้ evaluator มีขนาดเล็กและทำให้ภาษาสูตรมีความเฉพาะเจาะจงกับแอปพลิเคชันโดยไม่ต้องแก้ไข parser หากผลิตภัณฑ์ของคุณต้องการ `clamp`, `tier_name` หรือ `is_premium_user` คุณสามารถลงทะเบียนได้ในวิธีเดียวกับที่ crate ลงทะเบียน `len`, `sum` หรือ `date_add` AST เพียงแค่ต้องแสดงผล "การเรียกฟังก์ชันตามชื่อ" ซึ่งช่วยให้ `Expr::FunctionCall` มีความเสถียรแม้ว่าฟังก์ชันที่มีให้ใช้งานจะเปลี่ยนไปก็ตาม

## How It Works Internally

สาขาของ evaluator สำหรับ `Expr::FunctionCall` ใน `src/eval.rs` จะทำสี่สิ่งตามลำดับ:

1. ค้นหาฟังก์ชันใน `FunctionRegistry`
2. ส่งกลับ `E502` หากไม่พบฟังก์ชัน
3. ตรวจสอบจำนวนอาร์กิวเมนต์ที่แน่นอน (arity) และส่งกลับ `E503` หากไม่ตรงกัน
4. ประมวลผลทุกอาร์กิวเมนต์และเรียกใช้ function pointer ที่จัดเก็บไว้

ขั้นตอนที่สี่นั้นมีความสำคัญ ฟังก์ชันจะทำงานแบบ eager เพราะ evaluator จะประมวลผลอาร์กิวเมนต์ก่อนที่จะเรียกใช้การทำงานของฟังก์ชัน ดังนั้นตัวช่วย `if` ใน `src/builtins/logic.rs` จึงทำงานเหมือนฟังก์ชัน eager ปกติ ไม่ใช่รูปแบบ lazy control-flow พิเศษ

```mermaid
flowchart TD
  A[Expr::FunctionCall] --> B[FunctionRegistry::find]
  B -->|missing| C[E502 FunctionError]
  B -->|found| D[ตรวจสอบ arity]
  D -->|wrong| E[E503 FunctionError]
  D -->|ok| F[ประมวลผลทุกอาร์กิวเมนต์]
  F --> G[call fn(&[Value])]
  G --> H[Value หรือ FormulaError]
```

## How It Relates To Other Concepts

[Execution Pipeline](/docs/execution-pipeline) จะเข้าถึงระบบฟังก์ชันในระหว่างการประมวลผล (evaluation), [Runtime Data Model](/docs/runtime-data-model) กำหนดอาร์กิวเมนต์ `Value` ที่ฟังก์ชันได้รับ และโมเดล [Error Reporting](/docs/error-reporting) กำหนดสิ่งที่ฟังก์ชันควรส่งกลับเมื่อการตรวจสอบอาร์กิวเมนต์ล้มเหลว

## Basic Usage: Register All Built-Ins

```rust
use bl1z::builtins;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let ast = parse(&tokenize("join([\"a\", \"b\", \"c\"], \"-\")")?)?;
    let result = evaluate(&ast, &Context::new(), &registry)?;

    assert_eq!(format!("{result:?}"), "String(\"a-b-c\")");
    Ok(())
}
```

## Advanced Usage: Add A Custom Function

```rust
use bl1z::builtins;
use bl1z::error::{ErrorKind, FormulaError};
use bl1z::functions::BuiltinFunction;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry, Value};

fn clamp(args: &[Value]) -> Result<Value, FormulaError> {
    match (args.first(), args.get(1), args.get(2)) {
        (Some(Value::Number(value)), Some(Value::Number(min)), Some(Value::Number(max))) => {
            Ok(Value::Number(value.max(*min).min(*max)))
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "clamp expects value, min, and max as numbers",
            None,
        )),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    registry.register(BuiltinFunction {
        name: "clamp".to_string(),
        arity: 3,
        call: clamp,
    });

    let ast = parse(&tokenize("clamp(125, 0, 100)")?)?;
    let result = evaluate(&ast, &Context::new(), &registry)?;

    assert_eq!(format!("{result:?}"), "Number(100.0)");
    Ok(())
}
```

<Callout type="warn">อาร์กิวเมนต์ของฟังก์ชันจะถูกประมวลผลอย่างสมบูรณ์เสมอก่อนที่ฟังก์ชันของคุณจะทำงาน ห้ามลงทะเบียนฟังก์ชันที่ต้องพึ่งพาตรรกะแบบ lazy และอย่าคาดหวังว่า `if()` จะช่วยป้องกันการประมวลผลในสาขาที่ไม่ถูกต้อง นอกจากนี้โปรดทราบว่า `arity` มีความเข้มงวด โดย `FunctionRegistry` ไม่รองรับพารามิเตอร์แบบเลือกได้ (optional) หรือแบบรับค่าได้ไม่จำกัด (variadic)</Callout>

## Trade-Offs

<Accordions>
<Accordion title="ทำไมฟังก์ชันถึงใช้ function pointers ธรรมดาแทนที่จะเป็น closures หรือ trait objects">
`BuiltinFunction` จัดเก็บ `call: fn(&[Value]) -> Result<Value, FormulaError>` ซึ่งทำให้การลงทะเบียนตรงไปตรงมาและทำให้การแสดงผลในรันไทม์มีขนาดเล็ก function pointers แบบธรรมดาสามารถคัดลอกได้ง่าย, จัดเก็บใน `HashMap` ได้ง่าย และหลีกเลี่ยงความซับซ้อนของ lifetime หรือการจัดการหน่วยความจำบน heap ข้อเสียคือฟังก์ชันที่ลงทะเบียนไม่สามารถดึงข้อมูลสถานะภายนอก (capture external state) ได้โดยตรงเหมือนที่ closures ทำได้ หากคุณต้องการพฤติกรรมที่ขึ้นอยู่กับการกำหนดค่า ให้สร้างฟังก์ชันจากข้อมูลโกลบอลที่ไม่เปลี่ยนแปลง (immutable global data) หรือเปิดเผยข้อมูลผ่าน `Context` เพื่อให้ฟังก์ชันสามารถอ่านได้จากอาร์กิวเมนต์แทนที่จะเป็นสภาพแวดล้อมของมัน
</Accordion>
<Accordion title="ทำไม built-ins ถึงต้องเลือกใช้ (opt-in) แทนที่จะมีให้ใช้งานได้ทั่วโลก">
crate นี้สามารถส่งมอบโกลบอล registry ที่เติมข้อมูลไว้ล่วงหน้าได้ แต่ `src/functions.rs` และ `src/builtins/mod.rs` ตั้งใจที่จะรักษาการลงทะเบียนให้มีความชัดเจน สิ่งนี้ทำให้การเริ่มต้นระบบคาดเดาได้ และช่วยให้การทดสอบหรือแอปพลิเคชันโฮสต์ควบคุมได้ว่าฟังก์ชันใดบ้างที่มีอยู่ ข้อเสียคือมีโค้ดที่ซ้ำซ้อนเล็กน้อยที่จุดเรียกใช้ เพราะเกือบทุกแอปพลิเคชันจะเริ่มต้นด้วย `let mut registry = FunctionRegistry::new(); builtins::register_all(&mut registry);` ในทางกลับกัน แอปพลิเคชันของคุณสามารถลบ built-ins ที่ไม่ต้องการเปิดเผย หรือซ้อนฟังก์ชันที่กำหนดเองทับลงไปได้โดยไม่มีสถานะโกลบอลที่ซ่อนอยู่
</Accordion>
</Accordions>

รายการ built-in ทั้งหมดและลายเซ็นฟังก์ชันถูกระบุไว้ในเอกสาร [Built-Ins](/docs/api-reference/builtins)
