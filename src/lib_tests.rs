use super::*;
use context::Context;
use error::ErrorKind;
use eval::evaluate;
use functions::FunctionRegistry;
use lexer::tokenize;
use parser::parse;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

fn prepared_registry() -> FunctionRegistry {
    let mut reg = FunctionRegistry::new();
    builtins::register_all(&mut reg);
    reg
}

fn eval_formula(formula: &str) -> Result<Value, FormulaError> {
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    evaluate(&ast, &Context::new(), &prepared_registry())
}

fn eval_with_ctx(formula: &str, ctx: &Context) -> Result<Value, FormulaError> {
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    evaluate(&ast, ctx, &prepared_registry())
}

#[test]
fn evaluate_addition_returns_sum() {
    assert_eq!(eval_formula("1 + 2"), Ok(Value::Number(3.0)));
}

#[test]
fn evaluate_multiplication_returns_product() {
    assert_eq!(eval_formula("2 * 3"), Ok(Value::Number(6.0)));
}

#[test]
fn evaluate_precedence_mul_over_add() {
    assert_eq!(eval_formula("1 + 2 * 3"), Ok(Value::Number(7.0)));
}

#[test]
fn evaluate_parentheses_override_precedence() {
    assert_eq!(eval_formula("(1 + 2) * 3"), Ok(Value::Number(9.0)));
}

#[test]
fn evaluate_unary_negation() {
    assert_eq!(eval_formula("-1"), Ok(Value::Number(-1.0)));
    assert_eq!(eval_formula("--1"), Ok(Value::Number(1.0)));
}

#[test]
fn evaluate_unary_not() {
    assert_eq!(eval_formula("!true"), Ok(Value::Bool(false)));
    assert_eq!(eval_formula("!false"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_greater_than_true() {
    assert_eq!(eval_formula("2 > 1"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_greater_than_false() {
    assert_eq!(eval_formula("1 > 2"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_equality_same_type_true() {
    assert_eq!(eval_formula("1 == 1"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("\"a\" == \"a\""), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_equality_different_values_false() {
    assert_eq!(eval_formula("1 == 2"), Ok(Value::Bool(false)));
    assert_eq!(eval_formula("\"a\" == \"b\""), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_not_inequality() {
    assert_eq!(eval_formula("1 != 2"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("1 != 1"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_comparison_chained_and() {
    assert_eq!(eval_formula("1 < 2 && 2 < 3"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("1 < 2 && 3 < 2"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_comparison_chained_or() {
    assert_eq!(eval_formula("1 > 2 || 2 < 3"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("1 > 2 || 2 > 3"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_comparison_less_than_or_equal() {
    assert_eq!(eval_formula("1 <= 1"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("1 <= 2"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("2 <= 1"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_comparison_greater_than_or_equal() {
    assert_eq!(eval_formula("1 >= 1"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("2 >= 1"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("1 >= 2"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_string_concatenation() {
    assert_eq!(
        eval_formula("\"hello\" + \" world\""),
        Ok(Value::String("hello world".to_string()))
    );
}

#[test]
fn evaluate_string_concatenation_empty_left() {
    assert_eq!(
        eval_formula("\"\" + \"hello\""),
        Ok(Value::String("hello".to_string()))
    );
}

#[test]
fn evaluate_string_concatenation_empty_right() {
    assert_eq!(
        eval_formula("\"hello\" + \"\""),
        Ok(Value::String("hello".to_string()))
    );
}

#[test]
fn evaluate_builtin_function_len() {
    assert_eq!(eval_formula("len(\"hello\")"), Ok(Value::Number(5.0)));
}

#[test]
fn evaluate_builtin_function_upper() {
    assert_eq!(
        eval_formula("upper(\"hello\")"),
        Ok(Value::String("HELLO".to_string()))
    );
}

#[test]
fn evaluate_builtin_function_lower() {
    assert_eq!(
        eval_formula("lower(\"HELLO\")"),
        Ok(Value::String("hello".to_string()))
    );
}

#[test]
fn evaluate_variable_lookup() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(10.0));
    assert_eq!(eval_with_ctx("x + 5", &ctx), Ok(Value::Number(15.0)));
}

#[test]
fn evaluate_nested_variable_lookup() {
    let mut parent = Context::new();
    parent.set("x", Value::Number(10.0));
    let mut child = Context::with_parent(parent.clone());
    child.set("y", Value::Number(5.0));
    assert_eq!(eval_with_ctx("x + y", &child), Ok(Value::Number(15.0)));
}

#[test]
fn evaluate_variable_shadowing() {
    let mut parent = Context::new();
    parent.set("x", Value::Number(10.0));
    let mut child = Context::with_parent(parent.clone());
    child.set("x", Value::Number(5.0));
    assert_eq!(eval_with_ctx("x", &child), Ok(Value::Number(5.0)));
}

#[test]
fn evaluate_variable_not_found_error() {
    let result = eval_formula("x + 1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::VariableNotFound);
}

#[test]
fn evaluate_type_mismatch_error() {
    let result = eval_formula("1 + \"hello\"");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::TypeError);
}

#[test]
fn evaluate_function_not_found_error() {
    let result = eval_formula("unknown_func()");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::FunctionError);
}

#[test]
fn evaluate_wrong_arity_error() {
    let result = eval_formula("len(\"a\", \"b\")");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::FunctionError);
}

#[test]
fn evaluate_if_function_true() {
    assert_eq!(eval_formula("if(true, 1, 2)"), Ok(Value::Number(1.0)));
}

#[test]
fn evaluate_if_function_false() {
    assert_eq!(eval_formula("if(false, 1, 2)"), Ok(Value::Number(2.0)));
}

#[test]
fn evaluate_complex_expression() {
    let mut ctx = Context::new();
    ctx.set("price", Value::Number(100.0));
    ctx.set("tax_rate", Value::Number(0.07));
    assert_eq!(
        eval_with_ctx("if(price > 50, price * (1 + tax_rate), price)", &ctx),
        Ok(Value::Number(107.0))
    );
}

#[test]
fn evaluate_array_literal() {
    assert_eq!(
        eval_formula("[1, 2, 1 + 2]"),
        Ok(Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ]))
    );
}

#[test]
fn evaluate_map_literal() {
    let result = eval_formula("{a: 1, b: 2 + 3}").unwrap();
    if let Value::Map(map) = result {
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("a"), Some(&Value::Number(1.0)));
        assert_eq!(map.get("b"), Some(&Value::Number(5.0)));
    } else {
        panic!("Expected Map");
    }
}

#[test]
fn evaluate_property_access() {
    let result = eval_formula("{user: {name: \"Alice\"}}.user.name").unwrap();
    assert_eq!(result, Value::String("Alice".to_string()));
}

#[test]
fn evaluate_index_access() {
    let result = eval_formula("[10, 20, 30][1]").unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn evaluate_nested_access() {
    let result =
        eval_formula("{users: [{name: \"Alice\"}, {name: \"Bob\"}]}.users[1].name").unwrap();
    assert_eq!(result, Value::String("Bob".to_string()));
}

#[test]
fn evaluate_property_not_found_error() {
    let result = eval_formula("{a: 1}.b");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::PropertyNotFound);
}

#[test]
fn evaluate_index_out_of_bounds_error() {
    let result = eval_formula("[1, 2][5]");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::IndexOutOfBounds);
}

#[test]
fn evaluate_complex_access_chain() {
    let mut ctx = Context::new();
    let mut user = HashMap::new();
    user.insert("name".to_string(), Value::String("John".to_string()));
    user.insert(
        "scores".to_string(),
        Value::Array(vec![Value::Number(90.0), Value::Number(85.0)]),
    );
    ctx.set("user", Value::Map(user));

    assert_eq!(
        eval_with_ctx("user.name", &ctx),
        Ok(Value::String("John".to_string()))
    );
    assert_eq!(
        eval_with_ctx("user.scores[0]", &ctx),
        Ok(Value::Number(90.0))
    );
}

#[test]
fn evaluate_unary_minus_with_access() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(10.0));
    assert_eq!(eval_with_ctx("-x", &ctx), Ok(Value::Number(-10.0)));

    let mut map = HashMap::new();
    map.insert("val".to_string(), Value::Number(5.0));
    ctx.set("data", Value::Map(map));
    assert_eq!(eval_with_ctx("-data.val", &ctx), Ok(Value::Number(-5.0)));
}

#[test]
fn evaluate_unary_plus_with_access() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(10.0));
    assert_eq!(eval_with_ctx("+x", &ctx), Ok(Value::Number(10.0)));
}

#[test]
fn evaluate_postfix_precedence() {
    // -arr[0] should be -(arr[0])
    let mut ctx = Context::new();
    ctx.set("arr", Value::Array(vec![Value::Number(10.0)]));
    assert_eq!(eval_with_ctx("-arr[0]", &ctx), Ok(Value::Number(-10.0)));

    // !map.flag should be !(map.flag)
    let mut map = HashMap::new();
    map.insert("flag".to_string(), Value::Bool(true));
    ctx.set("m", Value::Map(map));
    assert_eq!(eval_with_ctx("!m.flag", &ctx), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_complex_chain_with_unary() {
    let mut ctx = Context::new();
    let mut map = HashMap::new();
    map.insert(
        "nums".to_string(),
        Value::Array(vec![Value::Number(-5.0), Value::Number(10.0)]),
    );
    ctx.set("data", Value::Map(map));

    // -data.nums[0] -> -(-5) -> 5
    assert_eq!(eval_with_ctx("-data.nums[0]", &ctx), Ok(Value::Number(5.0)));
}

// -- Phase 9: Lambda & Higher-Order Functions Tests --

#[test]
fn evaluate_single_identifier_lambda() {
    let result = eval_formula("map([1, 2, 3], x => x + 1)").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0)
        ])
    );
}

#[test]
fn evaluate_parenthesized_lambda() {
    let result = eval_formula("map([1, 2, 3], (x) => x * 2)").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(2.0),
            Value::Number(4.0),
            Value::Number(6.0)
        ])
    );
}

#[test]
fn evaluate_lambda_duplicate_params_fails() {
    let result = eval_formula("(x, x) => x");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ParseError);
    assert_eq!(err.code, "E209");
}

#[test]
fn evaluate_lambda_display_and_debug() {
    // Generate a Lambda value
    let tokens = tokenize("x => x + 1").unwrap();
    let ast = parse(&tokens).unwrap();
    let val = evaluate(&ast, &Context::new(), &prepared_registry()).unwrap();

    assert_eq!(format!("{}", val), "(x) => ...");
    assert_eq!(format!("{:?}", val), "Lambda((x) => ...)");
}

#[test]
fn evaluate_string_and_bool_comparison() {
    assert_eq!(
        eval_formula("\"apple\" < \"banana\""),
        Ok(Value::Bool(true))
    );
    assert_eq!(
        eval_formula("\"apple\" > \"banana\""),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        eval_formula("\"apple\" <= \"apple\""),
        Ok(Value::Bool(true))
    );
    assert_eq!(eval_formula("false < true"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("true >= false"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_type_mismatch_error() {
    let result = eval_formula("1 < \"apple\"");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_recursion_limit_exceeded() {
    // A self-calling lambda to exceed 100 limit
    // Define a recursive lambda: f => f(f)
    // We bind a lambda under a variable name and call it
    let mut ctx = Context::new();
    let tokens = tokenize("f => f(f)").unwrap();
    let ast = parse(&tokens).unwrap();
    let f_val = evaluate(&ast, &ctx, &prepared_registry()).unwrap();
    ctx.set("f", f_val.clone());

    let call_tokens = tokenize("f(f)").unwrap();
    let call_ast = parse(&call_tokens).unwrap();
    let result = evaluate(&call_ast, &ctx, &prepared_registry());

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::RecursionLimitExceeded);
    assert_eq!(err.code, "E303"); // Recursion limit exceeded
}

#[test]
fn evaluate_filter_function() {
    let result = eval_formula("filter([1, 2, 3, 4], x => x > 2)").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![Value::Number(3.0), Value::Number(4.0)])
    );
}

#[test]
fn evaluate_reduce_function() {
    let result = eval_formula("reduce([1, 2, 3], (acc, x) => acc + x, 10)").unwrap();
    assert_eq!(result, Value::Number(16.0));
}

#[test]
fn evaluate_sort_with_function() {
    let result = eval_formula("sort_with([3, 1, 2], (a, b) => b - a)").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(3.0),
            Value::Number(2.0),
            Value::Number(1.0)
        ])
    );
}

#[test]
fn evaluate_sort_with_error_propagation() {
    // When the comparator lambda raises an error, it should be propagated
    let result = eval_formula("sort_with([1, 2], (a, b) => a + \"error\")");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
}

#[test]
fn evaluate_unique_function_various_types() {
    let result = eval_formula("unique([1, 2, 2, 1, 3])").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])
    );
}

#[test]
fn evaluate_group_by_function() {
    let result = eval_formula("group_by([1, 2, 3, 4], x => if(x > 2, \"high\", \"low\"))").unwrap();
    match result {
        Value::Map(map) => {
            assert_eq!(map.len(), 2);
            assert_eq!(
                map.get("low").unwrap(),
                &Value::Array(vec![Value::Number(1.0), Value::Number(2.0)])
            );
            assert_eq!(
                map.get("high").unwrap(),
                &Value::Array(vec![Value::Number(3.0), Value::Number(4.0)])
            );
        }
        _ => panic!("Expected Map"),
    }
}

#[test]
fn evaluate_sort_with_key_lambda() {
    // sort based on age field in maps
    let result =
        eval_formula("sort([{age: 30, name: \"A\"}, {age: 20, name: \"B\"}], x => x.age)").unwrap();
    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 2);
            // First item should be the one with age 20 (B)
            match &arr[0] {
                Value::Map(map) => {
                    assert_eq!(map.get("name").unwrap(), &Value::String("B".to_string()));
                }
                _ => panic!("Expected Map"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn evaluate_unique_with_key_lambda() {
    // unique based on id field in maps
    let result = eval_formula(
        "unique([{id: 1, name: \"A\"}, {id: 2, name: \"B\"}, {id: 1, name: \"C\"}], x => x.id)",
    )
    .unwrap();
    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 2);
            // The items kept should be first occurrences (id 1 (A) and id 2 (B))
            match &arr[0] {
                Value::Map(map) => {
                    assert_eq!(map.get("name").unwrap(), &Value::String("A".to_string()));
                }
                _ => panic!("Expected Map"),
            }
            match &arr[1] {
                Value::Map(map) => {
                    assert_eq!(map.get("name").unwrap(), &Value::String("B".to_string()));
                }
                _ => panic!("Expected Map"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

// -- Phase 10: User-Defined Functions Tests --

fn eval_formula_mut(formula: &str) -> Result<Value, FormulaError> {
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    let mut ctx = Context::new();
    eval::evaluate_mut(&ast, &mut ctx, &prepared_registry())
}

#[test]
fn evaluate_user_defined_function_basic() {
    assert_eq!(
        eval_formula_mut("fn double(x) = x * 2; double(5)"),
        Ok(Value::Number(10.0))
    );
}

#[test]
fn evaluate_user_defined_function_two_params() {
    assert_eq!(
        eval_formula_mut("fn add(a, b) = a + b; add(3, 4)"),
        Ok(Value::Number(7.0))
    );
}

#[test]
fn evaluate_user_defined_function_zero_params() {
    assert_eq!(
        eval_formula_mut("fn val() = 1.2345; val()"),
        Ok(Value::Number(1.2345))
    );
}

#[test]
fn evaluate_user_defined_function_with_builtin() {
    assert_eq!(
        eval_formula_mut("fn double_len(s) = len(s) * 2; double_len(\"hello\")"),
        Ok(Value::Number(10.0))
    );
}

#[test]
fn evaluate_user_defined_function_factorial() {
    assert_eq!(
        eval_formula_mut("fn factorial(n) = if(n <= 1, 1, n * factorial(n - 1)); factorial(5)"),
        Ok(Value::Number(120.0))
    );
}

#[test]
fn evaluate_user_defined_function_factorial_base_case() {
    assert_eq!(
        eval_formula_mut("fn factorial(n) = if(n <= 1, 1, n * factorial(n - 1)); factorial(1)"),
        Ok(Value::Number(1.0))
    );
}

#[test]
fn evaluate_user_defined_function_multiple_definitions() {
    assert_eq!(
        eval_formula_mut("fn double(x) = x * 2; fn add_one(x) = x + 1; add_one(double(3))"),
        Ok(Value::Number(7.0))
    );
}

#[test]
fn evaluate_user_defined_function_wrong_arg_count() {
    let result = eval_formula_mut("fn double(x) = x * 2; double(1, 2)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E503");
}

#[test]
fn evaluate_user_defined_function_duplicate_params() {
    let result = eval_formula_mut("fn bad(x, x) = x");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ParseError);
    assert_eq!(err.code, "E209");
}

#[test]
fn evaluate_user_defined_function_recursion_limit() {
    let result = eval_formula_mut("fn f(n) = f(n + 1); f(0)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::RecursionLimitExceeded);
    assert_eq!(err.code, "E303");
}

#[test]
fn evaluate_user_defined_function_with_property_access() {
    assert_eq!(
        eval_formula_mut("fn get_name(obj) = obj.name; get_name({name: \"Alice\"})"),
        Ok(Value::String("Alice".to_string()))
    );
}

#[test]
fn evaluate_sequence_returns_last_value() {
    assert_eq!(
        eval_formula_mut("1 + 1; 2 + 2; 3 + 3"),
        Ok(Value::Number(6.0))
    );
}

#[test]
fn evaluate_trailing_semicolon() {
    assert_eq!(eval_formula_mut("42;"), Ok(Value::Number(42.0)));
}
