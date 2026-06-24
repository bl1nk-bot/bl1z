# Rust Architecture for bl1z

Rust implementation for building a **formula/calculation library** that grows incrementally, suitable for Notion-like bl1z and POE SDK.

Current status: **V2 Ready to Start**

---

## 1) System Goals

1. **Parse** formula text into internal structure ✅
2. **Evaluate** formula to get value ✅
3. **Extend** easily add functions/data types/context ✅
4. **Navigate** access nested data via dot/index notation 🚧 (Phase 8)
5. **Functional** support lambda, higher-order functions 🚧 (Phase 9)
6. **User-defined** allow users to create functions 🚧 (Phase 10)
7. **Rich Types** native DateTime/Duration (via `jiff`) 🚧 (Phase 11)
8. **Plugin SDK** open for third-party expansion 🚧 (Phase 13)

---

## 2) System Scope

### ✅ In scope (V1 – Complete)
- Math, comparison, and logic expressions
- String operations, function call, variable/context
- Error reporting at every layer, extensible built-in functions
- Built-in collection functions (sum, avg, min, max, count, join)
- Basic date functions (now, year, month, day, date_add, date_diff) using internal `jiff`

### ✅ V2 (Complete/In Progress)
- **Access chaining** (`obj.prop`, `arr[0]`) ✅
- **Lambda expression** `(x) => x * 2` ✅
- **Higher-order functions**: `map`, `filter`, `reduce` ✅
- **User-defined function**: `fn name(params) = expression` ✅
- **Plugin SDK foundation** (Trait + Manager) ✅
- **Serialization & caching** ✅
- **Advanced Data Types**: `DateTime`, `Duration`, `Set`, `Range` 🔄 (Refactor pending)
- **Math + String extensions** ✅

### ❌ Out of scope
- Asynchronous evaluation
- Complex static type system
- Null-safe navigation operator (`?.`)

---

## 3) High-Level Architecture (Extended)

### Layer 1: Input Layer ✅
### Layer 2: Lexing 🚧 (Added Dot token)
### Layer 3: Parsing 🚧 (Added postfix chain, lambda)
### Layer 4: Evaluation 🚧 (Added property/index access, lambda call, UDF)
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

หมายเหตุ: Dynamic loading, security ไม่อยู่ใน Session 2

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

11) Error Recovery + Security Limits (Phase 15)

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

**`parse_with_recovery()`** - Collects all parse errors instead of fail-fast, skipping to next semicolon on error.
**`evaluate_with_config()`** - Enforces `max_depth` and `max_time_ms` limits during evaluation.
**Error Code E901** - Recovery error code for partial parse results.

---

12) Error Handling Extensions

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
    RecoveryError, // E901
}
```

---

13) Testing & CI

· Unit tests สำหรับ AST parsing ทุกโหนดใหม่
· Integration tests สำหรับ higher-order functions กับ lambda
· Roundtrip serialization tests
· Fuzz testing สำหรับ parser ของ access chain
· CI: cargo fmt, cargo clippy, cargo test บน pure Rust toolchain (ไม่มี C dependency)

---

14) Migration from V1

· API เดิมทั้งหมดยังคงใช้ได้
· Value::DateTime และ Value::Duration เพิ่มเข้ามา แต่ไม่บังคับใช้
· ฟังก์ชัน date เดิมที่ return string ยังคงมีอยู่ (แต่ภายในใช้ native เพื่อความเร็ว)
· Plugin SDK เป็น opt-in ทั้งหมด

---

15) Future Phase 16-20 (Session 3+)

**Phase 16) JIT/Cranelift Compilation**

อ้างอิง: CapyScheme compilation pipeline

**Pipeline:**
```
Formula AST
    ↓
Lowered IR (Postfix + optimized)
    ↓
Cranelift IR Generation
    ↓
Register Allocation (regalloc2)
    ↓
Machine Code (x86-64/ARM64/WASM)
```

**API:**
```rust
pub struct JITCompiler {
    isa: isa::TargetIsa,
}

impl JITCompiler {
    pub fn compile(&self, ast: &SpannedExpr) -> Result<CompiledFunction, CompileError> {
        // Lower AST to Cranelift IR
    }
}
```

**Phase 17) WebAssembly Plugin Sandbox**

อ้างอิง: Wasmtime fuel consumption

```rust
pub struct WasmPlugin {
    engine: Engine,
    module: Module,
    store: Store<()>,
    fuel: u64,
}

impl WasmPlugin {
    pub fn new(wasm_bytes: &[u8], fuel_limit: u64) -> Result<Self, PluginError> {
        config.consume_fuel(true);
    }
}
```

**Phase 18) Language Server Protocol (LSP)**

อ้างอิง: tower-lsp

- `textDocument/completion`, `textDocument/hover`, `textDocument/publishDiagnostics`
- `textDocument/semanticTokens/full`, `textDocument/definition`, `textDocument/signatureHelp`

**Phase 19) User-Defined Types**

```
type Person {
    name: string,
    age: number,
    address: Address
}
```

**Phase 20) Pattern Matching**

```
match x {
    n if n > 100 => "big",
    n => "small"
}

match arr {
    [first, ..rest] => first,
    [] => null
}
```
