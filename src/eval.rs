use crate::ast::{BinaryOp, Expr, SpannedExpr, UnaryOp};
use crate::context::Context;
use crate::error::{ErrorKind, FormulaError};
use crate::functions::FunctionRegistry;
use crate::span::Span;
use crate::value::Value;

/// ประเมินผล SpannedExpr แล้วคืน Value
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError> {
    let span = expr.meta.span;
    match &expr.expr {
        Expr::Literal(val) => Ok(val.clone()),
        Expr::Variable(name) => {
            let parts: Vec<&str> = name.split('.').collect();
            if parts.is_empty() {
                return Err(FormulaError::new(
                    ErrorKind::ContextError,
                    "E005",
                    &format!("ไม่พบตัวแปร '{}'", name),
                    Some(span),
                ));
            }

            // Look up the first part in the root context
            let mut current = ctx.get(parts[0]).cloned();
            if current.is_none() {
                return Err(FormulaError::new(
                    ErrorKind::ContextError,
                    "E005",
                    &format!("ไม่พบตัวแปร '{}'", parts[0]),
                    Some(span),
                ));
            }

            // Traverse the remaining parts
            for i in 1..parts.len() {
                let part = parts[i];
                match &current {
                    Some(Value::Map(map)) => {
                        current = map.get(part).cloned();
                        if current.is_none() {
                            return Err(FormulaError::new(
                                ErrorKind::ContextError,
                                "E005",
                                &format!("ไม่พบตัวแปร '{}'", part),
                                Some(span),
                            ));
                        }
                    }
                    Some(Value::Array(_)) => {
                        return Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E006",
                            &format!("คาดหวังแผนที่ แต่ได้อาร์เรย์ที่ '{}'", parts[i - 1]),
                            Some(span),
                        ));
                    }
                    Some(_) => {
                        return Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E006",
                            &format!("คาดหวังแผนที่ แต่ได้ค่าที่ไม่ใช่แผนที่ที่ '{}'", parts[i - 1]),
                            Some(span),
                        ));
                    }
                    None => {
                        return Err(FormulaError::new(
                            ErrorKind::ContextError,
                            "E005",
                            &format!("ไม่พบตัวแปร '{}'", part),
                            Some(span),
                        ));
                    }
                }
            }

            current.ok_or_else(|| {
                FormulaError::new(
                    ErrorKind::ContextError,
                    "E005",
                    &format!("ไม่พบตัวแปร '{}'", name),
                    Some(span),
                )
            })
        }
        Expr::Grouping(inner) => evaluate(inner, ctx, registry),
        Expr::UnaryExpr { op, expr } => {
            let val = evaluate(expr, ctx, registry)?;
            match op {
                UnaryOp::Neg => {
                    if let Value::Number(n) = val {
                        Ok(Value::Number(-n))
                    } else {
                        Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E006",
                            "ตัวดำเนินการลบใช้ได้กับตัวเลขเท่านั้น",
                            Some(span),
                        ))
                    }
                }
                UnaryOp::Not => {
                    if let Value::Bool(b) = val {
                        Ok(Value::Bool(!b))
                    } else {
                        Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E006",
                            "ตัวดำเนินการ NOT ใช้ได้กับ boolean เท่านั้น",
                            Some(span),
                        ))
                    }
                }
            }
        }
        Expr::BinaryExpr { left, op, right } => {
            let l = evaluate(left, ctx, registry)?;
            let r = evaluate(right, ctx, registry)?;
            match op {
                BinaryOp::Add => add_values(l, r, span),
                BinaryOp::Sub => sub_values(l, r, span),
                BinaryOp::Mul => mul_values(l, r, span),
                BinaryOp::Div => div_values(l, r, span),
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::NotEq => Ok(Value::Bool(l != r)),
                BinaryOp::Lt => compare_values(l, r, span, |a, b| a < b),
                BinaryOp::Gt => compare_values(l, r, span, |a, b| a > b),
                BinaryOp::LtEq => compare_values(l, r, span, |a, b| a <= b),
                BinaryOp::GtEq => compare_values(l, r, span, |a, b| a >= b),
                BinaryOp::And => logic_and(l, r, span),
                BinaryOp::Or => logic_or(l, r, span),
            }
        }
        Expr::ArrayLiteral(elements) => {
            let values: Result<Vec<Value>, _> = elements
                .iter()
                .map(|e| evaluate(e, ctx, registry))
                .collect();
            Ok(Value::Array(values?))
        }
        Expr::MapLiteral(pairs) => {
            let mut map = std::collections::HashMap::new();
            for (key, expr) in pairs {
                let value = evaluate(expr, ctx, registry)?;
                map.insert(key.clone(), value);
            }
            Ok(Value::Map(map))
        }
        Expr::FunctionCall { name, args } => {
            let func = registry.find(name).ok_or_else(|| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E007",
                    &format!("ไม่พบฟังก์ชัน '{}'", name),
                    Some(span),
                )
            })?;
            if func.arity != args.len() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E008",
                    &format!(
                        "ฟังก์ชัน '{}' ต้องการ {} อาร์กิวเมนต์ แต่ได้ {}",
                        name,
                        func.arity,
                        args.len()
                    ),
                    Some(span),
                ));
            }
            let evaluated_args: Vec<Value> = args
                .iter()
                .map(|a| evaluate(a, ctx, registry))
                .collect::<Result<_, _>>()?;
            // เรียกฟังก์ชันที่ implement ด้วย FormulaError โดยตรง
            (func.call)(&evaluated_args)
        }
    }
}

// ฟังก์ชันช่วยเหลือทางคณิตศาสตร์
fn add_values(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    use Value::*;
    match (l, r) {
        (Number(a), Number(b)) => Ok(Number(a + b)),
        (String(a), String(b)) => Ok(String(format!("{}{}", a, b))),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "การบวกใช้ได้กับ number+number หรือ string+string เท่านั้น",
            Some(span),
        )),
    }
}

fn sub_values(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Number(a), Value::Number(b)) = (&l, &r) {
        Ok(Value::Number(a - b))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "การลบใช้ได้กับตัวเลขเท่านั้น",
            Some(span),
        ))
    }
}

fn mul_values(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Number(a), Value::Number(b)) = (&l, &r) {
        Ok(Value::Number(a * b))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "ใช้กับตัวเลข",
            Some(span),
        ))
    }
}

fn div_values(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Number(a), Value::Number(b)) = (&l, &r) {
        if *b == 0.0 {
            Err(FormulaError::new(
                ErrorKind::EvalError,
                "E010",
                "หารด้วยศูนย์",
                Some(span),
            ))
        } else {
            Ok(Value::Number(a / b))
        }
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "ใช้กับตัวเลข",
            Some(span),
        ))
    }
}

fn compare_values<F>(l: Value, r: Value, span: Span, f: F) -> Result<Value, FormulaError>
where
    F: Fn(f64, f64) -> bool,
{
    if let (Value::Number(a), Value::Number(b)) = (&l, &r) {
        Ok(Value::Bool(f(*a, *b)))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "การเปรียบเทียบใช้ได้กับตัวเลขเท่านั้น",
            Some(span),
        ))
    }
}

fn logic_and(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Bool(a), Value::Bool(b)) = (&l, &r) {
        Ok(Value::Bool(*a && *b))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "AND ใช้ได้กับ boolean เท่านั้น",
            Some(span),
        ))
    }
}

fn logic_or(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Bool(a), Value::Bool(b)) = (&l, &r) {
        Ok(Value::Bool(*a || *b))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E006",
            "OR ใช้ได้กับ boolean เท่านั้น",
            Some(span),
        ))
    }
}
