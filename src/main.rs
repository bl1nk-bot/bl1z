//! bl1z CLI — command-line interface for the bl1z formula engine.
//!
//! Design follows cargo conventions: subcommands with `-h/--help`,
//! `-V/--version`, and exit codes (0 = ok, 1 = eval error, 2 = usage error).

use bl1z::builtins;
use bl1z::context::Context;
use bl1z::functions::FunctionRegistry;
use bl1z::value::Value;
use bl1z::{evaluate, parse, tokenize};
use std::io::{BufRead, IsTerminal};

mod plugins_cmd;
mod table;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const MAIN_HELP: &str = concat!(
    "bl1z ", env!("CARGO_PKG_VERSION"), "\nA high-performance, extensible formula evaluation engine for Rust\n\nUSAGE:\n    bl1z [OPTIONS] <COMMAND>\n\nCOMMANDS:\n    eval       Evaluate a formula and print the result\n    repl       Interactive calculator (open with: bl1z)\n    functions  List all available built-in functions\n    plugins    Install, link, list and manage plugins\n    help       Print this message\n\nOPTIONS:\n    -h, --help       Print help\n    -V, --version    Print version\n"
);

const EVAL_HELP: &str = "\
Evaluate a formula and print the result

USAGE:
    bl1z eval [OPTIONS] <FORMULA>

ARGS:
    <FORMULA>    The formula to evaluate

OPTIONS:
    -v, --var <NAME=VALUE>    Set a context variable (can be repeated)
    -p, --plugin <PATH>       Load a JSON plugin manifest (can be repeated)
    -h, --help                Print help

EXAMPLES:
    bl1z eval 'sum([1, 2, 3]) > 5'
    bl1z eval -v rate=0.2 '1000 * (1 - rate)'
    bl1z eval -p examples/plugins/math_extra.json 'mod(10, 3)'
";

const REPL_HELP: &str = "\
Interactive calculator — type a formula, get the answer. `bl1z` (no command)
opens it directly.

USAGE:
    bl1z repl [OPTIONS]

OPTIONS:
    -v, --var <NAME=VALUE>    Set a context variable (can be repeated)
    -p, --plugin <PATH>       Load a JSON plugin manifest (can be repeated)
    -h, --help                Print help

EXAMPLES:
    bl1z                      open the calculator
    echo '1 + 2' | bl1z repl  evaluate piped lines (no prompt)
    printf 'sqrt(144)\\n' | bl1z repl -v x=10
";

const FUNCTIONS_HELP: &str = "\
List all available built-in functions

USAGE:
    bl1z functions [OPTIONS]

OPTIONS:
    -h, --help    Print help
";

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        // `bl1z` เฉยๆ = เปิดเครื่องคิดเลข (เร็ว/ง่ายกว่าเครื่องคิดเลขแอป)
        return cmd_repl(&[]);
    };
    match cmd.as_str() {
        "-h" | "--help" | "help" => {
            print!("{MAIN_HELP}");
            std::process::ExitCode::SUCCESS
        }
        "-V" | "--version" => {
            println!("bl1z {VERSION}");
            std::process::ExitCode::SUCCESS
        }
        "eval" => cmd_eval(&args[1..]),
        "repl" => cmd_repl(&args[1..]),
        "functions" => cmd_functions(&args[1..]),
        "plugins" => plugins_cmd::run_plugins(&args[1..]),
        other => {
            eprintln!("error: unknown command `{other}`");
            if let Some(s) = suggest(other, &["eval", "repl", "functions", "plugins", "help"]) {
                eprintln!("did you mean `{s}`?");
            }
            eprintln!();
            print!("{MAIN_HELP}");
            std::process::ExitCode::from(2)
        }
    }
}

/// Shared option parsing result: (vars, plugin paths, positional formula, help).
type ParseOutcome = (Vec<(String, Value)>, Vec<String>, Option<String>, bool);

/// Shared option parsing: returns (vars, plugin paths, positional formula, help).
fn parse_args(args: &[String], allow_formula: bool) -> Result<ParseOutcome, String> {
    let mut vars = Vec::new();
    let mut plugins = Vec::new();
    let mut formula = None;
    let mut help = false;
    let mut positional_only = false;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-h" | "--help" => help = true,
            "-v" | "--var" => {
                let Some(spec) = it.next() else {
                    return Err(format!("option `{a}` requires a value"));
                };
                let (name, val) = spec
                    .split_once('=')
                    .ok_or_else(|| format!("`{spec}` ต้องเป็น NAME=VALUE"))?;
                vars.push((name.to_string(), parse_value(val)));
            }
            "-p" | "--plugin" => {
                let Some(path) = it.next() else {
                    return Err(format!("option `{a}` requires a value"));
                };
                plugins.push(path.clone());
            }
            "--" => positional_only = true,
            s if positional_only
                || (!s.starts_with('-') || s.len() == 1 || s.parse::<f64>().is_ok()) =>
            {
                if formula.is_some() {
                    return Err(format!("unexpected argument `{s}`"));
                }
                formula = Some(s.to_string());
            }
            s => return Err(format!("unknown option `{s}`")),
        }
    }
    if allow_formula && formula.is_none() && !help {
        return Err("missing required argument `<FORMULA>`".to_string());
    }
    if !allow_formula {
        if let Some(ref f) = formula {
            return Err(format!(
                "unexpected positional argument `{f}` — use `bl1z eval` for formulas"
            ));
        }
    }
    Ok((vars, plugins, formula, help))
}

/// Parse a NAME=VALUE literal: true/false/null, numbers, otherwise string.
fn parse_value(s: &str) -> Value {
    match s {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        _ => match s.parse::<f64>() {
            Ok(n) => Value::Number(n),
            Err(_) => Value::String(s.to_string()),
        },
    }
}

/// Build a registry with built-ins, enabled store plugins, plus `-p` extras.
fn build_registry(plugins: &[String]) -> Result<FunctionRegistry, bl1z::FormulaError> {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    for path in plugins_cmd::enabled_plugin_paths()?
        .into_iter()
        .chain(plugins.iter().cloned())
    {
        let plugin = bl1z::load_json_plugin(&path)?;
        plugin.register_into(&mut registry)?;
    }
    Ok(registry)
}

/// Evaluate one formula against a shared registry/context.
fn evaluate_line(
    line: &str,
    registry: &FunctionRegistry,
    ctx: &Context,
) -> Result<Value, bl1z::FormulaError> {
    let tokens = tokenize(line)?;
    let ast = parse(&tokens)?;
    evaluate(&ast, ctx, registry)
}

fn cmd_eval(args: &[String]) -> std::process::ExitCode {
    match parse_args(args, true) {
        Err(msg) => {
            eprintln!("error: {msg}\n\n{EVAL_HELP}");
            std::process::ExitCode::from(2)
        }
        Ok((vars, plugins, formula, help)) => {
            if help {
                print!("{EVAL_HELP}");
                return std::process::ExitCode::SUCCESS;
            }
            let formula = formula.expect("checked by parse_args");
            let mut ctx = Context::new();
            for (name, val) in &vars {
                ctx.set(name, val.clone());
            }
            match build_registry(&plugins).and_then(|reg| evaluate_line(&formula, &reg, &ctx)) {
                Ok(v) => {
                    println!("{v}");
                    std::process::ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::ExitCode::from(1)
                }
            }
        }
    }
}

fn cmd_repl(args: &[String]) -> std::process::ExitCode {
    match parse_args(args, false) {
        Err(msg) => {
            eprintln!("error: {msg}\n\n{REPL_HELP}");
            std::process::ExitCode::from(2)
        }
        Ok((vars, plugins, _, help)) => {
            if help {
                print!("{REPL_HELP}");
                return std::process::ExitCode::SUCCESS;
            }
            let mut ctx = Context::new();
            for (name, val) in &vars {
                ctx.set(name, val.clone());
            }
            let registry = match build_registry(&plugins) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("error: {e}");
                    return std::process::ExitCode::from(1);
                }
            };
            // โหมดโต้ตอบ = มี prompt + ผลลัพธ์แบบ `= value`; โหมด pipe = เงียบ
            let interactive = std::io::stdin().is_terminal();
            if interactive {
                println!("bl1z {VERSION} — พิมพ์สูตรแล้วกด Enter (Ctrl-D หรือ `exit` เพื่อออก)");
            }
            let mut stdin = std::io::stdin().lock();
            let mut buf = String::new();
            let mut exit = std::process::ExitCode::SUCCESS;
            loop {
                if interactive {
                    use std::io::Write;
                    print!("bl1z> ");
                    let _ = std::io::stdout().flush();
                }
                buf.clear();
                match stdin.read_line(&mut buf) {
                    Ok(0) => break, // Ctrl-D / EOF
                    Ok(_) => {}
                    Err(_) => break,
                }
                let trimmed = buf.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if interactive && matches!(trimmed, "exit" | "quit" | ":q") {
                    break;
                }
                match evaluate_line(trimmed, &registry, &ctx) {
                    Ok(v) => {
                        if interactive {
                            println!("= {v}");
                        } else {
                            println!("{v}");
                        }
                    }
                    Err(e) => {
                        if interactive {
                            println!("error: {e}");
                        } else {
                            eprintln!("error: {e}");
                            exit = std::process::ExitCode::from(1);
                        }
                    }
                }
            }
            exit
        }
    }
}

fn cmd_functions(args: &[String]) -> std::process::ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FUNCTIONS_HELP}");
        return std::process::ExitCode::SUCCESS;
    }
    if let Some(a) = args.first() {
        if a.starts_with('-') {
            eprintln!("error: unknown option `{a}`\n\n{FUNCTIONS_HELP}");
        } else {
            eprintln!("error: unexpected argument `{a}`\n\n{FUNCTIONS_HELP}");
        }
        return std::process::ExitCode::from(2);
    }
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    for name in registry.names() {
        println!("{name}");
    }
    std::process::ExitCode::SUCCESS
}

/// Levenshtein distance — small, used only for command suggestions.
pub(crate) fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            cur[j + 1] = (prev[j + 1] + 1)
                .min(cur[j] + 1)
                .min(prev[j] + usize::from(ca != cb));
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

/// Closest known command within edit distance 2, for "did you mean?".
pub(crate) fn suggest(cmd: &str, known: &[&'static str]) -> Option<&'static str> {
    let mut best: Option<(&str, usize)> = None;
    for k in known {
        let d = edit_distance(cmd, k);
        if d <= 2 && best.is_none_or(|(_, bd)| d < bd) {
            best = Some((k, d));
        }
    }
    best.map(|(k, _)| k)
}
