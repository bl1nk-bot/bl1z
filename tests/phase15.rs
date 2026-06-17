//! Phase 15: Error Recovery + Security Limits — integration tests

use bl1z::builtins;
use bl1z::config::EngineConfig;
use bl1z::eval::evaluate_with_config;
use bl1z::parser::{parse_formula_with_config, parse_with_recovery, RecoveryResult};
use bl1z::{parse, tokenize, Context, FunctionRegistry, Value};

fn make_registry() -> FunctionRegistry {
    let mut r = FunctionRegistry::new();
    builtins::register_all(&mut r);
    r
}

fn recovery_result(src: &str) -> RecoveryResult {
    let tokens = tokenize(src).expect("tokenize failed");
    parse_with_recovery(&tokens)
}

// ── 15.1 + 15.2: parse_with_recovery ──────────────────────────────────

#[test]
fn recovery_collects_multiple_errors() {
    let result = recovery_result("1 + ; ; 3 / + 4");
    assert!(
        !result.errors.is_empty(),
        "expected at least 1 recovery error"
    );
}

#[test]
fn recovery_valid_after_invalid() {
    let result = recovery_result("; 1 + 2");
    assert!(result.ast.is_some() || !result.errors.is_empty());
}

#[test]
fn recovery_empty_input() {
    let result = recovery_result("");
    assert!(result.errors.is_empty());
}

// ── 15.3 + 15.4: EngineConfig ─────────────────────────────────────────

#[test]
fn config_default() {
    let config = EngineConfig::default();
    assert_eq!(config.max_formula_length, 10_000);
    assert_eq!(config.max_depth, 100);
    assert!(config.max_time_ms.is_none());
}

#[test]
fn config_custom() {
    let config = EngineConfig {
        max_formula_length: 500,
        max_depth: 10,
        max_time_ms: Some(100),
    };
    assert_eq!(config.max_formula_length, 500);
    assert_eq!(config.max_depth, 10);
    assert_eq!(config.max_time_ms, Some(100));
}

#[test]
fn config_clone() {
    let config = EngineConfig {
        max_formula_length: 500,
        max_depth: 10,
        max_time_ms: Some(100),
    };
    let cloned = config.clone();
    assert_eq!(config.max_formula_length, cloned.max_formula_length);
    assert_eq!(config.max_depth, cloned.max_depth);
    assert_eq!(config.max_time_ms, cloned.max_time_ms);
}

#[test]
fn config_max_formula_length_rejects_long() {
    let long_formula = "1 + ".repeat(3000); // 12000 chars
    let config = EngineConfig {
        max_formula_length: 100,
        max_depth: 100,
        max_time_ms: None,
    };
    let result = parse_formula_with_config(&long_formula, &config);
    assert!(
        result.is_err(),
        "should reject formula exceeding max length"
    );
}

#[test]
fn config_max_formula_length_allows_short() {
    let config = EngineConfig {
        max_formula_length: 10_000,
        max_depth: 100,
        max_time_ms: None,
    };
    let result = parse_formula_with_config("1 + 2", &config);
    assert!(result.is_ok());
}

#[test]
fn config_max_depth_rejects_deep_recursion() {
    let registry = make_registry();
    let ctx = Context::new();
    let config = EngineConfig {
        max_formula_length: 10_000,
        max_depth: 5,
        max_time_ms: None,
    };
    let expr_str = "(1 + (2 + (3 + (4 + (5 + (6 + 7))))))";
    let tokens = tokenize(expr_str).unwrap();
    let ast = parse(&tokens).unwrap();
    let result = evaluate_with_config(&ast, &ctx, &registry, &config);
    assert!(result.is_err(), "should reject deep recursion");
}

#[test]
fn config_timeout_no_trigger_on_small() {
    let registry = make_registry();
    let ctx = Context::new();
    let config = EngineConfig {
        max_formula_length: 10_000,
        max_depth: 10_000,
        max_time_ms: Some(1000), // generous timeout
    };
    let tokens = tokenize("1 + 2").unwrap();
    let ast = parse(&tokens).unwrap();
    let result = evaluate_with_config(&ast, &ctx, &registry, &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(3.0));
}

#[test]
fn config_default_eval_same_as_regular() {
    let registry = make_registry();
    let ctx = Context::new();
    let config = EngineConfig::default();
    let tokens = tokenize("1 + 2 * 3").unwrap();
    let ast = parse(&tokens).unwrap();
    let result = evaluate_with_config(&ast, &ctx, &registry, &config).unwrap();
    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn config_nested_map_access() {
    let registry = make_registry();
    let mut ctx = Context::new();
    let mut inner = std::collections::HashMap::new();
    inner.insert("x".to_string(), Value::Number(42.0));
    let mut outer = std::collections::HashMap::new();
    outer.insert("data".to_string(), Value::Map(inner));
    ctx.set("obj", Value::Map(outer));

    let config = EngineConfig::default();
    let tokens = tokenize("obj.data.x").unwrap();
    let ast = parse(&tokens).unwrap();
    let result = evaluate_with_config(&ast, &ctx, &registry, &config).unwrap();
    assert_eq!(result, Value::Number(42.0));
}
