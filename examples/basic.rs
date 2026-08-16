//! Basic usage example for bl1z
//!
//! This example demonstrates the core functionality:
//! - Tokenizing formulas
//! - Parsing into AST
//! - Evaluating with context and functions
//! - Error handling

use bl1z::builtins;
use bl1z::{Context, FunctionRegistry, Value, evaluate, parse, tokenize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bl1z - Basic Example");
    println!("==============================\n");

    // Create function registry with all built-ins
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    // Example 1: Simple arithmetic
    println!("1. Simple Arithmetic:");
    evaluate_and_print("1 + 2 * 3", &Context::new(), &registry)?;

    // Example 2: String operations
    println!("\n2. String Operations:");
    evaluate_and_print("upper(\"hello world\")", &Context::new(), &registry)?;
    evaluate_and_print("len(\"bl1z\")", &Context::new(), &registry)?;

    // Example 3: Using context variables
    println!("\n3. Context Variables:");
    let mut ctx = Context::new();
    ctx.set("score", Value::Number(85.0));
    ctx.set("bonus", Value::Number(15.0));
    ctx.set("name", Value::String("Alice".to_string()));
    evaluate_and_print("score + bonus", &ctx, &registry)?;
    evaluate_and_print("if(score >= 80, \"Pass\", \"Fail\")", &ctx, &registry)?;

    // Example 4: Array operations
    println!("\n4. Array Operations:");
    evaluate_and_print("sum([1, 2, 3, 4, 5])", &Context::new(), &registry)?;
    evaluate_and_print("avg([10, 20, 30])", &Context::new(), &registry)?;
    evaluate_and_print("count([\"a\", \"b\", \"c\"])", &Context::new(), &registry)?;
    evaluate_and_print("join([\"hello\", \"world\"], \" \")", &Context::new(), &registry)?;

    // Example 5: Date operations
    println!("\n5. Date Operations:");
    evaluate_and_print("year(now())", &Context::new(), &registry)?;
    evaluate_and_print("date_add(\"2023-01-01\", 30)", &Context::new(), &registry)?;

    // Example 6: Complex nested expressions
    println!("\n6. Complex Expressions:");
    let complex_expr = r#"
        if(
            sum([score, bonus]) > 90,
            upper(name + " passed!"),
            lower(name + " needs improvement")
        )
    "#
    .trim();
    evaluate_and_print(complex_expr, &ctx, &registry)?;

    // Example 7: Error handling
    println!("\n7. Error Handling:");
    show_error_example("1 + \"string\"", &Context::new(), &registry);
    show_error_example("undefined_var", &Context::new(), &registry);
    show_error_example("sum(\"not_array\")", &Context::new(), &registry);

    println!("\nAll examples completed successfully!");
    Ok(())
}

fn evaluate_and_print(
    formula: &str,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Formula: {} ", formula);

    let tokens = tokenize(formula)?;
    let ast = parse(&tokens)?;
    let result = evaluate(&ast, ctx, registry)?;

    println!("→ {:?}", result);
    Ok(())
}

fn show_error_example(formula: &str, ctx: &Context, registry: &FunctionRegistry) {
    print!("Formula: {} ", formula);

    match (|| {
        let tokens = tokenize(formula)?;
        let ast = parse(&tokens)?;
        evaluate(&ast, ctx, registry)
    })() {
        Ok(result) => println!("→ {:?}", result),
        Err(err) => {
            println!("→ ERROR: {}", err.message);
            if let Some(span) = err.span {
                println!("   Location: line {}, column {}", span.start.line, span.start.column);
            }
        }
    }
}
