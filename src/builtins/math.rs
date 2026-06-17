//! ฟังก์ชันทางคณิตศาสตร์

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

pub fn abs() -> BuiltinFunction {
    BuiltinFunction {
        name: "abs".to_string(),
        arity: 1,
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.abs()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("abs ต้องการ Number แต่ได้ {}", args[0].type_name()),
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
        call: |_, _| Ok(Value::Number(std::f64::consts::PI)),
    }
}

pub fn round() -> BuiltinFunction {
    BuiltinFunction {
        name: "round".to_string(),
        arity: 2,
        call: |args, _| {
            if let (Value::Number(n), Value::Number(d)) = (&args[0], &args[1]) {
                let factor = 10.0f64.powi(*d as i32);
                Ok(Value::Number((n * factor).round() / factor))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!(
                        "round ต้องการ Number, Number แต่ได้ {}, {}",
                        args[0].type_name(),
                        args[1].type_name()
                    ),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.ceil()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("ceil ต้องการ Number แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.floor()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("floor ต้องการ Number แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.sqrt()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("sqrt ต้องการ Number แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
                Ok(Value::Number(base.powf(*exp)))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!(
                        "pow ต้องการ Number, Number (base, exp) แต่ได้ {}, {}",
                        args[0].type_name(),
                        args[1].type_name()
                    ),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.sin()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("sin ต้องการ Number (เรเดียน) แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.cos()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("cos ต้องการ Number (เรเดียน) แต่ได้ {}", args[0].type_name()),
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
        call: |args, _| {
            if let Value::Number(n) = args[0] {
                Ok(Value::Number(n.tan()))
            } else {
                Err(FormulaError::new(
                    ErrorKind::FunctionError,
                    "E501",
                    &format!("tan ต้องการ Number (เรเดียน) แต่ได้ {}", args[0].type_name()),
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
        call: |_, _| {
            use rand::RngExt;
            let mut rng = rand::rng();
            Ok(Value::Number(rng.random()))
        },
    }
}

// min, max เดิมถูกลบออก เพราะถูกแทนที่ด้วย array min/max ใน collection.rs
