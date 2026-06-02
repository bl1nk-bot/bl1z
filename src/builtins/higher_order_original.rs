use crate::error::{ErrorKind, FormulaError};
use crate::eval::evaluate_recursive;
use crate::functions::{BuiltinFunction, FunctionRegistry};
use crate::value::Value;
use std::sync::Arc;

pub fn map(registry: &FunctionRegistry) -> BuiltinFunction {
    let registry_clone = registry.clone_box();
    BuiltinFunction {
        name: "map".to_string(),
        arity: 2,
        call: Arc::new(move |args| {
            let arr = match &args[0] {
                Value::Array(a) => a,
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 1 of map must be an array",
                        None,
                    ))
                }
            };
            let (params, body, env) = match &args[1] {
                Value::Closure { params, body, env } => (params, body, env),
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 2 of map must be a lambda",
                        None,
                    ))
                }
            };
            if params.len() != 1 {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    "Lambda for map must have 1 parameter",
                    None,
                ));
            }
            let mut results = Vec::new();
            for val in arr {
                let mut local_ctx = env.clone();
                local_ctx.set(&params[0], val.clone());
                results.push(evaluate_recursive(body, &local_ctx, &registry_clone, 0)?);
            }
            Ok(Value::Array(results))
        }),
    }
}

pub fn filter(registry: &FunctionRegistry) -> BuiltinFunction {
    let registry_clone = registry.clone_box();
    BuiltinFunction {
        name: "filter".to_string(),
        arity: 2,
        call: Arc::new(move |args| {
            let arr = match &args[0] {
                Value::Array(a) => a,
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 1 of filter must be an array",
                        None,
                    ))
                }
            };
            let (params, body, env) = match &args[1] {
                Value::Closure { params, body, env } => (params, body, env),
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 2 of filter must be a lambda",
                        None,
                    ))
                }
            };
            if params.len() != 1 {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    "Lambda for filter must have 1 parameter",
                    None,
                ));
            }
            let mut results = Vec::new();
            for val in arr {
                let mut local_ctx = env.clone();
                local_ctx.set(&params[0], val.clone());
                if let Value::Bool(true) = evaluate_recursive(body, &local_ctx, &registry_clone, 0)?
                {
                    results.push(val.clone());
                }
            }
            Ok(Value::Array(results))
        }),
    }
}

pub fn reduce(registry: &FunctionRegistry) -> BuiltinFunction {
    let registry_clone = registry.clone_box();
    BuiltinFunction {
        name: "reduce".to_string(),
        arity: 3,
        call: Arc::new(move |args| {
            let arr = match &args[0] {
                Value::Array(a) => a,
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 1 of reduce must be an array",
                        None,
                    ))
                }
            };
            let initial = args[1].clone();
            let (params, body, env) = match &args[2] {
                Value::Closure { params, body, env } => (params, body, env),
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "Argument 3 of reduce must be a lambda",
                        None,
                    ))
                }
            };
            if params.len() != 2 {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    "Lambda for reduce must have 2 parameters",
                    None,
                ));
            }
            let mut acc = initial;
            for val in arr {
                let mut local_ctx = env.clone();
                local_ctx.set(&params[0], acc);
                local_ctx.set(&params[1], val.clone());
                acc = evaluate_recursive(body, &local_ctx, &registry_clone, 0)?;
            }
            Ok(acc)
        }),
    }
}
