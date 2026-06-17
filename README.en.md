# bl1z

| Build | Quality | Security | License |
|:---:|:---:|:---:|:---:|
| [![CI](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml) | [![CodeQL](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml) | [![CI Failure Handler](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml) | [![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE) |

[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.2.0-green.svg)](https://crates.io/crates/bl1z)
[![Documentation](https://docs.rs/bl1z/badge.svg)](https://docs.rs/bl1z)

## 📖 Overview

**bl1z** is a library for parsing and evaluating Notion-like mathematical and logical formulas, written in Rust. It is designed for high flexibility, allowing easy expansion of functions and data types through a registry system.

### ✨ Key Features

- **Lexer & Parser**: Converts formula text into an AST (Abstract Syntax Tree) with span information for precise error reporting.
- **Evaluator**: Evaluates the AST, supporting 6 basic data types:
  - `Number`
  - `String`
  - `Bool`
  - `Null`
  - `Array`
  - `Map` (dictionary-like structures)
- **Function System**: Extensible function system supporting built-in functions and allowing users to add custom ones.
- **Context Support**: Supports variables and external value references.
- **Error Reporting**: Detailed error reporting with line and column positions.

---

## 🚀 Installation

Add this dependency to your `Cargo.toml` file:

```toml
[dependencies]
bl1z = "0.2.0"
```

Or to use from local source code:

```toml
[dependencies]
bl1z = { path = "./path/to/bl1z" }
```

---

## 📖 Usage

### Basic Example

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry};
use bl1z::builtins;

// Create registry with base functions
let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

// Parse and evaluate formula
let tokens = tokenize("1 + 2 * 3").unwrap();
let ast = parse(&tokens).unwrap();
let ctx = Context::new();
let result = evaluate(&ast, &ctx, &registry).unwrap();

println!("Result: {:?}", result); // Number(7.0)
```

### Using Built-in Functions

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry};
use bl1z::builtins;

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

// Example: if function
let tokens = tokenize("if(true, \"pass\", \"fail\")").unwrap();
let ast = parse(&tokens).unwrap();
let ctx = Context::new();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::String("pass".to_string()));

// Example: string function
let tokens = tokenize("len(\"hello\")").unwrap();
let ast = parse(&tokens).unwrap();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::Number(5.0));
```

### Using Variables (Context)

```rust
use bl1z::{tokenize, parse, evaluate, Context, FunctionRegistry, Value};
use bl1z::builtins;

let mut registry = FunctionRegistry::new();
builtins::register_all(&mut registry);

let mut ctx = Context::new();
ctx.set_variable("score", Value::Number(85.0));
ctx.set_variable("name", Value::String("John".to_string()));

let tokens = tokenize("if(score > 50, name, \"nobody\")").unwrap();
let ast = parse(&tokens).unwrap();
let result = evaluate(&ast, &ctx, &registry).unwrap();
assert_eq!(result, bl1z::Value::String("John".to_string()));
```

---

## 📋 Supported Syntax

### Operators

| Category | Operators | Description |
|----------|-------------|----------|
| Math | `+`, `-`, `*`, `/` | Add, Subtract, Multiply, Divide |
| Comparison | `<`, `>`, `<=`, `>=` | Value Comparison |
| Equality | `==`, `!=` | Equality Check |
| Logic | `&&`, `||`, `!` | AND, OR, NOT |
| Unary | `-`, `!` | Negation, Logical NOT |

### Data Types

- **Number**: Floating point numbers e.g., `1`, `3.14`, `-5`
- **String**: Quoted text e.g., `"hello"`, `"test"`
- **Bool**: Boolean values `true` or `false`
- **Null**: Empty value `null`
- **Array**: Arrays e.g., `[1, 2, 3]`, `["a", "b"]`, `[[1, 2], [3, 4]]`
- **Map**: Maps/Dictionaries e.g., `{name: "John", age: 30}`, `{key: value}`

### Built-in Functions

#### String Functions
| Function | Description | Example |
|----------|----------|----------|
| `len(str)` | Get string length | `len("hello")` → `5` |
| `upper(str)` | Convert to uppercase | `upper("abc")` → `"ABC"` |
| `lower(str)` | Convert to lowercase | `lower("ABC")` → `"abc"` |
| `contains(str, substr)` | Check for substring | `contains("hello", "ell")` → `true` |
| `starts_with(str, prefix)` | Check prefix | `starts_with("hello", "he")` → `true` |
| `ends_with(str, suffix)` | Check suffix | `ends_with("hello", "lo")` → `true` |

#### Math Functions
| Function | Description | Example |
|----------|----------|----------|
| `abs(num)` | Absolute value | `abs(-5)` → `5` |
| `min(a, b)` | Minimum of two values | `min(3, 1)` → `1` |
| `max(a, b)` | Maximum of two values | `max(3, 1)` → `3` |

#### Logic Functions
| Function | Description | Example |
|----------|----------|----------|
| `if(condition, true_val, false_val)` | Conditional logic | `if(true, 1, 0)` → `1` |

#### Collection Functions
| Function | Description | Example |
|----------|----------|----------|
| `sum(arr)` | Array sum | `sum([1, 2, 3])` → `6` |
| `avg(arr)` | Array average | `avg([1, 2, 3])` → `2` |
| `min(arr)` | Minimum in array | `min([1, 2, 3])` → `1` |
| `max(arr)` | Maximum in array | `max([1, 2, 3])` → `3` |
| `count(arr)` | Count elements | `count([1, 2, 3])` → `3` |
| `join(arr, sep)` | Join array into string | `join(["a","b"], ",")` → `"a,b"` |

#### Date Functions
| Function | Description | Example |
|----------|----------|----------|
| `now()` | Current date and time (ISO 8601) | `now()` → `"2023-12-01T12:00:00Z"` |
| `date_add(date, days)` | Add days to date | `date_add("2023-01-01", 5)` → `"2023-01-06T00:00:00Z"` |
| `date_diff(date1, date2)` | Days between dates | `date_diff("2023-01-05", "2023-01-01")` → `4` |
| `year(date)` | Extract year | `year("2023-05-15")` → `2023` |
| `month(date)` | Extract month | `month("2023-05-15")` → `5` |
| `day(date)` | Extract day | `day("2023-05-15")` → `15` |

---

## 🏗️ Architecture

bl1z uses a layered architecture:

```
┌─────────────────┐
│   Input Layer   │  ← Formula: "1 + 2 * if(x > 0, 3, 4)"
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Lexer       │  → Token Stream with Span
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Parser      │  → AST (Abstract Syntax Tree)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Evaluator    │  → Value (Number/String/Bool/Null)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Result      │
└─────────────────┘
```

### Project Structure

```
bl1z/
├── src/
│   ├── lib.rs           # Main entry point and re-exports
│   ├── lexer.rs         # Lexer: string → tokens
│   ├── parser.rs        # Parser: tokens → AST
│   ├── ast.rs           # AST node definitions
│   ├── eval.rs          # Evaluator: AST evaluation logic
│   ├── value.rs         # Value type definitions
│   ├── context.rs       # Context: variable management
│   ├── functions.rs     # Function Registry system
│   ├── error.rs         # Error definitions
│   ├── span.rs          # Position information (line/column)
│   ├── diagnostics.rs   # Diagnostic system
│   └── builtins/        # Base functions
│       ├── mod.rs
│       ├── string.rs    # String functions
│       ├── math.rs      # Math functions
│       ├── logic.rs     # Logic functions
│       ├── date.rs      # Date functions
│       └── collection.rs # Collection functions
├── docs/                # Documentation
├── Cargo.toml
├── LICENSE
├── README.md
├── SPEC.md              # Technical Specification
└── PLAN.md              # Development Roadmap
```

---

## 📊 Development Status

### ✅ Completed (V1)

- [x] Lexer: Tokenization with span tracking
- [x] Parser: Recursive descent parser with precedence handling
- [x] AST: Tree structure with spans on every node
- [x] Evaluator: Support for number, string, bool, null
- [x] Operators: Math, comparison, equality, logic
- [x] Function System: Registry and 10+ built-in functions
- [x] Context: Variable management and referencing
- [x] Error Reporting: Detailed reporting with positions
- [x] Documentation: Doc-tests and integration tests

### ✅ Completed (V2)

- [x] Access chaining: `obj.prop`, `arr[0]`, `users[0].name`
- [x] Functional programming: Lambda expressions `(x) => ...`
- [x] Higher-order functions: `map`, `filter`, `reduce`, `sort`, etc.
- [x] User-defined functions: `fn name(params) = expr`
- [x] Advanced types: `DateTime`, `Duration`, `Set`, `Range`
- [x] Serialization & Caching: JSON support and LRU Formula Cache
- [x] Plugin SDK foundation
- [x] Extended Math & String functions

### 📋 Future Plans (Phase 14+)

- [ ] Performance optimization (Constant folding, Vectorization)
- [ ] Native Date type refactor for all built-ins
- [ ] Error recovery & Security limits
- [ ] WASM-based plugin sandbox (V3)

---

## 🧪 Testing

Run all tests:

```bash
cargo test
```

Run tests for specific modules:

```bash
cargo test lexer
cargo test parser
cargo test eval
```

Run doc-tests:

```bash
cargo test --doc
```

---

## 📄 Additional Documents

- **[SPEC.md](SPEC.md)**: Detailed technical specification and architecture
- **[PLAN.md](PLAN.md)**: Phase-by-phase development roadmap
- **[docs/PRD.md](docs/PRD.md)**: Product Requirements Document
- **[docs/idea-extension-gemini.md](docs/idea-extension-gemini.md)**: System extension ideas
- **[docs/overview-extension-poe.md](docs/overview-extension-poe.md)**: Overview for POE SDK extension

---

## 🤝 Contributing

We encourage community contributions! If you find an issue or have suggestions:

1. Open an Issue to report a bug or request a feature
2. Fork the project and create a Pull Request
3. Write tests covering your changes
4. Update documentation if API changes occur

---

## 📝 License

This project is released under the [MIT License](LICENSE)

---

## 🙏 Acknowledgements

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Inspired by Notion bl1z
- Developed for the POE SDK ecosystem

---

## 📞 Contact

For questions or help, please open an Issue in this GitHub repository.
