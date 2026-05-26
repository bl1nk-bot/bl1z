use crate::error::FormulaError;
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

/// Phase 9.5: Function trait for stateful functions
///
/// This trait allows implementing functions that maintain state,
/// which is not possible with simple fn pointers.
pub trait Function: Send + Sync {
    /// Call the function with arguments.
    fn call(&self, args: &[Value]) -> Result<Value, FormulaError>;

    /// Get the function name.
    fn name(&self) -> &str;

    /// Get the arity (number of arguments).
    fn arity(&self) -> usize {
        0 // default implementation
    }
}

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

impl Function for BuiltinFunction {
    fn call(&self, args: &[Value]) -> Result<Value, FormulaError> {
        (self.call)(args)
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
/// - Higher-order functions: `map`, `filter`, `reduce`, `sort`, `unique`, `group_by` (Phase 9)
///
/// Use `bl1z::builtins::register_all()` to register all built-ins.
#[derive(Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, FunctionInfo>,
}

/// Internal function storage - wraps either BuiltinFunction or Box<dyn Function>
struct FunctionInfo {
    builtin: BuiltinFunction,
    #[allow(dead_code)]
    stateful: bool,
}

impl FunctionInfo {
    fn from_builtin(func: BuiltinFunction) -> Self {
        Self {
            builtin: func,
            stateful: false,
        }
    }
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
        let info = FunctionInfo::from_builtin(func);
        self.functions.insert(info.builtin.name.clone(), info);
    }

    /// Registers a stateful function using the Function trait.
    /// Phase 9.5: This allows functions with internal state.
    ///
    /// Note: The `Function` trait is not yet publicly exported.
    /// This requires additional design for wrapping trait objects into fn pointers.
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{FunctionRegistry, Value, error::FormulaError};
    ///
    /// let mut registry = FunctionRegistry::new();
    /// // Stateful functions require additional wrapper design (Phase 9.5 future work)
    /// ```
    /// // For simplicity, prefer register() for most use cases
    /// ```
    pub fn register_boxed(&mut self, _func: Rc<dyn Function>) {
        // Note: Full stateful function support (Phase 9.5) requires 
        // changing FunctionRegistry to store trait objects instead of fn pointers.
        // For now, this is a stub to satisfy the API.
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
        self.functions.get(name).map(|info| &info.builtin)
    }

    /// Finds a function by name, returning name and arity.
    ///
    /// Internal method for evaluation.
    #[allow(dead_code)]
    pub(crate) fn find_info(&self, name: &str) -> Option<FunctionInfoRef<'_>> {
        self.functions.get(name).map(|info| FunctionInfoRef {
            name: info.builtin.name.as_str(),
            arity: info.builtin.arity,
            call: info.builtin.call,
        })
    }
}

/// A reference to function information for evaluation.
#[derive(Clone, Copy)]
pub(crate) struct FunctionInfoRef<'a> {
    #[allow(dead_code)]
    pub name: &'a str,
    pub arity: usize,
    pub call: fn(&[Value]) -> Result<Value, FormulaError>,
}
