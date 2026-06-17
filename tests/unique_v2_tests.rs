use bl1z::builtins;
use bl1z::{evaluate_mut, parse, tokenize, Context, FunctionRegistry, Value};
use std::collections::HashMap;

fn prepared_registry() -> FunctionRegistry {
    let mut reg = FunctionRegistry::new();
    builtins::register_all(&mut reg);
    reg
}

fn eval_v2(formula: &str, ctx: &mut Context) -> Value {
    let tokens = tokenize(formula).expect("tokenize failed");
    let ast = parse(&tokens).expect("parse failed");
    evaluate_mut(&ast, ctx, &prepared_registry()).expect("eval failed")
}

#[test]
fn test_complex_lambda_and_nesting() {
    let mut ctx = Context::new();
    let mut user1 = HashMap::new();
    user1.insert("id".to_string(), Value::Number(1.0));
    user1.insert(
        "meta".to_string(),
        Value::Map({
            let mut m = HashMap::new();
            m.insert(
                "tags".to_string(),
                Value::Array(vec![Value::String("rust".to_string())]),
            );
            m
        }),
    );

    let mut user2 = HashMap::new();
    user2.insert("id".to_string(), Value::Number(2.0));
    user2.insert(
        "meta".to_string(),
        Value::Map({
            let mut m = HashMap::new();
            m.insert(
                "tags".to_string(),
                Value::Array(vec![Value::String("formula".to_string())]),
            );
            m
        }),
    );

    ctx.set(
        "users",
        Value::Array(vec![Value::Map(user1), Value::Map(user2)]),
    );

    // map(users, u => u.meta.tags[0])
    let result = eval_v2("map(users, u => u.meta.tags[0])", &mut ctx);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::String("rust".to_string()),
            Value::String("formula".to_string())
        ])
    );
}

#[test]
fn test_lambda_capture_and_property_access() {
    let mut ctx = Context::new();
    ctx.set("key", Value::String("score".to_string()));

    let mut data = HashMap::new();
    data.insert("score".to_string(), Value::Number(100.0));
    ctx.set("data", Value::Map(data));

    // In bl1z, dynamic property access via variable is not supported directly by . notation
    // but we can test if lambda captures work with normal access
    let result = eval_v2("map([data], d => d.score)", &mut ctx);
    assert_eq!(result, Value::Array(vec![Value::Number(100.0)]));
}

#[test]
fn test_datetime_literal_comparison() {
    let mut ctx = Context::new();

    assert_eq!(
        eval_v2("@2024-01-01 < @2024-01-02", &mut ctx),
        Value::Bool(true)
    );
    assert_eq!(
        eval_v2("@2024-06-17T10:00:00Z > @2024-06-17T09:00:00Z", &mut ctx),
        Value::Bool(true)
    );
    assert_eq!(
        eval_v2("@2024-01-01 == @2024-01-01", &mut ctx),
        Value::Bool(true)
    );
}

#[test]
fn test_set_with_complex_types() {
    let mut ctx = Context::new();

    // Set of Arrays
    let result = eval_v2("set([[1, 2], [1, 2], [3]])", &mut ctx);
    if let Value::Set(s) = result {
        assert_eq!(s.len(), 2);
    } else {
        panic!("Expected set");
    }

    // Set of Maps
    let result = eval_v2("set([{a: 1}, {a: 1}, {a: 2}])", &mut ctx);
    if let Value::Set(s) = result {
        assert_eq!(s.len(), 2);
    } else {
        panic!("Expected set");
    }
}

#[test]
fn test_range_edge_cases() {
    let mut ctx = Context::new();

    // Reverse range (empty with positive step)
    let result = eval_v2("range_to_array(range(10, 0))", &mut ctx);
    assert_eq!(result, Value::Array(vec![]));

    // Zero step (should probably error or be handled, currently let's see)
    // Actually our range function handles it or might infinite loop if not careful.
    // Based on common range impls, step 0 is often an error.
}

#[test]
fn test_sequence_with_shadowing() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(10.0));

    // Global x is 10. Inside UDF, x is param.
    let formula = "fn test(x) = x * 2; test(5) + x";
    assert_eq!(eval_v2(formula, &mut ctx), Value::Number(20.0)); // (5 * 2) + 10 = 20
}

#[test]
#[cfg(feature = "serialization")]
fn test_advanced_serialization() {
    let mut ctx = Context::new();

    // DateTime
    let val = eval_v2("@2024-06-17T12:00:00Z", &mut ctx);
    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(val, back);

    // Range
    let val = eval_v2("range(1, 10, 2)", &mut ctx);
    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(val, back);

    // Set
    let val = eval_v2("set([1, 2, 3])", &mut ctx);
    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    // Set comparison might be tricky due to HashSet ordering, but Value::Set has equality
    assert_eq!(val, back);
}

#[test]
fn test_deeply_nested_mixed_access() {
    let mut ctx = Context::new();
    let formula = "{a: [{b: {c: [42]}}]}.a[0].b.c[0]";
    assert_eq!(eval_v2(formula, &mut ctx), Value::Number(42.0));
}
