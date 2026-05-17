# Architecture Rust สำหรับ Formula Engine

โครงแบบสำหรับสร้าง **formula/calculation library** ด้วย Rust ให้เติบโตแบบค่อยเป็นค่อยไป เหมาะกับแนว Notion-like formula engine และ POE SDK

สถานะปัจจุบัน: **V2 พร้อมเริ่มดำเนินการ**

---

## 1) เป้าหมายของระบบ

1. **Parse** ข้อความสูตรเป็นโครงสร้างภายใน ✅
2. **Evaluate** สูตรให้ได้ค่า ✅
3. **Extend** เพิ่มฟังก์ชัน/ชนิดข้อมูล/บริบทได้ง่าย ✅
4. **Navigate** เข้าถึงข้อมูล nested ผ่าน dot/index notation 🚧 (Phase 8)
5. **Functional** รองรับ lambda, higher-order functions 🚧 (Phase 9)
6. **User-defined** ให้ผู้ใช้สร้างฟังก์ชันเองได้ 🚧 (Phase 10)
7. **Rich Types** มี DateTime/Duration แบบ native (ผ่าน `jiff`) 🚧 (Phase 11)
8. **Plugin SDK** เปิดให้ third-party ขยายความสามารถ 🚧 (Phase 13)

---

## 2) ขอบเขตของระบบ

### ✅ In scope (V1 – เสร็จแล้ว)
- นิพจน์คณิตศาสตร์, เปรียบเทียบ, logic
- String operations, function call, variable/context
- Error reporting ทุกชั้น, extensible built-in functions
- Built-in collection functions (sum, avg, min, max, count, join)
- Basic date functions (now, year, month, day, date_add, date_diff) ใช้ `jiff` ภายใน

### 🚧 v2 (กำลังพัฒนา)
- **Access chaining** (`obj.prop`, `arr[0]`)
- **Lambda expression** `(x) => x * 2`
- **Higher-order functions**: `map`, `filter`, `reduce`
- **User-defined function**: `fn name(params) = expression`
- **Native Date/Time/Duration** via `jiff`
- **Set, Range literals**
- **Serialization & caching**
- **Plugin SDK foundation** (Trait + Manager)

### ❌ Out of scope
- WASM sandboxing สำหรับ plugin
- JIT compilation
- Asynchronous evaluation
- Static type system ซับซ้อน
- Null-safe navigation operator (`?.`)

---

## 3) Architecture ระดับสูง (Extended)

### Layer 1: Input Layer ✅
### Layer 2: Lexing 🚧 (เพิ่ม token Dot)
### Layer 3: Parsing 🚧 (เพิ่ม postfix chain, lambda)
### Layer 4: Evaluation 🚧 (เพิ่ม property/index access, lambda call, UDF)
### Layer 5: Plugin SDK 🆕 (Phase 13)

---

## 4) AST Extensions (Session 2)

```rust
pub enum Expr {
    // V1 expressions...
    Literal(Value),
    Variable(String),
    UnaryExpr { op: UnaryOp, expr: Box<SpannedExpr> },
    BinaryExpr { left: Box<SpannedExpr>, op: BinaryOp, right: Box<SpannedExpr> },
    FunctionCall { name: String, args: Vec<SpannedExpr> },
    Grouping(Box<SpannedExpr>),

    // Phase 8: Property & Index Access
    PropertyAccess {
        object: Box<SpannedExpr>,
        property: String,
    },
    IndexAccess {
        object: Box<SpannedExpr>,
        index: Box<SpannedExpr>,
    },

    // Phase 9: Lambda
    LambdaExpr {
        params: Vec<String>,
        body: Box<SpannedExpr>,
    },

    // Phase 10: User-defined function
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<SpannedExpr>,
    },
}
```

---

5) Value Extensions (Jiff-based)

```rust
pub enum Value {
    // V1 values
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Map(HashMap<String, Value>),

    // Phase 11: Advanced (pure Rust, no C dependency)
    DateTime(jiff::Timestamp),   // Native timestamp
    Duration(jiff::Span),        // Time interval
    Set(BTreeSet<Value>),        // Unique collection (sorted)
    Range { start: i64, end: i64 },
}
```

---

6) Context & User Functions

```rust
pub struct Context {
    variables: HashMap<String, Value>,
    functions: HashMap<String, UserFunction>,  // Phase 10
}

pub struct UserFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<SpannedExpr>,
    pub metadata: FunctionMetadata,
}
```

---

7) Plugin SDK Foundation (Phase 13)

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn functions(&self) -> Vec<BuiltinFunction>;
    fn types(&self) -> Vec<CustomType>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self { /* ... */ }
    pub fn register(&mut self, plugin: Box<dyn Plugin>) { /* ... */ }
    pub fn merge_functions(&self, registry: &mut FunctionRegistry) { /* ... */ }
}
```

หมายเหตุ: WASM sandboxing, dynamic loading, security ไม่อยู่ใน Session 2

---

8) Syntax Extensions

Property & Index Access (Phase 8)

```
user.name
user.profile.email
data.items[0].price
matrix[0][1]
```

Lambda Expressions (Phase 9)

```
(x) => x * 2
(x, y) => x + y
(item) => item.price > 100
```

User-Defined Functions (Phase 10)

```
fn double(x) = x * 2
fn factorial(n) = if(n <= 1, 1, n * factorial(n - 1))
```

Literals (Phase 11)

```
# DateTime (jiff)
@2023-12-25T10:30:00Z
@2023-01-01

# Duration
1h30m
2d3h45m

# Range
1..10
'a'..'z'

# Set
{1, 2, 3}
```

---

9) Function Categories (Session 2 Additions)

Higher-Order (Phase 9)

map(array, lambda), filter(array, lambda), reduce(array, lambda, initial), sort(array, lambda), group_by(array, lambda), unique(array)

Date/Time Extended (Phase 11)

hour(dt), minute(dt), second(dt), weekday(dt), format_date(dt, fmt), parse_date(str), is_weekend(dt), date_between(dt, start, end)

String Extended

trim(s), split(s, sep), replace(s, old, new), substring(s, start, len)

Math Extended

round(n, d), ceil(n), floor(n), sqrt(n), pow(b, e), log(n, base), sin, cos, tan, pi(), random()

---

10) Performance & Caching

· Constant folding optimization pass (Phase 14)
· AST caching สำหรับ repeated formulas (Phase 12)
· Short-circuit evaluation for boolean ops
· Vectorized operations สำหรับ arrays ขนาดใหญ่
· Benchmark suite ด้วย criterion (Phase 14)

---

11) Error Handling Extensions

```rust
pub enum ErrorKind {
    // V1 errors...
    LexError, ParseError, EvalError, TypeError,
    FunctionError, ContextError,

    // Session 2
    PropertyNotFound,
    IndexOutOfBounds,
    RecursionLimitExceeded,
    LambdaArityMismatch,
    PluginError,
    SerializationError,
}
```

---

12) Testing & CI

· Unit tests สำหรับ AST parsing ทุกโหนดใหม่
· Integration tests สำหรับ higher-order functions กับ lambda
· Roundtrip serialization tests
· Fuzz testing สำหรับ parser ของ access chain
· CI: cargo fmt, cargo clippy, cargo test บน pure Rust toolchain (ไม่มี C dependency)

---

13) Migration from V1

· API เดิมทั้งหมดยังคงใช้ได้
· Value::DateTime และ Value::Duration เพิ่มเข้ามา แต่ไม่บังคับใช้
· ฟังก์ชัน date เดิมที่ return string ยังคงมีอยู่ (แต่ภายในใช้ native เพื่อความเร็ว)
· Plugin SDK เป็น opt-in ทั้งหมด

---

14) Future (Session 3+)

· JIT/Cranelift compilation
· WebAssembly-based plugin sandbox
· IDE Language Server Protocol (LSP)
· User-defined types
· Pattern matching
