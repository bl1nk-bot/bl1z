// src/builtins/string.rs
use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

pub fn len() -> BuiltinFunction {
    BuiltinFunction {
        name: "len".to_string(),
        arity: 1,
        call: |args, _| match &args[0] {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!("len ต้องการ String หรือ Array แต่ได้ {}", args[0].type_name()),
                None,
            )),
        },
    }
}

pub fn upper() -> BuiltinFunction {
    BuiltinFunction {
        name: "upper".to_string(),
        arity: 1,
        call: |args, _| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_uppercase()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("upper ต้องการ String แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_lowercase()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("lower ต้องการ String แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| match (&args[0], &args[1]) {
            (Value::String(haystack), Value::String(needle)) => {
                Ok(Value::Bool(haystack.contains(needle)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!(
                    "contains ต้องการ String, String แต่ได้ {}, {}",
                    args[0].type_name(),
                    args[1].type_name()
                ),
                None,
            )),
        },
    }
}

pub fn starts_with() -> BuiltinFunction {
    BuiltinFunction {
        name: "starts_with".to_string(),
        arity: 2,
        call: |args, _| match (&args[0], &args[1]) {
            (Value::String(text), Value::String(prefix)) => {
                Ok(Value::Bool(text.starts_with(prefix)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!(
                    "starts_with ต้องการ String, String แต่ได้ {}, {}",
                    args[0].type_name(),
                    args[1].type_name()
                ),
                None,
            )),
        },
    }
}

pub fn ends_with() -> BuiltinFunction {
    BuiltinFunction {
        name: "ends_with".to_string(),
        arity: 2,
        call: |args, _| match (&args[0], &args[1]) {
            (Value::String(text), Value::String(suffix)) => Ok(Value::Bool(text.ends_with(suffix))),
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!(
                    "ends_with ต้องการ String, String แต่ได้ {}, {}",
                    args[0].type_name(),
                    args[1].type_name()
                ),
                None,
            )),
        },
    }
}

pub fn trim() -> BuiltinFunction {
    BuiltinFunction {
        name: "trim".to_string(),
        arity: 1,
        call: |args, _| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("trim ต้องการ String แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim_start().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("trim_start ต้องการ String แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim_end().to_string()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("trim_end ต้องการ String แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(sep)) => {
                let parts: Vec<Value> =
                    s.split(sep).map(|p| Value::String(p.to_string())).collect();
                Ok(Value::Array(parts))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!(
                    "split ต้องการ String, String (ข้อความ, ตัวคั่น) แต่ได้ {}, {}",
                    args[0].type_name(),
                    args[1].type_name()
                ),
                None,
            )),
        },
    }
}

pub fn replace() -> BuiltinFunction {
    BuiltinFunction {
        name: "replace".to_string(),
        arity: 3,
        call: |args, _| match (&args[0], &args[1], &args[2]) {
            (Value::String(s), Value::String(from), Value::String(to)) => {
                Ok(Value::String(s.replace(from, to)))
            }
            _ => Err(FormulaError::new(
                ErrorKind::FunctionError,
                "E501",
                &format!(
                    "replace ต้องการ String, String, String แต่ได้ {}, {}, {}",
                    args[0].type_name(),
                    args[1].type_name(),
                    args[2].type_name()
                ),
                None,
            )),
        },
    }
}

pub fn substring() -> BuiltinFunction {
    BuiltinFunction {
        name: "substring".to_string(),
        arity: 3,
        call: |args, _| {
            match (&args[0], &args[1], &args[2]) {
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
                    &format!(
                        "substring ต้องการ String, Number, Number (ข้อความ, ตำแหน่งเริ่ม, ความยาว) แต่ได้ {}, {}, {}",
                        args[0].type_name(),
                        args[1].type_name(),
                        args[2].type_name()
                    ),
                    None,
                )),
            }
        },
    }
}
