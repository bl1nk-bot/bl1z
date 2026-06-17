use crate::ast::SpannedExpr;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

/// แทนค่าขอบเขตที่ถูกจับ (Captured Scope) สำหรับ Closures
/// เก็บสำเนาของตัวแปรที่มองเห็นได้ ณ เวลาที่สร้าง Lambda
pub type CapturedScope = BTreeMap<String, Value>;

/// Wrapper สำหรับ jiff::Span เพื่อให้รองรับ hashing และ equality
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

/// แทนค่าชนิดข้อมูลต่างๆ ในระบบ bl1z
///
/// # ตัวอย่างการใช้งาน
/// ```
/// use bl1z::Value;
/// let val = Value::Number(42.0);
/// assert_eq!(val.to_string(), "42");
/// ```
#[derive(Clone)]
pub enum Value {
    /// ตัวเลขทศนิยม 64-bit
    Number(f64),
    /// ข้อความ UTF-8
    String(String),
    /// ค่าทางตรรกศาสตร์ (true/false)
    Bool(bool),
    /// ค่าว่าง
    Null,
    /// รายการข้อมูล (Array)
    Array(Vec<Value>),
    /// ข้อมูลแบบ Key-Value (Map)
    Map(HashMap<String, Value>),
    /// ฟังก์ชัน Lambda หรือ Closure
    /// เก็บ (Body, Params, Captured Variables, Context Functions)
    Lambda(
        Rc<SpannedExpr>,
        Vec<String>,
        CapturedScope,
        BTreeMap<String, crate::context::UserFunction>,
    ),
    /// วันที่และเวลา (Native Timestamp via jiff)
    DateTime(jiff::Timestamp),
    /// ช่วงเวลา (Duration via jiff)
    Duration(Duration),
    /// เซตของข้อมูลที่ไม่มีค่าซ้ำ (Set)
    Set(HashSet<Value>),
    /// ช่วงของตัวเลข (Range)
    Range { start: i64, end: i64, step: i64 },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
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
            (Value::Lambda(_, _, _, _), Value::Lambda(_, _, _, _)) => false,
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

impl Value {
    /// คืนชื่อชนิดข้อมูลของค่า (type name) สำหรับใช้ในข้อความ error
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Bool(_) => "Bool",
            Value::Null => "Null",
            Value::Array(_) => "Array",
            Value::Map(_) => "Map",
            Value::Lambda(..) => "Lambda",
            Value::DateTime(_) => "DateTime",
            Value::Duration(_) => "Duration",
            Value::Set(_) => "Set",
            Value::Range { .. } => "Range",
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

// ── Serialization (Phase 12) ─────────────────────────────────────────────
#[cfg(feature = "serialization")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Value {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap;
            match self {
                Value::Number(n) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "number")?;
                    map.serialize_entry("value", n)?;
                    map.end()
                }
                Value::String(s) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "string")?;
                    map.serialize_entry("value", s)?;
                    map.end()
                }
                Value::Bool(b) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "bool")?;
                    map.serialize_entry("value", b)?;
                    map.end()
                }
                Value::Null => {
                    let mut map = serializer.serialize_map(Some(1))?;
                    map.serialize_entry("type", "null")?;
                    map.end()
                }
                Value::Array(arr) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "array")?;
                    map.serialize_entry("value", arr)?;
                    map.end()
                }
                Value::Map(m) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "map")?;
                    map.serialize_entry("value", m)?;
                    map.end()
                }
                Value::Lambda(..) => Err(serde::ser::Error::custom(
                    "cannot serialize Lambda (runtime closure)",
                )),
                Value::DateTime(dt) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "datetime")?;
                    map.serialize_entry("value", &dt.to_string())?;
                    map.end()
                }
                Value::Duration(d) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "duration")?;
                    map.serialize_entry("value", &d.0.to_string())?;
                    map.end()
                }
                Value::Set(set) => {
                    let mut sorted: Vec<&Value> = set.iter().collect();
                    sorted.sort_by_key(|v| format!("{:?}", v));
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", "set")?;
                    map.serialize_entry("value", &sorted)?;
                    map.end()
                }
                Value::Range { start, end, step } => {
                    let mut map = serializer.serialize_map(Some(4))?;
                    map.serialize_entry("type", "range")?;
                    map.serialize_entry("start", start)?;
                    map.serialize_entry("end", end)?;
                    map.serialize_entry("step", step)?;
                    map.end()
                }
            }
        }
    }

    impl<'de> Deserialize<'de> for Value {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let map = serde_json::Value::deserialize(deserializer)?;
            let obj = map
                .as_object()
                .ok_or_else(|| serde::de::Error::custom("expected a JSON object"))?;
            let type_str = obj
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| serde::de::Error::custom("missing \"type\" field"))?;
            match type_str {
                "number" => {
                    let val = obj
                        .get("value")
                        .and_then(|v| v.as_f64())
                        .ok_or_else(|| serde::de::Error::custom("invalid number value"))?;
                    Ok(Value::Number(val))
                }
                "string" => {
                    let val = obj
                        .get("value")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| serde::de::Error::custom("invalid string value"))?;
                    Ok(Value::String(val.to_string()))
                }
                "bool" => {
                    let val = obj
                        .get("value")
                        .and_then(|v| v.as_bool())
                        .ok_or_else(|| serde::de::Error::custom("invalid bool value"))?;
                    Ok(Value::Bool(val))
                }
                "null" => Ok(Value::Null),
                "array" => {
                    let val: Vec<Value> = serde_json::from_value(
                        obj.get("value")
                            .cloned()
                            .ok_or_else(|| serde::de::Error::custom("missing array value"))?,
                    )
                    .map_err(serde::de::Error::custom)?;
                    Ok(Value::Array(val))
                }
                "map" => {
                    let val: HashMap<String, Value> = serde_json::from_value(
                        obj.get("value")
                            .cloned()
                            .ok_or_else(|| serde::de::Error::custom("missing map value"))?,
                    )
                    .map_err(serde::de::Error::custom)?;
                    Ok(Value::Map(val))
                }
                "datetime" => {
                    let s = obj
                        .get("value")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| serde::de::Error::custom("invalid datetime value"))?;
                    let ts: jiff::Timestamp = s
                        .parse()
                        .map_err(|e| serde::de::Error::custom(format!("invalid timestamp: {e}")))?;
                    Ok(Value::DateTime(ts))
                }
                "duration" => {
                    let s = obj
                        .get("value")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| serde::de::Error::custom("invalid duration value"))?;
                    let span: jiff::Span = s
                        .parse()
                        .map_err(|e| serde::de::Error::custom(format!("invalid duration: {e}")))?;
                    Ok(Value::Duration(Duration(span)))
                }
                "set" => {
                    let vals: Vec<Value> = serde_json::from_value(
                        obj.get("value")
                            .cloned()
                            .ok_or_else(|| serde::de::Error::custom("missing set value"))?,
                    )
                    .map_err(serde::de::Error::custom)?;
                    Ok(Value::Set(vals.into_iter().collect()))
                }
                "range" => {
                    let start = obj
                        .get("start")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| serde::de::Error::custom("invalid range start"))?;
                    let end = obj
                        .get("end")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| serde::de::Error::custom("invalid range end"))?;
                    let step = obj
                        .get("step")
                        .and_then(|v| v.as_i64())
                        .ok_or_else(|| serde::de::Error::custom("invalid range step"))?;
                    Ok(Value::Range { start, end, step })
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown Value type: {other}"
                ))),
            }
        }
    }
}
