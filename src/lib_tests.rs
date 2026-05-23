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

// -- Arithmetic & Precedence --

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
    assert_eq!(eval_formula("-5 + 3"), Ok(Value::Number(-2.0)));
}

#[test]
fn evaluate_unary_not() {
    assert_eq!(eval_formula("!true && false"), Ok(Value::Bool(false)));
}

// -- Comparison & Logic --

#[test]
fn evaluate_comparison_greater_than_true() {
    assert_eq!(eval_formula("5 > 3"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_greater_than_false() {
    assert_eq!(eval_formula("3 > 5"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_equality_same_type_true() {
    assert_eq!(eval_formula("2 == 2"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_equality_different_values_false() {
    assert_eq!(eval_formula("2 == 3"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_not_inequality() {
    assert_eq!(eval_formula("2 != 3"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_chained_and() {
    assert_eq!(eval_formula("5 > 3 && 2 == 2"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_chained_or() {
    assert_eq!(eval_formula("1 > 5 || 2 == 2"), Ok(Value::Bool(true)));
}

#[test]
fn evaluate_comparison_less_than_or_equal() {
    assert_eq!(eval_formula("5 <= 5"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("6 <= 5"), Ok(Value::Bool(false)));
}

#[test]
fn evaluate_comparison_greater_than_or_equal() {
    assert_eq!(eval_formula("5 >= 5"), Ok(Value::Bool(true)));
    assert_eq!(eval_formula("4 >= 5"), Ok(Value::Bool(false)));
}

// -- String Operations --

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

// -- Variables & Context --

#[test]
fn evaluate_variable_lookup_returns_value() {
    let mut ctx = Context::new();
    ctx.set("score", Value::Number(40.0));
    assert_eq!(eval_with_ctx("score + 10", &ctx), Ok(Value::Number(50.0)));
}

#[test]
fn evaluate_undefined_variable_returns_context_error() {
    let result = eval_formula("undefined_var + 1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ContextError);
    assert_eq!(err.code, "E601");
}

#[test]
fn evaluate_variable_shadowing_uses_latest() {
    let mut ctx = Context::new();
    ctx.set("x", Value::Number(1.0));
    ctx.set("x", Value::Number(10.0));
    assert_eq!(eval_with_ctx("x", &ctx), Ok(Value::Number(10.0)));
}

// -- Function Calls --

#[test]
fn evaluate_if_function_returns_true_branch() {
    assert_eq!(eval_formula("if(true, 100, 200)"), Ok(Value::Number(100.0)));
}

#[test]
fn evaluate_if_function_returns_false_branch() {
    assert_eq!(
        eval_formula("if(false, 100, 200)"),
        Ok(Value::Number(200.0))
    );
}

#[test]
fn evaluate_nested_function_calls() {
    assert_eq!(
        eval_formula("len(upper(\"hello\"))"),
        Ok(Value::Number(5.0))
    );
}

#[test]
fn evaluate_function_call_wrong_arg_count_returns_function_error() {
    let result = eval_formula("len(\"hello\", \"world\")");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E503");
}

#[test]
fn evaluate_undefined_function_returns_function_error() {
    let result = eval_formula("nonexistent(123)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E502");
}

#[test]
fn evaluate_function_call_zero_args() {
    assert_eq!(eval_formula("now()").is_ok(), true);
}

// -- Complex Expressions --

#[test]
fn evaluate_complex_conditional_expression() {
    assert_eq!(
        eval_formula("if(len(\"hello\") > 3 && 5 >= 5, upper(\"test\"), \"fail\")"),
        Ok(Value::String("TEST".to_string()))
    );
}

// -- Type Errors --

#[test]
fn evaluate_add_string_number_returns_type_error() {
    let result = eval_formula("\"hello\" + 123");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_division_by_zero_returns_eval_error() {
    let result = eval_formula("1 / 0");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "E301");
}

// -- Array Literals & Operations --

#[test]
fn parse_array_literal_produces_correct_length() {
    let tokens = tokenize("[1, 2, 3]").unwrap();
    let ast = parse(&tokens).unwrap();
    match ast.expr {
        Expr::ArrayLiteral(elems) => assert_eq!(elems.len(), 3),
        _ => panic!("expected ArrayLiteral"),
    }
}

#[test]
fn evaluate_empty_array_literal() {
    assert_eq!(eval_formula("[]"), Ok(Value::Array(vec![])));
}

#[test]
fn evaluate_nested_array_literal_preserves_structure() {
    let result = eval_formula("[[1, 2], [3, 4]]").unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
            Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
        ])
    );
}

#[test]
fn evaluate_array_with_bool_elements() {
    assert_eq!(
        eval_formula("[true, false]"),
        Ok(Value::Array(vec![Value::Bool(true), Value::Bool(false)]))
    );
}

#[test]
fn evaluate_array_with_expression_elements() {
    assert_eq!(
        eval_formula("[1 + 1, 2 * 3]"),
        Ok(Value::Array(vec![Value::Number(2.0), Value::Number(6.0)]))
    );
}

#[test]
fn evaluate_sum_array_returns_total() {
    assert_eq!(eval_formula("sum([1, 2, 3, 4])"), Ok(Value::Number(10.0)));
}

#[test]
fn evaluate_sum_empty_array_returns_zero() {
    assert_eq!(eval_formula("sum([])"), Ok(Value::Number(0.0)));
}

#[test]
fn evaluate_sum_with_float_values() {
    assert_eq!(eval_formula("sum([1.5, 2.5, 3.0])"), Ok(Value::Number(7.0)));
}

#[test]
fn evaluate_sum_with_negative_values() {
    assert_eq!(eval_formula("sum([-1, -2, 3])"), Ok(Value::Number(0.0)));
}

#[test]
fn evaluate_sum_non_array_arg_returns_function_error() {
    let result = eval_formula("sum(42)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E501");
}

#[test]
fn evaluate_sum_non_number_element_returns_function_error() {
    let result = eval_formula("sum([\"a\", \"b\"])");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E501");
}

#[test]
fn evaluate_avg_array_returns_mean() {
    assert_eq!(eval_formula("avg([10, 20])"), Ok(Value::Number(15.0)));
}

#[test]
fn evaluate_avg_empty_array_returns_function_error() {
    let result = eval_formula("avg([])");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E504");
}

#[test]
fn evaluate_len_array_returns_count() {
    assert_eq!(eval_formula("len([1, 2, 3])"), Ok(Value::Number(3.0)));
}

#[test]
fn evaluate_len_empty_array_returns_zero() {
    assert_eq!(eval_formula("len([])"), Ok(Value::Number(0.0)));
}

#[test]
fn evaluate_len_non_array_non_string_returns_function_error() {
    let result = eval_formula("len(42)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E501");
}

#[test]
fn evaluate_min_array_returns_smallest() {
    assert_eq!(eval_formula("min([5, 2, 8])"), Ok(Value::Number(2.0)));
}

#[test]
fn evaluate_min_with_negative_values() {
    assert_eq!(eval_formula("min([0, -5, 3])"), Ok(Value::Number(-5.0)));
}

#[test]
fn evaluate_min_empty_array_returns_function_error() {
    let result = eval_formula("min([])");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E504");
}

#[test]
fn evaluate_max_array_returns_largest() {
    assert_eq!(eval_formula("max([5, 2, 8])"), Ok(Value::Number(8.0)));
}

#[test]
fn evaluate_max_with_negative_values() {
    assert_eq!(eval_formula("max([-1, -2, -3])"), Ok(Value::Number(-1.0)));
}

#[test]
fn evaluate_max_empty_array_returns_function_error() {
    let result = eval_formula("max([])");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E504");
}

#[test]
fn evaluate_count_array_returns_length() {
    assert_eq!(
        eval_formula("count([1, 2, 3, 4, 5])"),
        Ok(Value::Number(5.0))
    );
}

#[test]
fn evaluate_count_empty_array_returns_zero() {
    assert_eq!(eval_formula("count([])"), Ok(Value::Number(0.0)));
}

#[test]
fn evaluate_count_equals_len_for_same_array() {
    let reg = prepared_registry();
    let tokens_count = tokenize("count([1, 2, 3])").unwrap();
    let ast_count = parse(&tokens_count).unwrap();
    let tokens_len = tokenize("len([1, 2, 3])").unwrap();
    let ast_len = parse(&tokens_len).unwrap();
    let result_count = evaluate(&ast_count, &Context::new(), &reg).unwrap();
    let result_len = evaluate(&ast_len, &Context::new(), &reg).unwrap();
    assert_eq!(result_count, result_len);
}

#[test]
fn evaluate_join_array_returns_concatenated_string() {
    assert_eq!(
        eval_formula("join([\"a\", \"b\"], \", \")"),
        Ok(Value::String("a, b".to_string()))
    );
}

#[test]
fn evaluate_join_empty_separator_returns_direct_concatenation() {
    assert_eq!(
        eval_formula("join([\"x\", \"y\", \"z\"], \"\")"),
        Ok(Value::String("xyz".to_string()))
    );
}

#[test]
fn evaluate_join_empty_array_returns_empty_string() {
    assert_eq!(
        eval_formula("join([], \",\")"),
        Ok(Value::String("".to_string()))
    );
}

#[test]
fn evaluate_join_non_string_element_returns_function_error() {
    let result = eval_formula("join([1, 2], \"-\")");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::FunctionError);
    assert_eq!(err.code, "E501");
}

#[test]
fn evaluate_array_in_if_condition() {
    assert_eq!(
        eval_formula("if(len([1,2,3]) == 3, \"yes\", \"no\")"),
        Ok(Value::String("yes".to_string()))
    );
}

#[test]
fn evaluate_array_equality_same_elements_true() {
    assert_eq!(
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)])
    );
}

#[test]
fn evaluate_array_equality_different_elements_false() {
    assert_ne!(
        Value::Array(vec![Value::Number(1.0)]),
        Value::Array(vec![Value::Number(2.0)])
    );
}

// -- Map Literals --

#[test]
fn evaluate_empty_map_literal() {
    let result = eval_formula("{}").unwrap();
    match result {
        Value::Map(map) => assert!(map.is_empty()),
        _ => panic!("expected Map"),
    }
}

#[test]
fn evaluate_map_literal_with_single_pair() {
    let result = eval_formula("{name: \"Alice\"}").unwrap();
    let mut expected = HashMap::new();
    expected.insert("name".to_string(), Value::String("Alice".to_string()));
    match result {
        Value::Map(map) => {
            assert_eq!(map, expected);
        }
        _ => panic!("expected Map"),
    }
}

// -- Phase 8: Property Access & Indexing --

#[test]
fn evaluate_property_access_on_map_literal_returns_value() {
    assert_eq!(
        eval_formula("{name: \"Alice\"}.name"),
        Ok(Value::String("Alice".to_string()))
    );
}

#[test]
fn evaluate_property_access_nested_map_returns_deep_value() {
    assert_eq!(
        eval_formula("{user: {name: \"Bob\"}}.user.name"),
        Ok(Value::String("Bob".to_string()))
    );
}

#[test]
fn evaluate_property_access_from_context_map_returns_value() {
    let mut ctx = Context::new();
    let mut inner = HashMap::new();
    inner.insert("score".to_string(), Value::Number(95.0));
    ctx.set("user", Value::Map(inner));
    assert_eq!(eval_with_ctx("user.score", &ctx), Ok(Value::Number(95.0)));
}

#[test]
fn evaluate_property_not_found_returns_property_not_found_error() {
    let result = eval_formula("{name: \"Alice\"}.age");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::PropertyNotFound);
    assert_eq!(err.code, "E207");
}

#[test]
fn evaluate_property_access_on_non_map_returns_type_error() {
    let result = eval_formula("(1 + 1).name");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_property_access_on_null_returns_type_error() {
    let result = eval_formula("null.name");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_index_access_on_array_returns_element() {
    assert_eq!(eval_formula("[10, 20, 30][1]"), Ok(Value::Number(20.0)));
}

#[test]
fn evaluate_index_access_zero_returns_first_element() {
    assert_eq!(
        eval_formula("[\"a\", \"b\"][0]"),
        Ok(Value::String("a".to_string()))
    );
}

#[test]
fn evaluate_index_out_of_bounds_returns_index_out_of_bounds_error() {
    let result = eval_formula("[1, 2][5]");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::IndexOutOfBounds);
    assert_eq!(err.code, "E208");
}

#[test]
fn evaluate_index_negative_returns_index_out_of_bounds_error() {
    let result = eval_formula("[1, 2][-1]");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::IndexOutOfBounds);
    assert_eq!(err.code, "E208");
}

#[test]
fn evaluate_index_on_non_array_returns_type_error() {
    let result = eval_formula("\"hello\"[0]");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_index_with_non_number_returns_type_error() {
    let result = eval_formula("[1, 2][\"a\"]");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_index_on_map_returns_type_error() {
    let result = eval_formula("{a: 1}[0]");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::TypeError);
    assert_eq!(err.code, "E401");
}

#[test]
fn evaluate_chained_property_and_index_returns_nested_value() {
    assert_eq!(
        eval_formula("{users: [{name: \"Alice\"}, {name: \"Bob\"}]}.users[0].name"),
        Ok(Value::String("Alice".to_string()))
    );
}

#[test]
fn evaluate_deeply_nested_property_access_returns_leaf_value() {
    assert_eq!(
        eval_formula("{a: {b: {c: 42}}}.a.b.c"),
        Ok(Value::Number(42.0))
    );
}

#[test]
fn evaluate_index_with_expression_returns_computed_element() {
    assert_eq!(eval_formula("[10, 20, 30][1 + 1]"), Ok(Value::Number(30.0)));
}

#[test]
fn evaluate_property_access_in_function_call() {
    assert_eq!(
        eval_formula("len({items: [1, 2, 3]}.items)"),
        Ok(Value::Number(3.0))
    );
}

#[test]
fn evaluate_index_access_on_context_array_returns_element() {
    let mut ctx = Context::new();
    ctx.set(
        "arr",
        Value::Array(vec![Value::Number(100.0), Value::Number(200.0)]),
    );
    assert_eq!(eval_with_ctx("arr[1]", &ctx), Ok(Value::Number(200.0)));
}

#[test]
fn evaluate_chained_index_access_on_nested_arrays() {
    assert_eq!(
        eval_formula("[[1, 2], [3, 4]][1][0]"),
        Ok(Value::Number(3.0))
    );
}

#[test]
fn evaluate_property_access_last_element_out_of_chain_returns_error() {
    // {a: {b: 1}}.a.c — "c" doesn't exist
    let result = eval_formula("{a: {b: 1}}.a.c");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::PropertyNotFound);
    assert_eq!(err.code, "E207");
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
    assert_eq!(err.kind, ErrorKind::EvalError);
    assert_eq!(err.code, "E302"); // Recursion limit exceeded
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
    let result = eval_formula("sort([{age: 30, name: \"A\"}, {age: 20, name: \"B\"}], x => x.age)").unwrap();
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
    let result = eval_formula("unique([{id: 1, name: \"A\"}, {id: 2, name: \"B\"}, {id: 1, name: \"C\"}], x => x.id)").unwrap();
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

