//! Plugin store management for the bl1z CLI.
//!
//! Store layout: `<store>/<id>/plugin.json` + `<store>/state.json`
//! mapping each plugin id to `{ enabled, path }` (path may point outside
//! the store for `link`ed plugins). Default store: `~/.bl1z/plugins`,
//! overridable with `BL1Z_PLUGINS_DIR`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use bl1z::error::{ErrorKind, FormulaError};
use bl1z::load_json_plugin;

#[derive(Serialize, Deserialize)]
struct PluginEntry {
    #[serde(default = "default_true")]
    enabled: bool,
    path: String,
}

fn default_true() -> bool {
    true
}

/// Store root. `BL1Z_PLUGINS_DIR` overrides `~/.bl1z/plugins`.
pub fn store_dir() -> PathBuf {
    if let Ok(d) = std::env::var("BL1Z_PLUGINS_DIR") {
        return PathBuf::from(d);
    }
    // ponytail: HOME-only, no XDG_DATA_HOME; add when Linux desktop matters
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".bl1z").join("plugins")
}

fn state_path() -> PathBuf {
    store_dir().join("state.json")
}

fn load_state() -> Result<BTreeMap<String, PluginEntry>, FormulaError> {
    let p = state_path();
    if !p.exists() {
        return Ok(BTreeMap::new());
    }
    let text = fs::read_to_string(&p).map_err(io_err("อ่าน state.json"))?;
    serde_json::from_str(&text).map_err(|e| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E805",
            &format!("state.json เสีย: {e} (ลบไฟล์ได้เพื่อรีเซ็ต)"),
            None,
        )
    })
}

fn save_state(state: &BTreeMap<String, PluginEntry>) -> Result<(), FormulaError> {
    fs::create_dir_all(store_dir()).map_err(io_err("สร้าง plugin store"))?;
    let text = serde_json::to_string_pretty(state).map_err(|e| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E805",
            &format!("state.json เขียนไม่ได้: {e}"),
            None,
        )
    })?;
    fs::write(state_path(), text).map_err(io_err("เขียน state.json"))
}

fn io_err(what: &str) -> impl Fn(std::io::Error) -> FormulaError + '_ {
    move |e| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E805",
            &format!("{what}: {e}"),
            None,
        )
    }
}

/// Manifest paths of all enabled plugins — used by eval/repl auto-load.
pub fn enabled_plugin_paths() -> Result<Vec<String>, FormulaError> {
    Ok(load_state()?
        .into_iter()
        .filter(|(_, e)| e.enabled)
        .map(|(_, e)| e.path)
        .collect())
}

fn get<'a>(
    state: &'a BTreeMap<String, PluginEntry>,
    id: &str,
) -> Result<&'a PluginEntry, FormulaError> {
    state.get(id).ok_or_else(|| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E806",
            &format!("ไม่มีปลั๊กอิน '{id}' — ดู `bl1z plugins list`"),
            None,
        )
    })
}

// ── subcommands ───────────────────────────────────────────────────────────

pub fn run_plugins(args: &[String]) -> std::process::ExitCode {
    let Some(cmd) = args.first() else {
        print!("{PLUGINS_HELP}");
        return std::process::ExitCode::SUCCESS;
    };
    match cmd.as_str() {
        "-h" | "--help" | "help" => {
            print!("{PLUGINS_HELP}");
            std::process::ExitCode::SUCCESS
        }
        "install" => sub_install(&args[1..]),
        "list" => sub_list(&args[1..]),
        "uninstall" => sub_uninstall(&args[1..]),
        "enable" => sub_set_enabled(&args[1..], true),
        "disable" => sub_set_enabled(&args[1..], false),
        "link" => sub_link(&args[1..]),
        "reload" => sub_reload(&args[1..]),
        "debug" => sub_debug(&args[1..]),
        "fmt" => sub_fmt(&args[1..], false),
        "fix" => sub_fmt(&args[1..], true),
        other => {
            eprintln!("error: unknown plugin command `{other}`\n\n{PLUGINS_HELP}");
            std::process::ExitCode::from(2)
        }
    }
}

fn sub_install(args: &[String]) -> std::process::ExitCode {
    let Some(src) = args.first() else {
        eprintln!("error: missing argument `<SOURCE>`\n\n{INSTALL_HELP}");
        return std::process::ExitCode::from(2);
    };
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{INSTALL_HELP}");
        return std::process::ExitCode::SUCCESS;
    }
    let url = match resolve_source(src) {
        Ok(u) => u,
        Err(msg) => {
            eprintln!("error: {msg}");
            return std::process::ExitCode::from(2);
        }
    };
    // ponytail: shelling out to curl (present on Termux/most Unix); swap to
    // ureq/reqwest when Windows support or sandboxing matters
    let tmp = store_dir().join(".download.json");
    if let Err(e) = fs::create_dir_all(store_dir()) {
        return report(&io_err("สร้าง plugin store")(e));
    }
    let status = Command::new("curl")
        .args(["-fsSL", &url, "-o"])
        .arg(&tmp)
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(_) => {
            eprintln!("error: ดาวน์โหลด {url} ไม่สำเร็จ (curl exit non-zero)");
            return std::process::ExitCode::from(1);
        }
        Err(e) => {
            eprintln!("error: เรียก curl ไม่ได้: {e}");
            return std::process::ExitCode::from(1);
        }
    }
    match install_from(&tmp, url.as_str()) {
        Ok(msg) => {
            println!("{msg}");
            std::process::ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::ExitCode::from(1)
        }
    }
}

/// Accepts a URL, `github:user/repo`, or bare `user/repo`.
fn resolve_source(src: &str) -> Result<String, String> {
    if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("file://") {
        return Ok(src.to_string());
    }
    let repo = src.strip_prefix("github:").unwrap_or(src);
    if !repo.contains('/') {
        return Err(format!(
            "source `{src}` ไม่ใช่ URL หรือ GitHub repo (user/repo)"
        ));
    }
    if Path::new(src).exists() {
        return Err(format!(
            "`{src}` เป็นไฟล์ในเครื่อง — ใช้ `bl1z plugins link` แทน"
        ));
    }
    Ok(format!(
        "https://raw.githubusercontent.com/{repo}/HEAD/plugin.json"
    ))
}

fn install_from(manifest_path: &Path, source: &str) -> Result<String, FormulaError> {
    let plugin = load_json_plugin(manifest_path.to_str().expect("path"))?;
    let dest_dir = store_dir().join(&plugin.id);
    fs::create_dir_all(&dest_dir).map_err(io_err("สร้าง plugin dir"))?;
    let dest = dest_dir.join("plugin.json");
    fs::copy(manifest_path, &dest).map_err(io_err("คัดลอก plugin.json"))?;
    let _ = fs::remove_file(manifest_path);

    let mut state = load_state()?;
    state.insert(
        plugin.id.clone(),
        PluginEntry {
            enabled: true,
            path: dest.display().to_string(),
        },
    );
    save_state(&state)?;
    Ok(format!(
        "Installed {} v{} (id={}) จาก {source}",
        plugin.name, plugin.version, plugin.id
    ))
}

fn sub_list(args: &[String]) -> std::process::ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{LIST_HELP}");
        return std::process::ExitCode::SUCCESS;
    }
    let state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    if state.is_empty() {
        println!(
            "ยังไม่มีปลั๊กอิน — `bl1z plugins install <url|user/repo>` หรือ `bl1z plugins link <path>`"
        );
        return std::process::ExitCode::SUCCESS;
    }
    println!(
        "{:<20} {:<24} {:<10} {:<10}",
        "ID", "NAME", "VERSION", "STATUS"
    );
    for (id, entry) in &state {
        let (name, version) = match load_json_plugin(&entry.path) {
            Ok(p) => (p.name, p.version),
            Err(_) => ("?".to_string(), "?".to_string()),
        };
        let status = if entry.enabled { "enabled" } else { "disabled" };
        println!("{id:<20} {name:<24} {version:<10} {status}");
    }
    println!("\nstore: {}", store_dir().display());
    std::process::ExitCode::SUCCESS
}

fn sub_uninstall(args: &[String]) -> std::process::ExitCode {
    let Some(id) = args.first() else {
        eprintln!("error: missing argument `<ID>`\n\n{UNINSTALL_HELP}");
        return std::process::ExitCode::from(2);
    };
    let mut state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    let Some(entry) = state.remove(id) else {
        eprintln!("error: ไม่มีปลั๊กอิน '{id}'");
        return std::process::ExitCode::from(1);
    };
    // ลบไฟล์ใน store เท่านั้น — linked path ภายนอกไม่แตะ
    let entry_path = PathBuf::from(&entry.path);
    if entry_path.starts_with(store_dir()) {
        if let Some(parent) = entry_path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
    match save_state(&state) {
        Ok(()) => {
            println!("Uninstalled {id}");
            std::process::ExitCode::SUCCESS
        }
        Err(e) => report(&e),
    }
}

fn sub_set_enabled(args: &[String], enabled: bool) -> std::process::ExitCode {
    let verb = if enabled { "Enabled" } else { "Disabled" };
    let Some(id) = args.first() else {
        eprintln!("error: missing argument `<ID>`");
        return std::process::ExitCode::from(2);
    };
    let mut state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    let entry = match get(&state, id) {
        Ok(e) => e,
        Err(e) => return report(&e),
    };
    let _ = entry; // borrow ends here
    if let Some(e) = state.get_mut(id) {
        e.enabled = enabled;
    }
    match save_state(&state) {
        Ok(()) => {
            println!("{verb} {id}");
            std::process::ExitCode::SUCCESS
        }
        Err(e) => report(&e),
    }
}

fn sub_link(args: &[String]) -> std::process::ExitCode {
    let Some(path) = args.first() else {
        eprintln!("error: missing argument `<PATH>`\n\n{LINK_HELP}");
        return std::process::ExitCode::from(2);
    };
    let abs = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {path}: {e}");
            return std::process::ExitCode::from(1);
        }
    };
    let manifest = if abs.is_dir() {
        abs.join("plugin.json")
    } else {
        abs.clone()
    };
    let plugin = match load_json_plugin(manifest.to_str().expect("path")) {
        Ok(p) => p,
        Err(e) => return report(&e),
    };
    let mut state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    state.insert(
        plugin.id.clone(),
        PluginEntry {
            enabled: true,
            path: manifest.display().to_string(),
        },
    );
    match save_state(&state) {
        Ok(()) => {
            println!(
                "Linked {} v{} (id={}) ← {}",
                plugin.name,
                plugin.version,
                plugin.id,
                manifest.display()
            );
            std::process::ExitCode::SUCCESS
        }
        Err(e) => report(&e),
    }
}

fn sub_reload(args: &[String]) -> std::process::ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{RELOAD_HELP}");
        return std::process::ExitCode::SUCCESS;
    }
    let mut state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    // รับ plugin ที่อยู่ใน store แต่ยังไม่ลง state
    if let Ok(entries) = fs::read_dir(store_dir()) {
        for dir in entries.flatten() {
            let manifest = dir.path().join("plugin.json");
            if manifest.is_file() {
                if let Ok(p) = load_json_plugin(manifest.to_str().expect("path")) {
                    state.entry(p.id.clone()).or_insert(PluginEntry {
                        enabled: true,
                        path: manifest.display().to_string(),
                    });
                }
            }
        }
    }
    if let Err(e) = save_state(&state) {
        return report(&e);
    }
    let (mut ok, mut broken) = (0, 0);
    for (id, entry) in &state {
        match load_json_plugin(&entry.path) {
            Ok(p) => {
                let status = if entry.enabled { "enabled" } else { "disabled" };
                println!("  {id:<20} v{:<10} {status}  {}", p.version, p.name);
                ok += 1;
            }
            Err(e) => {
                println!("  {id:<20} BROKEN: {e}");
                broken += 1;
            }
        }
    }
    println!("Reloaded: {ok} ok, {broken} broken");
    std::process::ExitCode::SUCCESS
}

fn sub_debug(args: &[String]) -> std::process::ExitCode {
    let Some(id) = args.first() else {
        eprintln!("error: missing argument `<ID>`\n\n{DEBUG_HELP}");
        return std::process::ExitCode::from(2);
    };
    let state = match load_state() {
        Ok(s) => s,
        Err(e) => return report(&e),
    };
    let entry = match get(&state, id) {
        Ok(e) => e,
        Err(e) => return report(&e),
    };
    println!("id:        {id}");
    println!("path:      {}", entry.path);
    println!("enabled:   {}", entry.enabled);
    match load_json_plugin(&entry.path) {
        Ok(p) => {
            println!("name:      {}", p.name);
            println!("version:   {}", p.version);
            println!("description: {}", p.description);
            println!("author:    {}", p.author);
        }
        Err(e) => {
            eprintln!("LOAD ERROR: {e}");
            return std::process::ExitCode::from(1);
        }
    }
    std::process::ExitCode::SUCCESS
}

/// fmt = pretty-print; fix = fill defaults (id/description/author) + pretty.
fn sub_fmt(args: &[String], fix: bool) -> std::process::ExitCode {
    let verb = if fix { "Fixed" } else { "Formatted" };
    let targets: Vec<PathBuf> = if args.is_empty() {
        match load_state() {
            Ok(s) => s.into_values().map(|e| PathBuf::from(e.path)).collect(),
            Err(e) => return report(&e),
        }
    } else {
        args.iter().map(PathBuf::from).collect()
    };
    let mut all_ok = true;
    for path in targets {
        match format_manifest(&path, fix) {
            Ok(()) => println!("{verb} {}", path.display()),
            Err(e) => {
                eprintln!("error: {}: {e}", path.display());
                all_ok = false;
            }
        }
    }
    if all_ok {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::from(1)
    }
}

fn format_manifest(path: &Path, fix: bool) -> Result<(), FormulaError> {
    let original = fs::read_to_string(path).map_err(io_err("อ่าน manifest"))?;
    let mut value: serde_json::Value = serde_json::from_str(&original).map_err(|e| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E802",
            &format!("JSON ไม่ถูกต้อง: {e}"),
            None,
        )
    })?;
    if fix {
        if let serde_json::Value::Object(m) = &mut value {
            if m.get("id").is_none() {
                if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                    m.insert("id".into(), serde_json::Value::String(name.to_string()));
                }
            }
            m.entry(String::from("description"))
                .or_insert(serde_json::Value::String(String::new()));
            m.entry(String::from("author"))
                .or_insert(serde_json::Value::String(String::new()));
        }
    }
    let pretty = serde_json::to_string_pretty(&value).map_err(|e| {
        FormulaError::new(
            ErrorKind::PluginError,
            "E802",
            &format!("serialize ไม่ได้: {e}"),
            None,
        )
    })?;
    // เขียนก่อน แล้ว validate — ถ้าเสีย ให้คืนไฟล์เดิม (กันข้อมูลหาย)
    fs::write(path, format!("{pretty}\n")).map_err(io_err("เขียน manifest"))?;
    if let Err(e) = load_json_plugin(path.to_str().expect("path")) {
        let _ = fs::write(path, original);
        return Err(e);
    }
    Ok(())
}

fn report(e: &FormulaError) -> std::process::ExitCode {
    eprintln!("error: {e}");
    std::process::ExitCode::from(1)
}

// ── help texts ────────────────────────────────────────────────────────────

const PLUGINS_HELP: &str = "\
Manage bl1z plugins

USAGE:
    bl1z plugins <COMMAND>

COMMANDS:
    install    Install a plugin from a URL or GitHub repo (user/repo)
    list       List installed plugins
    uninstall  Remove an installed plugin
    enable     Enable a plugin
    disable    Disable a plugin
    link       Link a local plugin.json file or directory
    reload     Re-scan the plugin store and validate all manifests
    debug      Show details of an installed plugin
    fmt        Pretty-print plugin.json manifests
    fix        Fill missing defaults and re-validate manifests
    help       Print this message or the help of the given subcommand(s)

OPTIONS:
    -h, --help    Print help

Store: ~/.bl1z/plugins (override with BL1Z_PLUGINS_DIR)
";

const INSTALL_HELP: &str = "\
Install a plugin from a URL or GitHub repo

USAGE:
    bl1z plugins install <SOURCE>

ARGS:
    <SOURCE>    https://... URL, github:user/repo, or user/repo

EXAMPLES:
    bl1z plugins install https://example.com/plugin.json
    bl1z plugins install github:bl1nk-bot/bl1z
    bl1z plugins install bl1nk-bot/bl1z
";

const LIST_HELP: &str = "\
List installed plugins

USAGE:
    bl1z plugins list [OPTIONS]

OPTIONS:
    -h, --help    Print help
";

const UNINSTALL_HELP: &str = "\
Remove an installed plugin

USAGE:
    bl1z plugins uninstall <ID>

ARGS:
    <ID>    Plugin id (see `bl1z plugins list`)
";

const LINK_HELP: &str = "\
Link a local plugin.json file or directory (no copy)

USAGE:
    bl1z plugins link <PATH>

ARGS:
    <PATH>    Path to plugin.json or a directory containing plugin.json
";

const RELOAD_HELP: &str = "\
Re-scan the plugin store and validate all manifests

USAGE:
    bl1z plugins reload [OPTIONS]

OPTIONS:
    -h, --help    Print help
";

const DEBUG_HELP: &str = "\
Show details of an installed plugin

USAGE:
    bl1z plugins debug <ID>

ARGS:
    <ID>    Plugin id (see `bl1z plugins list`)
";
