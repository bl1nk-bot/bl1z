use crate::error::{ErrorKind, FormulaError};
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

/// Phase 9.5: Function trait for stateful functions
///
/// This trait allows implementing functions that maintain state,
/// which is not possible with simple fn pointers.
pub trait Function: Send + Sync {
    /// Call the function with arguments and the registry.
    fn call(&self, args: &[Value], registry: &FunctionRegistry) -> Result<Value, FormulaError>;

    /// Get the function name.
    fn name(&self) -> &str;

    /// Get the arity (number of arguments).
    fn arity(&self) -> usize {
        0 // default implementation
    }
}

/// Represents a built-in function that can be called during evaluation.
///
/// Functions are the primary extension mechanism of the bl1z.
/// Each function has a name, expected number of arguments (arity), and
/// an implementation that takes arguments and returns a result.
///
/// # Function Signatures
///
/// Functions receive arguments as a slice of `Value`s and return `Result<Value, FormulaError>`.
/// This allows functions to perform type checking and return detailed error information.
///
/// # Examples
///
/// ```
/// use bl1z::functions::BuiltinFunction;
/// use bl1z::{Value, error::FormulaError, FunctionRegistry};
///
/// fn my_add(args: &[Value], _registry: &FunctionRegistry) -> Result<Value, FormulaError> {
///     match (args.get(0), args.get(1)) {
///         (Some(Value::Number(a)), Some(Value::Number(b))) => Ok(Value::Number(a + b)),
///         _ => Err(FormulaError::new(
///             bl1z::error::ErrorKind::TypeError,
///             "E401",
///             "Expected two numbers",
///             None
///         ))
///     }
/// }
///
/// let add_func = BuiltinFunction {
///     name: "add".to_string(),
///     arity: 2,
///     call: my_add,
/// };
/// ```
///
/// # Error Handling
///
/// Functions should return appropriate `FormulaError`s for:
/// - Wrong argument types (`TypeError`)
/// - Wrong number of arguments (`ArgumentCountMismatch`)
/// - Domain errors (e.g., division by zero)
/// - Any other function-specific errors
pub struct BuiltinFunction {
    /// Function name as it appears in formulas.
    /// Must be unique within a registry.
    pub name: String,

    /// Expected number of arguments.
    /// The engine validates this before calling the function.
    pub arity: usize,

    /// Function implementation.
    /// Takes a slice of arguments and returns a result.
    pub call: fn(&[Value], &FunctionRegistry) -> Result<Value, FormulaError>,
}

impl Function for BuiltinFunction {
    fn call(&self, args: &[Value], registry: &FunctionRegistry) -> Result<Value, FormulaError> {
        (self.call)(args, registry)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

/// Registry for storing and looking up built-in functions.
///
/// The FunctionRegistry manages all available functions during evaluation.
/// It provides fast O(1) lookup by function name and ensures functions
/// are properly registered before use.
///
/// # Thread Safety
///
/// FunctionRegistry uses a `Rc<dyn Function>` internally which is not thread-safe.
/// For concurrent access, use synchronization primitives.
///
/// # Examples
///
/// ```
/// use bl1z::{FunctionRegistry, Value, error::FormulaError};
/// use bl1z::functions::BuiltinFunction;
///
/// let mut registry = FunctionRegistry::new();
///
/// // Register a custom function
/// fn greet(args: &[Value], _registry: &FunctionRegistry) -> Result<Value, FormulaError> {
///     match args.get(0) {
///         Some(Value::String(name)) => Ok(Value::String(format!("Hello, {}!", name))),
///         _ => Err(FormulaError::new(
///             bl1z::error::ErrorKind::TypeError,
///             "E401",
///             "Expected string argument",
///             None
///         ))
///     }
/// }
///
/// registry.register(BuiltinFunction {
///     name: "greet".to_string(),
///     arity: 1,
///     call: greet,
/// });
///
/// // Look up the function
/// let func = registry.find("greet").unwrap();
/// assert_eq!(func.name, "greet");
/// assert_eq!(func.arity, 1);
/// ```
///
/// # Built-in Functions
///
/// The engine comes with many built-in functions:
/// - String functions: `len`, `upper`, `lower`, `contains`, etc.
/// - Math functions: `abs`, `min`, `max`
/// - Logic functions: `if`
/// - Collection functions: `sum`, `avg`, `count`, `join`
/// - Date functions: `now`, `date_add`, `year`, `month`, `day`
/// - Higher-order functions: `map`, `filter`, `reduce`, `sort`, `unique`, `group_by` (Phase 9)
///
/// Use `bl1z::builtins::register_all()` to register all built-ins.
#[derive(Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, FunctionInfo>,
}

/// Internal function storage — wraps either `BuiltinFunction` or `Box<dyn Function>`.
struct FunctionInfo {
    builtin: BuiltinFunction,
    #[allow(dead_code)]
    stateful: bool,
    /// Runtime-defined functions (JSON plugins, stateful fns) dispatch through this.
    boxed: Option<Rc<dyn Function>>,
}

impl FunctionInfo {
    fn from_builtin(func: BuiltinFunction) -> Self {
        Self { builtin: func, stateful: false, boxed: None }
    }

    fn from_boxed(func: Rc<dyn Function>) -> Self {
        // Proxy builtin keeps `find()`/arity working; the real call
        // is dispatched through `boxed` in eval. Calling the proxy
        // fn pointer directly is an error — boxed fns need the registry.
        let proxy = BuiltinFunction {
            name: func.name().to_string(),
            arity: func.arity(),
            call: boxed_proxy_call,
        };
        Self { builtin: proxy, stateful: true, boxed: Some(func) }
    }
}

fn boxed_proxy_call(_args: &[Value], _registry: &FunctionRegistry) -> Result<Value, FormulaError> {
    Err(FormulaError::new(
        ErrorKind::PluginError,
        "E803",
        "ฟังก์ชันนี้เป็น runtime function ต้องเรียกผ่าน evaluate เท่านั้น",
        None,
    ))
}

impl FunctionRegistry {
    /// Creates a new empty function registry.
    ///
    /// The registry starts with no functions registered.
    /// Use `register()` to add functions before evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::FunctionRegistry;
    /// let registry = FunctionRegistry::new();
    /// assert!(registry.find("nonexistent").is_none());
    /// ```
    pub fn new() -> Self {
        Self { functions: HashMap::new() }
    }

    /// Registers a function in the registry.
    ///
    /// If a function with the same name already exists, it is replaced.
    /// Function names are case-sensitive.
    ///
    /// # Arguments
    /// * `func` - The function to register
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{FunctionRegistry, functions::BuiltinFunction, Value, error::FormulaError};
    ///
    /// fn double(args: &[Value], _registry: &FunctionRegistry) -> Result<Value, FormulaError> {
    ///     match args.get(0) {
    ///         Some(Value::Number(n)) => Ok(Value::Number(n * 2.0)),
    ///         _ => Err(FormulaError::new(
    ///             bl1z::error::ErrorKind::TypeError,
    ///             "E401",
    ///             "Expected number",
    ///             None
    ///         ))
    ///     }
    /// }
    ///
    /// let mut registry = FunctionRegistry::new();
    /// registry.register(BuiltinFunction {
    ///     name: "double".to_string(),
    ///     arity: 1,
    ///     call: double,
    /// });
    ///
    /// let func = registry.find("double").unwrap();
    /// assert_eq!(func.arity, 1);
    /// ```
    pub fn register(&mut self, func: BuiltinFunction) {
        let info = FunctionInfo::from_builtin(func);
        self.functions.insert(info.builtin.name.clone(), info);
    }

    /// Registers a stateful function using the Function trait.
    ///
    /// Runtime-defined functions (e.g. loaded from plugin.json) implement
    /// `Function` and are stored as trait objects, dispatched during eval.
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{FunctionRegistry, Value, error::FormulaError};
    /// use bl1z::functions::Function;
    /// use std::rc::Rc;
    ///
    /// struct Double;
    /// impl Function for Double {
    ///     fn call(&self, args: &[Value], _registry: &FunctionRegistry) -> Result<Value, FormulaError> {
    ///         match args.get(0) {
    ///             Some(Value::Number(n)) => Ok(Value::Number(n * 2.0)),
    ///             _ => Err(FormulaError::new(
    ///                 bl1z::error::ErrorKind::TypeError,
    ///                 "E401",
    ///                 "ต้องการตัวเลข",
    ///                 None
    ///             ))
    ///         }
    ///     }
    ///     fn name(&self) -> &str { "double" }
    ///     fn arity(&self) -> usize { 1 }
    /// }
    ///
    /// let mut registry = FunctionRegistry::new();
    /// registry.register_boxed(Rc::new(Double));
    /// ```
    pub fn register_boxed(&mut self, func: Rc<dyn Function>) {
        let info = FunctionInfo::from_boxed(func);
        self.functions.insert(info.builtin.name.clone(), info);
    }

    /// Returns all registered function names, sorted.
    ///
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// Finds a function by name in the registry.
    ///
    /// Returns `None` if no function with the given name is registered.
    /// During evaluation, missing functions cause a `FunctionError`.
    ///
    /// # Arguments
    /// * `name` - Function name to look up
    ///
    /// # Returns
    /// * `Some(&BuiltinFunction)` - Reference to the function
    /// * `None` - Function not found
    ///
    /// # Examples
    /// ```
    /// use bl1z::{FunctionRegistry, builtins};
    /// let mut registry = FunctionRegistry::new();
    /// builtins::register_all(&mut registry);
    /// assert!(registry.find("len").is_some());
    /// assert!(registry.find("nonexistent").is_none());
    /// ```
    pub fn find(&self, name: &str) -> Option<&BuiltinFunction> {
        self.functions.get(name).map(|info| &info.builtin)
    }

    /// Finds a function by name, returning name and arity.
    ///
    /// Internal method for evaluation.
    pub(crate) fn find_info(&self, name: &str) -> Option<FunctionInfoRef<'_>> {
        self.functions.get(name).map(|info| FunctionInfoRef {
            name: info.builtin.name.as_str(),
            arity: info.builtin.arity,
            call: info.builtin.call,
            boxed: info.boxed.as_deref(),
        })
    }
}

/// A reference to function information for evaluation.
#[derive(Clone, Copy)]
pub(crate) struct FunctionInfoRef<'a> {
    #[allow(dead_code)]
    pub name: &'a str,
    pub arity: usize,
    pub call: fn(&[Value], &FunctionRegistry) -> Result<Value, FormulaError>,
    /// Runtime-defined function (JSON plugin/stateful) to dispatch to.
    pub boxed: Option<&'a dyn Function>,
}
