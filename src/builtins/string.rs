// src/builtins/string.rs
use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

pub fn len() -> BuiltinFunction {
    BuiltinFunction {
        name: "len".to_string(),
        arity: 1,
        call: |args| match &args[0] {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "len ต้องการข้อความหรือ array",
                None,
            )),
        },
    }
}

pub fn upper() -> BuiltinFunction {
    BuiltinFunction {
        name: "upper".to_string(),
        arity: 1,
        call: |args| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_uppercase()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "upper ต้องการข้อความ",
                    None,
                ))
            }
        },
    }
}

pub fn lower() -> BuiltinFunction {
    BuiltinFunction {
        name: "lower".to_string(),
        arity: 1,
        call: |args| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_lowercase()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "lower ต้องการข้อความ",
                    None,
                ))
            }
        },
    }
}

pub fn contains() -> BuiltinFunction {
    BuiltinFunction {
        name: "contains".to_string(),
        arity: 2,
        call: |args| match (&args[0], &args[1]) {
            (Value::String(haystack), Value::String(needle)) => {
                Ok(Value::Bool(haystack.contains(needle)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "contains ต้องการข้อความสองตัว",
                None,
            )),
        },
    }
}

pub fn starts_with() -> BuiltinFunction {
    BuiltinFunction {
        name: "starts_with".to_string(),
        arity: 2,
        call: |args| match (&args[0], &args[1]) {
            (Value::String(text), Value::String(prefix)) => {
                Ok(Value::Bool(text.starts_with(prefix)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "starts_with ต้องการข้อความสองตัว",
                None,
            )),
        },
    }
}

pub fn ends_with() -> BuiltinFunction {
    BuiltinFunction {
        name: "ends_with".to_string(),
        arity: 2,
        call: |args| match (&args[0], &args[1]) {
            (Value::String(text), Value::String(suffix)) => Ok(Value::Bool(text.ends_with(suffix))),
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "ends_with ต้องการข้อความสองตัว",
                None,
            )),
        },
    }
}

pub fn trim() -> BuiltinFunction {
    BuiltinFunction {
        name: "trim".to_string(),
        arity: 1,
        call: |args| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "trim ต้องการข้อความ",
                    None,
                ))
            }
        },
    }
}

pub fn trim_start() -> BuiltinFunction {
    BuiltinFunction {
        name: "trim_start".to_string(),
        arity: 1,
        call: |args| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim_start().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "trim_start ต้องการข้อความ",
                    None,
                ))
            }
        },
    }
}

pub fn trim_end() -> BuiltinFunction {
    BuiltinFunction {
        name: "trim_end".to_string(),
        arity: 1,
        call: |args| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim_end().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "trim_end ต้องการข้อความ",
                    None,
                ))
            }
        },
    }
}

pub fn split() -> BuiltinFunction {
    BuiltinFunction {
        name: "split".to_string(),
        arity: 2,
        call: |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(sep)) => {
                let parts: Vec<Value> =
                    s.split(sep).map(|p| Value::String(p.to_string())).collect();
                Ok(Value::Array(parts))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "split ต้องการข้อความสองตัว (ข้อความ, ตัวคั่น)",
                None,
            )),
        },
    }
}

pub fn replace() -> BuiltinFunction {
    BuiltinFunction {
        name: "replace".to_string(),
        arity: 3,
        call: |args| match (&args[0], &args[1], &args[2]) {
            (Value::String(s), Value::String(from), Value::String(to)) => {
                Ok(Value::String(s.replace(from, to)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "replace ต้องการข้อความสามตัว (ข้อความ, จาก, ไปยัง)",
                None,
            )),
        },
    }
}

pub fn substring() -> BuiltinFunction {
    BuiltinFunction {
        name: "substring".to_string(),
        arity: 3,
        call: |args| match (&args[0], &args[1], &args[2]) {
            (Value::String(s), Value::Number(start), Value::Number(len)) => {
                let start = *start as usize;
                let len = *len as usize;

                // Rust substring handling (safe)
                let sub: String = s.chars().skip(start).take(len).collect();
                Ok(Value::String(sub))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                "substring ต้องการ (ข้อความ, ตำแหน่งเริ่ม, ความยาว)",
                None,
            )),
        },
    }
}
