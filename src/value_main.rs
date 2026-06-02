use crate::ast::SpannedExpr;
use std::rc::Rc;

/// Represents a captured scope snapshot for closures.
/// Stores a copy of variables visible at lambda creation time.
/// Phase 9: Lambda & Higher-Order Functions
pub type CapturedScope = std::collections::BTreeMap<String, Value>;

/// Wrapper for jiff::Span to enable hashing/equality.
/// Phase 11: Advanced Data Types
#[derive(Clone, Debug)]
pub struct Duration(pub jiff::Span);

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl Eq for Duration {}

impl std::hash::Hash for Duration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a value in the bl1z.
#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Map(std::collections::HashMap<String, Value>),
    Lambda(
        Rc<SpannedExpr>,
        Vec<String>,
        CapturedScope,
        std::collections::BTreeMap<String, crate::context::UserFunction>,
    ),
    DateTime(jiff::Timestamp),
    Duration(Duration),
    Set(std::collections::HashSet<Value>),
    Range {
        start: i64,
        end: i64,
        step: i64,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                // Handle NaN correctly (NaN != NaN in standard float)
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => {
                // Map equality by string comparison of each key-value pair
                if a.len() != b.len() {
                    return false;
                }
                for (k, v) in a.iter() {
                    match b.get(k) {
                        Some(other_v) => {
                            if v != other_v {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            }
            (Value::Lambda(_, _, _, _), Value::Lambda(_, _, _, _)) => false, // Lambdas never equal
            (Value::DateTime(a), Value::DateTime(b)) => a == b,
            (Value::Duration(a), Value::Duration(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            (
                Value::Range {
                    start: a_start,
                    end: a_end,
                    step: a_step,
                },
                Value::Range {
                    start: b_start,
                    end: b_end,
                    step: b_step,
                },
            ) => a_start == b_start && a_end == b_end && a_step == b_step,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Number(n) => {
                0u8.hash(state);
                if n.is_nan() {
                    u64::MAX.hash(state);
                } else if n.is_infinite() {
                    (*n as u64).hash(state);
                } else {
                    n.to_bits().hash(state);
                }
            }
            Value::String(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            Value::Bool(b) => {
                2u8.hash(state);
                b.hash(state);
            }
            Value::Null => {
                3u8.hash(state);
            }
            Value::Array(arr) => {
                4u8.hash(state);
                for v in arr {
                    v.hash(state);
                }
            }
            Value::Map(map) => {
                5u8.hash(state);
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                for k in keys {
                    k.hash(state);
                    map.get(k).unwrap().hash(state);
                }
            }
            Value::Lambda(expr, params, _, _) => {
                6u8.hash(state);
                params.hash(state);
                (expr.as_ref() as *const SpannedExpr).hash(state);
            }
            Value::DateTime(dt) => {
                7u8.hash(state);
                dt.to_string().hash(state);
            }
            Value::Duration(d) => {
                8u8.hash(state);
                d.0.to_string().hash(state);
            }
            Value::Set(set) => {
                9u8.hash(state);
                let mut sorted: Vec<&Value> = set.iter().collect();
                sorted.sort_by_key(|v| format!("{:?}", v));
                for v in sorted {
                    v.hash(state);
                }
            }
            Value::Range { start, end, step } => {
                10u8.hash(state);
                start.hash(state);
                end.hash(state);
                step.hash(state);
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(map) => {
                let mut sorted_keys: Vec<&String> = map.keys().collect();
                sorted_keys.sort();
                write!(f, "{{")?;
                for (i, k) in sorted_keys.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, map.get(*k).unwrap())?;
                }
                write!(f, "}}")
            }
            Value::Lambda(_, params, _, _) => {
                write!(f, "({}) => ...", params.join(", "))
            }
            Value::DateTime(dt) => {
                write!(f, "@{}", dt)
            }
            Value::Duration(d) => {
                write!(f, "{}", d.0)
            }
            Value::Set(set) => {
                let mut sorted: Vec<&Value> = set.iter().collect();
                sorted.sort_by_key(|v| format!("{}", v));
                write!(f, "{{")?;
                for (i, v) in sorted.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "}}")
            }
            Value::Range { start, end, step } => {
                if *step == 1 {
                    write!(f, "{}..{}", start, end)
                } else {
                    write!(f, "{}..{}:{}", start, end, step)
                }
            }
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Bool(b) => write!(f, "Bool({})", b),
            Value::Null => write!(f, "Null"),
            Value::Array(arr) => {
                write!(f, "Array([")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", v)?;
                }
                write!(f, "])")
            }
            Value::Map(map) => {
                let mut sorted_keys: Vec<&String> = map.keys().collect();
                sorted_keys.sort();
                write!(f, "Map({{",)?;
                for (i, k) in sorted_keys.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}: {:?}", k, map.get(*k).unwrap())?;
                }
                write!(f, "}})")
            }
            Value::Lambda(_, params, _, _) => {
                write!(f, "Lambda(({}) => ...)", params.join(", "))
            }
            Value::DateTime(dt) => write!(f, "DateTime({})", dt),
            Value::Duration(d) => write!(f, "Duration({})", d.0),
            Value::Set(set) => {
                let mut sorted: Vec<&Value> = set.iter().collect();
                sorted.sort_by_key(|v| format!("{:?}", v));
                write!(f, "Set({{",)?;
                for (i, v) in sorted.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", v)?;
                }
                write!(f, "}})")
            }
            Value::Range { start, end, step } => {
                write!(f, "Range({}..{}:{})", start, end, step)
            }
        }
    }
}
