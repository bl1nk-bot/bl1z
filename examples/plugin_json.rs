//! JSON plugin example for bl1z
//!
//! Demonstrates loading plugin functions from a plugin.json file — no Rust
//! code, no compile step. The function body is a bl1z expression, so the
//! plugin language is the formula language.
//!
//! Run with: cargo run --example plugin_json --features serialization

use bl1z::builtins;
use bl1z::{evaluate, load_json_plugin, parse, tokenize, Context, FunctionRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plugin_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/plugins/math_extra.json"
    );

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
