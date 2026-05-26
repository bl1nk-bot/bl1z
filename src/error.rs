use crate::span::Span;

/// ประเภทของข้อผิดพลาด
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // E1xx — Lexer errors
    LexError,
    // E2xx — Parser errors
    ParseError,
    // E3xx — Evaluation errors
    EvalError,
    // E4xx — Type errors
    TypeError,
    // E5xx — Function errors
    FunctionError,
    // E6xx — Context errors
    ContextError,
    VariableNotFound,
    // Phase 8: Property and index access errors (reserved)
    PropertyNotFound, // E207
    IndexOutOfBounds, // E208
    // Phase 10: Recursion errors
    RecursionLimitExceeded, // E303
    // Phase 12: Serialization errors
    SerializationError, // E701
    // Phase 13: Plugin errors
    PluginError, // E801
}

/// ข้อผิดพลาดที่เกิดขึ้นใน engine
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaError {
    pub kind: ErrorKind,
    pub code: String,       // เช่น "E101"
    pub message: String,    // คำอธิบายสั้น ๆ
    pub span: Option<Span>, // ตำแหน่งที่เกิด error
}

impl FormulaError {
    /// สร้างข้อผิดพลาดใหม่
    pub fn new(kind: ErrorKind, code: &str, message: &str, span: Option<Span>) -> Self {
        Self {
            kind,
            code: code.to_string(),
            message: message.to_string(),
            span,
        }
    }
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for FormulaError {}
