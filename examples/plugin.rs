//! Plugin example for bl1z
//!
//! Demonstrates the plugin SDK:
//! - Defining a `Plugin` with custom functions
//! - Registering it via `PluginManager`
//! - Merging plugin functions into the registry
//! - Evaluating formulas that use them

use bl1z::builtins;
use bl1z::error::{ErrorKind, FormulaError};
use bl1z::functions::{BuiltinFunction, FunctionRegistry};
use bl1z::plugins::{Plugin, PluginManager};
use bl1z::{evaluate, parse, tokenize, Context, Value};

/// A plugin adding extra math functions on top of the built-ins.
struct MathExtraPlugin;

impl Plugin for MathExtraPlugin {
    fn name(&self) -> &str {
        "math_extra"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn functions(&self) -> Vec<BuiltinFunction> {
        vec![
            BuiltinFunction {
                name: "square".to_string(),
                arity: 1,
                call: |args, _| match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n * n)),
                    _ => Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "square ต้องการตัวเลข",
                        None,
                    )),
                },
            },
            BuiltinFunction {
                name: "cube".to_string(),
                arity: 1,
                call: |args, _| match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n * n * n)),
                    _ => Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "cube ต้องการตัวเลข",
                        None,
                    )),
                },
            },
        ]
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Registry with built-ins, then merge plugin functions on top.
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);

    let mut manager = PluginManager::new();
    manager.register(Box::new(MathExtraPlugin));
    manager.merge_functions(&mut registry)?;

    println!("Registered plugins: {:?}", manager.plugin_names());

    let formulas = [
        ("square(5)", Context::new()),
        ("cube(3) + square(2)", Context::new()),
        ("square(sqrt(16))", Context::new()),
    ];

    for (formula, ctx) in formulas {
        let tokens = tokenize(formula)?;
        let ast = parse(&tokens)?;
        let result = evaluate(&ast, &ctx, &registry)?;
        println!("{formula} → {result:?}");
    }

    // Wrong argument type hits the plugin's own error path.
    let tokens = tokenize("square(\"x\")")?;
    let ast = parse(&tokens)?;
    match evaluate(&ast, &Context::new(), &registry) {
        Ok(v) => println!("square(\"x\") → {v:?}"),
        Err(e) => println!("square(\"x\") → ERROR: {}", e.message),
    }

    Ok(())
}
