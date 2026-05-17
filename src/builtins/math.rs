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

// min, max เดิมถูกลบออก เพราะถูกแทนที่ด้วย array min/max ใน collection.rs
