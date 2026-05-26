use crate::span::Span;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Pos,
    Not,
}

/// ข้อมูลตำแหน่งของ expression ใน source
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMeta {
    pub span: Span,
}

/// นิพจน์แต่ละตัวจะมีข้อมูล Span ติดไปด้วย
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedExpr {
    pub expr: Expr,
    pub meta: ExprMeta,
}

// ทำให้ SpannedExpr สามารถแกะค่า expr ได้ง่าย
impl SpannedExpr {
    pub fn new(expr: Expr, span: Span) -> Self {
        Self {
            expr,
            meta: ExprMeta { span },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    UnaryExpr {
        op: UnaryOp,
        expr: Box<SpannedExpr>,
    },
    BinaryExpr {
        left: Box<SpannedExpr>,
        op: BinaryOp,
        right: Box<SpannedExpr>,
    },
    FunctionCall {
        name: String,
        args: Vec<SpannedExpr>,
    },
    Grouping(Box<SpannedExpr>),
    ArrayLiteral(Vec<SpannedExpr>),
    MapLiteral(Vec<(String, SpannedExpr)>),
    PropertyAccess {
        object: Box<SpannedExpr>,
        property: String,
    },
    IndexAccess {
        object: Box<SpannedExpr>,
        index: Box<SpannedExpr>,
    },
    /// Lambda expression: (x, y) => x + y
    /// Phase 9: Lambda & Higher-Order Functions
    Lambda {
        params: Vec<String>,
        body: Box<SpannedExpr>,
    },
    /// User-defined function definition: fn name(params) = body
    /// Phase 10: User-Defined Functions
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<SpannedExpr>,
    },
    /// Sequence of expressions separated by ';'
    /// Phase 10: Enables multi-expression evaluation (fn defs + usage)
    Sequence(Vec<SpannedExpr>),
}
