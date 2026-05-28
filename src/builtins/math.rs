//! ฟังก์ชันทางคณิตศาสตร์

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

pub fn abs() -> BuiltinFunction {
    BuiltinFunction {
        name: "abs".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.abs()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "abs ต้องการตัวเลข",
                    None,
                ))
            }
        },
    }
}

pub fn pi() -> BuiltinFunction {
    BuiltinFunction {
        name: "pi".to_string(),
        arity: 0,
        call: |_| Ok(Value::Number(std::f64::consts::PI)),
    }
}

pub fn round() -> BuiltinFunction {
    BuiltinFunction {
        name: "round".to_string(),
        arity: 2,
        call: |args| {
            if let (Value::Number(n), Value::Number(d)) = (&args[0], &args[1]) {
                let factor = 10.0f64.powi(*d as i32);
                Ok(Value::Number((n * factor).round() / factor))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "round ต้องการตัวเลข (ค่า, จำนวนทศนิยม)",
                    None,
                ))
            }
        },
    }
}

pub fn ceil() -> BuiltinFunction {
    BuiltinFunction {
        name: "ceil".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.ceil()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "ceil ต้องการตัวเลข",
                    None,
                ))
            }
        },
    }
}

pub fn floor() -> BuiltinFunction {
    BuiltinFunction {
        name: "floor".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.floor()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "floor ต้องการตัวเลข",
                    None,
                ))
            }
        },
    }
}

pub fn sqrt() -> BuiltinFunction {
    BuiltinFunction {
        name: "sqrt".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.sqrt()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "sqrt ต้องการตัวเลข",
                    None,
                ))
            }
        },
    }
}

pub fn pow() -> BuiltinFunction {
    BuiltinFunction {
        name: "pow".to_string(),
        arity: 2,
        call: |args| {
            if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
                Ok(Value::Number(base.powf(*exp)))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "pow ต้องการตัวเลข (base, exp)",
                    None,
                ))
            }
        },
    }
}

pub fn sin() -> BuiltinFunction {
    BuiltinFunction {
        name: "sin".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.sin()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "sin ต้องการตัวเลข (เรเดียน)",
                    None,
                ))
            }
        },
    }
}

pub fn cos() -> BuiltinFunction {
    BuiltinFunction {
        name: "cos".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.cos()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "cos ต้องการตัวเลข (เรเดียน)",
                    None,
                ))
            }
        },
    }
}

pub fn tan() -> BuiltinFunction {
    BuiltinFunction {
        name: "tan".to_string(),
        arity: 1,
        call: |args| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.tan()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    "tan ต้องการตัวเลข (เรเดียน)",
                    None,
                ))
            }
        },
    }
}

pub fn random() -> BuiltinFunction {
    BuiltinFunction {
        name: "random".to_string(),
        arity: 0,
        call: |_| {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            Ok(Value::Number(rng.gen()))
        },
    }
}

// min, max เดิมถูกลบออก เพราะถูกแทนที่ด้วย array min/max ใน collection.rs
