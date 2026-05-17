// src/builtins/logic.rs
use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;

pub fn if_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "if".to_string(),
        arity: 3,
        call: |args| {
            let cond = &args[0];
            let is_true = match cond {
                Value::Bool(b) => *b,
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::FunctionError,
                        "E501",
                        "if เงื่อนไขต้องเป็น boolean",
                        None,
                    ))
                }
            };
            if is_true {
                Ok(args[1].clone())
            } else {
                Ok(args[2].clone())
            }
        },
    }
}
