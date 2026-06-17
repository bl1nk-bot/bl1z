//! ฟังก์ชันสำหรับ Set operations (Phase 11.6)

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;
use std::collections::HashSet;

/// Helper: extract a HashSet from either Value::Set or Value::Array
fn extract_set(val: &Value, _param_name: &str) -> Result<HashSet<Value>, FormulaError> {
    match val {
        Value::Set(s) => Ok(s.clone()),
        Value::Array(arr) => Ok(arr.iter().cloned().collect()),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            &format!("ต้องการ Set แต่ได้ {}", val.type_name()),
            None,
        )),
    }
}

/// set_union(set1, set2) → Value::Set (union of two sets)
pub fn set_union() -> BuiltinFunction {
    BuiltinFunction {
        name: "set_union".to_string(),
        arity: 2,
        call: |args, _| {
            let s1 = extract_set(&args[0], "set1")?;
            let s2 = extract_set(&args[1], "set2")?;
            let mut result = s1;
            result.extend(s2);
            Ok(Value::Set(result))
        },
    }
}

/// set_intersection(set1, set2) → Value::Set (intersection)
pub fn set_intersection() -> BuiltinFunction {
    BuiltinFunction {
        name: "set_intersection".to_string(),
        arity: 2,
        call: |args, _| {
            let s1 = extract_set(&args[0], "set1")?;
            let s2 = extract_set(&args[1], "set2")?;
            let result: HashSet<Value> = s1.into_iter().filter(|v| s2.contains(v)).collect();
            Ok(Value::Set(result))
        },
    }
}

/// set_difference(set1, set2) → Value::Set (elements in set1 not in set2)
pub fn set_difference() -> BuiltinFunction {
    BuiltinFunction {
        name: "set_difference".to_string(),
        arity: 2,
        call: |args, _| {
            let s1 = extract_set(&args[0], "set1")?;
            let s2 = extract_set(&args[1], "set2")?;
            let result: HashSet<Value> = s1.into_iter().filter(|v| !s2.contains(v)).collect();
            Ok(Value::Set(result))
        },
    }
}

/// set_in(value, set) → Value::Bool (check membership)
pub fn set_in() -> BuiltinFunction {
    BuiltinFunction {
        name: "set_in".to_string(),
        arity: 2,
        call: |args, _| {
            let container = &args[1];
            match container {
                Value::Set(s) => Ok(Value::Bool(s.contains(&args[0]))),
                Value::Array(arr) => Ok(Value::Bool(arr.contains(&args[0]))),
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    &format!("ต้องการ Set แต่ได้ {}", container.type_name()),
                    None,
                )),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_fn(f: BuiltinFunction, args: Vec<Value>) -> Result<Value, FormulaError> {
        let registry = crate::functions::FunctionRegistry::new();
        (f.call)(&args, &registry)
    }

    fn num_set(vals: &[f64]) -> Value {
        Value::Set(vals.iter().map(|&n| Value::Number(n)).collect())
    }

    fn num_arr(vals: &[f64]) -> Value {
        Value::Array(vals.iter().map(|&n| Value::Number(n)).collect())
    }

    // -- set_union tests --

    #[test]
    fn test_set_union_basic() {
        let result = call_fn(
            set_union(),
            vec![num_set(&[1.0, 2.0]), num_set(&[2.0, 3.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_set_union_disjoint() {
        let result = call_fn(
            set_union(),
            vec![num_set(&[1.0, 2.0]), num_set(&[3.0, 4.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_set_union_empty() {
        let result = call_fn(
            set_union(),
            vec![num_set(&[1.0]), Value::Set(HashSet::new())],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0]));
    }

    #[test]
    fn test_set_union_array_as_set() {
        let result = call_fn(
            set_union(),
            vec![num_arr(&[1.0, 2.0]), num_set(&[2.0, 3.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_set_union_type_error() {
        let result = call_fn(set_union(), vec![Value::Number(1.0), num_set(&[1.0])]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::TypeError);
    }

    // -- set_intersection tests --

    #[test]
    fn test_set_intersection_basic() {
        let result = call_fn(
            set_intersection(),
            vec![num_set(&[1.0, 2.0, 3.0]), num_set(&[2.0, 3.0, 4.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[2.0, 3.0]));
    }

    #[test]
    fn test_set_intersection_disjoint() {
        let result = call_fn(
            set_intersection(),
            vec![num_set(&[1.0, 2.0]), num_set(&[3.0, 4.0])],
        )
        .unwrap();
        assert_eq!(result, Value::Set(HashSet::new()));
    }

    #[test]
    fn test_set_intersection_empty() {
        let result = call_fn(
            set_intersection(),
            vec![num_set(&[1.0]), Value::Set(HashSet::new())],
        )
        .unwrap();
        assert_eq!(result, Value::Set(HashSet::new()));
    }

    #[test]
    fn test_set_intersection_type_error() {
        let result = call_fn(
            set_intersection(),
            vec![Value::String("bad".to_string()), num_set(&[1.0])],
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::TypeError);
    }

    // -- set_difference tests --

    #[test]
    fn test_set_difference_basic() {
        let result = call_fn(
            set_difference(),
            vec![num_set(&[1.0, 2.0, 3.0]), num_set(&[2.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 3.0]));
    }

    #[test]
    fn test_set_difference_no_overlap() {
        let result = call_fn(
            set_difference(),
            vec![num_set(&[1.0, 2.0]), num_set(&[3.0, 4.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 2.0]));
    }

    #[test]
    fn test_set_difference_subtract_all() {
        let result = call_fn(
            set_difference(),
            vec![num_set(&[1.0, 2.0]), num_set(&[1.0, 2.0])],
        )
        .unwrap();
        assert_eq!(result, Value::Set(HashSet::new()));
    }

    #[test]
    fn test_set_difference_array_as_set() {
        let result = call_fn(
            set_difference(),
            vec![num_arr(&[1.0, 2.0, 3.0]), num_set(&[2.0])],
        )
        .unwrap();
        assert_eq!(result, num_set(&[1.0, 3.0]));
    }

    #[test]
    fn test_set_difference_type_error() {
        let result = call_fn(set_difference(), vec![Value::Bool(true), num_set(&[1.0])]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::TypeError);
    }

    // -- set_in tests --

    #[test]
    fn test_set_in_found() {
        let result = call_fn(
            set_in(),
            vec![Value::Number(2.0), num_set(&[1.0, 2.0, 3.0])],
        )
        .unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_set_in_not_found() {
        let result = call_fn(
            set_in(),
            vec![Value::Number(5.0), num_set(&[1.0, 2.0, 3.0])],
        )
        .unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_set_in_array() {
        let result = call_fn(
            set_in(),
            vec![Value::Number(2.0), num_arr(&[1.0, 2.0, 3.0])],
        )
        .unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_set_in_type_error() {
        let result = call_fn(
            set_in(),
            vec![Value::Number(1.0), Value::String("bad".to_string())],
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::TypeError);
    }

    #[test]
    fn test_set_in_string_values() {
        let s: HashSet<Value> = ["a", "b", "c"]
            .iter()
            .map(|s| Value::String(s.to_string()))
            .collect();
        let result = call_fn(
            set_in(),
            vec![Value::String("b".to_string()), Value::Set(s.clone())],
        )
        .unwrap();
        assert_eq!(result, Value::Bool(true));

        let result = call_fn(
            set_in(),
            vec![Value::String("d".to_string()), Value::Set(s)],
        )
        .unwrap();
        assert_eq!(result, Value::Bool(false));
    }
}
