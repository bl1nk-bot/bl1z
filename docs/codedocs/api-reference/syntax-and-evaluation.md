---
title: "Syntax And Evaluation"
description: "Inspect tokens, AST types, spans, parser methods, runtime values, and the evaluator."
---

This page documents the language-facing core of the crate: tokenization, parsing, AST shapes, spans, runtime values, and evaluation. The relevant source files are `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/span.rs`, `src/value.rs`, and `src/eval.rs`.

## `formula_engine::lexer`

### `TokenKind`

Source: `src/lexer.rs`

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

Represents every token the lexer can emit.

### `Token`

```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}
```

### `tokenize`

Import path: `formula_engine::tokenize` or `formula_engine::lexer::tokenize`

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, FormulaError>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `source` | `&str` | — | Formula source text to tokenize |

Returns `Vec<Token>` ending with `TokenKind::Eof`.

Example:

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

Used for constructing source ranges manually when needed.

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

`MapLiteral` keys are identifiers parsed in `src/parser.rs`, not arbitrary strings.

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

Import path: `formula_engine::parse` or `formula_engine::parser::parse`

```rust
pub fn parse(tokens: &[Token]) -> Result<SpannedExpr, FormulaError>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `tokens` | `&[Token]` | — | Token slice returned by `tokenize` |

Example:

```rust
let tokens = formula_engine::tokenize("1 + 2 * 3").unwrap();
let ast = formula_engine::parse(&tokens).unwrap();
assert_eq!(format!("{:?}", ast.expr), "BinaryExpr { left: SpannedExpr { expr: Literal(Number(1.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 2 } } } }, op: Add, right: SpannedExpr { expr: BinaryExpr { left: SpannedExpr { expr: Literal(Number(2.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 5 }, end: Position { line: 1, column: 6 } } } }, op: Mul, right: SpannedExpr { expr: Literal(Number(3.0)), meta: ExprMeta { span: Span { start: Position { line: 1, column: 9 }, end: Position { line: 1, column: 10 } } } } }, meta: ExprMeta { span: Span { start: Position { line: 1, column: 5 }, end: Position { line: 1, column: 10 } } } } }");
```

In practice, you usually inspect or evaluate the AST rather than compare its debug string.

## `formula_engine::value`

### `Value`

Import path: `formula_engine::Value` or `formula_engine::value::Value`

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

This is the evaluator’s return type and the payload type stored in `Context`.

## `formula_engine::eval`

### `evaluate`

Import path: `formula_engine::evaluate` or `formula_engine::eval::evaluate`

```rust
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError>
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `expr` | `&SpannedExpr` | — | Parsed AST |
| `ctx` | `&Context` | — | Variable lookup source |
| `registry` | `&FunctionRegistry` | — | Available callable functions |

The evaluator is strict and eager:

- `+` accepts only `Number + Number` and `String + String`
- comparisons operate on numbers
- `&&` and `||` require booleans
- both sides of binary expressions are evaluated before the operator is applied
- function arguments are evaluated before the registered function is called

Example:

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

## Related Pages

- [Context and Functions](/docs/api-reference/context-and-functions)
- [Built-Ins](/docs/api-reference/builtins)
- [Diagnostics, Errors, and Profiling](/docs/api-reference/diagnostics-and-profiling)
