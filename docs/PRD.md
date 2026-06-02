# bl1z Technical Requirements Document

This document describes the technical specifications for the bl1z calculation library.

## Overview and Goals

> The bl1z system is an internal calculation library (similar to Notion formulas) with three main capabilities:

1. **Parse** – Convert formula text into AST format.
2. **Evaluate** – Calculate AST and return the result value.
3. **Extend** – Easily add new data types, functions, and contexts with minimal impact on other code.

## System Scope

### In Scope (V1)

- Math expressions (add, subtract, multiply, divide)
- Comparison (>, <, >=, <=, ==, !=)
- Logic (AND, OR, NOT)
- Basic string management (concatenation)
- Function calls
- Variable/context referencing
- Error reporting (lex, parse, eval)
- Registrable built-in functions

### Out of Scope (V1)

- Full compiler
- Advanced optimizations
- Fully complex static type system
- User-defined functions
- Async processing
- Sandbox execution

### High-Level Architecture

#### Layered System Architecture

| Layer | Name | Responsibility |
|-------|------|----------------|
| 1 | Input | Receives formula text e.g., `if(score > 50, "pass", "fail")` |
| 2 | Lexer | Converts text → Token Stream |
| 3 | Parser | Converts Token Stream → AST (Abstract Syntax Tree) |
| 4 | Semantic / Validation | Basic semantic checks (function existence, argument count, data types) |
| 5 | Evaluator | Walks AST and produces Value |
| 6 | Context | Stores runtime data (variables, record info, environment) |
| 7 | Function Registry | Stores built-in functions |
| 8 | Error System | Manages errors with spans (positions) |

## Component Specification

### 1. AST (ast.rs)

- Middle structure with no calculation logic.
- Consists of `enum Expr` (Literal, UnaryExpr, BinaryExpr, FunctionCall, VariableRef, Grouping).
- Separate `BinaryOp` and `UnaryOp`.

### 2. Lexer (lexer.rs)

- Receives text and generates tokens one by one.
- Token consists of `TokenKind` (Identifier, Number, String, LParen, RParen, Comma, Operator, Keyword).
- Stores position (line/column) for `Span`.
- Ignores whitespace, handles literals, separates operators.

### 3. Parser (parser.rs)

- Uses recursive descent with precedence table and associativity rules.
- Operator precedence:
  1. Parentheses `()`
  2. Unary `-`, `!`
  3. `*`, `/`
  4. `+`, `-`
  5. Comparison (`<`, `>`, `<=`, `>=`)
  6. Equality (`==`, `!=`)
  7. Logical AND
  8. Logical OR
- Supports function calls and variable references.
- Reports syntax errors with spans.
- Basic error recovery system (optional).

### 4. Value (value.rs)

- Data type returned by evaluator.
- `Enum Value`:
  - `Number(f64)`
  - `String(String)`
  - `Bool(bool)`
  - `Null`
  - (Phase 2: Array, DateTime, Object/Map)
- Uses `f64` initially; consider decimal type if high precision is required.

### 5. Context (context.rs)

- Stores runtime variables in `HashMap<String, Value>`.
- Functions `get(name) -> Option<Value>` and `set(name, value)`.
- Used for identifier resolution like `score`, `user.name` (nested lookup later).

### 6. Function Registry (functions.rs)

- Stores built-in functions in `HashMap<String, BuiltinFunction>`.
- `BuiltinFunction` has name, arity (argument count), and `call(args: &[Value]) -> Value`.
- Validates parameter count before calling.
- Add new functions without modifying the evaluator.

### 7. Evaluator (eval.rs)

- Function `eval(expr: &Expr, ctx: &Context, functions: &FunctionRegistry) -> Result<Value, EvalError>`.
- Recursive: matches `Expr` and processes by type.
- Returns error for invalid operations (e.g., division by zero, type mismatch).

### 8. Error System (error.rs, span.rs, diagnostics.rs)

- Every error consists of code, message, span, and category.
- Categories: LexError, ParseError, EvalError, TypeError, FunctionError, ContextError.
- `span.rs` stores `Span { start: Position, end: Position }` for position tracking.
- `diagnostics.rs` helps format error messages with context.

### 9. Built-in Functions (builtins/)

Grouped into modules:

- `string.rs`: len, upper, lower, contains, starts_with, ends_with, replace
- `math.rs`: abs, round, min, max, sqrt
- `logic.rs`: if, and, or, not
- `date.rs`: now, date_add, date_diff (added later)

#### Minimum V1 Functions

| Function | Description | Parameter Count |
|----------|-------------|-----------------|
| `if(cond, a, b)` | If cond is true, return a, else b | 3 |
| `len(text)` | String length | 1 |
| `upper(text)` | Convert to uppercase | 1 |
| `lower(text)` | Convert to lowercase | 1 |

## Syntax Specification (Basic Grammar)

```
expression → logical_or
logical_or → logical_and ('||' logical_and)*
logical_and → equality ('&&' equality)*
equality → comparison (('==' | '!=') comparison)*
comparison → term (('<' | '>' | '<=' | '>=') term)*
term → factor (('+' | '-') factor)*
factor → unary (('*' | '/') unary)*
unary → ('-' | '!')? primary
primary → NUMBER | STRING | IDENTIFIER | '(' expression ')' | function_call
function_call → IDENTIFIER '(' expression (',' expression)* ')'
```

## Error Specification

Every error must have:

- **code** (Identifier like E001)
- **message** (Explanation)
- **span** (Start-End position in text)
- **category** (Error group)

### Example Error Codes:

- E001 – UnexpectedToken
- E002 – UnknownIdentifier
- E003 – UnsupportedOperator
- E004 – DivisionByZero
- E005 – ArgumentCountMismatch
- E006 – TypeMismatch

## Recommended Rust File Structure

```
src/
  lib.rs
  lexer.rs
  parser.rs
  ast.rs
  value.rs
  eval.rs
  context.rs
  functions.rs
  error.rs
  span.rs
  diagnostics.rs
  builtins/
    mod.rs
    string.rs
    math.rs
    logic.rs
    date.rs
```

## Phased Development Roadmap

### Phase 0: Design & Scope Lock
- Define use case, syntax, value types, operators, built-ins, error format
- Document spec v1

### Phase 1: Lexer + AST + Parser
- Create tokenizer and parser
- Parse `1 + 2 * 3` into correct AST

### Phase 2: Basic Evaluator
- Implement Value and evaluator
- Process math, comparison, logic

### Phase 3: Function System
- Create registry, call `if`, `len`, `upper`

### Phase 4: Context Resolution
- Support variables from context

### Phase 5: Type Handling & Validation (Later)
- Validate data types before evaluation for clearer errors

### Phase 6: Advanced Features (Array, DateTime, Object, chaining)

### Phase 7: Quality & Tooling (tests, docs, benchmarks)

## Non-Functional Requirements (NFR)

- **Performance**: Response to hundreds of tokens within milliseconds.
- **Maintainability**: Add new functions without modifying core evaluator.
- **Error Reporting**: Identify exact positions.
- **Stability**: Never panic from abnormal input (use `Result` everywhere).