---
title: "บริบทและฟังก์ชัน (Context And Functions)"
description: "เอกสารอ้างอิงสำหรับ Context, BuiltinFunction, FunctionRegistry และ builtins::register_all"
---

หน้านี้อธิบายรายละเอียด API สำหรับการตั้งค่ารันไทม์ที่เปิดเผยผ่าน `src/context.rs`, `src/functions.rs` และ `src/builtins/mod.rs`

## `formula_engine::Context`

ไฟล์ต้นฉบับ: `src/context.rs`

### ชนิดข้อมูล (Type)

```rust
pub struct Context
```

ทำหน้าที่จัดเก็บตัวแปรใน `HashMap<String, Value>` ส่วนตัว

### `Context::new`

เส้นทางการนำเข้า (Import path): `formula_engine::Context::new`

```rust
pub fn new() -> Self
```

สร้างบริบทใหม่ที่ว่างเปล่า

### `Context::set`

```rust
pub fn set(&mut self, name: &str, value: Value)
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `name` | `&str` | — | ชื่อตัวแปรที่จัดเก็บในบริบท |
| `value` | `Value` | — | ค่ารันไทม์ที่เชื่อมโยงกับตัวแปร |

### `Context::get`

```rust
pub fn get(&self, name: &str) -> Option<&Value>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `name` | `&str` | — | ชื่อตัวแปรที่ต้องการดึงข้อมูล |

ตัวอย่าง:

```rust
use bl1z::{Context, Value};

let mut ctx = Context::new();
ctx.set("enabled", Value::Bool(true));
assert_eq!(ctx.get("enabled"), Some(&Value::Bool(true)));
```

## `formula_engine::functions::BuiltinFunction`

ไฟล์ต้นฉบับ: `src/functions.rs`

```rust
pub struct BuiltinFunction {
    pub name: String,
    pub arity: usize,
    pub call: fn(&[Value]) -> Result<Value, FormulaError>,
}
```

นี่คือหน่วยพื้นฐานสำหรับการลงทะเบียนใน `FunctionRegistry`

ตัวอย่าง:

```rust
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::Value;

fn double(args: &[Value]) -> Result<Value, FormulaError> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n * 2.0)),
        _ => Err(FormulaError::new(ErrorKind::TypeError, "E401", "Expected number", None)),
    }
}

let func = BuiltinFunction {
    name: "double".to_string(),
    arity: 1,
    call: double,
};
```

## `formula_engine::FunctionRegistry`

ไฟล์ต้นฉบับ: `src/functions.rs`

### ชนิดข้อมูล (Type)

```rust
pub struct FunctionRegistry
```

### `FunctionRegistry::new`

```rust
pub fn new() -> Self
```

สร้างทะเบียนฟังก์ชันใหม่ที่ว่างเปล่า

### `FunctionRegistry::register`

```rust
pub fn register(&mut self, func: BuiltinFunction)
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `func` | `BuiltinFunction` | — | รายการฟังก์ชันที่จะเพิ่มหรือแทนที่ |

### `FunctionRegistry::find`

```rust
pub fn find(&self, name: &str) -> Option<&BuiltinFunction>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `name` | `&str` | — | ชื่อฟังก์ชันที่ต้องการค้นหา |

ตัวอย่าง:

```rust
use formula_engine::{FunctionRegistry, builtins};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
assert!(registry.find("len").is_some());
```

## `formula_engine::builtins::register_all`

ไฟล์ต้นฉบับ: `src/builtins/mod.rs`

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::register_all`

```rust
pub fn register_all(registry: &mut FunctionRegistry)
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `registry` | `&mut FunctionRegistry` | — | ทะเบียนเป้าหมายสำหรับฟังก์ชันภายในมาตรฐานทั้งหมด |

ลงทะเบียนฟังก์ชันภายในจาก:

- `formula_engine::builtins::string`
- `formula_engine::builtins::math`
- `formula_engine::builtins::logic`
- `formula_engine::builtins::collection`
- `formula_engine::builtins::date`

## รูปแบบการใช้งานทั่วไป: ฟังก์ชันภายในร่วมกับฟังก์ชันกำหนดเอง

```rust
use formula_engine::builtins;
use formula_engine::error::{ErrorKind, FormulaError};
use formula_engine::functions::BuiltinFunction;
use formula_engine::{FunctionRegistry, Value};

fn is_even(args: &[Value]) -> Result<Value, FormulaError> {
    match args.first() {
        Some(Value::Number(n)) if n.fract() == 0.0 => Ok(Value::Bool((*n as i64) % 2 == 0)),
        _ => Err(FormulaError::new(ErrorKind::TypeError, "E401", "Expected integer", None)),
    }
}

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);
registry.register(BuiltinFunction {
    name: "is_even".to_string(),
    arity: 1,
    call: is_even,
});
```

ดูที่ [ฟังก์ชันภายใน (Built-Ins)](/docs/api-reference/builtins) สำหรับรายการมาตรฐานทั้งหมด
