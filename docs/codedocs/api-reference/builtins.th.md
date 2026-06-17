---
title: "ฟังก์ชันภายใน (Built-Ins)"
description: "เอกสารอ้างอิงสำหรับทุกฟังก์ชันภายในที่ส่งออกมาจากโมดูล builtins"
---

ฟังก์ชันภายในถูกนิยามไว้ภายใต้ `src/builtins` และถูกส่งกลับมาในรูปแบบค่าคงที่ `BuiltinFunction` ซึ่งจะยังไม่สามารถใช้งานได้จนกว่าจะมีการเรียกใช้ `formula_engine::builtins::register_all(&mut registry)`

## ฟังก์ชันจัดการข้อความ (String Built-Ins)

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::string`

### โครงสร้างฟังก์ชัน (Signatures)

```rust
pub fn len() -> BuiltinFunction
pub fn upper() -> BuiltinFunction
pub fn lower() -> BuiltinFunction
pub fn contains() -> BuiltinFunction
pub fn starts_with() -> BuiltinFunction
pub fn ends_with() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `len` | `1` | `String` หรือ `Array` | `Value::Number` | `src/builtins/string.rs` |
| `upper` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `lower` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `contains` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `starts_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `ends_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |

ตัวอย่าง:

```rust
let result = formula_engine::evaluate(
    &formula_engine::parse(&formula_engine::tokenize("upper(\"hello\")").unwrap()).unwrap(),
    &formula_engine::Context::new(),
    &{
        let mut registry = formula_engine::FunctionRegistry::new();
        formula_engine::builtins::register_all(&mut registry);
        registry
    },
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"HELLO\")");
```

## ฟังก์ชันคณิตศาสตร์ (Math Built-Ins)

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::math`

```rust
pub fn abs() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `abs` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |

## ฟังก์ชันตรรกะ (Logic Built-Ins)

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::logic`

```rust
pub fn if_fn() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `if` | `3` | `Bool`, ข้อมูลใดๆ, ข้อมูลใดๆ | ค่าของสาขาที่เลือก | `src/builtins/logic.rs` |

`if_fn()` จะลงทะเบียนฟังก์ชันในชื่อ `if`

## ฟังก์ชันคอลเลกชัน (Collection Built-Ins)

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::collection`

```rust
pub fn sum() -> BuiltinFunction
pub fn avg() -> BuiltinFunction
pub fn min_arr() -> BuiltinFunction
pub fn max_arr() -> BuiltinFunction
pub fn join() -> BuiltinFunction
pub fn count() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `sum` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `avg` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `min` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `max` | `1` | `Array<Number>` | `Value::Number` | `src/builtins/collection.rs` |
| `join` | `2` | `Array<String>`, `String` | `Value::String` | `src/builtins/collection.rs` |
| `count` | `1` | `Array<any>` | `Value::Number` | `src/builtins/collection.rs` |

หมายเหตุ:

- `sum([])` จะคืนค่า `0`
- `avg([])`, `min([])` และ `max([])` จะคืนรหัสข้อผิดพลาด `E504`
- `count` ทำงานคล้ายกับ `len` สำหรับอาร์เรย์

ตัวอย่าง:

```rust
let source = "join([\"north\", \"south\"], \"/\")";
let mut registry = formula_engine::FunctionRegistry::new();
formula_engine::builtins::register_all(&mut registry);
let result = formula_engine::evaluate(
    &formula_engine::parse(&formula_engine::tokenize(source).unwrap()).unwrap(),
    &formula_engine::Context::new(),
    &registry,
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"north/south\")");
```

## ฟังก์ชันวันที่ (Date Built-Ins)

เส้นทางการนำเข้า (Import path): `formula_engine::builtins::date`

```rust
pub fn now() -> BuiltinFunction
pub fn date_add() -> BuiltinFunction
pub fn date() -> BuiltinFunction
pub fn year() -> BuiltinFunction
pub fn month() -> BuiltinFunction
pub fn day() -> BuiltinFunction
pub fn date_diff() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `now` | `0` | ไม่มี | ข้อความประทับเวลาปัจจุบัน | `src/builtins/date.rs` |
| `date_add` | `2` | ข้อความวันที่, จำนวนวัน | ข้อความวันที่ | `src/builtins/date.rs` |
| `date` | `3` | ตัวเลข ปี, เดือน, วัน | ข้อความวันที่ | `src/builtins/date.rs` |
| `year` | `1` | ข้อความวันที่หรือประทับเวลา | `Value::Number` | `src/builtins/date.rs` |
| `month` | `1` | ข้อความวันที่หรือประทับเวลา | `Value::Number` | `src/builtins/date.rs` |
| `day` | `1` | ข้อความวันที่หรือประทับเวลา | `Value::Number` | `src/builtins/date.rs` |
| `date_diff` | `3` | ข้อความวันที่, ข้อความวันที่, ข้อความหน่วย | `Value::Number` | `src/builtins/date.rs` |

พฤติกรรมที่สำคัญ:

- `date_add` จะวิเคราะห์วันที่แบบ civil และเพิ่มจำนวนวันปฏิทินโดยใช้ `jiff`
- `year`, `month` และ `day` รองรับทั้งข้อความวันที่และข้อความประทับเวลา เนื่องจาก `parse_to_timestamp` จะพยายามวิเคราะห์ทั้งสองรูปแบบ
- `date_diff` ในปัจจุบันยังไม่ได้ใช้งานอาร์กิวเมนต์ที่สาม (`unit`) และจะคืนค่าเป็นจำนวนวันเสมอ

## การใช้งานฟังก์ชันร่วมกัน (Combining Built-Ins)

```rust
use formula_engine::builtins;
use formula_engine::{Context, FunctionRegistry, evaluate, parse, tokenize};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let source = "if(count([1,2,3]) == 3, month(date_add(\"2023-01-01\", 31)), 0)";
let result = evaluate(&parse(&tokenize(source).unwrap()).unwrap(), &Context::new(), &registry).unwrap();
assert_eq!(format!("{result:?}"), "Number(2.0)");
```

ดูที่ [ระบบฟังก์ชัน (Function System)](/docs/function-registry) สำหรับรูปแบบการขยายระบบ และ [บริบทและฟังก์ชัน (Context and Functions)](/docs/api-reference/context-and-functions) สำหรับรายละเอียดของ registry
