//! Phase 12: Serialization & Caching — integration tests
//!
//! These tests validate round-trip Value serialization, Context JSON
//! snapshot/restore, and FormulaCache hit/miss behaviour. They only
//! compile when the `serialization` feature is enabled.

#![cfg(feature = "serialization")]

use bl1z::builtins;
use bl1z::cache::FormulaCache;
use bl1z::context::Context;
use bl1z::{evaluate, parse, tokenize, FunctionRegistry, Value};

// ── Value round-trip ────────────────────────────────────────────────────

fn roundtrip(val: &Value) -> Value {
    let json = serde_json::to_string(val).expect("serialize failed");
    serde_json::from_str(&json).expect("deserialize failed")
}

#[test]
fn value_roundtrip_number() {
    let v = Value::Number(42.0);
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_string() {
    let v = Value::String("hello".to_string());
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_bool() {
    assert_eq!(roundtrip(&Value::Bool(true)), Value::Bool(true));
    assert_eq!(roundtrip(&Value::Bool(false)), Value::Bool(false));
}

#[test]
fn value_roundtrip_null() {
    assert_eq!(roundtrip(&Value::Null), Value::Null);
}

#[test]
fn value_roundtrip_array() {
    let v = Value::Array(vec![
        Value::Number(1.0),
        Value::String("two".to_string()),
        Value::Bool(false),
        Value::Null,
    ]);
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), Value::Number(42.0));
    map.insert("name".to_string(), Value::String("test".to_string()));
    let v = Value::Map(map);
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_nested() {
    let mut inner_map = std::collections::HashMap::new();
    inner_map.insert("nested".to_string(), Value::Number(1.0));
    let v = Value::Array(vec![
        Value::Number(1.0),
        Value::Map(inner_map),
        Value::Array(vec![Value::Null]),
    ]);
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_range() {
    let v = Value::Range {
        start: 1,
        end: 10,
        step: 2,
    };
    assert_eq!(roundtrip(&v), v);
}

#[test]
fn value_roundtrip_set() {
    let mut set = std::collections::HashSet::new();
    set.insert(Value::Number(1.0));
    set.insert(Value::Number(2.0));
    set.insert(Value::Number(3.0));
    let v = Value::Set(set);
    let rt = roundtrip(&v);
    // Sets lose ordering, compare via values
    match rt {
        Value::Set(s) => assert_eq!(s.len(), 3),
        _ => panic!("expected Set"),
    }
}

#[test]
fn value_lambda_not_serializable() {
    use bl1z::ast::{Expr, ExprMeta, SpannedExpr};
    use bl1z::span::{Position, Span};
    use std::rc::Rc;
    let body = SpannedExpr {
        expr: Expr::Variable("x".to_string()),
        meta: ExprMeta {
            span: Span::new(
                Position { line: 1, column: 1 },
                Position { line: 1, column: 2 },
            ),
        },
    };
    let v = Value::Lambda(
        Rc::new(body),
        vec!["x".to_string()],
        std::collections::BTreeMap::new(),
        std::collections::BTreeMap::new(),
    );
    let result = serde_json::to_string(&v);
    assert!(result.is_err());
}

// ── Context JSON snapshot ───────────────────────────────────────────────

#[test]
fn context_to_json_and_back() {
    let mut ctx = Context::new();
    ctx.set("score", Value::Number(100.0));
    ctx.set("name", Value::String("Alice".to_string()));
    ctx.set("active", Value::Bool(true));

    let json = ctx.to_json().expect("to_json failed");
    let restored = Context::from_json(&json).expect("from_json failed");

    assert_eq!(restored.get("score"), Some(&Value::Number(100.0)));
    assert_eq!(
        restored.get("name"),
        Some(&Value::String("Alice".to_string()))
    );
    assert_eq!(restored.get("active"), Some(&Value::Bool(true)));
}

#[test]
fn context_json_pretty() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(42.0));
    let json = ctx.to_json_pretty().expect("to_json_pretty failed");
    assert!(json.contains('\n'));
}

#[test]
fn context_from_json_empty() {
    let ctx = Context::from_json("{}").expect("from_json failed");
    assert!(ctx.get("anything").is_none());
}

// ── FormulaCache ────────────────────────────────────────────────────────

#[test]
fn cache_integration_roundtrip() {
    let mut cache = FormulaCache::new(10);

    // Parse and cache
    let tokens = tokenize("1 + 2 * 3").unwrap();
    let ast = parse(&tokens).unwrap();
    cache.insert("1 + 2 * 3".to_string(), ast);

    // Retrieve from cache
    let cached = cache.get("1 + 2 * 3").unwrap();

    // Evaluate cached AST
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();
    let result = evaluate(cached, &ctx, &registry).unwrap();
    assert_eq!(result, Value::Number(7.0));
}

// ── Eval result → JSON → display ────────────────────────────────────────

#[test]
fn eval_result_to_json() {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    let tokens = tokenize("if(true, 42, 0)").unwrap();
    let ast = parse(&tokens).unwrap();
    let result = evaluate(&ast, &ctx, &registry).unwrap();
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("42"));
}
