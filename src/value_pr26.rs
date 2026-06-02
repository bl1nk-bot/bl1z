/// Represents a value in the formula engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 64-bit floating point number.
    Number(f64),
    /// UTF-8 string.
    String(String),
    /// Boolean value.
    Bool(bool),
    /// Represents null/absent value.
    Null,
    /// Ordered collection of values.
    Array(Vec<Value>),
    /// Key-value dictionary.
    Map(std::collections::HashMap<String, Value>),
    /// A closure representing a lambda or user-defined function.
    Closure {
        /// Names of the parameters.
        params: Vec<String>,
        /// The parsed expression.
        body: Box<crate::ast::SpannedExpr>,
        /// The captured environment.
        env: crate::context::Context,
    },
}
