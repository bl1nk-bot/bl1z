//! Higher-order functions (Phase 9: Lambda & Higher-Order Functions)

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

/// Apply a lambda to arguments.
pub fn apply_lambda(
    lambda: &Value,
    args: &[Value],
    registry: &crate::functions::FunctionRegistry,
) -> Result<Value, FormulaError> {
    crate::eval::apply_lambda(lambda, args, registry)
}

/// map(array, lambda) -> Array
/// Apply lambda to each element and return new array
pub fn map_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "map".to_string(),
        arity: 2,
        call: |args| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            match lambda {
                Value::Lambda(body_expr, params, captured_scope, _) => {
                    if params.len() != 1 {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E503",
                            &format!("lambda ใน map ต้องมี 1 พารามิเตอร์ แต่ได้ {}", params.len()),
                            None,
                        ));
                    }
                    let mut registry = crate::functions::FunctionRegistry::new();
                    crate::builtins::register_all(&mut registry);

                    let mut result = Vec::new();
                    for item in arr {
                        // Create context with captured scope
                        let mut item_ctx = crate::context::Context::new();
                        for (k, v) in captured_scope.iter() {
                            item_ctx.set(k, v.clone());
                        }
                        // Set the lambda parameter
                        if let Some(param) = params.first() {
                            item_ctx.set(param, item.clone());
                        }
                        // Evaluate lambda body
                        let item_result = crate::eval::evaluate(body_expr, &item_ctx, &registry)
                            .map_err(|e| {
                                FormulaError::new(
                                    ErrorKind::EvalError,
                                    "E301",
                                    &format!("เกิดข้อผิดพลาดใน lambda: {}", e.message),
                                    e.span,
                                )
                            })?;
                        result.push(item_result);
                    }
                    Ok(Value::Array(result))
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "พารามิเตอร์ที่สองของ map ต้องเป็น lambda",
                    None,
                )),
            }
        },
    }
}

/// filter(array, lambda) -> Array
/// Keep elements where lambda returns true
pub fn filter_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "filter".to_string(),
        arity: 2,
        call: |args| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            match lambda {
                Value::Lambda(body_expr, params, captured_scope, _) => {
                    if params.len() != 1 {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E503",
                            &format!("lambda ใน filter ต้องมี 1 พารามิเตอร์ แต่ได้ {}", params.len()),
                            None,
                        ));
                    }
                    let mut registry = crate::functions::FunctionRegistry::new();
                    crate::builtins::register_all(&mut registry);

                    let mut result = Vec::new();
                    for item in arr {
                        // Create context with captured scope
                        let mut item_ctx = crate::context::Context::new();
                        for (k, v) in captured_scope.iter() {
                            item_ctx.set(k, v.clone());
                        }
                        // Set the lambda parameter
                        if let Some(param) = params.first() {
                            item_ctx.set(param, item.clone());
                        }
                        // Evaluate lambda body
                        let item_result = crate::eval::evaluate(body_expr, &item_ctx, &registry)
                            .map_err(|e| {
                                FormulaError::new(
                                    ErrorKind::EvalError,
                                    "E301",
                                    &format!("เกิดข้อผิดพลาดใน lambda: {}", e.message),
                                    e.span,
                                )
                            })?;
                        // Keep if result is truthy (true)
                        if item_result == Value::Bool(true) {
                            result.push(item.clone());
                        }
                    }
                    Ok(Value::Array(result))
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "พารามิเตอร์ที่สองของ filter ต้องเป็น lambda",
                    None,
                )),
            }
        },
    }
}

/// reduce(array, lambda, initial) -> Value
/// Accumulate array into single value using lambda
pub fn reduce_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "reduce".to_string(),
        arity: 3,
        call: |args| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            let initial = &args[2];
            match lambda {
                Value::Lambda(body_expr, params, captured_scope, _) => {
                    if params.len() != 2 {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E503",
                            &format!(
                                "lambda ใน reduce ต้องมี 2 พารามิเตอร์ (accumulator, current) แต่ได้ {}",
                                params.len()
                            ),
                            None,
                        ));
                    }
                    let mut registry = crate::functions::FunctionRegistry::new();
                    crate::builtins::register_all(&mut registry);

                    let mut accumulator = initial.clone();
                    for item in arr {
                        // Create context with captured scope
                        let mut item_ctx = crate::context::Context::new();
                        for (k, v) in captured_scope.iter() {
                            item_ctx.set(k, v.clone());
                        }
                        // Set the lambda parameters (accumulator, current)
                        if let Some(param) = params.first() {
                            item_ctx.set(param, accumulator);
                        }
                        if let Some(param) = params.get(1) {
                            item_ctx.set(param, item.clone());
                        }
                        // Evaluate lambda body
                        accumulator = crate::eval::evaluate(body_expr, &item_ctx, &registry)
                            .map_err(|e| {
                                FormulaError::new(
                                    ErrorKind::EvalError,
                                    "E301",
                                    &format!("เกิดข้อผิดพลาดใน lambda: {}", e.message),
                                    e.span,
                                )
                            })?;
                    }
                    Ok(accumulator)
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "พารามิเตอร์ที่สองของ reduce ต้องเป็น lambda",
                    None,
                )),
            }
        },
    }
}

/// Helper to compare two Values for sorting.
pub fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    use Value::*;
    match (a, b) {
        (Number(na), Number(nb)) => na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal),
        (String(sa), String(sb)) => sa.cmp(sb),
        (Bool(ba), Bool(bb)) => ba.cmp(bb),
        (Null, Null) => std::cmp::Ordering::Equal,
        (Null, _) => std::cmp::Ordering::Greater, // null goes to the end
        (_, Null) => std::cmp::Ordering::Less,    // null goes to the end

        // Type precedence comparison when types are different
        (Number(_), String(_)) => std::cmp::Ordering::Less,
        (String(_), Number(_)) => std::cmp::Ordering::Greater,
        (Number(_), Bool(_)) => std::cmp::Ordering::Less,
        (Bool(_), Number(_)) => std::cmp::Ordering::Greater,
        (String(_), Bool(_)) => std::cmp::Ordering::Less,
        (Bool(_), String(_)) => std::cmp::Ordering::Greater,

        // Fallback comparison for other types
        (x, y) => {
            let type_val = |v: &Value| match v {
                Number(_) => 1,
                String(_) => 2,
                Bool(_) => 3,
                Array(_) => 4,
                Map(_) => 5,
                Lambda(_, _, _, _) => 6,
                Null => 7,
            };
            type_val(x).cmp(&type_val(y))
        }
    }
}

/// sort(array, key_lambda?) -> Array
/// Sort array of numbers/strings/booleans ascending, optionally using key computed by lambda
pub fn sort_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "sort".to_string(),
        arity: 999, // Variadic (1 or 2 args)
        call: |args| {
            if args.is_empty() || args.len() > 2 {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    &format!("ฟังก์ชัน 'sort' ต้องการ 1 หรือ 2 อาร์กิวเมนต์ แต่ได้ {}", args.len()),
                    None,
                ));
            }
            let arr = require_array(&args[0])?;
            let mut items = arr.clone();

            if args.len() == 1 {
                items.sort_by(compare_values);
                Ok(Value::Array(items))
            } else {
                let lambda = &args[1];
                match lambda {
                    Value::Lambda(body_expr, params, captured_scope, _) => {
                        if params.len() != 1 {
                            return Err(FormulaError::new(
                                ErrorKind::FunctionError,
                                "E503",
                                &format!("lambda ใน sort ต้องมี 1 พารามิเตอร์ แต่ได้ {}", params.len()),
                                None,
                            ));
                        }
                        let mut registry = crate::functions::FunctionRegistry::new();
                        crate::builtins::register_all(&mut registry);

                        let mut eval_error = None;
                        let mut items_with_keys = Vec::new();

                        for item in items {
                            let mut item_ctx = crate::context::Context::new();
                            for (k, v) in captured_scope.iter() {
                                item_ctx.set(k, v.clone());
                            }
                            if let Some(param) = params.first() {
                                item_ctx.set(param, item.clone());
                            }
                            match crate::eval::evaluate(body_expr, &item_ctx, &registry) {
                                Ok(key) => items_with_keys.push((item, key)),
                                Err(e) => {
                                    eval_error = Some(e);
                                    break;
                                }
                            }
                        }

                        if let Some(err) = eval_error {
                            return Err(err);
                        }

                        items_with_keys.sort_by(|(_, ka), (_, kb)| compare_values(ka, kb));

                        let sorted_items =
                            items_with_keys.into_iter().map(|(item, _)| item).collect();
                        Ok(Value::Array(sorted_items))
                    }
                    _ => Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "พารามิเตอร์ที่สองของ sort ต้องเป็น lambda",
                        None,
                    )),
                }
            }
        },
    }
}

/// sort_with(array, comparator_lambda) -> Array
/// Sort array using custom comparator, with error propagation
pub fn sort_with_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "sort_with".to_string(),
        arity: 2,
        call: |args| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            match lambda {
                Value::Lambda(body_expr, params, captured_scope, _) => {
                    if params.len() != 2 {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E503",
                            &format!(
                                "lambda ใน sort_with ต้องมี 2 พารามิเตอร์ (a, b) แต่ได้ {}",
                                params.len()
                            ),
                            None,
                        ));
                    }
                    let mut registry = crate::functions::FunctionRegistry::new();
                    crate::builtins::register_all(&mut registry);

                    let mut items = arr.clone();
                    let eval_error = std::cell::RefCell::new(None);

                    items.sort_by(|a, b| {
                        if eval_error.borrow().is_some() {
                            return std::cmp::Ordering::Equal;
                        }

                        // Create context for evaluating comparator
                        let mut cmp_ctx = crate::context::Context::new();
                        for (k, v) in captured_scope.iter() {
                            cmp_ctx.set(k, v.clone());
                        }
                        // Set lambda parameters (a, b)
                        if let Some(param) = params.first() {
                            cmp_ctx.set(param, a.clone());
                        }
                        if let Some(param) = params.get(1) {
                            cmp_ctx.set(param, b.clone());
                        }
                        // Evaluate comparator - should return negative, 0, or positive
                        match crate::eval::evaluate(body_expr, &cmp_ctx, &registry) {
                            Ok(Value::Number(n)) => {
                                if n < 0.0 {
                                    std::cmp::Ordering::Less
                                } else if n > 0.0 {
                                    std::cmp::Ordering::Greater
                                } else {
                                    std::cmp::Ordering::Equal
                                }
                            }
                            Ok(other) => {
                                *eval_error.borrow_mut() = Some(FormulaError::new(
                                    ErrorKind::TypeError,
                                    "E401",
                                    &format!("comparator lambda ต้องคืนค่าเป็นตัวเลข แต่ได้ {:?}", other),
                                    None,
                                ));
                                std::cmp::Ordering::Equal
                            }
                            Err(e) => {
                                *eval_error.borrow_mut() = Some(e);
                                std::cmp::Ordering::Equal
                            }
                        }
                    });

                    // Propagate error if any occurred during sorting
                    if let Some(err) = eval_error.into_inner() {
                        return Err(err);
                    }

                    Ok(Value::Array(items))
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "พารามิเตอร์ที่สองของ sort_with ต้องเป็น lambda",
                    None,
                )),
            }
        },
    }
}

/// unique(array, key_lambda?) -> Array
/// Return array with duplicate values removed, optionally using a key lambda
pub fn unique_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "unique".to_string(),
        arity: 999, // Variadic (1 or 2 args)
        call: |args| {
            if args.is_empty() || args.len() > 2 {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E503",
                    &format!(
                        "ฟังก์ชัน 'unique' ต้องการ 1 หรือ 2 อาร์กิวเมนต์ แต่ได้ {}",
                        args.len()
                    ),
                    None,
                ));
            }
            let arr = require_array(&args[0])?;

            if args.len() == 1 {
                let mut result = Vec::new();
                for v in arr {
                    if !result.contains(v) {
                        result.push(v.clone());
                    }
                }
                Ok(Value::Array(result))
            } else {
                let lambda = &args[1];
                match lambda {
                    Value::Lambda(body_expr, params, captured_scope, _) => {
                        if params.len() != 1 {
                            return Err(FormulaError::new(
                                ErrorKind::FunctionError,
                                "E503",
                                &format!("lambda ใน unique ต้องมี 1 พารามิเตอร์ แต่ได้ {}", params.len()),
                                None,
                            ));
                        }
                        let mut registry = crate::functions::FunctionRegistry::new();
                        crate::builtins::register_all(&mut registry);

                        let mut result = Vec::new();
                        let mut seen_keys = Vec::new();

                        for item in arr {
                            let mut item_ctx = crate::context::Context::new();
                            for (k, v) in captured_scope.iter() {
                                item_ctx.set(k, v.clone());
                            }
                            if let Some(param) = params.first() {
                                item_ctx.set(param, item.clone());
                            }
                            let key = crate::eval::evaluate(body_expr, &item_ctx, &registry)
                                .map_err(|e| {
                                    FormulaError::new(
                                        ErrorKind::EvalError,
                                        "E301",
                                        &format!("เกิดข้อผิดพลาดใน lambda: {}", e.message),
                                        e.span,
                                    )
                                })?;

                            if !seen_keys.contains(&key) {
                                seen_keys.push(key);
                                result.push(item.clone());
                            }
                        }
                        Ok(Value::Array(result))
                    }
                    _ => Err(FormulaError::new(
                        ErrorKind::TypeError,
                        "E401",
                        "พารามิเตอร์ที่สองของ unique ต้องเป็น lambda",
                        None,
                    )),
                }
            }
        },
    }
}

/// group_by(array, lambda) -> Map
/// Group array elements by key computed by lambda
pub fn group_by_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "group_by".to_string(),
        arity: 2,
        call: |args| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            match lambda {
                Value::Lambda(body_expr, params, captured_scope, _) => {
                    if params.len() != 1 {
                        return Err(FormulaError::new(
                            ErrorKind::FunctionError,
                            "E503",
                            &format!("lambda ใน group_by ต้องมี 1 พารามิเตอร์ แต่ได้ {}", params.len()),
                            None,
                        ));
                    }
                    let mut registry = crate::functions::FunctionRegistry::new();
                    crate::builtins::register_all(&mut registry);

                    let mut groups: std::collections::HashMap<String, Vec<Value>> =
                        std::collections::HashMap::new();
                    for item in arr {
                        // Create context with captured scope
                        let mut item_ctx = crate::context::Context::new();
                        for (k, v) in captured_scope.iter() {
                            item_ctx.set(k, v.clone());
                        }
                        // Set the lambda parameter
                        if let Some(param) = params.first() {
                            item_ctx.set(param, item.clone());
                        }
                        // Evaluate lambda body to get key
                        let key_result = crate::eval::evaluate(body_expr, &item_ctx, &registry)
                            .map_err(|e| {
                                FormulaError::new(
                                    ErrorKind::EvalError,
                                    "E301",
                                    &format!("เกิดข้อผิดพลาดใน lambda: {}", e.message),
                                    e.span,
                                )
                            })?;
                        // Convert key to string
                        let key = match &key_result {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => format!("{:?}", key_result),
                        };
                        groups.entry(key).or_default().push(item.clone());
                    }
                    // Convert Vec<Value> to Value::Array
                    let result_map: std::collections::HashMap<String, Value> = groups
                        .into_iter()
                        .map(|(k, v)| (k, Value::Array(v)))
                        .collect();
                    Ok(Value::Map(result_map))
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    "พารามิเตอร์ที่สองของ group_by ต้องเป็น lambda",
                    None,
                )),
            }
        },
    }
}

// Helper functions

fn require_array(value: &Value) -> Result<&Vec<Value>, FormulaError> {
    match value {
        Value::Array(arr) => Ok(arr),
        _ => Err(FormulaError::new(
            ErrorKind::FunctionError,
            "E501",
            "ต้องการ array",
            None,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_fn(f: BuiltinFunction, args: Vec<Value>) -> Result<Value, FormulaError> {
        (f.call)(&args)
    }

    // -- map() tests --

    #[test]
    fn test_map_requires_array() {
        // map with only 1 argument (non-array) - should fail type check on first arg
        let result = call_fn(map_fn(), vec![Value::Number(5.0)]);
        assert!(result.is_err());
    }

    // -- sort() tests --

    #[test]
    fn test_sort_numbers_ascending() {
        let result = call_fn(
            sort_fn(),
            vec![Value::Array(vec![
                Value::Number(3.0),
                Value::Number(1.0),
                Value::Number(2.0),
            ])],
        )
        .unwrap();
        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::Number(1.0));
                assert_eq!(arr[1], Value::Number(2.0));
                assert_eq!(arr[2], Value::Number(3.0));
            }
            _ => panic!("expected Array"),
        }
    }

    #[test]
    fn test_sort_empty_array() {
        let result = call_fn(sort_fn(), vec![Value::Array(vec![])]).unwrap();
        match result {
            Value::Array(arr) => assert!(arr.is_empty()),
            _ => panic!("expected Array"),
        }
    }

    #[test]
    fn test_sort_non_array_returns_error() {
        let result = call_fn(sort_fn(), vec![Value::Number(5.0)]);
        assert!(result.is_err());
    }

    // -- unique() tests --

    #[test]
    fn test_unique_removes_duplicates() {
        let result = call_fn(
            unique_fn(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(2.0),
            ])],
        )
        .unwrap();
        match result {
            Value::Array(arr) => assert_eq!(arr.len(), 3),
            _ => panic!("expected Array"),
        }
    }

    #[test]
    fn test_unique_empty_array() {
        let result = call_fn(unique_fn(), vec![Value::Array(vec![])]).unwrap();
        match result {
            Value::Array(arr) => assert!(arr.is_empty()),
            _ => panic!("expected Array"),
        }
    }

    #[test]
    fn test_unique_non_array_returns_error() {
        let result = call_fn(unique_fn(), vec![Value::String("test".to_string())]);
        assert!(result.is_err());
    }
}
