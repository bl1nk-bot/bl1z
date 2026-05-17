//! Performance profiling utilities for Formula Engine.
//!
//! This module provides tools for analyzing and optimizing formula evaluation performance.

use crate::{error::FormulaError, evaluate, parse, tokenize, Context, FunctionRegistry};
use std::time::{Duration, Instant};

/// Performance metrics for formula evaluation.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total time spent tokenizing
    pub tokenize_time: Duration,
    /// Total time spent parsing
    pub parse_time: Duration,
    /// Total time spent evaluating
    pub eval_time: Duration,
    /// Total time for complete process
    pub total_time: Duration,
    /// Number of iterations measured
    pub iterations: usize,
}

/// Profile the performance of formula evaluation.
///
/// Measures timing for tokenization, parsing, and evaluation phases
/// across multiple iterations for statistical accuracy.
///
/// # Arguments
/// * `formula` - The formula to profile
/// * `ctx` - Context for variable resolution
/// * `registry` - Function registry
/// * `iterations` - Number of times to run the measurement
///
/// # Returns
/// * `Ok(PerformanceMetrics)` - Detailed timing information
/// * `Err(FormulaError)` - If formula evaluation fails
///
/// # Examples
///
/// ```
/// use bl1z::{Context, FunctionRegistry, builtins};
/// use bl1z::profiling::profile_formula;
///
/// let mut registry = FunctionRegistry::new();
/// builtins::register_all(&mut registry);
/// let ctx = Context::new();
///
/// let metrics = profile_formula("sum([1,2,3,4,5])", &ctx, &registry, 1000).unwrap();
/// println!("Average evaluation time: {:?}", metrics.eval_time / metrics.iterations as u32);
/// ```
pub fn profile_formula(
    formula: &str,
    ctx: &Context,
    registry: &FunctionRegistry,
    iterations: usize,
) -> Result<PerformanceMetrics, FormulaError> {
    let mut tokenize_times = Vec::with_capacity(iterations);
    let mut parse_times = Vec::with_capacity(iterations);
    let mut eval_times = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        // Tokenize
        let tokenize_start = Instant::now();
        let tokens = tokenize(formula)?;
        let tokenize_time = tokenize_start.elapsed();
        tokenize_times.push(tokenize_time);

        // Parse
        let parse_start = Instant::now();
        let ast = parse(&tokens)?;
        let parse_time = parse_start.elapsed();
        parse_times.push(parse_time);

        // Evaluate
        let eval_start = Instant::now();
        let _result = evaluate(&ast, ctx, registry)?;
        let eval_time = eval_start.elapsed();
        eval_times.push(eval_time);
    }

    // Calculate averages
    let avg_tokenize = tokenize_times.iter().sum::<Duration>() / iterations as u32;
    let avg_parse = parse_times.iter().sum::<Duration>() / iterations as u32;
    let avg_eval = eval_times.iter().sum::<Duration>() / iterations as u32;
    let total_avg = avg_tokenize + avg_parse + avg_eval;

    Ok(PerformanceMetrics {
        tokenize_time: avg_tokenize,
        parse_time: avg_parse,
        eval_time: avg_eval,
        total_time: total_avg,
        iterations,
    })
}

/// Performance optimization suggestions based on formula analysis.
#[derive(Debug, Clone)]
pub struct OptimizationSuggestions {
    /// Suggestions for improving performance
    pub suggestions: Vec<String>,
    /// Estimated complexity of the formula
    pub complexity: FormulaComplexity,
}

/// Estimated computational complexity of a formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum FormulaComplexity {
    /// Simple arithmetic or single function call
    Simple,
    /// Multiple operations, small arrays
    Moderate,
    /// Large arrays, nested functions, complex expressions
    Complex,
    /// Very large datasets, recursive operations
    High,
}

/// Analyze a formula and provide optimization suggestions.
///
/// Examines the AST structure to identify potential performance bottlenecks
/// and suggests optimizations.
///
/// # Arguments
/// * `formula` - The formula to analyze
///
/// # Returns
/// * `Ok(OptimizationSuggestions)` - Analysis results and suggestions
/// * `Err(FormulaError)` - If formula parsing fails
///
/// # Examples
///
/// ```
/// use bl1z::profiling::analyze_formula;
///
/// let analysis = analyze_formula("sum([1,2,3,4,5,6,7,8,9,10])").unwrap();
/// println!("Complexity: {:?}", analysis.complexity);
/// for suggestion in &analysis.suggestions {
///     println!("- {}", suggestion);
/// }
/// ```
pub fn analyze_formula(formula: &str) -> Result<OptimizationSuggestions, FormulaError> {
    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;

    let mut suggestions = Vec::new();
    let mut complexity = FormulaComplexity::Simple;

    // Analyze AST structure
    analyze_ast(&ast, &mut suggestions, &mut complexity);

    Ok(OptimizationSuggestions {
        suggestions,
        complexity,
    })
}

fn analyze_ast(
    ast: &crate::ast::SpannedExpr,
    suggestions: &mut Vec<String>,
    complexity: &mut FormulaComplexity,
) {
    use crate::ast::Expr;

    match &ast.expr {
        Expr::ArrayLiteral(elements) => {
            if elements.len() > 100 {
                *complexity = FormulaComplexity::High;
                suggestions.push(
                    "Consider using external data sources for large arrays (>100 elements)"
                        .to_string(),
                );
            } else if elements.len() > 20 {
                *complexity = std::cmp::max(*complexity, FormulaComplexity::Complex);
                suggestions.push(
                    "Large arrays detected. Consider if all elements are needed.".to_string(),
                );
            }

            // Analyze nested elements
            for element in elements {
                analyze_ast(element, suggestions, complexity);
            }
        }
        Expr::FunctionCall { name, args } => {
            match name.as_str() {
                "sum" | "avg" | "min" | "max" if args.len() == 1 => {
                    suggestions.push(format!("Consider caching results of {} operations", name));
                }
                _ => {}
            }

            // Analyze arguments
            for arg in args {
                analyze_ast(arg, suggestions, complexity);
            }
        }
        Expr::BinaryExpr { left, right, .. } => {
            analyze_ast(left, suggestions, complexity);
            analyze_ast(right, suggestions, complexity);
        }
        Expr::UnaryExpr { expr, .. } => {
            analyze_ast(expr, suggestions, complexity);
        }
        _ => {} // Other expressions are typically simple
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins;

    #[test]
    fn test_profile_formula() {
        let mut registry = FunctionRegistry::new();
        builtins::register_all(&mut registry);
        let ctx = Context::new();

        let metrics = profile_formula("1 + 2 * 3", &ctx, &registry, 10).unwrap();

        assert_eq!(metrics.iterations, 10);
        assert!(metrics.total_time > Duration::ZERO);
        assert!(metrics.tokenize_time >= Duration::ZERO);
        assert!(metrics.parse_time >= Duration::ZERO);
        assert!(metrics.eval_time >= Duration::ZERO);
    }

    #[test]
    fn test_analyze_simple_formula() {
        let analysis = analyze_formula("1 + 2").unwrap();
        assert_eq!(analysis.complexity, FormulaComplexity::Simple);
        assert!(analysis.suggestions.is_empty());
    }

    #[test]
    fn test_analyze_large_array() {
        let large_array = format!(
            "sum([{}])",
            (0..150)
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let analysis = analyze_formula(&large_array).unwrap();
        assert_eq!(analysis.complexity, FormulaComplexity::High);
        assert!(!analysis.suggestions.is_empty());
        assert!(analysis
            .suggestions
            .iter()
            .any(|s| s.contains("external data sources")));
    }
}
