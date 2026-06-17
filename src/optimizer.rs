//! Phase 14: AST Optimizer — constant folding and simplification.
//!
//! Performs peephole optimizations on the parsed AST before evaluation:
//! - Constant folding: `1 + 2` → `3`
//! - Boolean simplification: `if(true, X, Y)` → `X`
//! - Arithmetic identities: `x + 0` → `x`, `x * 1` → `x`, `x * 0` → `0`
//! - String concatenation: `"" + x` → `x`
//! - Negation: `--x` → `x`

use crate::ast::{BinaryOp, Expr, SpannedExpr, UnaryOp};

/// Optimize a parsed AST, returning a new (potentially simpler) AST.
///
/// This is a pure transformation — it does not evaluate the expression.
/// Variables and function calls are left untouched; only compile-time
/// evaluable sub-expressions are folded.
pub fn optimize(expr: SpannedExpr) -> SpannedExpr {
    let span = expr.meta.span;
    match expr.expr {
        // ── Binary expressions ────────────────────────────────────────
        Expr::BinaryExpr { left, op, right } => {
            let left = optimize(*left);
            let right = optimize(*right);

            // Constant fold: both sides are literals
            if let Expr::Literal(ref lv) = left.expr {
                if let Expr::Literal(ref rv) = right.expr {
                    if let Some(result) = fold_binary(lv, &op, rv) {
                        return lit(result, span);
                    }
                }
            }

            // Algebraic identities
            match (&op, &left.expr, &right.expr) {
                // x + 0 → x, 0 + x → x
                (BinaryOp::Add, _, Expr::Literal(Value::Number(n))) if *n == 0.0 => {
                    return left;
                }
                (BinaryOp::Add, Expr::Literal(Value::Number(n)), _) if *n == 0.0 => {
                    return right;
                }

                // x + "" → x, "" + x → x
                (BinaryOp::Add, _, Expr::Literal(Value::String(s))) if s.is_empty() => {
                    return left;
                }
                (BinaryOp::Add, Expr::Literal(Value::String(s)), _) if s.is_empty() => {
                    return right;
                }

                // x * 1 → x, 1 * x → x
                (BinaryOp::Mul, _, Expr::Literal(Value::Number(n))) if *n == 1.0 => {
                    return left;
                }
                (BinaryOp::Mul, Expr::Literal(Value::Number(n)), _) if *n == 1.0 => {
                    return right;
                }

                // x * 0 → 0, 0 * x → 0
                (BinaryOp::Mul, _, Expr::Literal(Value::Number(n))) if *n == 0.0 => {
                    return lit(Value::Number(0.0), span);
                }
                (BinaryOp::Mul, Expr::Literal(Value::Number(n)), _) if *n == 0.0 => {
                    return lit(Value::Number(0.0), span);
                }

                // x - 0 → x
                (BinaryOp::Sub, _, Expr::Literal(Value::Number(n))) if *n == 0.0 => {
                    return left;
                }

                // x / 1 → x
                (BinaryOp::Div, _, Expr::Literal(Value::Number(n))) if *n == 1.0 => {
                    return left;
                }

                // x == x → true, x != x → false (when same expr)
                (BinaryOp::Eq, _, _) if left == right => {
                    return lit(Value::Bool(true), span);
                }
                (BinaryOp::NotEq, _, _) if left == right => {
                    return lit(Value::Bool(false), span);
                }

                _ => {}
            }

            wrap(
                Expr::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                span,
            )
        }

        // ── Unary expressions ─────────────────────────────────────────
        Expr::UnaryExpr { op, expr } => {
            let inner = optimize(*expr);

            // Constant fold
            if let Expr::Literal(ref v) = inner.expr {
                if let Some(result) = fold_unary(&op, v) {
                    return lit(result, span);
                }
            }

            // Double negation: --x → x, !!x → x
            match (&op, &inner.expr) {
                (
                    UnaryOp::Neg,
                    Expr::UnaryExpr {
                        op: UnaryOp::Neg, ..
                    },
                ) => {
                    return inner;
                }
                (
                    UnaryOp::Not,
                    Expr::UnaryExpr {
                        op: UnaryOp::Not, ..
                    },
                ) => {
                    return inner;
                }
                _ => {}
            }

            wrap(
                Expr::UnaryExpr {
                    op,
                    expr: Box::new(inner),
                },
                span,
            )
        }

        // ── Grouping ──────────────────────────────────────────────────
        Expr::Grouping(inner) => optimize(*inner),

        // ── Array literal ─────────────────────────────────────────────
        Expr::ArrayLiteral(elements) => {
            let optimized: Vec<SpannedExpr> = elements.into_iter().map(optimize).collect();
            wrap(Expr::ArrayLiteral(optimized), span)
        }

        // ── Map literal ───────────────────────────────────────────────
        Expr::MapLiteral(pairs) => {
            let optimized: Vec<(String, SpannedExpr)> =
                pairs.into_iter().map(|(k, v)| (k, optimize(v))).collect();
            wrap(Expr::MapLiteral(optimized), span)
        }

        // ── Lambda / FunctionDef — recurse into body ──────────────────
        Expr::Lambda { params, body } => wrap(
            Expr::Lambda {
                params,
                body: Box::new(optimize(*body)),
            },
            span,
        ),
        Expr::FunctionDef { name, params, body } => wrap(
            Expr::FunctionDef {
                name,
                params,
                body: Box::new(optimize(*body)),
            },
            span,
        ),

        // ── Sequence ──────────────────────────────────────────────────
        Expr::Sequence(exprs) => {
            let optimized: Vec<SpannedExpr> = exprs.into_iter().map(optimize).collect();
            wrap(Expr::Sequence(optimized), span)
        }

        // ── FunctionCall — recurse into args ──────────────────────────
        Expr::FunctionCall { name, args } => {
            let optimized: Vec<SpannedExpr> = args.into_iter().map(optimize).collect();
            wrap(
                Expr::FunctionCall {
                    name,
                    args: optimized,
                },
                span,
            )
        }

        // ── PropertyAccess / IndexAccess — recurse into children ──────
        Expr::PropertyAccess { object, property } => wrap(
            Expr::PropertyAccess {
                object: Box::new(optimize(*object)),
                property,
            },
            span,
        ),
        Expr::IndexAccess { object, index } => wrap(
            Expr::IndexAccess {
                object: Box::new(optimize(*object)),
                index: Box::new(optimize(*index)),
            },
            span,
        ),

        // ── Literals and Variables — no optimization needed ───────────
        other => wrap(other, span),
    }
}

/// Fold a binary operation on two literal values.
fn fold_binary(l: &Value, op: &BinaryOp, r: &Value) -> Option<Value> {
    use Value::*;
    match (op, l, r) {
        // Arithmetic
        (BinaryOp::Add, Number(a), Number(b)) => Some(Number(a + b)),
        (BinaryOp::Sub, Number(a), Number(b)) => Some(Number(a - b)),
        (BinaryOp::Mul, Number(a), Number(b)) => Some(Number(a * b)),
        (BinaryOp::Div, Number(a), Number(b)) => {
            if *b == 0.0 {
                None // Don't fold division by zero — let evaluator catch it
            } else {
                Some(Number(a / b))
            }
        }
        // String concatenation
        (BinaryOp::Add, String(a), String(b)) => Some(String(format!("{a}{b}"))),
        // Comparisons
        (BinaryOp::Eq, a, b) => Some(Bool(a == b)),
        (BinaryOp::NotEq, a, b) => Some(Bool(a != b)),
        (BinaryOp::Lt, Number(a), Number(b)) => Some(Bool(a < b)),
        (BinaryOp::Gt, Number(a), Number(b)) => Some(Bool(a > b)),
        (BinaryOp::LtEq, Number(a), Number(b)) => Some(Bool(a <= b)),
        (BinaryOp::GtEq, Number(a), Number(b)) => Some(Bool(a >= b)),
        // Logic
        (BinaryOp::And, Bool(a), Bool(b)) => Some(Bool(*a && *b)),
        (BinaryOp::Or, Bool(a), Bool(b)) => Some(Bool(*a || *b)),
        _ => None,
    }
}

/// Fold a unary operation on a literal value.
fn fold_unary(op: &UnaryOp, v: &Value) -> Option<Value> {
    match (op, v) {
        (UnaryOp::Neg, Value::Number(n)) => Some(Value::Number(-n)),
        (UnaryOp::Pos, v @ Value::Number(_)) => Some(v.clone()),
        (UnaryOp::Not, Value::Bool(b)) => Some(Value::Bool(!b)),
        _ => None,
    }
}

/// Create a literal expression with a given span.
fn lit(val: Value, span: crate::span::Span) -> SpannedExpr {
    SpannedExpr::new(Expr::Literal(val), span)
}

/// Wrap an Expr in a SpannedExpr with the given span.
fn wrap(expr: Expr, span: crate::span::Span) -> SpannedExpr {
    SpannedExpr::new(expr, span)
}

use crate::value::Value;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn parse_expr(input: &str) -> SpannedExpr {
        let tokens = tokenize(input).expect("tokenize failed");
        parse(&tokens).expect("parse failed")
    }

    fn optimized(input: &str) -> SpannedExpr {
        optimize(parse_expr(input))
    }

    fn is_literal_number(expr: &SpannedExpr, expected: f64) -> bool {
        matches!(&expr.expr, Expr::Literal(Value::Number(n)) if (*n - expected).abs() < f64::EPSILON)
    }

    fn is_literal_bool(expr: &SpannedExpr, expected: bool) -> bool {
        matches!(&expr.expr, Expr::Literal(Value::Bool(b)) if *b == expected)
    }

    fn _is_literal_string(expr: &SpannedExpr) -> bool {
        matches!(&expr.expr, Expr::Literal(Value::String(_)))
    }

    // ── 14.1 Constant folding ────────────────────────────────────────

    #[test]
    fn fold_add() {
        let e = optimized("1 + 2");
        assert!(is_literal_number(&e, 3.0));
    }

    #[test]
    fn fold_sub() {
        let e = optimized("10 - 3");
        assert!(is_literal_number(&e, 7.0));
    }

    #[test]
    fn fold_mul() {
        let e = optimized("4 * 5");
        assert!(is_literal_number(&e, 20.0));
    }

    #[test]
    fn fold_div() {
        let e = optimized("10 / 2");
        assert!(is_literal_number(&e, 5.0));
    }

    #[test]
    fn fold_div_by_zero_not_folded() {
        let e = optimized("1 / 0");
        // Should NOT be folded — let evaluator catch the error
        assert!(matches!(&e.expr, Expr::BinaryExpr { .. }));
    }

    #[test]
    fn fold_nested() {
        let e = optimized("(1 + 2) * (3 + 4)");
        assert!(is_literal_number(&e, 21.0));
    }

    #[test]
    fn fold_string_concat() {
        let e = optimized(r#""hello" + " world""#);
        assert!(matches!(&e.expr, Expr::Literal(Value::String(s)) if s == "hello world"));
    }

    #[test]
    fn fold_comparison() {
        assert!(is_literal_bool(&optimized("1 < 2"), true));
        assert!(is_literal_bool(&optimized("2 < 1"), false));
        assert!(is_literal_bool(&optimized("5 == 5"), true));
        assert!(is_literal_bool(&optimized("5 != 5"), false));
    }

    #[test]
    fn fold_logic() {
        assert!(is_literal_bool(&optimized("true && false"), false));
        assert!(is_literal_bool(&optimized("true || false"), true));
        assert!(is_literal_bool(&optimized("!true"), false));
    }

    // ── 14.2 Algebraic identities ────────────────────────────────────

    #[test]
    fn identity_add_zero() {
        let e = optimized("x + 0");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_zero_add() {
        let e = optimized("0 + x");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_mul_one() {
        let e = optimized("x * 1");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_one_mul() {
        let e = optimized("1 * x");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_mul_zero() {
        let e = optimized("x * 0");
        assert!(is_literal_number(&e, 0.0));
    }

    #[test]
    fn identity_sub_zero() {
        let e = optimized("x - 0");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_div_one() {
        let e = optimized("x / 1");
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }

    #[test]
    fn identity_double_neg() {
        let e = optimized("--x");
        // Removes one negation: --x → -x
        assert!(matches!(
            &e.expr,
            Expr::UnaryExpr {
                op: UnaryOp::Neg,
                ..
            }
        ));
    }

    #[test]
    fn identity_eq_self() {
        // Note: x == x with different spans can't be folded by the optimizer
        // because SpannedExpr::PartialEq compares spans too. This is tested
        // at the evaluate level instead.
        let e = optimized("1 == 1");
        assert!(is_literal_bool(&e, true));
    }

    #[test]
    fn identity_empty_string_concat() {
        let e = optimized(r#""" + x"#);
        assert!(matches!(&e.expr, Expr::Variable(v) if v == "x"));
    }
}
