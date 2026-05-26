use crate::ast::{BinaryOp, Expr, SpannedExpr, UnaryOp};
use crate::context::{Context, UserFunction};
use crate::error::{ErrorKind, FormulaError};
use crate::functions::FunctionRegistry;
use crate::span::Span;
use crate::value::Value;
use std::rc::Rc;

/// ประเมินผล SpannedExpr แล้วคืน Value
pub fn evaluate(
    expr: &SpannedExpr,
    ctx: &Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError> {
    let mut ctx_clone = ctx.clone();
    evaluate_impl(expr, &mut ctx_clone, registry, 0)
}

/// ประเมินผล SpannedExpr กับ mutable context (สำหรับ UDF registration)
pub fn evaluate_mut(
    expr: &SpannedExpr,
    ctx: &mut Context,
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError> {
    evaluate_impl(expr, ctx, registry, 0)
}

fn evaluate_impl(
    expr: &SpannedExpr,
    ctx: &mut Context,
    registry: &FunctionRegistry,
    depth: usize,
) -> Result<Value, FormulaError> {
    if depth > 100 {
        return Err(FormulaError::new(
            ErrorKind::RecursionLimitExceeded,
            "E303",
            "การประมวลผลซ้อนลึกเกินกำหนด (Recursion limit exceeded)",
            Some(expr.meta.span),
        ));
    }
    let span = expr.meta.span;
    match &expr.expr {
        Expr::Literal(val) => Ok(val.clone()),
        Expr::Variable(name) => ctx.get(name).cloned().ok_or_else(|| {
            FormulaError::new(
                ErrorKind::VariableNotFound,
                "E601",
                &format!("ไม่พบตัวแปร '{}'", name),
                Some(span),
            )
        }),
        Expr::PropertyAccess { object, property } => {
            let obj = evaluate_impl(object, ctx, registry, depth + 1)?;
            match obj {
                Value::Map(map) => map.get(property).cloned().ok_or_else(|| {
                    FormulaError::new(
                        ErrorKind::PropertyNotFound,
                        "E307",
                        &format!("ไม่พบ property '{}'", property),
                        Some(span),
                    )
                }),
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "ไม่สามารถเข้าถึง property ของค่าที่ไม่ใช่ map ได้",
                    Some(span),
                )),
            }
        }
        Expr::IndexAccess { object, index } => {
            let obj = evaluate_impl(object, ctx, registry, depth + 1)?;
            let idx = evaluate_impl(index, ctx, registry, depth + 1)?;
            match (obj, idx) {
                (Value::Array(arr), Value::Number(n)) => {
                    if !n.is_finite() || n.fract() != 0.0 {
                        return Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E401",
                            &format!("index ของ array ต้องเป็นจำนวนเต็ม แต่ได้ {}", n),
                            Some(span),
                        ));
                    }
                    let i = n as usize;
                    if n < 0.0 || i >= arr.len() {
                        Err(FormulaError::new(
                            ErrorKind::IndexOutOfBounds,
                            "E308",
                            &format!("index {} นอกขอบเขต (ขนาด {})", n, arr.len()),
                            Some(span),
                        ))
                    } else {
                        Ok(arr[i].clone())
                    }
                }
                (Value::Array(_), _) => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "index ของ array ต้องเป็นตัวเลข",
                    Some(span),
                )),
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "ไม่สามารถ index ค่าที่ไม่ใช่ array ได้",
                    Some(span),
                )),
            }
        }
        Expr::Grouping(inner) => evaluate_impl(inner, ctx, registry, depth + 1),
        Expr::UnaryExpr { op, expr } => {
            let val = evaluate_impl(expr, ctx, registry, depth + 1)?;
            match op {
                UnaryOp::Neg => {
                    if let Value::Number(n) = val {
                        Ok(Value::Number(-n))
                    } else {
                        Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E401",
                            "ตัวดำเนินการลบใช้ได้กับตัวเลขเท่านั้น",
                            Some(span),
                        ))
                    }
                }
                UnaryOp::Pos => {
                    if let Value::Number(_) = val {
                        Ok(val)
                    } else {
                        Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E401",
                            "ตัวดำเนินการบวกใช้ได้กับตัวเลขเท่านั้น",
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
                            "E401",
                            "ตัวดำเนินการ NOT ใช้ได้กับ boolean เท่านั้น",
                            Some(span),
                        ))
                    }
                }
            }
        }
        Expr::BinaryExpr { left, op, right } => {
            let l = evaluate_impl(left, ctx, registry, depth + 1)?;
            let r = evaluate_impl(right, ctx, registry, depth + 1)?;
            match op {
                BinaryOp::Add => add_values(l, r, span),
                BinaryOp::Sub => sub_values(l, r, span),
                BinaryOp::Mul => mul_values(l, r, span),
                BinaryOp::Div => div_values(l, r, span),
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::NotEq => Ok(Value::Bool(l != r)),
                BinaryOp::Lt => compare_values(l, r, span, BinaryOp::Lt),
                BinaryOp::Gt => compare_values(l, r, span, BinaryOp::Gt),
                BinaryOp::LtEq => compare_values(l, r, span, BinaryOp::LtEq),
                BinaryOp::GtEq => compare_values(l, r, span, BinaryOp::GtEq),
                BinaryOp::And => logic_and(l, r, span),
                BinaryOp::Or => logic_or(l, r, span),
            }
        }
        Expr::ArrayLiteral(elements) => {
            let values: Result<Vec<Value>, _> = elements
                .iter()
                .map(|e| evaluate_impl(e, ctx, registry, depth + 1))
                .collect();
            Ok(Value::Array(values?))
        }
        Expr::MapLiteral(pairs) => {
            let mut map = std::collections::HashMap::new();
            for (key, expr) in pairs {
                let value = evaluate_impl(expr, ctx, registry, depth + 1)?;
                map.insert(key.clone(), value);
            }
            Ok(Value::Map(map))
        }
        Expr::Lambda { params, body } => {
            // Capture current scope for closure
            let captured: std::collections::BTreeMap<String, Value> = ctx
                .get_all()
                .into_iter()
                .map(|(k, v)| (k, (*v).clone()))
                .collect();
            Ok(Value::Lambda(
                Rc::new((**body).clone()),
                params.clone(),
                captured,
                ctx.get_functions(),
            ))
        }
        Expr::FunctionDef { name, params, body } => {
            // Phase 10: Register user-defined function in context
            let func = UserFunction {
                name: name.clone(),
                params: params.clone(),
                body: Rc::new((**body).clone()),
            };
            ctx.set_function(func);
            Ok(Value::Null)
        }
        Expr::Sequence(exprs) => {
            // Phase 10: Evaluate each expression in sequence, return last result
            let mut result = Value::Null;
            for e in exprs {
                result = evaluate_impl(e, ctx, registry, depth + 1)?;
            }
            Ok(result)
        }
        Expr::FunctionCall { name, args } => {
            // Special handling for short-circuiting 'if' function (Phase 10 / recursion support)
            if name == "if" {
                if args.len() != 3 {
                    return Err(FormulaError::new(
                        ErrorKind::FunctionError,
                        "E503",
                        &format!(
                            "ฟังก์ชัน '{}' ต้องการ 3 อาร์กิวเมนต์ แต่ได้ {}",
                            name,
                            args.len()
                        ),
                        Some(span),
                    ));
                }
                let cond = evaluate_impl(&args[0], ctx, registry, depth + 1)?;
                match cond {
                    Value::Bool(true) => {
                        return evaluate_impl(&args[1], ctx, registry, depth + 1);
                    }
                    Value::Bool(false) => {
                        return evaluate_impl(&args[2], ctx, registry, depth + 1);
                    }
                    _ => {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E501",
                            "if เงื่อนไขต้องเป็น boolean",
                            Some(span),
                        ));
                    }
                }
            }

            // Phase 10: Check user-defined functions first
            if let Some(user_func) = ctx.get_function(name.as_str()).cloned() {
                let evaluated_args: Vec<Value> = args
                    .iter()
                    .map(|a| evaluate_impl(a, ctx, registry, depth + 1))
                    .collect::<Result<_, _>>()?;

                if user_func.params.len() != evaluated_args.len() {
                    return Err(FormulaError::new(
                        ErrorKind::FunctionError,
                        "E503",
                        &format!(
                            "ฟังก์ชัน '{}' ต้องการ {} อาร์กิวเมนต์ แต่ได้ {}",
                            name,
                            user_func.params.len(),
                            evaluated_args.len()
                        ),
                        Some(span),
                    ));
                }

                // Create new context with current as parent
                let mut func_ctx = ctx.clone();
                for (i, param) in user_func.params.iter().enumerate() {
                    func_ctx.set(param, evaluated_args[i].clone());
                }
                return evaluate_impl(&user_func.body, &mut func_ctx, registry, depth + 1);
            }

            // Check if function name exists as a variable (might be a lambda)
            if let Some(Value::Lambda(_, _, _, _)) = ctx.get(name.as_str()) {
                let func_val = ctx.get(name.as_str()).unwrap().clone();
                let evaluated_args: Vec<Value> = args
                    .iter()
                    .map(|a| evaluate_impl(a, ctx, registry, depth + 1))
                    .collect::<Result<_, _>>()?;
                return apply_lambda_impl(&func_val, &evaluated_args, registry, depth + 1);
            }

            let func_info = registry.find_info(name).ok_or_else(|| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E502",
                    &format!("ไม่พบฟังก์ชัน '{}'", name),
                    Some(span),
                )
            })?;
            let is_variadic = func_info.arity == 999;
            if !is_variadic && func_info.arity != args.len() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    &format!(
                        "ฟังก์ชัน '{}' ต้องการ {} อาร์กิวเมนต์ แต่ได้ {}",
                        name,
                        func_info.arity,
                        args.len()
                    ),
                    Some(span),
                ));
            }
            let evaluated_args: Vec<Value> = args
                .iter()
                .map(|a| evaluate_impl(a, ctx, registry, depth + 1))
                .collect::<Result<_, _>>()?;
            // เรียกฟังก์ชันที่ implement ด้วย FormulaError โดยตรง
            (func_info.call)(&evaluated_args)
        }
    }
}

/// Apply a lambda value to arguments.
/// Phase 9: Lambda & Higher-Order Functions
pub fn apply_lambda(
    lambda: &Value,
    args: &[Value],
    registry: &FunctionRegistry,
) -> Result<Value, FormulaError> {
    apply_lambda_impl(lambda, args, registry, 0)
}

fn apply_lambda_impl(
    lambda: &Value,
    args: &[Value],
    registry: &FunctionRegistry,
    depth: usize,
) -> Result<Value, FormulaError> {
    match lambda {
        Value::Lambda(body_expr, params, captured_vars, captured_funcs) => {
            if params.len() != args.len() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    &format!(
                        "lambda ต้องการ {} อาร์กิวเมนต์ แต่ได้ {}",
                        params.len(),
                        args.len()
                    ),
                    None,
                ));
            }

            // Build context from captured scope
            let mut lambda_ctx = Context::new();
            for (k, v) in captured_vars.iter() {
                lambda_ctx.set(k, v.clone());
            }
            for func in captured_funcs.values() {
                lambda_ctx.set_function(func.clone());
            }

            // Bind parameters
            for (i, param) in params.iter().enumerate() {
                lambda_ctx.set(param, args[i].clone());
            }

            // Evaluate body with the lambda context
            evaluate_impl(body_expr, &mut lambda_ctx, registry, depth + 1)
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "ค่านี้ไม่ใช่ lambda ไม่สามารถเรียกได้",
            None,
        )),
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
            "E401",
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
            "E401",
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
            "E401",
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
                "E301",
                "หารด้วยศูนย์",
                Some(span),
            ))
        } else {
            Ok(Value::Number(a / b))
        }
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            "ใช้กับตัวเลข",
            Some(span),
        ))
    }
}

fn compare_values(l: Value, r: Value, span: Span, op: BinaryOp) -> Result<Value, FormulaError> {
    use std::cmp::Ordering;
    let ord = match (&l, &r) {
        (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
        (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
        (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
        (Value::Null, Value::Null) => Some(Ordering::Equal),
        _ => {
            return Err(FormulaError::new(
                ErrorKind::TypeError,
                "E401",
                &format!(
                    "การเปรียบเทียบใช้ได้กับประเภทเดียวกันเท่านั้น (ได้รับ {:?} และ {:?})",
                    l, r
                ),
                Some(span),
            ));
        }
    };

    match ord {
        Some(ordering) => {
            let res = match op {
                BinaryOp::Lt => ordering == Ordering::Less,
                BinaryOp::Gt => ordering == Ordering::Greater,
                BinaryOp::LtEq => ordering != Ordering::Greater,
                BinaryOp::GtEq => ordering != Ordering::Less,
                _ => false,
            };
            Ok(Value::Bool(res))
        }
        None => Ok(Value::Bool(false)),
    }
}

fn logic_and(l: Value, r: Value, span: Span) -> Result<Value, FormulaError> {
    if let (Value::Bool(a), Value::Bool(b)) = (&l, &r) {
        Ok(Value::Bool(*a && *b))
    } else {
        Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
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
            "E401",
            "OR ใช้ได้กับ boolean เท่านั้น",
            Some(span),
        ))
    }
}
