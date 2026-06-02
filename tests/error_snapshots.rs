use bl1z::builtins;
use bl1z::diagnostics::format_error;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry};
use insta::assert_snapshot;

/// Test error formatting snapshots
#[cfg(test)]
mod error_snapshots {
    use super::*;

    fn setup() -> (Context, FunctionRegistry) {
        let mut registry = FunctionRegistry::new();
        builtins::register_all(&mut registry);
        let ctx = Context::new();
        (ctx, registry)
    }

    #[test]
    fn test_syntax_error_unterminated_string() {
        let result = tokenize("\"hello world");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unterminated_string", format_error("\"hello world", &err));
    }

    #[test]
    fn test_syntax_error_invalid_token() {
        let result = tokenize("1 + \\");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("invalid_token", format_error("1 + \\", &err));
    }

    #[test]
    fn test_parse_error_unclosed_parenthesis() {
        let tokens = tokenize("1 + (2 * 3").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unclosed_parenthesis", format_error("1 + (2 * 3", &err));
    }

    #[test]
    fn test_parse_error_unclosed_array() {
        let tokens = tokenize("[1, 2, 3").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unclosed_array", format_error("[1, 2, 3", &err));
    }

    #[test]
    fn test_parse_error_unclosed_map() {
        let tokens = tokenize("{key: value").unwrap();
        let result = parse(&tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unclosed_map", format_error("{key: value", &err));
    }

    #[test]
    fn test_eval_error_division_by_zero() {
        let (ctx, registry) = setup();
        let tokens = tokenize("10 / 0").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("division_by_zero", format_error("10 / 0", &err));
    }

    #[test]
    fn test_eval_error_unknown_function() {
        let (ctx, registry) = setup();
        let tokens = tokenize("unknown_func(1)").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unknown_function", format_error("unknown_func(1)", &err));
    }

    #[test]
    fn test_eval_error_wrong_argument_count() {
        let (ctx, registry) = setup();
        let tokens = tokenize("len(\"hello\", \"world\")").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!(
            "wrong_argument_count",
            format_error("len(\"hello\", \"world\")", &err)
        );
    }

    #[test]
    fn test_eval_error_type_mismatch() {
        let (ctx, registry) = setup();
        let tokens = tokenize("1 + \"string\"").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("type_mismatch", format_error("1 + \"string\"", &err));
    }

    #[test]
    fn test_eval_error_unknown_variable() {
        let (ctx, registry) = setup();
        let tokens = tokenize("unknown_var").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!("unknown_variable", format_error("unknown_var", &err));
    }

    #[test]
    fn test_eval_error_array_function_on_wrong_type() {
        let (ctx, registry) = setup();
        let tokens = tokenize("sum(\"not_an_array\")").unwrap();
        let ast = parse(&tokens).unwrap();
        let result = evaluate(&ast, &ctx, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_snapshot!(
            "array_function_wrong_type",
            format_error("sum(\"not_an_array\")", &err)
        );
    }
}
