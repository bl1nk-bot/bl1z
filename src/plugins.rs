//! Plugin SDK Foundation (Phase 13)
//!
//! ระบบปลั๊กอินเพื่อให้ third-party ขยายความสามารถของ bl1z ได้
//! ปลั๊กอินสามารถเพิ่มฟังก์ชันใหม่ลงใน engine ผ่าน trait `Plugin`

use crate::error::{ErrorKind, FormulaError};
use crate::functions::{BuiltinFunction, FunctionRegistry};

/// Trait สำหรับ plugin ของ bl1z
///
/// Plugin เป็นหน่วยขยายที่ third-party สามารถสร้างเพื่อเพิ่มฟังก์ชัน
/// ใหม่ให้กับ engine ได้
///
/// # Examples
///
/// ```
/// use bl1z::plugins::Plugin;
/// use bl1z::functions::BuiltinFunction;
/// use bl1z::Value;
/// use bl1z::error::FormulaError;
///
/// struct MathPlugin;
///
/// impl Plugin for MathPlugin {
///     fn name(&self) -> &str {
///         "math_extra"
///     }
///
///     fn version(&self) -> &str {
///         "0.1.0"
///     }
///
///     fn functions(&self) -> Vec<BuiltinFunction> {
///         vec![
///             BuiltinFunction {
///                 name: "square".to_string(),
///                 arity: 1,
///                 call: |args, _| {
///                     match &args[0] {
///                         Value::Number(n) => Ok(Value::Number(n * n)),
///                         _ => Err(FormulaError::new(
///                             bl1z::error::ErrorKind::TypeError,
///                             "E401",
///                             "square ต้องการตัวเลข",
///                             None,
///                         ))
///                     }
///                 },
///             }
///         ]
///     }
/// }
/// ```
pub trait Plugin: Send + Sync {
    /// ชื่อของปลั๊กอิน
    fn name(&self) -> &str;

    /// เวอร์ชันของปลั๊กอิน
    fn version(&self) -> &str;

    /// รายการฟังก์ชันที่ปลั๊กอินให้บริการ
    fn functions(&self) -> Vec<BuiltinFunction>;
}

/// ตัวจัดการปลั๊กอิน (Plugin Manager)
///
/// จัดเก็บปลั๊กอินที่ลงทะเบียนไว้ และสามารถรวมฟังก์ชันจากทุกปลั๊กอิน
/// เข้ากับ FunctionRegistry ได้
///
/// # Examples
///
/// ```
/// use bl1z::plugins::{Plugin, PluginManager};
/// use bl1z::functions::{BuiltinFunction, FunctionRegistry};
/// use bl1z::Value;
/// use bl1z::error::FormulaError;
///
/// struct MyPlugin;
/// impl Plugin for MyPlugin {
///     fn name(&self) -> &str { "test" }
///     fn version(&self) -> &str { "0.1.0" }
///     fn functions(&self) -> Vec<BuiltinFunction> {
///         vec![
///             BuiltinFunction {
///                 name: "hello".to_string(),
///                 arity: 0,
///                 call: |_, _| Ok(Value::String("hello!".to_string())),
///             }
///         ]
///     }
/// }
///
/// let mut manager = PluginManager::new();
/// manager.register(Box::new(MyPlugin));
///
/// let mut registry = FunctionRegistry::new();
/// manager.merge_functions(&mut registry).unwrap();
///
/// assert!(registry.find("hello").is_some());
/// ```
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// สร้าง PluginManager ใหม่
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// ลงทะเบียนปลั๊กอินใหม่
    ///
    /// # Arguments
    /// * `plugin` - ปลั๊กอินที่ต้องการลงทะเบียน
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    /// รวมฟังก์ชันจากทุกปลั๊กอินเข้ากับ FunctionRegistry
    ///
    /// ถ้าพบชื่อฟังก์ชันซ้ำกัน จะคืน `Err(FormulaError)` พร้อมรายละเอียด
    ///
    /// # Arguments
    /// * `registry` - FunctionRegistry ที่ต้องการเพิ่มฟังก์ชัน
    ///
    /// # Returns
    /// * `Ok(())` - รวมฟังก์ชันสำเร็จ
    /// * `Err(FormulaError)` - พบชื่อฟังก์ชันซ้ำกัน
    pub fn merge_functions(&self, registry: &mut FunctionRegistry) -> Result<(), FormulaError> {
        let mut to_register = Vec::new();
        for plugin in &self.plugins {
            for func in plugin.functions() {
                // Check for conflicts
                if registry.find(&func.name).is_some() {
                    return Err(FormulaError::new(
                        ErrorKind::PluginError,
                        "E801",
                        &format!(
                            "ฟังก์ชัน '{}' จากปลั๊กอิน '{}' ขัดแย้งกับฟังก์ชันที่มีอยู่แล้ว",
                            func.name,
                            plugin.name()
                        ),
                        None,
                    ));
                }
                to_register.push(func);
            }
        }

        // Apply all if no errors
        for func in to_register {
            registry.register(func);
        }
        Ok(())
    }

    /// คืนจำนวนปลั๊กอินที่ลงทะเบียนไว้
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// คืนรายชื่อปลั๊กอินที่ลงทะเบียนไว้
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Script plugins — the manifest (plugin.json) REGISTERS the plugin: metadata
// plus the function names/params it exposes. The runner is something else:
// a script (Python, Node, shell, ...) executed by its own interpreter. bl1z
// never runs plugin logic — it spawns the script per call, passes the args
// as JSON on stdin, and reads the result as JSON from stdout. Same shape as
// Obsidian's manifest.json + main.js (Electron runs the JS), or Hermes'
// SKILL.md + scripts (python runs those). The script can do anything bl1z
// cannot — I/O, network, state — that's the point of a plugin.
//
// The plugin contract is defined in proto/bl1z_plugin.proto — the single
// source of truth. That IDL generates the JSON Schema
// (tools/gen_schema.py → plugin-manifest.schema.json) for editors/SDK
// tooling. The engine does NOT carry proto machinery: like a backend
// connecting to a DB without carrying the schema, it just consumes the
// manifest's wire format (JSON, proto3 JSON mapping). Schema change = edit
// the proto + regen — nothing hand-maintained.
//
// plugin.json:
// {
//   "id": "csv_tools",
//   "name": "CSV Tools",
//   "version": "0.1.0",
//   "description": "Read/merge CSV files",
//   "author": "bl1z team",
//   "minEngineVersion": "0.2.16",
//   "runner": "python3",                 // interpreter that runs the script
//   "script": "csv_tools.py",            // relative to the manifest
//   "functions": [
//     { "name": "csv_rows", "params": ["path"] },
//     { "name": "csv_merge", "params": ["a", "b"] }
//   ]
// }
//
// csv_tools.py reads JSON args from stdin, prints JSON to stdout:
//   import json, sys
//   args = json.load(sys.stdin)
//   ...
//   print(json.dumps(result))
//
// Load with `load_json_plugin(path)` (requires the `serialization` feature).
// ---------------------------------------------------------------------------
#[cfg(feature = "serialization")]
mod json {
    use super::*;
    use crate::functions::Function;
    use crate::value::Value;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::rc::Rc;

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PluginFile {
        id: Option<String>,
        name: String,
        version: String,
        #[serde(default)]
        description: String,
        #[serde(default)]
        author: String,
        #[serde(default)]
        min_engine_version: Option<String>,
        #[serde(default)]
        runner: String,
        script: String,
        #[serde(default)]
        functions: Vec<FunctionDef>,
    }

    #[derive(serde::Deserialize)]
    struct FunctionDef {
        name: String,
        #[serde(default)]
        params: Vec<String>,
    }

    /// A plugin loaded from a plugin.json manifest file.
    pub struct JsonPlugin {
        pub id: String,
        pub name: String,
        pub version: String,
        pub description: String,
        pub author: String,
        pub script: String,
        functions: Vec<Rc<dyn Function>>,
    }

    impl JsonPlugin {
        /// Registers this plugin's functions into the registry.
        ///
        /// Fails with `E801` if any function name collides with an existing one.
        pub fn register_into(&self, registry: &mut FunctionRegistry) -> Result<(), FormulaError> {
            for f in &self.functions {
                if registry.find(f.name()).is_some() {
                    return Err(FormulaError::new(
                        ErrorKind::PluginError,
                        "E801",
                        &format!(
                            "ฟังก์ชัน '{}' จากปลั๊กอิน '{}' ขัดแย้งกับฟังก์ชันที่มีอยู่แล้ว",
                            f.name(),
                            self.name
                        ),
                        None,
                    ));
                }
            }
            for f in &self.functions {
                registry.register_boxed(f.clone());
            }
            Ok(())
        }
    }

    /// Loads a plugin from a plugin.json manifest file.
    pub fn load_json_plugin(path: &str) -> Result<JsonPlugin, FormulaError> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            FormulaError::new(
                ErrorKind::PluginError,
                "E802",
                &format!("อ่านไฟล์ปลั๊กอิน '{}' ไม่ได้: {}", path, e),
                None,
            )
        })?;
        let file: PluginFile = serde_json::from_str(&text).map_err(|e| {
            FormulaError::new(
                ErrorKind::PluginError,
                "E802",
                &format!("plugin.json ไม่ถูกต้อง: {}", e),
                None,
            )
        })?;
        if let Some(min) = &file.min_engine_version {
            if engine_is_older_than(min) {
                return Err(FormulaError::new(
                    ErrorKind::PluginError,
                    "E804",
                    &format!(
                        "ปลั๊กอิน '{}' ต้องการ engine >= {} แต่ bl1z นี้คือ {}",
                        file.name,
                        min,
                        env!("CARGO_PKG_VERSION")
                    ),
                    None,
                ));
            }
        }
        let script_path = {
            let base = Path::new(path).parent().unwrap_or_else(|| Path::new("."));
            let raw = base.join(&file.script);
            // security: ไม่อนุญาต path traversal ใน script path
            if file.script.contains("..") || file.script.starts_with('/') {
                return Err(FormulaError::new(
                    ErrorKind::PluginError,
                    "E805",
                    &format!("เส้นทาง runner script ไม่ถูกต้อง: `{}`", file.script),
                    None,
                ));
            }
            raw
        };
        let runner = if file.runner.is_empty() {
            "python3".to_string()
        } else {
            // security: allowlist of safe runners (no arbitrary commands)
            const ALLOWED_RUNNERS: &[&str] = &[
                "python3",
                "python",
                "python3.11",
                "python3.12",
                "python3.13",
                "node",
                "deno",
                "bun",
            ];
            if !ALLOWED_RUNNERS.contains(&file.runner.as_str()) {
                return Err(FormulaError::new(
                    ErrorKind::PluginError,
                    "E806",
                    &format!(
                        "runner '{}' ไม่อนุญาต (อนุญาต: python3, node, deno, bun)",
                        file.runner
                    ),
                    None,
                ));
            }
            file.runner.clone()
        };
        let mut seen_names = std::collections::HashSet::new();
        let functions = file
            .functions
            .into_iter()
            .map(|f| {
                // Validate function name: must be a valid identifier [a-zA-Z_][a-zA-Z0-9_]*
                if !f
                    .name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    || f.name.is_empty()
                    || f.name.starts_with(|c: char| c.is_ascii_digit())
                {
                    return Err(FormulaError::new(
                        ErrorKind::PluginError,
                        "E804",
                        &format!(
                            "ชื่อฟังก์ชัน '{}' ไม่ถูกต้อง (อนุญาตเฉพาะ a-z A-Z 0-9 _ และขึ้นต้นด้วยตัวอักษรหรือ _)",
                            f.name
                        ),
                        None,
                    ));
                }
                if !seen_names.insert(f.name.clone()) {
                    return Err(FormulaError::new(
                        ErrorKind::PluginError,
                        "E804",
                        &format!("ชื่อฟังก์ชัน '{}' ซ้ำกันใน manifest เดียวกัน", f.name),
                        None,
                    ));
                }
                Ok(Rc::new(ScriptFunction {
                    name: f.name,
                    params: f.params,
                    runner: runner.clone(),
                    script_path: script_path.clone(),
                }) as Rc<dyn Function>)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(JsonPlugin {
            id: file.id.unwrap_or_else(|| file.name.clone()),
            name: file.name,
            version: file.version,
            description: file.description,
            author: file.author,
            script: file.script,
            functions,
        })
    }

    /// True if the running engine is older than the requested version.
    /// Strict numeric-tuple compare; rejects versions with non-numeric parts.
    fn engine_is_older_than(requested: &str) -> bool {
        let parse = |s: &str| -> Option<Vec<u32>> {
            let parts: Vec<_> = s.split('.').collect();
            if parts.is_empty() {
                return None;
            }
            parts.iter().map(|p| p.parse::<u32>().ok()).collect()
        };
        match (parse(env!("CARGO_PKG_VERSION")), parse(requested)) {
            (Some(current), Some(req)) => current < req,
            _ => false, // unparseable requested version = don't block
        }
    }

    /// A plugin function backed by an external script: spawn the runner
    /// (e.g. `python3 script.py <fn>`), send the args as JSON on stdin,
    /// read the result as JSON from stdout. The script's interpreter runs
    /// the logic — bl1z only opens the pipe. This is what lets a plugin do
    /// anything bl1z itself cannot (I/O, network, state).
    struct ScriptFunction {
        name: String,
        params: Vec<String>,
        runner: String,
        script_path: PathBuf,
    }

    impl ScriptFunction {
        fn run(&self, args: &[Value]) -> Result<Value, FormulaError> {
            let mut child = Command::new(&self.runner)
                .arg(&self.script_path)
                .arg(&self.name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    FormulaError::new(
                        ErrorKind::PluginError,
                        "E805",
                        &format!(
                            "รัน runner '{}' สำหรับปลั๊กอิน '{}' ไม่ได้: {}",
                            self.runner, self.name, e
                        ),
                        None,
                    )
                })?;
            {
                use std::io::Write;
                // Plain JSON on the wire (not the engine's tagged Value
                // format) so scripts get real numbers/strings/arrays.
                let payload =
                    serde_json::to_vec(&args.iter().map(to_plain_json).collect::<Vec<_>>())
                        .unwrap_or_default();
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(&payload).map_err(|e| {
                        FormulaError::new(
                            ErrorKind::PluginError,
                            "E805",
                            &format!("ส่งข้อมูลเข้า script '{}' ไม่ได้: {}", self.name, e),
                            None,
                        )
                    })?;
                }
            }
            // Run with 30s timeout to prevent stalls from hanging scripts
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
            let output = loop {
                match child.try_wait() {
                    Ok(Some(_status)) => {
                        // Process exited — drain remaining output
                        break child.wait_with_output().map_err(|e| {
                            FormulaError::new(
                                ErrorKind::PluginError,
                                "E805",
                                &format!("รอ script '{}' ไม่ได้: {}", self.name, e),
                                None,
                            )
                        })?;
                    }
                    Ok(None) => {
                        if std::time::Instant::now() >= deadline {
                            let _ = child.kill();
                            return Err(FormulaError::new(
                                ErrorKind::PluginError,
                                "E805",
                                &format!("script '{}' timeout (30s)", self.name),
                                None,
                            ));
                        }
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(e) => {
                        return Err(FormulaError::new(
                            ErrorKind::PluginError,
                            "E805",
                            &format!("รอ script '{}' ไม่ได้: {}", self.name, e),
                            None,
                        ));
                    }
                }
            };
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(FormulaError::new(
                    ErrorKind::PluginError,
                    "E805",
                    &format!("script '{}' ออกจากด้วย error: {}", self.name, stderr.trim()),
                    None,
                ));
            }
            let text = String::from_utf8_lossy(&output.stdout);
            let plain: serde_json::Value = serde_json::from_str(text.trim()).map_err(|e| {
                FormulaError::new(
                    ErrorKind::PluginError,
                    "E805",
                    &format!("ผลลัพธ์จาก script '{}' ไม่ใช่ JSON: {}", self.name, e),
                    None,
                )
            })?;
            from_plain_json(&plain).ok_or_else(|| {
                FormulaError::new(
                    ErrorKind::PluginError,
                    "E805",
                    &format!("ผลลัพธ์จาก script '{}' แปลงเป็น Value ไม่ได้", self.name),
                    None,
                )
            })
        }
    }

    /// Value → plain JSON (numbers as numbers, strings as strings), the
    /// inverse of `from_plain_json`. Unsupported values become JSON null.
    fn to_plain_json(v: &Value) -> serde_json::Value {
        match v {
            // Integral f64 → JSON int, so scripts see 7 not 7.0.
            Value::Number(n)
                if n.fract() == 0.0
                    && n.is_finite()
                    && *n >= i64::MIN as f64
                    && *n < 2_f64.powi(63) =>
            {
                serde_json::json!(*n as i64)
            }
            Value::Number(n) => serde_json::json!(n),
            Value::String(s) => serde_json::json!(s),
            Value::Bool(b) => serde_json::json!(b),
            Value::Null => serde_json::Value::Null,
            Value::DateTime(dt) => serde_json::json!(dt.to_string()),
            Value::Duration(d) => serde_json::json!(d.0.to_string()),
            Value::Array(items) => {
                serde_json::Value::Array(items.iter().map(to_plain_json).collect())
            }
            Value::Set(set) => {
                let mut sorted: Vec<&Value> = set.iter().collect();
                sorted.sort_by_key(|v| format!("{:?}", v));
                serde_json::Value::Array(sorted.iter().map(|v| to_plain_json(v)).collect())
            }
            Value::Range { start, end, step } => serde_json::json!({
                "range": [start, end, step],
            }),
            Value::Map(m) => {
                let mut out = serde_json::Map::new();
                for (k, val) in m {
                    out.insert(k.clone(), to_plain_json(val));
                }
                serde_json::Value::Object(out)
            }
            Value::Lambda(..) => serde_json::Value::Null,
        }
    }

    /// Plain JSON → Value (numbers as Number, arrays as Array, objects as
    /// Map). Null is ambiguous with Lambda — callers get Null.
    fn from_plain_json(v: &serde_json::Value) -> Option<Value> {
        match v {
            serde_json::Value::Number(n) => n.as_f64().map(Value::Number),
            serde_json::Value::String(s) => Some(Value::String(s.clone())),
            serde_json::Value::Bool(b) => Some(Value::Bool(*b)),
            serde_json::Value::Null => Some(Value::Null),
            serde_json::Value::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(from_plain_json(item)?);
                }
                Some(Value::Array(out))
            }
            serde_json::Value::Object(map) => {
                if map.len() == 1 {
                    if let Some(serde_json::Value::Array(items)) = map.get("range") {
                        if items.len() == 3 {
                            if let (Some(start), Some(end), Some(step)) =
                                (items[0].as_i64(), items[1].as_i64(), items[2].as_i64())
                            {
                                return Some(Value::Range { start, end, step });
                            }
                        }
                    }
                }
                let mut out = std::collections::HashMap::new();
                for (k, val) in map {
                    out.insert(k.clone(), from_plain_json(val)?);
                }
                Some(Value::Map(out))
            }
        }
    }

    impl Function for ScriptFunction {
        fn call(
            &self,
            args: &[Value],
            _registry: &FunctionRegistry,
        ) -> Result<Value, FormulaError> {
            self.run(args)
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn arity(&self) -> usize {
            self.params.len()
        }
    }
}

#[cfg(feature = "serialization")]
pub use json::{load_json_plugin, JsonPlugin};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test_plugin"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn functions(&self) -> Vec<BuiltinFunction> {
            vec![BuiltinFunction {
                name: "test_square".to_string(),
                arity: 1,
                call: |args, _| {
                    if let Value::Number(n) = &args[0] {
                        Ok(Value::Number(n * n))
                    } else {
                        Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E401",
                            "ต้องการตัวเลข",
                            None,
                        ))
                    }
                },
            }]
        }
    }

    #[test]
    fn plugin_manager_register_and_merge() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(TestPlugin));

        assert_eq!(manager.plugin_count(), 1);
        assert_eq!(manager.plugin_names(), vec!["test_plugin"]);

        let mut registry = FunctionRegistry::new();
        manager.merge_functions(&mut registry).unwrap();

        let func = registry.find("test_square").unwrap();
        assert_eq!(func.name, "test_square");
        assert_eq!(func.arity, 1);

        // Test calling the plugin function
        let result = (func.call)(&[Value::Number(5.0)], &registry).unwrap();
        assert_eq!(result, Value::Number(25.0));
    }

    #[test]
    fn plugin_manager_conflict_detection() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(TestPlugin));

        let mut registry = FunctionRegistry::new();
        // Pre-register a function with same name
        registry.register(BuiltinFunction {
            name: "test_square".to_string(),
            arity: 1,
            call: |_, _| Ok(Value::Null),
        });

        let result = manager.merge_functions(&mut registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::PluginError);
        assert_eq!(err.code, "E801");
    }

    #[test]
    fn plugin_manager_empty_is_ok() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugin_count(), 0);

        let mut registry = FunctionRegistry::new();
        assert!(manager.merge_functions(&mut registry).is_ok());
    }

    #[test]
    fn boxed_function_dispatches_through_eval() {
        use crate::context::Context;
        use crate::eval::evaluate;
        use crate::functions::Function;
        use std::rc::Rc;

        struct Triple;
        impl Function for Triple {
            fn call(
                &self,
                args: &[Value],
                _registry: &FunctionRegistry,
            ) -> Result<Value, FormulaError> {
                if let Value::Number(n) = &args[0] {
                    Ok(Value::Number(n * 3.0))
                } else {
                    Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "ต้องการตัวเลข",
                        None,
                    ))
                }
            }
            fn name(&self) -> &str {
                "triple"
            }
            fn arity(&self) -> usize {
                1
            }
        }

        let mut registry = FunctionRegistry::new();
        registry.register_boxed(Rc::new(Triple));

        let tokens = crate::lexer::tokenize("triple(4) + 1").unwrap();
        let ast = crate::parser::parse(&tokens).unwrap();
        let result = evaluate(&ast, &Context::new(), &registry).unwrap();
        assert_eq!(result, Value::Number(13.0));
    }
}
