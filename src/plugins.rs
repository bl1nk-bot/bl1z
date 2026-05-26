//! Plugin SDK Foundation (Phase 13)
//!
//! ระบบปลั๊กอินเพื่อให้ third-party ขยายความสามารถของ formula engine ได้
//! ปลั๊กอินสามารถเพิ่มฟังก์ชันใหม่ลงใน engine ผ่าน trait `Plugin`

use crate::error::{ErrorKind, FormulaError};
use crate::functions::{BuiltinFunction, FunctionRegistry};

/// Trait สำหรับ plugin ของ formula engine
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
///                 call: |args| {
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
///                 call: |_| Ok(Value::String("hello!".to_string())),
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
    pub fn merge_functions(
        &self,
        registry: &mut FunctionRegistry,
    ) -> Result<(), FormulaError> {
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
                call: |args| {
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
        let result =
            (func.call)(&[Value::Number(5.0)]).unwrap();
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
            call: |_| Ok(Value::Null),
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
}
