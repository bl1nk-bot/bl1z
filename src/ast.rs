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
    // Phase 8: Access Chaining
    PropertyAccess {
        object: Box<SpannedExpr>,
        field: String,
    },
    IndexAccess {
        object: Box<SpannedExpr>,
        index: Box<SpannedExpr>,
    },
}
