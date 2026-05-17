use crate::error::FormulaError;
use crate::value::Value;
use std::collections::HashMap;

/// Represents a built-in function that can be called during evaluation.
///
/// Functions are the primary extension mechanism of the formula engine.
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
/// use bl1z::{Value, error::FormulaError};
///
/// fn my_add(args: &[Value]) -> Result<Value, FormulaError> {
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
    pub call: fn(&[Value]) -> Result<Value, FormulaError>,
}

/// Registry for storing and looking up built-in functions.
///
/// The FunctionRegistry manages all available functions during evaluation.
/// It provides fast O(1) lookup by function name and ensures functions
/// are properly registered before use.
///
/// # Thread Safety
///
/// FunctionRegistry is not thread-safe. If you need concurrent access
/// to the same registry, wrap it in synchronization primitives.
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
/// fn greet(args: &[Value]) -> Result<Value, FormulaError> {
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
///
/// Use `bl1z::builtins::register_all()` to register all built-ins.
#[derive(Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, BuiltinFunction>,
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
        Self {
            functions: HashMap::new(),
        }
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
    /// fn double(args: &[Value]) -> Result<Value, FormulaError> {
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
        self.functions.insert(func.name.clone(), func);
    }

    /// Finds a function by name.
    ///
    /// Returns `None` if no function with the given name is registered.
    /// During evaluation, missing functions cause a `FunctionError`.
    ///
    /// # Arguments
    /// * `name` - Function name to look up
    ///
    /// # Returns
    /// * `Some(&BuiltinFunction)` - Reference to the registered function
    /// * `None` - Function not found
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{FunctionRegistry, builtins};
    ///
    /// let mut registry = FunctionRegistry::new();
    /// builtins::register_all(&mut registry);
    ///
    /// // Built-in functions are available
    /// assert!(registry.find("len").is_some());
    /// assert!(registry.find("sum").is_some());
    ///
    /// // Non-existent functions return None
    /// assert!(registry.find("nonexistent").is_none());
    /// ```
    pub fn find(&self, name: &str) -> Option<&BuiltinFunction> {
        self.functions.get(name)
    }
}
