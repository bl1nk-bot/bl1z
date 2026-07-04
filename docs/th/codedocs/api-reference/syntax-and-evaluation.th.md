---
title: "ไวยากรณ์และการประเมินค่า (Syntax And Evaluation)"
description: "ตรวจสอบโทเค็น, ชนิดข้อมูล AST, ข้อมูลตำแหน่ง (spans), เมธอดของ parser, ค่าในขณะรันไทม์ และตัวประเมินค่า"
---

หน้านี้อธิบายรายละเอียดส่วนสำคัญของไลบรารีที่เกี่ยวข้องกับภาษา: การตัดคำ (tokenization), การวิเคราะห์ไวยากรณ์ (parsing), รูปแบบของ AST, ข้อมูลตำแหน่ง, ค่าในขณะรันไทม์ และการประเมินค่า ไฟล์ต้นฉบับที่เกี่ยวข้องคือ `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/span.rs`, `src/value.rs` และ `src/eval.rs`

## `formula_engine::lexer`

### `TokenKind`

ไฟล์ต้นฉบับ: `src/lexer.rs`

```rust
pub enum TokenKind {
    Identifier,
    Number,
    String,
    LParen,
    RParen,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    AndAnd,
    OrOr,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    True,
    False,
    Null,
    Dot,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
    Eof,
}
```

แทนค่าโทเค็นทุกประเภทที่ lexer สามารถสร้างออกมาได้

### `Token`

```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}
```

### `tokenize`

เส้นทางการนำเข้า (Import path): `formula_engine::tokenize` หรือ `formula_engine::lexer::tokenize`

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, FormulaError>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `source` | `&str` | — | ข้อความต้นฉบับของสูตรที่ต้องการตัดคำ |

คืนค่าเป็น `Vec<Token>` ซึ่งจะจบด้วย `TokenKind::Eof`

ตัวอย่าง:

```rust
let tokens = formula_engine::tokenize("user.score >= 80").unwrap();
assert_eq!(tokens[0].lexeme, "user");
```

## `formula_engine::span`

### `Position`

```rust
pub struct Position {
    pub line: usize,
    pub column: usize,
}
```

### `Span`

```rust
pub struct Span {
    pub start: Position,
    pub end: Position,
}
```

### `Span::new`

```rust
pub fn new(start: Position, end: Position) -> Self
```

ใช้สำหรับการสร้างช่วงตำแหน่งในซอร์สโค้ดด้วยตนเองเมื่อจำเป็น

## `formula_engine::ast`

### `BinaryOp`

```rust
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}
```

### `UnaryOp`

```rust
pub enum UnaryOp {
    Neg,
    Not,
}
```

### `ExprMeta`

```rust
pub struct ExprMeta {
    pub span: Span,
}
```

### `SpannedExpr`

```rust
pub struct SpannedExpr {
    pub expr: Expr,
    pub meta: ExprMeta,
}
```

### `SpannedExpr::new`

```rust
pub fn new(expr: Expr, span: Span) -> Self
```

### `Expr`

```rust
pub enum Expr {
    Literal(Value),
    Variable(String),
    UnaryExpr {
        op: UnaryOp,
        expr: Box<SpannedExpr>,
    },
    BinaryExpr {
        left: Box<SpannedExpr>,
        op: BinaryOp,
        right: Box<SpannedExpr>,
    },
    FunctionCall {
        name: String,
        args: Vec<SpannedExpr>,
    },
    Grouping(Box<SpannedExpr>),
    ArrayLiteral(Vec<SpannedExpr>),
    MapLiteral(Vec<(String, SpannedExpr)>),
}
```

คีย์ของ `MapLiteral` จะต้องเป็นตัวระบุ (identifiers) ที่ได้รับการวิเคราะห์ใน `src/parser.rs` ไม่ใช่ข้อความตามอำเภอใจ

## `formula_engine::parser`

### `Parser<'a>`

```rust
pub struct Parser<'a>
```

### `Parser::new`

```rust
pub fn new(tokens: &'a [Token]) -> Self
```

### `Parser::parse_expression`

```rust
pub fn parse_expression(&mut self) -> Result<SpannedExpr, FormulaError>
```

### `parse`

เส้นทางการนำเข้า (Import path): `formula_engine::parse` หรือ `formula_engine::parser::parse`

```rust
pub fn parse(tokens: &[Token]) -> Result<SpannedExpr, FormulaError>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `tokens` | `&[Token]` | — | ลำดับโทเค็นที่ส่งคืนจาก `tokenize` |

ตัวอย่าง:

```rust
let tokens = formula_engine::tokenize("1 + 2 * 3").unwrap();
let ast = formula_engine::parse(&tokens).unwrap();
assert_eq!(format!("{:?}", ast.expr), "BinaryExpr { left: SpannedExpr { expr: Literal(Number(1.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 2 } } } }, op: Add, right: SpannedExpr { expr: BinaryExpr { left: SpannedExpr { expr: Literal(Number(2.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 5 }, end: Position { line: 1, column: 6 } } } }, op: Mul, right: SpannedExpr { expr: Literal(Number(3.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 9 }, end: Position { line: 1, column: 10 } } } } }, meta: ExprMeta { span: Span { start: Position { line: 1, column: 5 }, end: Position { line: 1, column: 10 } } } } }");
```

ในทางปฏิบัติ คุณมักจะตรวจสอบหรือประเมินค่า AST แทนที่จะเปรียบเทียบจากข้อความ debug string

## `formula_engine::value`

### `Value`

เส้นทางการนำเข้า (Import path): `formula_engine::Value` หรือ `formula_engine::value::Value`

```rust
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Map(std::collections::HashMap<String, Value>),
}
```

นี่คือชนิดข้อมูลที่ส่งคืนจากตัวประเมินค่า และเป็นชนิดข้อมูลที่จัดเก็บใน `Context`

## `formula_engine::eval`

### `evaluate`

เส้นทางการนำเข้า (Import path): `formula_engine::evaluate` หรือ `formula_engine::eval::evaluate`

```rust
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError>
```

| พารามิเตอร์ | ชนิดข้อมูล | ค่าเริ่มต้น | คำอธิบาย |
|-----------|------|---------|-------------|
| `expr` | `&SpannedExpr` | — | AST ที่ผ่านการวิเคราะห์แล้ว |
| `ctx` | `&Context` | — | แหล่งข้อมูลสำหรับค้นหาตัวแปร |
| `registry` | `&FunctionRegistry` | — | ทะเบียนฟังก์ชันที่เรียกใช้งานได้ |

ตัวประเมินค่าทำงานแบบเข้มงวดและประมวลผลทันที (strict and eager):

- `+` รองรับเฉพาะ `Number + Number` และ `String + String`
- การเปรียบเทียบทำงานกับตัวเลขเท่านั้น
- `&&` และ `||` ต้องการค่าบูลีน
- ทั้งสองฝั่งของนิพจน์แบบไบนารีจะได้รับการประเมินค่าก่อนที่จะใช้ตัวดำเนินการ
- อาร์กิวเมนต์ของฟังก์ชันจะได้รับการประเมินค่าก่อนที่จะเรียกใช้ฟังก์ชันที่ลงทะเบียนไว้

ตัวอย่าง:

```rust
use formula_engine::builtins;
use formula_engine::{Context, FunctionRegistry, Value, evaluate, parse, tokenize};

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let mut ctx = Context::new();
ctx.set("score", Value::Number(95.0));

let ast = parse(&tokenize("if(score > 90, \"gold\", \"silver\")").unwrap()).unwrap();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(format!("{result:?}"), "String(\"gold\")");
```

## หน้าที่เกี่ยวข้อง (Related Pages)

- [บริบทและฟังก์ชัน (Context and Functions)](/docs/api-reference/context-and-functions)
- [ฟังก์ชันภายใน (Built-Ins)](/docs/api-reference/builtins)
- [การวินิจฉัย, ข้อผิดพลาด และการวิเคราะห์ประสิทธิภาพ (Diagnostics, Errors, and Profiling)](/docs/api-reference/diagnostics-and-profiling)
