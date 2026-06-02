# bl1z Rust Architecture (Updated at commit 6d2529a)

Rust implementation for building a **formula/calculation library** that grows incrementally, suitable for Notion-like bl1z.
Overall Status: **V1 Complete** (number, string, bool, null + operators + functions + context + error reporting)
Phase 6 (Advanced Features) not yet started. Phase 7 (Quality & Tooling) partially implemented.

---

## 1) System Goals

The system should achieve three main goals:

1. **Parse** formula text into internal structures ✅
2. **Evaluate** formulas to get values ✅
3. **Extend** easily add functions/data types/contexts ✅

---

## 2) System Scope

### In scope (V1 – Complete)
- Math expressions (+, -, *, /)
- Comparison (<, >, <=, >=)
- Equality (==, !=)
- Boolean logic (&&, ||, !)
- String operations (concatenation with +)
- Function calls (including nested calls)
- Variable / context lookup
- Error reporting (all layers)
- Extensible built-in functions (via registry)

### Out of scope (Initial Stage – No changes)
- Full compiler
- Heavy optimizations
- Fully complex static type system
- Full language user-defined functions
- Async evaluation
- Script engine-like sandbox execution

---

# High-Level Architecture (Current Status)

## Layer 1: Input Layer ✅
Receives formulas as strings, e.g.:
`if(score > 50, "pass", "fail")`

**Responsibilities:**
- Accepts text input.
- Sends to lexer/parser (via `tokenize` in `lexer.rs`).

---

## Layer 2: Lexing ✅
Converts string → token stream.

**Example tokens (Added Null, True, False, Array/Map from Phase 6):**
- Identifier, Number, String, LParen, RParen, Comma, LBracket, RBracket, LBrace, RBrace, Colon
- Operator: Plus, Minus, Star, Slash, Bang, AndAnd, OrOr, EqEq, NotEq, Lt, Gt, LtEq, GtEq
- Keyword: True, False, Null
- Eof

**Lexer Responsibilities (Implemented):**
- Trims whitespace.
- Reads literals (number, string with escapes).
- Separates operators (including &&, ||, <=, >=, !=, ==).
- Manages line/column positions to create spans for every token.
- Errors on unrecognized characters.

Implemented in `src/lexer.rs` using zero-cost `Chars` iterator.

---

## Layer 3: Parsing ✅
Converts tokens → AST.
AST is a tree of `SpannedExpr` (`Expr` + `Span`).

**Example Nodes (Expr enum):**
- Literal(Value) // Supports Number, String, Bool, Null, Array, Map
- Variable(String)
- UnaryExpr { op, expr } // op: Neg(-), Not(!)
- BinaryExpr { left, op, right } // op: +, -, *, /, ==, !=, <, >, <=, >=, &&, ||
- FunctionCall { name, args }
- Grouping(Box<SpannedExpr>)
- ArrayLiteral(Vec<SpannedExpr>) // [elem1, elem2, ...]
- MapLiteral(Vec<(String, SpannedExpr)>) // {key1: val1, key2: val2, ...}

**Parser Responsibilities (Implemented):**
- Uses recursive descent with generic `parse_left_associative_binary` for precedence.
- Creates AST with spans for every node.
- Precedence / Associativity rules:
  1. Parentheses
  2. Unary -, !
  3. *, /
  4. +, -
  5. Comparison (<, >, <=, >=)
  6. Equality (==, !=)
  7. Logical AND (&&)
  8. Logical OR (||)
- Supports function calls (identifier + ( args )) and nested calls.
- Reports syntax errors with spans.

Implemented in `src/parser.rs`.

---

## Layer 4: Semantic / Validation 🟡
Logical semantic checks (mostly at runtime).

**Current Validations:**
- Function existence in registry.
- Argument count (arity checks).
- Operator support for types (type validation in evaluator – produces `TypeError`).
- Variable existence in context (produces `ContextError`).

Currently using runtime validation (no static type checker yet) — sufficient for V1.

---

## Layer 5: Evaluation ✅
Walks the AST and returns a `Value`.

**Implemented Values (enum):**
- Number(f64)
- String(String)
- Bool(bool)
- Null
- (Phase 6: DateTime, Array, Object)

**Process:**
- Accepts `&SpannedExpr`, `&Context`, `&FunctionRegistry`.
- Recursive evaluation.
- Returns `Result<Value, FormulaError>`.
- Supports all operators + function calls + variable lookups.
- Checks for division by zero and type mismatches.

Implemented in `src/eval.rs`, `src/value.rs`.

---

## Layer 6: Context ✅
Runtime data store.

- `HashMap<String, Value>`
- `get(name)` returns `Option<Value>`
- `set(name, value)`

**Usage:**
- `score + 10` (when score exists in context).
- `if(done, ...)` (when done is a Bool in context).

Implemented in `src/context.rs`.

---

## Layer 7: Built-in Function Registry ✅
Stores functions with signatures and implementations.

**Implemented Functions (23 functions):**
- `if(cond, a, b)` – logic
- `len(text)`, `upper(text)`, `lower(text)`, `contains(text, pattern)`, `starts_with(text, pattern)`, `ends_with(text, pattern)` – string
- `abs(number)`, `min(a, b)`, `max(a, b)` – math
- `min(array)`, `max(array)`, `sum(array)`, `avg(array)`, `join(array, separator)`, `count(array)` – collection
- `now()`, `date_add(date_str, days)`, `date_diff(date1, date2)`, `year(date_str)`, `month(date_str)`, `day(date_str)` – date

Registry uses `HashMap<String, BuiltinFunction>`.
`BuiltinFunction` has name, arity, and `call` (returns `Result<Value, FormulaError>`).

Implemented in `src/functions.rs`, `src/builtins/`.

---

## Layer 8: Error System ✅
Manages errors at all layers.

**Error Types (ErrorKind):**
- LexError, ParseError, EvalError, TypeError, FunctionError, ContextError

Every error (`FormulaError`) consists of:
- `kind`: ErrorKind
- `code`: String (e.g., "E001", "E010")
- `message`: Thai description
- `span`: `Option<Span>` (source position)

**Display:**
- `diagnostics.rs` can show source snippets with position indicators.

Implemented in `src/error.rs`, `src/span.rs`, `src/diagnostics.rs`.

---

## Actual Rust File Structure

```text
src/
  lib.rs              # re-export, doc-tests
  lexer.rs             # tokenization + tests
  parser.rs            # recursive descent parser + tests
  ast.rs               # SpannedExpr, Expr, BinaryOp, UnaryOp
  value.rs             # Value enum
  eval.rs              # evaluator + helper functions
  context.rs           # Context struct
  functions.rs         # BuiltinFunction, FunctionRegistry
  error.rs             # FormulaError, ErrorKind
  span.rs              # Position, Span
  diagnostics.rs       # format_error with snippet
  builtins/
    mod.rs             # register_all
    string.rs          # len, upper, lower, contains, starts_with, ends_with
    math.rs            # abs, min, max
    logic.rs           # if_fn
    date.rs            # now, date_add, date_diff, year, month, day
```

---

## Actual Status Specifications

### A. Syntax Spec
Parser-supported grammar:

**Expression Types:**
- literals: Number, String, Bool (true, false), Null (null), Array ([elem1, elem2, ...]), Map ({key1: val1, key2: val2, ...})
- variables: identifiers (e.g., score)
- unary operators: - (Neg), ! (Not)
- binary operators: +, -, *, /, ==, !=, <, >, <=, >=, &&, ||
- function calls: identifier(expr, expr, ...)
- parentheses: (expr)

**Operator Precedence (Implemented):**
1. Parentheses
2. Unary -, !
3. *, /
4. +, -
5. Comparison (<, >, <=, >=)
6. Equality (==, !=)
7. Logical AND (&&)
8. Logical OR (||)

All binary operators are left-associative (except unary).

### B. Value Spec
Data types:

**Phase 1 (Implemented):**
- Number(f64)
- String(String)
- Bool(bool)
- Null

**Phase 6 (Implemented):**
- Array(Vec<Value>) ✅
- Map(HashMap<String, Value>) ✅
- DateTime (via ISO 8601 String) ✅

### C. Function Spec
Built-in functions:

**Phase 1 (Implemented – 4 functions):**
- `if(cond, a, b)`
- `len(text)`
- `upper(text)`
- `lower(text)`

**Phase 2 (Partially Implemented – 6 additional functions):**
- `contains(text, pattern)`
- `starts_with(text, pattern)`
- `ends_with(text, pattern)`
- `abs(number)`
- `min(a, b)`
- `max(a, b)`

**Not yet implemented:**
- `replace`
- `date` functions (now, date_add, ...)

### D. Error Spec
Every error has:
- `code`: string code (e.g., "E001")
- `message`: Thai explanation
- `span`: Position in source (optional)
- `category`: One of LexError, ParseError, EvalError, TypeError, FunctionError, ContextError

**Current Error Codes:**
- E001 – Unexpected character (lexer)
- E002 – Unexpected token (parser)
- E003 – Invalid number literal (parser)
- E004 – Unexpected token (parser, fallback)
- E005 – Unknown variable (context)
- E006 – Type mismatch (eval/builtins)
- E007 – Function not found (eval)
- E008 – Argument count mismatch (eval)
- E009 – Built-in function error (eval)
- E010 – Division by zero (eval)

### E. Context Spec
Variable resolution:
- Uses `HashMap<String, Value>` within `Context`.
- Resolves identifiers directly in context.
- Dot notation (`user.name`) deferred to Phase 6.
- No separation of variable types (local vs record) currently.

---

## Remaining Roadmap Summary

- **Phase 6**: arrays, objects, date/time, access chaining, additional functions
- **Phase 7**: benchmarks, snapshot tests, API docs (rustdoc), CI/CD

V1 achieves its goals within the defined scope.
