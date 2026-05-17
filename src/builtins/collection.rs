//! ฟังก์ชันเกี่ยวกับ array (Phase 6.1)

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

/// sum(array) -> Number
/// หาผลรวมของตัวเลขใน array
pub fn sum() -> BuiltinFunction {
    BuiltinFunction {
        name: "sum".to_string(),
        arity: 1,
        call: |args| {
            let arr = require_array(&args[0])?;
            let total: f64 = arr
                .iter()
                .map(require_number)
                .sum::<Result<f64, FormulaError>>()?;
            Ok(Value::Number(total))
        },
    }
}

/// avg(array) -> Number
/// หาค่าเฉลี่ยของตัวเลขใน array
pub fn avg() -> BuiltinFunction {
    BuiltinFunction {
        name: "avg".to_string(),
        arity: 1,
        call: |args| {
            let arr = require_array(&args[0])?;
            if arr.is_empty() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E504",
                    "avg ไม่สามารถใช้กับ array ว่าง",
                    None,
                ));
            }
            let total: f64 = arr
                .iter()
                .map(require_number)
                .sum::<Result<f64, FormulaError>>()?;
            Ok(Value::Number(total / arr.len() as f64))
        },
    }
}

/// min(array) -> Number
/// คืนค่าต่ำสุดใน array
pub fn min_arr() -> BuiltinFunction {
    BuiltinFunction {
        name: "min".to_string(),
        arity: 1,
        call: |args| {
            let arr = require_array(&args[0])?;
            if arr.is_empty() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E504",
                    "min ไม่สามารถใช้กับ array ว่าง",
                    None,
                ));
            }
            let mut min_val: Option<f64> = None;
            for v in arr {
                let n = require_number(v)?;
                min_val = Some(match min_val {
                    None => n,
                    Some(cur) => {
                        if n < cur {
                            n
                        } else {
                            cur
                        }
                    }
                });
            }
            Ok(Value::Number(min_val.unwrap()))
        },
    }
}

/// max(array) -> Number
/// คืนค่าสูงสุดใน array
pub fn max_arr() -> BuiltinFunction {
    BuiltinFunction {
        name: "max".to_string(),
        arity: 1,
        call: |args| {
            let arr = require_array(&args[0])?;
            if arr.is_empty() {
                return Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E504",
                    "max ไม่สามารถใช้กับ array ว่าง",
                    None,
                ));
            }
            let mut max_val: Option<f64> = None;
            for v in arr {
                let n = require_number(v)?;
                max_val = Some(match max_val {
                    None => n,
                    Some(cur) => {
                        if n > cur {
                            n
                        } else {
                            cur
                        }
                    }
                });
            }
            Ok(Value::Number(max_val.unwrap()))
        },
    }
}

/// join(array, separator) -> String
/// ต่อสมาชิก array ที่เป็น string ด้วยตัวคั่น
pub fn join() -> BuiltinFunction {
    BuiltinFunction {
        name: "join".to_string(),
        arity: 2,
        call: |args| {
            let arr = require_array(&args[0])?;
            let sep = require_string(&args[1])?;
            let parts: Result<Vec<String>, FormulaError> = arr.iter().map(require_string).collect();
            Ok(Value::String(parts?.join(&sep)))
        },
    }
}

/// count(array) -> Number
/// นับจำนวนสมาชิกใน array (เหมือน len)
pub fn count() -> BuiltinFunction {
    BuiltinFunction {
        name: "count".to_string(),
        arity: 1,
        call: |args| {
            let arr = require_array(&args[0])?;
            Ok(Value::Number(arr.len() as f64))
        },
    }
}

// -- Helpers --

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

fn require_number(value: &Value) -> Result<f64, FormulaError> {
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(FormulaError::new(
            ErrorKind::FunctionError,
            "E501",
            "ต้องการตัวเลข",
            None,
        )),
    }
}

fn require_string(value: &Value) -> Result<String, FormulaError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => Err(FormulaError::new(
            ErrorKind::FunctionError,
            "E501",
            "ต้องการข้อความ",
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

    // -- sum() tests --

    #[test]
    fn test_sum_basic() {
        let result = call_fn(
            sum(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(6.0));
    }

    #[test]
    fn test_sum_empty_array() {
        let result = call_fn(sum(), vec![Value::Array(vec![])]).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_sum_single_element() {
        let result = call_fn(sum(), vec![Value::Array(vec![Value::Number(42.0)])]).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_sum_with_floats() {
        let result = call_fn(
            sum(),
            vec![Value::Array(vec![Value::Number(1.5), Value::Number(2.5)])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn test_sum_non_array_arg() {
        let result = call_fn(sum(), vec![Value::Number(5.0)]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::FunctionError);
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_sum_non_number_element() {
        let result = call_fn(
            sum(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::String("x".to_string()),
            ])],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::FunctionError);
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_sum_negative_numbers() {
        let result = call_fn(
            sum(),
            vec![Value::Array(vec![
                Value::Number(-1.0),
                Value::Number(-2.0),
                Value::Number(3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    // -- avg() tests --

    #[test]
    fn test_avg_basic() {
        let result = call_fn(
            avg(),
            vec![Value::Array(vec![
                Value::Number(10.0),
                Value::Number(20.0),
                Value::Number(30.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    #[test]
    fn test_avg_single_element() {
        let result = call_fn(avg(), vec![Value::Array(vec![Value::Number(7.0)])]).unwrap();
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_avg_empty_array_error() {
        let result = call_fn(avg(), vec![Value::Array(vec![])]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::FunctionError);
        assert_eq!(err.code, "E504");
    }

    #[test]
    fn test_avg_non_array_arg() {
        let result = call_fn(avg(), vec![Value::Bool(true)]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_avg_non_number_element() {
        let result = call_fn(
            avg(),
            vec![Value::Array(vec![Value::Number(1.0), Value::Bool(false)])],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    // -- min_arr() tests --

    #[test]
    fn test_min_basic() {
        let result = call_fn(
            min_arr(),
            vec![Value::Array(vec![
                Value::Number(5.0),
                Value::Number(2.0),
                Value::Number(8.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_min_single_element() {
        let result = call_fn(min_arr(), vec![Value::Array(vec![Value::Number(99.0)])]).unwrap();
        assert_eq!(result, Value::Number(99.0));
    }

    #[test]
    fn test_min_all_same_values() {
        let result = call_fn(
            min_arr(),
            vec![Value::Array(vec![
                Value::Number(3.0),
                Value::Number(3.0),
                Value::Number(3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_min_with_negative() {
        let result = call_fn(
            min_arr(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::Number(-5.0),
                Value::Number(3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    #[test]
    fn test_min_empty_array_error() {
        let result = call_fn(min_arr(), vec![Value::Array(vec![])]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::FunctionError);
        assert_eq!(err.code, "E504");
    }

    #[test]
    fn test_min_non_array_arg() {
        let result = call_fn(min_arr(), vec![Value::Null]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_min_non_number_element() {
        let result = call_fn(
            min_arr(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::String("bad".to_string()),
            ])],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    // -- max_arr() tests --

    #[test]
    fn test_max_basic() {
        let result = call_fn(
            max_arr(),
            vec![Value::Array(vec![
                Value::Number(5.0),
                Value::Number(2.0),
                Value::Number(8.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_max_single_element() {
        let result = call_fn(max_arr(), vec![Value::Array(vec![Value::Number(42.0)])]).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_max_all_same_values() {
        let result = call_fn(
            max_arr(),
            vec![Value::Array(vec![Value::Number(7.0), Value::Number(7.0)])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_max_with_negative() {
        let result = call_fn(
            max_arr(),
            vec![Value::Array(vec![
                Value::Number(-1.0),
                Value::Number(-5.0),
                Value::Number(-3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    #[test]
    fn test_max_empty_array_error() {
        let result = call_fn(max_arr(), vec![Value::Array(vec![])]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::FunctionError);
        assert_eq!(err.code, "E504");
    }

    #[test]
    fn test_max_non_array_arg() {
        let result = call_fn(max_arr(), vec![Value::String("hello".to_string())]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_max_non_number_element() {
        let result = call_fn(
            max_arr(),
            vec![Value::Array(vec![Value::Bool(true), Value::Number(1.0)])],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    // -- join() tests --

    #[test]
    fn test_join_basic() {
        let result = call_fn(
            join(),
            vec![
                Value::Array(vec![
                    Value::String("a".to_string()),
                    Value::String("b".to_string()),
                    Value::String("c".to_string()),
                ]),
                Value::String(", ".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::String("a, b, c".to_string()));
    }

    #[test]
    fn test_join_empty_array() {
        let result = call_fn(
            join(),
            vec![Value::Array(vec![]), Value::String("-".to_string())],
        )
        .unwrap();
        assert_eq!(result, Value::String("".to_string()));
    }

    #[test]
    fn test_join_empty_separator() {
        let result = call_fn(
            join(),
            vec![
                Value::Array(vec![
                    Value::String("x".to_string()),
                    Value::String("y".to_string()),
                ]),
                Value::String("".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::String("xy".to_string()));
    }

    #[test]
    fn test_join_single_element() {
        let result = call_fn(
            join(),
            vec![
                Value::Array(vec![Value::String("only".to_string())]),
                Value::String("-".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::String("only".to_string()));
    }

    #[test]
    fn test_join_non_array_arg() {
        let result = call_fn(
            join(),
            vec![
                Value::String("not_array".to_string()),
                Value::String("-".to_string()),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_join_non_string_element() {
        let result = call_fn(
            join(),
            vec![
                Value::Array(vec![Value::String("a".to_string()), Value::Number(1.0)]),
                Value::String("-".to_string()),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    #[test]
    fn test_join_non_string_separator() {
        let result = call_fn(
            join(),
            vec![
                Value::Array(vec![Value::String("a".to_string())]),
                Value::Number(1.0),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }

    // -- count() tests --

    #[test]
    fn test_count_basic() {
        let result = call_fn(
            count(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_count_empty_array() {
        let result = call_fn(count(), vec![Value::Array(vec![])]).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_count_mixed_types() {
        // count does not validate element types, just counts
        let result = call_fn(
            count(),
            vec![Value::Array(vec![
                Value::Number(1.0),
                Value::String("a".to_string()),
                Value::Bool(true),
                Value::Null,
            ])],
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn test_count_non_array_arg() {
        let result = call_fn(count(), vec![Value::Number(5.0)]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "E501");
    }
}
