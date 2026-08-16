//! JSON plugin example for bl1z
//!
//! Loads and runs an external Python plugin (math_extra.py) via python3.
//! Plugin functions are implemented as Python scripts; bl1z spawns the
//! interpreter, passes args as JSON on stdin, and reads results from stdout.
//!
//! Requires: python3 on PATH.
//! Run with: cargo run --example plugin_json --features serialization

use bl1z::builtins;
use bl1z::{Context, FunctionRegistry, evaluate, load_json_plugin, parse, tokenize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plugin_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/plugins/math_extra.json");

    let plugin = load_json_plugin(plugin_path)?;
    println!(
        "Loaded plugin: {} (id={}, v{}) — {} by {}",
        plugin.name, plugin.id, plugin.version, plugin.description, plugin.author
    );

    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    plugin.register_into(&mut registry)?;

    let formulas = [
        ("mod(7, 3)", Context::new()),
        ("gcd(48, 18)", Context::new()),
        ("is_prime(17)", Context::new()),
        ("primes_up_to(20)", Context::new()),
    ];

    for (formula, ctx) in formulas {
        let tokens = tokenize(formula)?;
        let ast = parse(&tokens)?;
        let result = evaluate(&ast, &ctx, &registry)?;
        println!("{formula} → {result:?}");
    }

    Ok(())
}
