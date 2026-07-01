# สถาปัตยกรรม Rust สำหรับ bl1z (Rust Architecture for bl1z)

การติดตั้งภาษา Rust สำหรับสร้าง **ไลบรารีสูตร/การคำนวณ** ที่เติบโตอย่างต่อเนื่อง เหมาะสำหรับ bl1z รูปแบบ Notion-like และ POE SDK

สถานะปัจจุบัน: **V2 พร้อมเริ่มต้น**

---

## 1) เป้าหมายของระบบ (System Goals)

1. **การแยกส่วน (Parse)** ข้อความสูตรเป็นโครงสร้างภายใน ✅
2. **การประเมินค่า (Evaluate)** สูตรเพื่อรับค่า ✅
3. **การขยาย (Extend)** เพิ่มฟังก์ชัน/ชนิดข้อมูล/บริบทได้ง่าย ✅
4. **การนำทาง (Navigate)** เข้าถึงข้อมูลที่ซ้อนกันผ่านสัญลักษณ์จุด/ดัชนี ✅ (Phase 8)
5. **ฟังก์ชันนัล (Functional)** รองรับ lambda และฟังก์ชันระดับสูง ✅ (Phase 9)
6. **นิยามโดยผู้ใช้ (User-defined)** อนุญาตให้ผู้ใช้สร้างฟังก์ชันเอง ✅ (Phase 10)
7. **ชนิดข้อมูลที่หลากหลาย (Rich Types)** รองรับ DateTime/Duration แบบดั้งเดิม (ผ่าน `jiff`) ✅ (Phase 11)
8. **Plugin SDK** เปิดสำหรับการขยายโดยบุคคลที่สาม ✅ (Phase 13)

---

## 2) ขอบเขตของระบบ (System Scope)

### ✅ อยู่ในขอบเขต (V1 – เสร็จสมบูรณ์)
- นิพจน์ทางคณิตศาสตร์, การเปรียบเทียบ และตรรกะ
- การดำเนินการกับข้อความ, การเรียกใช้ฟังก์ชัน, ตัวแปร/บริบท
- การรายงานข้อผิดพลาดในทุกระดับ, ฟังก์ชันภายในที่ขยายได้
- ฟังก์ชันคอลเลกชันภายใน (sum, avg, min, max, count, join)
- ฟังก์ชันวันที่พื้นฐาน (now, year, month, day, date_add, date_diff) โดยใช้ `jiff` ภายใน

### ✅ V2 (เสร็จสมบูรณ์/กำลังดำเนินการ)
- **การเชื่อมต่อการเข้าถึง (Access chaining)** (`obj.prop`, `arr[0]`) ✅
- **นิพจน์ Lambda (Lambda expression)** `(x) => x * 2` ✅
- **ฟังก์ชันระดับสูง (Higher-order functions)**: `map`, `filter`, `reduce` ✅
- **ฟังก์ชันนิยามโดยผู้ใช้ (User-defined function)**: `fn name(params) = expression` ✅
- **โครงสร้างพื้นฐาน Plugin SDK** (Trait + Manager) ✅
- **การแปลงข้อมูลและการทำแคช (Serialization & caching)** ✅
- **ชนิดข้อมูลขั้นสูง (Advanced Data Types)**: `DateTime`, `Duration`, `Set`, `Range` 🔄 (อยู่ระหว่างการปรับปรุงโครงสร้าง)
- **ส่วนขยายคณิตศาสตร์และข้อความ** ✅

### ❌ นอกเหนือขอบเขต
- การทำ WASM sandboxing สำหรับปลั๊กอิน
- การทำ JIT compilation
- การประเมินค่าแบบอะซิงโครนัส (Asynchronous evaluation)
- ระบบชนิดข้อมูลสถิตที่ซับซ้อน (Complex static type system)
- ตัวดำเนินการนำทางแบบปลอดภัยต่อค่าว่าง (Null-safe navigation operator `?.`)

---

## 3) สถาปัตยกรรมระดับสูง (High-Level Architecture - Extended)

### ชั้นที่ 1: ชั้นข้อมูลนำเข้า (Input Layer) ✅
### ชั้นที่ 2: การวิเคราะห์คำศัพท์ (Lexing) ✅ (เพิ่มโทเค็นจุด)
### ชั้นที่ 3: การวิเคราะห์ไวยากรณ์ (Parsing) ✅ (เพิ่ม postfix chain, lambda)
### ชั้นที่ 4: การประเมินค่า (Evaluation) ✅ (เพิ่มการเข้าถึงคุณสมบัติ/ดัชนี, การเรียก lambda, UDF)
### ชั้นที่ 5: Plugin SDK 🆕 (Phase 13)

---

## 4) ส่วนขยาย AST (Session 2)

```rust
pub enum Expr {
    // นิพจน์จาก V1...
    Literal(Value),
    Variable(String),
    UnaryExpr { op: UnaryOp, expr: Box<SpannedExpr> },
    BinaryExpr { left: Box<SpannedExpr>, op: BinaryOp, right: Box<SpannedExpr> },
    FunctionCall { name: String, args: Vec<SpannedExpr> },
    Grouping(Box<SpannedExpr>),

    // Phase 8: การเข้าถึงคุณสมบัติและดัชนี (Property & Index Access)
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

    // Phase 10: ฟังก์ชันนิยามโดยผู้ใช้ (User-defined function)
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<SpannedExpr>,
    },
}
```

---

5) ส่วนขยายคุณค่า (อ้างอิง Jiff)

```rust
pub enum Value {
    // ค่าจาก V1
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Map(HashMap<String, Value>),

    // Phase 11: ขั้นสูง (pure Rust, ไม่พึ่งพาภาษา C)
    DateTime(jiff::Timestamp),   // Timestamp แบบดั้งเดิม
    Duration(jiff::Span),        // ช่วงเวลา
    Set(BTreeSet<Value>),        // คอลเลกชันที่ไม่ซ้ำกัน (เรียงลำดับ)
    Range { start: i64, end: i64 },
}
```

---

6) บริบทและฟังก์ชันของผู้ใช้ (Context & User Functions)

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

7) โครงสร้างพื้นฐาน Plugin SDK (Phase 13)

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

หมายเหตุ: WASM sandboxing, dynamic loading และ security ไม่รวมอยู่ใน Session 2

---

8) ส่วนขยายไวยากรณ์ (Syntax Extensions)

การเข้าถึงคุณสมบัติและดัชนี (Phase 8)

```
user.name
user.profile.email
data.items[0].price
matrix[0][1]
```

นิพจน์ Lambda (Phase 9)

```
(x) => x * 2
(x, y) => x + y
(item) => item.price > 100
```

ฟังก์ชันนิยามโดยผู้ใช้ (Phase 10)

```
fn double(x) = x * 2
fn factorial(n) = if(n <= 1, 1, n * factorial(n - 1))
```

ค่าคงที่ (Phase 11)

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

9) หมวดหมู่ฟังก์ชัน (ส่วนเพิ่มใน Session 2)

ระดับสูง (Higher-Order - Phase 9)

map(array, lambda), filter(array, lambda), reduce(array, lambda, initial), sort(array, lambda), group_by(array, lambda), unique(array)

วันที่/เวลา ส่วนขยาย (Phase 11)

hour(dt), minute(dt), second(dt), weekday(dt), format_date(dt, fmt), parse_date(str), is_weekend(dt), date_between(dt, start, end)

ข้อความ ส่วนขยาย

trim(s), split(s, sep), replace(s, old, new), substring(s, start, len)

คณิตศาสตร์ ส่วนขยาย

round(n, d), ceil(n), floor(n), sqrt(n), pow(b, e), log(n, base), sin, cos, tan, pi(), random()

---

10) ประสิทธิภาพและการทำแคช (Performance & Caching)

· ขั้นตอนการปรับแต่ง Constant folding (Phase 14)
· การทำ AST caching สำหรับสูตรที่ใช้ซ้ำ (Phase 12)
· การประเมินค่าแบบ Short-circuit สำหรับตัวดำเนินการบูลีน
· การดำเนินการแบบ Vectorized สำหรับอาร์เรย์ขนาดใหญ่
· ชุดการทดสอบประสิทธิภาพด้วย criterion (Phase 14)

---

12) การกู้คืนข้อผิดพลาด + ขีดจำกัดด้านความปลอดภัย (Phase 15)

```rust
pub struct EngineConfig {
    pub max_formula_length: usize,  // default: 10,000
    pub max_depth: usize,           // default: 100
    pub max_time_ms: Option<u64>,   // default: None
}

pub struct RecoveryResult {
    pub ast: Option<SpannedExpr>,
    pub errors: Vec<FormulaError>,
}
```

**`parse_with_recovery()`** - เก็บรวบรวมข้อผิดพลาดทั้งหมดแทนการยุติทันที ข้ามไปที่ semi-colon ถัดไปเมื่อเจอข้อผิดพลาด
**`evaluate_with_config()`** - บังคับใช้ `max_depth` และ `max_time_ms` ระหว่างการประเมินผล
**รหัสข้อผิดพลาด E901** - รหัสสำหรับผลลัพธ์การวิเคราะห์แบบกู้คืน

---

13) ส่วนขยายการจัดการข้อผิดพลาด (Error Handling Extensions)

```rust
pub enum ErrorKind {
    // ข้อผิดพลาดจาก V1...
    LexError, ParseError, EvalError, TypeError,
    FunctionError, ContextError,

    // Session 2
    PropertyNotFound,
    IndexOutOfBounds,
    RecursionLimitExceeded,
    LambdaArityMismatch,
    PluginError,
    SerializationError,
    RecoveryError, // E901
}
```

---

14) การทดสอบและ CI (Testing & CI)

· การทดสอบระดับหน่วย (Unit tests) สำหรับการแยกส่วน AST ทุกโหนดใหม่
· การทดสอบระดับรวม (Integration tests) สำหรับฟังก์ชันระดับสูงร่วมกับ lambda
· การทดสอบการแปลงข้อมูลแบบไป-กลับ (Roundtrip serialization tests)
· การทดสอบแบบ Fuzz สำหรับการแยกส่วนลำดับชั้นการเข้าถึง (access chain parser)
· CI: cargo fmt, cargo clippy, cargo test บน pure Rust toolchain (ไม่มีการพึ่งพาภาษา C)

---

15) การย้ายข้อมูลจาก V1 (Migration from V1)

· API เดิมทั้งหมดยังคงใช้งานได้
· เพิ่ม Value::DateTime และ Value::Duration เข้ามา แต่ไม่บังคับใช้งาน
· ฟังก์ชันวันที่เดิมที่คืนค่าเป็นข้อความยังคงมีอยู่ (แต่ภายในใช้ชนิดข้อมูลดั้งเดิมเพื่อความเร็ว)
· Plugin SDK เป็นแบบเลือกใช้งาน (opt-in) ทั้งหมด

---

16) อนาคต (Session 3+)

· การคอมไพล์แบบ JIT/Cranelift
· ปลั๊กอินแซนด์บ็อกซ์บนพื้นฐาน WebAssembly
· IDE Language Server Protocol (LSP)
· ชนิดข้อมูลนิยามโดยผู้ใช้
· การทำ Pattern matching
