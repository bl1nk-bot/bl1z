use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;
use std::collections::HashMap;

/// map(array, lambda) -> Array
pub fn map() -> BuiltinFunction {
    BuiltinFunction {
        name: "map".to_string(),
        arity: 2,
        call: |args, reg| {
            let arr = require_array(&args[0])?;
            let mut results = Vec::new();
            for val in arr {
                results.push(crate::eval::apply_lambda(
                    &args[1],
                    std::slice::from_ref(val),
                    reg,
                )?);
            }
            Ok(Value::Array(results))
        },
    }
}

/// filter(array, lambda) -> Array
pub fn filter() -> BuiltinFunction {
    BuiltinFunction {
        name: "filter".to_string(),
        arity: 2,
        call: |args, reg| {
            let arr = require_array(&args[0])?;
            let mut results = Vec::new();
            for val in arr {
                let res = crate::eval::apply_lambda(&args[1], std::slice::from_ref(val), reg)?;
                if is_truthy(&res) {
                    results.push(val.clone());
                }
            }
            Ok(Value::Array(results))
        },
    }
}

/// reduce(array, lambda, initial) -> Value
/// หมายเหตุ: อ้างอิงลำดับตามเทสใน lib_tests.rs
pub fn reduce() -> BuiltinFunction {
    BuiltinFunction {
        name: "reduce".to_string(),
        arity: 3,
        call: |args, reg| {
            let arr = require_array(&args[0])?;
            let lambda = &args[1];
            let initial = &args[2];

            let mut acc = initial.clone();
            for val in arr {
                // ส่ง acc และ val เข้า lambda (arity 2)
                acc = crate::eval::apply_lambda(lambda, &[acc, val.clone()], reg)?;
            }
            Ok(acc)
        },
    }
}

/// sort(array, \[key_lambda\]) -> Array
pub fn sort() -> BuiltinFunction {
    BuiltinFunction {
        name: "sort".to_string(),
        arity: 999,
        call: |args, reg| {
            if args.is_empty() {
                return Ok(Value::Array(Vec::new()));
            }
            let mut arr = require_array(&args[0])?.clone();
            if args.len() == 1 {
                arr.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
            } else {
                let mut mapped: Vec<(String, Value)> = Vec::new();
                for val in &arr {
                    let key = crate::eval::apply_lambda(&args[1], std::slice::from_ref(val), reg)?;
                    mapped.push((format!("{}", key), val.clone()));
                }
                mapped.sort_by(|a, b| a.0.cmp(&b.0));
                arr = mapped.into_iter().map(|(_, v)| v).collect();
            }
            Ok(Value::Array(arr))
        },
    }
}

/// sort_with(array, comparator_lambda) -> Array
pub fn sort_with() -> BuiltinFunction {
    BuiltinFunction {
        name: "sort_with".to_string(),
        arity: 2,
        call: |args, reg| {
            let mut arr = require_array(&args[0])?.clone();
            let lambda = &args[1];

            let mut sort_error = None;
            arr.sort_by(|a, b| {
                match crate::eval::apply_lambda(lambda, &[a.clone(), b.clone()], reg) {
                    Ok(Value::Number(n)) => {
                        if n < 0.0 {
                            std::cmp::Ordering::Less
                        } else if n > 0.0 {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    Ok(_) => std::cmp::Ordering::Equal,
                    Err(e) => {
                        sort_error = Some(e);
                        std::cmp::Ordering::Equal
                    }
                }
            });

            if let Some(e) = sort_error {
                return Err(e);
            }
            Ok(Value::Array(arr))
        },
    }
}

/// unique(array, \[key_lambda\]) -> Array
pub fn unique() -> BuiltinFunction {
    BuiltinFunction {
        name: "unique".to_string(),
        arity: 999,
        call: |args, reg| {
            if args.is_empty() {
                return Ok(Value::Array(Vec::new()));
            }
            let arr = require_array(&args[0])?;
            let mut seen = std::collections::HashSet::new();
            let mut results = Vec::new();
            for val in arr {
                let key = if args.len() > 1 {
                    crate::eval::apply_lambda(&args[1], std::slice::from_ref(val), reg)?
                } else {
                    val.clone()
                };
                if seen.insert(key) {
                    results.push(val.clone());
                }
            }
            Ok(Value::Array(results))
        },
    }
}

/// group_by(array, key_lambda) -> Map
pub fn group_by() -> BuiltinFunction {
    BuiltinFunction {
        name: "group_by".to_string(),
        arity: 2,
        call: |args, reg| {
            let arr = require_array(&args[0])?;
            let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
            for val in arr {
                let key = crate::eval::apply_lambda(&args[1], std::slice::from_ref(val), reg)?;
                let key_str = format!("{}", key);
                groups.entry(key_str).or_default().push(val.clone());
            }
            let mut result_map = HashMap::new();
            for (k, v) in groups {
                result_map.insert(k, Value::Array(v));
            }
            Ok(Value::Map(result_map))
        },
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Map(m) => !m.is_empty(),
        _ => true,
    }
}

fn require_array(val: &Value) -> Result<&Vec<Value>, FormulaError> {
    match val {
        Value::Array(a) => Ok(a),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            &format!("ต้องการ Array แต่ได้ {}", val.type_name()),
            None,
        )),
    }
}

// Full compatibility for mod.rs and tests
pub fn map_fn() -> BuiltinFunction {
    map()
}
pub fn filter_fn() -> BuiltinFunction {
    filter()
}
pub fn reduce_fn() -> BuiltinFunction {
    reduce()
}
pub fn sort_fn() -> BuiltinFunction {
    let mut f = sort();
    f.arity = 999;
    f
}
pub fn sort_with_fn() -> BuiltinFunction {
    let mut f = sort_with();
    f.arity = 2;
    f
}
pub fn unique_fn() -> BuiltinFunction {
    let mut f = unique();
    f.arity = 999;
    f
}
pub fn group_by_fn() -> BuiltinFunction {
    group_by()
}
pub fn set_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "set".to_string(),
        arity: 1,
        call: |args, _| {
            Ok(Value::Set(match &args[0] {
                Value::Array(a) => a.iter().cloned().collect(),
                _ => std::collections::HashSet::new(),
            }))
        },
    }
}
pub fn range_fn() -> BuiltinFunction {
    BuiltinFunction {
        name: "range".to_string(),
        arity: 999,
        call: |args, _| {
            match args.len() {
                1 => {
                    // range(end) → 0..end:1
                    let end = to_i64(&args[0], "end")?;
                    Ok(Value::Range {
                        start: 0,
                        end,
                        step: 1,
                    })
                }
                2 => {
                    // range(start, end) → start..end:1
                    let start = to_i64(&args[0], "start")?;
                    let end = to_i64(&args[1], "end")?;
                    Ok(Value::Range {
                        start,
                        end,
                        step: 1,
                    })
                }
                3 => {
                    // range(start, end, step) → start..end:step
                    let start = to_i64(&args[0], "start")?;
                    let end = to_i64(&args[1], "end")?;
                    let step = to_i64(&args[2], "step")?;
                    if step == 0 {
                        return Err(FormulaError::new(
                            ErrorKind::TypeError,
                            "E401",
                            "step ต้องไม่เท่ากับ 0",
                            None,
                        ));
                    }
                    Ok(Value::Range { start, end, step })
                }
                _ => Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    &format!("range ต้องการ 1-3 อาร์กิวเมนต์ แต่ได้ {}", args.len()),
                    None,
                )),
            }
        },
    }
}

fn to_i64(val: &Value, param_name: &str) -> Result<i64, FormulaError> {
    match val {
        Value::Number(n) => {
            let i = *n as i64;
            if (*n - i as f64).abs() > f64::EPSILON {
                return Err(FormulaError::new(
                    ErrorKind::TypeError,
                    "E401",
                    &format!("{} ต้องเป็นจำนวนเต็ม", param_name),
                    None,
                ));
            }
            Ok(i)
        }
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E401",
            &format!("ต้องการ Number แต่ได้ {}", val.type_name()),
            None,
        )),
    }
}

/// range_to_array(range) → Value::Array (convert range to array, e.g. 1..3 → \[1,2\])
pub fn range_to_array() -> BuiltinFunction {
    BuiltinFunction {
        name: "range_to_array".to_string(),
        arity: 1,
        call: |args, _| match &args[0] {
            Value::Range { start, end, step } => {
                let mut arr = Vec::new();
                if *step > 0 {
                    let mut i = *start;
                    while i < *end {
                        arr.push(Value::Number(i as f64));
                        i += step;
                    }
                } else {
                    let mut i = *start;
                    while i > *end {
                        arr.push(Value::Number(i as f64));
                        i += step;
                    }
                }
                Ok(Value::Array(arr))
            }
            _ => Err(FormulaError::new(
                ErrorKind::TypeError,
                "E401",
                &format!("ต้องการ Range แต่ได้ {}", args[0].type_name()),
                None,
            )),
        },
    }
}
