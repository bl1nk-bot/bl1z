//! Advanced usage example demonstrating:
//! - Custom function registration
//! - Complex data structures
//! - Performance considerations
//! - Error handling patterns

use bl1z::builtins;
use bl1z::{
    error::FormulaError, evaluate, functions::BuiltinFunction, parse, tokenize, Context,
    FunctionRegistry, Value,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Formula Engine - Advanced Example");
    println!("=================================\n");

    // Create registry with built-ins
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    // Register custom functions
    register_custom_functions(&mut registry);

    // Create complex context
    let ctx = create_complex_context();

    // Demonstrate custom functions
    println!("1. Custom Functions:");
    evaluate_and_print("fibonacci(8)", &ctx, &registry)?;
    evaluate_and_print("power(2, 10)", &ctx, &registry)?;
    evaluate_and_print("is_even(42)", &ctx, &registry)?;
    evaluate_and_print("clamp(15, 10, 20)", &ctx, &registry)?;

    // Complex data structures
    println!("\n2. Complex Data Structures:");
    evaluate_and_print("user.score", &ctx, &registry)?;
    evaluate_and_print("sum(user.scores)", &ctx, &registry)?;
    evaluate_and_print("count(user.tags)", &ctx, &registry)?;

    // Advanced array operations
    println!("\n3. Advanced Array Operations:");
    evaluate_and_print(
        "max([fibonacci(1), fibonacci(2), fibonacci(3)])",
        &ctx,
        &registry,
    )?;
    evaluate_and_print("avg([10.5, 20.3, 15.8, 12.1])", &ctx, &registry)?;

    // Date manipulation
    println!("\n4. Date Manipulation:");
    evaluate_and_print(
        "date_diff(now(), \"2023-01-01\", \"days\")",
        &ctx,
        &registry,
    )?;
    evaluate_and_print("month(date_add(user.created_at, 365))", &ctx, &registry)?;

    // Nested expressions
    println!("\n5. Nested Expressions:");
    let nested = r#"
        if(
            user.score >= 80 && count(user.completed_tasks) > 5,
            "Advanced: " + upper(user.name),
            "Beginner: " + lower(user.name)
        )
    "#
    .trim();
    evaluate_and_print(nested, &ctx, &registry)?;

    // Performance demonstration
    println!("\n6. Performance Test (100 iterations):");
    performance_test(&ctx, &registry)?;

    println!("\nAdvanced examples completed!");
    Ok(())
}

fn register_custom_functions(registry: &mut FunctionRegistry) {
    // Fibonacci function
    fn fibonacci(args: &[Value]) -> Result<Value, FormulaError> {
        match args.first() {
            Some(Value::Number(n)) if *n >= 0.0 && n.fract() == 0.0 => {
                let n = *n as u32;
                let result = fib_recursive(n);
                Ok(Value::Number(result as f64))
            }
            _ => Err(FormulaError::new(
                bl1z::error::ErrorKind::TypeError,
                "E401",
                "fibonacci expects non-negative integer",
                None,
            )),
        }
    }

    fn fib_recursive(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib_recursive(n - 1) + fib_recursive(n - 2),
        }
    }

    // Power function
    fn power(args: &[Value]) -> Result<Value, FormulaError> {
        match (args.first(), args.get(1)) {
            (Some(Value::Number(base)), Some(Value::Number(exp))) => {
                Ok(Value::Number(base.powf(*exp)))
            }
            _ => Err(FormulaError::new(
                bl1z::error::ErrorKind::TypeError,
                "E401",
                "power expects two numbers",
                None,
            )),
        }
    }

    // Is even function
    fn is_even(args: &[Value]) -> Result<Value, FormulaError> {
        match args.first() {
            Some(Value::Number(n)) if n.fract() == 0.0 => Ok(Value::Bool((*n as i64) % 2 == 0)),
            _ => Err(FormulaError::new(
                bl1z::error::ErrorKind::TypeError,
                "E401",
                "is_even expects integer",
                None,
            )),
        }
    }

    // Clamp function
    fn clamp(args: &[Value]) -> Result<Value, FormulaError> {
        match (args.first(), args.get(1), args.get(2)) {
            (Some(Value::Number(val)), Some(Value::Number(min)), Some(Value::Number(max))) => {
                let clamped = val.max(*min).min(*max);
                Ok(Value::Number(clamped))
            }
            _ => Err(FormulaError::new(
                bl1z::error::ErrorKind::TypeError,
                "E401",
                "clamp expects three numbers: value, min, max",
                None,
            )),
        }
    }

    registry.register(BuiltinFunction {
        name: "fibonacci".to_string(),
        arity: 1,
        call: fibonacci,
    });

    registry.register(BuiltinFunction {
        name: "power".to_string(),
        arity: 2,
        call: power,
    });

    registry.register(BuiltinFunction {
        name: "is_even".to_string(),
        arity: 1,
        call: is_even,
    });

    registry.register(BuiltinFunction {
        name: "clamp".to_string(),
        arity: 3,
        call: clamp,
    });
}

fn create_complex_context() -> Context {
    let mut ctx = Context::new();

    // Basic values
    ctx.set("app_version", Value::String("1.2.3".to_string()));
    ctx.set("debug_mode", Value::Bool(true));

    // User object (using Map)
    let mut user = std::collections::HashMap::new();
    user.insert(
        "name".to_string(),
        Value::String("Alice Johnson".to_string()),
    );
    user.insert("score".to_string(), Value::Number(87.5));
    user.insert("level".to_string(), Value::Number(3.0));
    user.insert("active".to_string(), Value::Bool(true));
    user.insert(
        "created_at".to_string(),
        Value::String("2023-06-15".to_string()),
    );

    // Nested arrays
    user.insert(
        "scores".to_string(),
        Value::Array(vec![
            Value::Number(85.0),
            Value::Number(90.0),
            Value::Number(87.5),
            Value::Number(92.0),
        ]),
    );

    user.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("premium".to_string()),
            Value::String("active".to_string()),
            Value::String("beta_tester".to_string()),
        ]),
    );

    user.insert(
        "completed_tasks".to_string(),
        Value::Array(vec![
            Value::String("tutorial".to_string()),
            Value::String("level_1".to_string()),
            Value::String("level_2".to_string()),
            Value::String("level_3".to_string()),
            Value::String("advanced".to_string()),
            Value::String("expert".to_string()),
        ]),
    );

    ctx.set("user", Value::Map(user));

    ctx
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

fn performance_test(
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let formula = "sum([fibonacci(10), fibonacci(12), fibonacci(8)]) + user.score";
    let start = Instant::now();

    for _ in 0..100 {
        let tokens = tokenize(formula)?;
        let ast = parse(&tokens)?;
        let _result = evaluate(&ast, ctx, registry)?;
    }

    let elapsed = start.elapsed();
    println!(
        "100 iterations took: {:.2}ms (avg: {:.3}ms per iteration)",
        elapsed.as_millis(),
        elapsed.as_millis() as f64 / 100.0
    );

    Ok(())
}
