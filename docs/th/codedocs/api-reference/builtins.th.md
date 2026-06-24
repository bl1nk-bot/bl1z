---
title: "ฟังก์ชันภายใน (Built-Ins)"
description: "เอกสารอ้างอิงสำหรับทุกฟังก์ชันภายในที่ส่งออกมาจากโมดูล builtins"
---

ฟังก์ชันภายในถูกนิยามไว้ภายใต้ `src/builtins` และถูกส่งกลับมาในรูปแบบค่าคงที่ `BuiltinFunction` ซึ่งจะยังไม่สามารถใช้งานได้จนกว่าจะมีการเรียกใช้ `bl1z::builtins::register_all(&mut registry)`

## ฟังก์ชันจัดการข้อความ (String Built-Ins)

เส้นทางการนำเข้า (Import path): `bl1z::builtins::string`

### โครงสร้างฟังก์ชัน (Signatures)

```rust
pub fn len() -> BuiltinFunction
pub fn upper() -> BuiltinFunction
pub fn lower() -> BuiltinFunction
pub fn contains() -> BuiltinFunction
pub fn starts_with() -> BuiltinFunction
pub fn ends_with() -> BuiltinFunction
pub fn trim() -> BuiltinFunction
pub fn trim_start() -> BuiltinFunction
pub fn trim_end() -> BuiltinFunction
pub fn split() -> BuiltinFunction
pub fn replace() -> BuiltinFunction
pub fn substring() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `len` | `1` | `String` หรือ `Array` | `Value::Number` | `src/builtins/string.rs` |
| `upper` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `lower` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `contains` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `starts_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `ends_with` | `2` | `String`, `String` | `Value::Bool` | `src/builtins/string.rs` |
| `trim` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `trim_start` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `trim_end` | `1` | `String` | `Value::String` | `src/builtins/string.rs` |
| `split` | `2` | `String`, `String` | `Value::Array` | `src/builtins/string.rs` |
| `replace` | `3` | `String`, `String`, `String` | `Value::String` | `src/builtins/string.rs` |
| `substring` | `3` | `String`, `Number`, `Number` | `Value::String` | `src/builtins/string.rs` |

ตัวอย่าง:

```rust
let result = bl1z::evaluate(
    &bl1z::parse(&bl1z::tokenize("upper(\"hello\")").unwrap()).unwrap(),
    &bl1z::Context::new(),
    &{
        let mut registry = bl1z::FunctionRegistry::new();
        bl1z::builtins::register_all(&mut registry);
        registry
    },
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"HELLO\")");
```

## ฟังก์ชันคณิตศาสตร์ (Math Built-Ins)

เส้นทางการนำเข้า (Import path): `bl1z::builtins::math`

```rust
pub fn abs() -> BuiltinFunction
pub fn pi() -> BuiltinFunction
pub fn round() -> BuiltinFunction
pub fn ceil() -> BuiltinFunction
pub fn floor() -> BuiltinFunction
pub fn sqrt() -> BuiltinFunction
pub fn pow() -> BuiltinFunction
pub fn sin() -> BuiltinFunction
pub fn cos() -> BuiltinFunction
pub fn tan() -> BuiltinFunction
pub fn random() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `abs` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `pi` | `0` | ไม่มี | `Value::Number` | `src/builtins/math.rs` |
| `round` | `2` | `Number`, `Number` | `Value::Number` | `src/builtins/math.rs` |
| `ceil` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `floor` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `sqrt` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `pow` | `2` | `Number`, `Number` | `Value::Number` | `src/builtins/math.rs` |
| `sin` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `cos` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `tan` | `1` | `Number` | `Value::Number` | `src/builtins/math.rs` |
| `random` | `0` | ไม่มี | `Value::Number` | `src/builtins/math.rs` |

## ฟังก์ชันตรรกะ (Logic Built-Ins)

เส้นทางการนำเข้า (Import path): `bl1z::builtins::logic`

```rust
pub fn if_fn() -> BuiltinFunction
```

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `if` | `3` | `Bool`, ข้อมูลใดๆ, ข้อมูลใดๆ | ค่าของสาขาที่เลือก | `src/builtins/logic.rs` |

`if_fn()` จะลงทะเบียนฟังก์ชันในชื่อ `if`

## ฟังก์ชันคอลเลกชัน (Collection Built-Ins)

เส้นทางการนำเข้า (Import path): `bl1z::builtins::collection`

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

## ฟังก์ชัน Higher-Order และ Set

เส้นทางการนำเข้า (Import path): `bl1z::builtins::higher_order` และ `bl1z::builtins::sets`

| ชื่อฟังก์ชัน | จำนวนพารามิเตอร์ | ข้อมูลที่รับ | ข้อมูลที่ส่งคืน | ไฟล์ต้นฉบับ |
|---------------|-------|---------|---------|--------|
| `map` | `2` | `Array`, `Lambda` | `Value::Array` | `src/builtins/higher_order.rs` |
| `filter` | `2` | `Array`, `Lambda` | `Value::Array` | `src/builtins/higher_order.rs` |
| `reduce` | `3` | `Array`, `Lambda`, ค่าเริ่มต้น | ค่าใดก็ได้ | `src/builtins/higher_order.rs` |
| `sort` | variable | `Array`, optional key lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `sort_with` | `2` | `Array`, comparator lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `unique` | variable | `Array`, optional key lambda | `Value::Array` | `src/builtins/higher_order.rs` |
| `group_by` | `2` | `Array`, key lambda | `Value::Map` | `src/builtins/higher_order.rs` |
| `set` | `1` | `Array` | `Value::Set` | `src/builtins/higher_order.rs` |
| `range` | variable | ขอบเขตตัวเลข | `Value::Range` | `src/builtins/higher_order.rs` |
| `range_to_array` | `1` | `Range` | `Value::Array` | `src/builtins/higher_order.rs` |
| `set_union` | `2` | ค่าแบบ `Set` | `Value::Set` | `src/builtins/sets.rs` |
| `set_intersection` | `2` | ค่าแบบ `Set` | `Value::Set` | `src/builtins/sets.rs` |
| `set_difference` | `2` | ค่าแบบ `Set` | `Value::Set` | `src/builtins/sets.rs` |
| `set_in` | `2` | ค่าใดก็ได้, ค่าแบบ `Set` | `Value::Bool` | `src/builtins/sets.rs` |

ตัวอย่าง:

```rust
let source = "join([\"north\", \"south\"], \"/\")";
let mut registry = bl1z::FunctionRegistry::new();
bl1z::builtins::register_all(&mut registry);
let result = bl1z::evaluate(
    &bl1z::parse(&bl1z::tokenize(source).unwrap()).unwrap(),
    &bl1z::Context::new(),
    &registry,
)
.unwrap();
assert_eq!(format!("{result:?}"), "String(\"north/south\")");
```

## ฟังก์ชันวันที่ (Date Built-Ins)

เส้นทางการนำเข้า (Import path): `bl1z::builtins::date`

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
| `now` | `0` | ไม่มี | `Value::DateTime` | `src/builtins/date.rs` |
| `date_add` | `2` | `DateTime` หรือข้อความวันที่, จำนวนวัน | `Value::DateTime` | `src/builtins/date.rs` |
| `date` | `3` | ตัวเลข ปี, เดือน, วัน | `Value::DateTime` | `src/builtins/date.rs` |
| `year` | `1` | `DateTime` หรือข้อความวันที่ | `Value::Number` | `src/builtins/date.rs` |
| `month` | `1` | `DateTime` หรือข้อความวันที่ | `Value::Number` | `src/builtins/date.rs` |
| `day` | `1` | `DateTime` หรือข้อความวันที่ | `Value::Number` | `src/builtins/date.rs` |
| `date_diff` | `3` | ค่าประเภทวันเวลา, ค่าประเภทวันเวลา, ข้อความหน่วย | `Value::Number` | `src/builtins/date.rs` |

พฤติกรรมที่สำคัญ:

- `date_add` และ `date_diff` รับได้ทั้ง `Value::DateTime` แบบ native และสตริง เพื่อรองรับการใช้งานย้อนหลัง
- `date_diff` รองรับหน่วยเช่น วัน ชั่วโมง นาที เดือน และปี
- `now()` และ `date()` คืนค่าเป็นชนิดวันเวลาแบบ native แล้ว ไม่ใช่สตริง

## การใช้งานฟังก์ชันร่วมกัน (Combining Built-Ins)

```rust
use bl1z::builtins;
use bl1z::{Context, FunctionRegistry, evaluate, parse, tokenize};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let source = "if(count([1,2,3]) == 3, month(date_add(\"2023-01-01\", 31)), 0)";
let result = evaluate(&parse(&tokenize(source).unwrap()).unwrap(), &Context::new(), &registry).unwrap();
assert_eq!(format!("{result:?}"), "Number(2.0)");
```

ดูที่ [ระบบฟังก์ชัน (Function System)](/docs/function-registry) สำหรับรูปแบบการขยายระบบ และ [บริบทและฟังก์ชัน (Context and Functions)](/docs/api-reference/context-and-functions) สำหรับรายละเอียดของ registry
