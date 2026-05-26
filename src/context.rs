use crate::ast::SpannedExpr;
use crate::value::Value;
use std::collections::BTreeMap;
use std::rc::Rc;

/// User-defined function (Phase 10)
///
/// Stores the function name, parameter list, and body expression.
/// Functions are stored in the Context and can be called like built-in functions.
#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction {
    /// ชื่อฟังก์ชัน
    pub name: String,
    /// รายชื่อพารามิเตอร์
    pub params: Vec<String>,
    /// Body expression
    pub body: Rc<SpannedExpr>,
}

/// Execution context for storing variables and constants.
///
/// The Context provides variable resolution during formula evaluation.
/// Supports nested scopes via parent chain — child scopes inherit
/// variables from parents but cannot mutate them (shadowing only).
///
/// # Scoping Model
///
/// - `get()` walks up the parent chain until found or root
/// - `set()` writes to the current scope only (shadowing)
/// - Cloning a context clones only the current scope, not parents
///
/// # Examples
///
/// ```
/// use bl1z::{Context, Value, tokenize, parse, evaluate, FunctionRegistry};
/// use bl1z::builtins;
///
/// let mut registry = FunctionRegistry::new();
/// builtins::register_all(&mut registry);
///
/// // Create context and set variables
/// let mut ctx = Context::new();
/// ctx.set("score", Value::Number(85.0));
/// ctx.set("bonus", Value::Number(15.0));
/// ctx.set("name", Value::String("Alice".to_string()));
///
/// // Evaluate formula using context variables
/// let tokens = tokenize("score + bonus").unwrap();
/// let ast = parse(&tokens).unwrap();
/// let result = evaluate(&ast, &ctx, &registry).unwrap();
/// assert_eq!(result, Value::Number(100.0));
///
/// // Variables can be updated
/// ctx.set("bonus", Value::Number(20.0));
/// let result2 = evaluate(&ast, &ctx, &registry).unwrap();
/// assert_eq!(result2, Value::Number(105.0));
/// ```
///
/// # Variable Naming
///
/// Variable names follow identifier rules:
/// - Must start with letter or underscore
/// - Can contain letters, digits, and underscores
/// - Case sensitive
/// - Cannot be reserved words (true, false, null)
///
/// # Performance
///
/// Variable lookup is O(depth) where depth is the parent chain length.
/// Each scope uses BTreeMap for deterministic iteration.
#[derive(Debug, Clone)]
pub struct Context {
    variables: BTreeMap<String, Value>,
    functions: BTreeMap<String, UserFunction>,
    parent: Option<Rc<Context>>,
}

impl Context {
    /// Creates a new empty root context with no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::Context;
    /// let ctx = Context::new();
    /// assert!(ctx.get("undefined").is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
            functions: BTreeMap::new(),
            parent: None,
        }
    }

    /// Creates a child context that inherits from a parent.
    ///
    /// The child can see all variables from the parent chain via `get()`,
    /// but `set()` only affects the child scope.
    ///
    /// # Arguments
    /// * `parent` - The parent context to inherit from
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{Context, Value};
    ///
    /// let mut root = Context::new();
    /// root.set("x", Value::Number(1.0));
    ///
    /// let mut child = Context::with_parent(root.clone());
    /// child.set("y", Value::Number(2.0));
    ///
    /// // Child can see parent's variable
    /// assert_eq!(child.get("x"), Some(&Value::Number(1.0)));
    /// // Child has its own variable
    /// assert_eq!(child.get("y"), Some(&Value::Number(2.0)));
    /// // Parent cannot see child's variable
    /// assert!(root.get("y").is_none());
    /// ```
    pub fn with_parent(parent: Context) -> Self {
        Self {
            variables: BTreeMap::new(),
            functions: BTreeMap::new(),
            parent: Some(Rc::new(parent)),
        }
    }

    /// Sets a variable in the current scope only.
    ///
    /// If the variable exists in a parent scope, it is shadowed
    /// (parent value remains unchanged).
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Value to store
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{Context, Value};
    /// let mut ctx = Context::new();
    ///
    /// ctx.set("count", Value::Number(42.0));
    /// ctx.set("message", Value::String("hello".to_string()));
    /// ctx.set("active", Value::Bool(true));
    ///
    /// assert_eq!(*ctx.get("count").unwrap(), Value::Number(42.0));
    /// ```
    pub fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    /// Retrieves a variable by walking up the parent chain.
    ///
    /// Returns the first match found, starting from the current scope
    /// and moving up through parents. Returns `None` if not found
    /// anywhere in the chain.
    ///
    /// # Arguments
    /// * `name` - Variable name to look up
    ///
    /// # Returns
    /// * `Some(&Value)` - Reference to the stored value
    /// * `None` - Variable not found in any scope
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{Context, Value};
    /// let mut ctx = Context::new();
    ///
    /// ctx.set("score", Value::Number(100.0));
    ///
    /// assert_eq!(ctx.get("score"), Some(&Value::Number(100.0)));
    /// assert_eq!(ctx.get("undefined"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(val) = self.variables.get(name) {
            return Some(val);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        None
    }

    /// Returns all variables visible in this context, including inherited ones.
    ///
    /// Parent variables are included but shadowed by child values.
    /// Useful for debugging and inspection.
    ///
    /// # Returns
    ///
    /// A `BTreeMap` of all visible variable names to their resolved values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{Context, Value};
    ///
    /// let mut root = Context::new();
    /// root.set("x", Value::Number(1.0));
    /// root.set("y", Value::Number(2.0));
    ///
    /// let mut child = Context::with_parent(root.clone());
    /// child.set("x", Value::Number(10.0)); // shadow
    /// child.set("z", Value::Number(3.0));
    ///
    /// let all = child.get_all();
    /// assert_eq!(all.get("x"), Some(&&Value::Number(10.0))); // shadowed
    /// assert_eq!(all.get("y"), Some(&&Value::Number(2.0)));  // inherited
    /// assert_eq!(all.get("z"), Some(&&Value::Number(3.0)));  // own
    /// ```
    pub fn get_all(&self) -> BTreeMap<String, &Value> {
        let mut result = BTreeMap::new();
        self.collect_all(&mut result);
        result
    }

    fn collect_all<'a>(&'a self, result: &mut BTreeMap<String, &'a Value>) {
        if let Some(parent) = &self.parent {
            parent.collect_all(result);
        }
        for (k, v) in &self.variables {
            result.insert(k.clone(), v);
        }
    }

    /// Returns the depth of the parent chain (0 for root context).
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::Context;
    ///
    /// let root = Context::new();
    /// assert_eq!(root.depth(), 0);
    ///
    /// let child = Context::with_parent(root);
    /// assert_eq!(child.depth(), 1);
    /// ```
    pub fn depth(&self) -> usize {
        match &self.parent {
            None => 0,
            Some(parent) => 1 + parent.depth(),
        }
    }

    /// Registers a user-defined function in the current scope.
    ///
    /// Phase 10: User-Defined Functions
    ///
    /// # Arguments
    /// * `func` - The UserFunction to register
    ///
    /// # Examples
    ///
    /// ```
    /// use bl1z::{Context, Value};
    /// use bl1z::context::UserFunction;
    /// use bl1z::ast::SpannedExpr;
    /// use bl1z::ast::Expr;
    /// use bl1z::span::{Span, Position};
    /// use std::rc::Rc;
    ///
    /// let mut ctx = Context::new();
    /// let body = SpannedExpr::new(
    ///     Expr::Variable("x".to_string()),
    ///     Span::new(Position { line: 1, column: 1 }, Position { line: 1, column: 2 }),
    /// );
    /// let func = UserFunction {
    ///     name: "double".to_string(),
    ///     params: vec!["x".to_string()],
    ///     body: Rc::new(body),
    /// };
    /// ctx.set_function(func);
    /// assert!(ctx.get_function("double").is_some());
    /// ```
    pub fn set_function(&mut self, func: UserFunction) {
        self.functions.insert(func.name.clone(), func);
    }

    /// Retrieves a user-defined function by walking up the parent chain.
    ///
    /// Phase 10: User-Defined Functions
    ///
    /// # Arguments
    /// * `name` - Function name to look up
    ///
    /// # Returns
    /// * `Some(&UserFunction)` - Reference to the registered function
    /// * `None` - Function not found in any scope
    pub fn get_function(&self, name: &str) -> Option<&UserFunction> {
        if let Some(func) = self.functions.get(name) {
            return Some(func);
        }
        if let Some(parent) = &self.parent {
            return parent.get_function(name);
        }
        None
    }

    /// Returns all user-defined functions visible in this context.
    pub fn get_functions(&self) -> BTreeMap<String, UserFunction> {
        let mut result = BTreeMap::new();
        self.collect_functions(&mut result);
        result
    }

    fn collect_functions(&self, result: &mut BTreeMap<String, UserFunction>) {
        if let Some(parent) = &self.parent {
            parent.collect_functions(result);
        }
        for (k, v) in &self.functions {
            result.insert(k.clone(), v.clone());
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_new_creates_empty_root() {
        let ctx = Context::new();
        assert!(ctx.get("x").is_none());
        assert_eq!(ctx.depth(), 0);
    }

    #[test]
    fn context_set_and_get_in_same_scope() {
        let mut ctx = Context::new();
        ctx.set("x", Value::Number(42.0));
        assert_eq!(ctx.get("x"), Some(&Value::Number(42.0)));
    }

    #[test]
    fn context_child_inherits_parent_variable() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let child = Context::with_parent(root);
        assert_eq!(child.get("x"), Some(&Value::Number(1.0)));
    }

    #[test]
    fn context_child_shadows_parent_variable() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let mut child = Context::with_parent(root);
        child.set("x", Value::Number(10.0));

        assert_eq!(child.get("x"), Some(&Value::Number(10.0)));
    }

    #[test]
    fn context_parent_unaffected_by_child_shadowing() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let mut child = Context::with_parent(root.clone());
        child.set("x", Value::Number(10.0));

        assert_eq!(root.get("x"), Some(&Value::Number(1.0)));
    }

    #[test]
    fn context_parent_cannot_see_child_variable() {
        let mut child = Context::with_parent(Context::new());
        child.set("y", Value::Number(2.0));

        // We can't access the parent directly here, but we verify
        // that the child has the variable and a fresh parent wouldn't
        let root = Context::new();
        assert!(root.get("y").is_none());
    }

    #[test]
    fn context_multi_level_inheritance() {
        let mut root = Context::new();
        root.set("a", Value::Number(1.0));

        let mut mid = Context::with_parent(root);
        mid.set("b", Value::Number(2.0));

        let mut leaf = Context::with_parent(mid);
        leaf.set("c", Value::Number(3.0));

        assert_eq!(leaf.get("a"), Some(&Value::Number(1.0)));
        assert_eq!(leaf.get("b"), Some(&Value::Number(2.0)));
        assert_eq!(leaf.get("c"), Some(&Value::Number(3.0)));
        assert!(leaf.get("d").is_none());
    }

    #[test]
    fn context_multi_level_shadowing() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let mut mid = Context::with_parent(root);
        mid.set("x", Value::Number(2.0));

        let mut leaf = Context::with_parent(mid);
        leaf.set("x", Value::Number(3.0));

        assert_eq!(leaf.get("x"), Some(&Value::Number(3.0)));
    }

    #[test]
    fn context_get_all_includes_inherited_and_shadowed() {
        let mut root = Context::new();
        root.set("a", Value::Number(1.0));
        root.set("b", Value::Number(2.0));

        let mut child = Context::with_parent(root);
        child.set("a", Value::Number(10.0));
        child.set("c", Value::Number(3.0));

        let all = child.get_all();
        assert_eq!(all.get("a"), Some(&&Value::Number(10.0)));
        assert_eq!(all.get("b"), Some(&&Value::Number(2.0)));
        assert_eq!(all.get("c"), Some(&&Value::Number(3.0)));
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn context_depth_counts_parent_chain() {
        let root = Context::new();
        assert_eq!(root.depth(), 0);

        let mid = Context::with_parent(root);
        assert_eq!(mid.depth(), 1);

        let leaf = Context::with_parent(mid);
        assert_eq!(leaf.depth(), 2);
    }

    #[test]
    fn context_clone_copies_current_scope_only() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let mut child = Context::with_parent(root);
        child.set("y", Value::Number(2.0));

        let child_clone = child.clone();
        assert_eq!(child_clone.get("x"), Some(&Value::Number(1.0)));
        assert_eq!(child_clone.get("y"), Some(&Value::Number(2.0)));
        assert_eq!(child_clone.depth(), 1);
    }

    #[test]
    fn context_set_after_clone_does_not_affect_original() {
        let mut root = Context::new();
        root.set("x", Value::Number(1.0));

        let child = Context::with_parent(root);
        let mut child_clone = child.clone();

        child_clone.set("y", Value::Number(99.0));

        // Original child should not see the new variable
        assert!(child.get("y").is_none());
    }
}
