//! ฟังก์ชันเกี่ยวกับวันที่และเวลา (Phase 6.3)

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;
use std::str::FromStr;

/// now() -> String
/// คืนวันที่และเวลาปัจจุบันในรูปแบบ ISO 8601
pub fn now() -> BuiltinFunction {
    BuiltinFunction {
        name: "now".to_string(),
        arity: 0,
        call: |_| {
            let now = jiff::Timestamp::now();
            Ok(Value::String(now.to_string()))
        },
    }
}

/// date_add(date_str, days) -> String
/// เพิ่มจำนวนวันให้กับวันที่
pub fn date_add() -> BuiltinFunction {
    BuiltinFunction {
        name: "date_add".to_string(),
        arity: 2,
        call: |args| {
            let date_str = require_string(&args[0])?;
            let days = require_number(&args[1])?;

            // Parse as Date (not Timestamp, since we need calendar days)
            let date = jiff::civil::Date::from_str(&date_str).map_err(|_| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E010",
                    "Invalid date format",
                    None,
                )
            })?;

            let days_i64 = days as i64;
            let span = jiff::Span::new().days(days_i64);

            let new_date = date.checked_add(span).map_err(|_| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E010",
                    "Date calculation error",
                    None,
                )
            })?;

            Ok(Value::String(new_date.to_string()))
        },
    }
}

/// date(year, month, day) -> String
/// สร้างวันที่และคืนเป็น string "YYYY-MM-DD"
pub fn date() -> BuiltinFunction {
    BuiltinFunction {
        name: "date".to_string(),
        arity: 3,
        call: |args| {
            let year = require_number(&args[0])? as i16;
            let month = require_number(&args[1])? as i8;
            let day = require_number(&args[2])? as i8;

            let d = jiff::civil::Date::new(year, month, day).map_err(|_| {
                FormulaError::new(ErrorKind::FunctionError, "E010", "Invalid date", None)
            })?;
            Ok(Value::String(d.to_string()))
        },
    }
}

/// year(date_str) -> Number
/// คืนปีจากวันที่/เวลา string
pub fn year() -> BuiltinFunction {
    BuiltinFunction {
        name: "year".to_string(),
        arity: 1,
        call: |args| {
            let date_str = require_string(&args[0])?;
            let ts = parse_to_timestamp(&date_str)?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.year() as f64))
        },
    }
}

/// month(date_str) -> Number
/// คืนเดือนจากวันที่/เวลา string (1-12)
pub fn month() -> BuiltinFunction {
    BuiltinFunction {
        name: "month".to_string(),
        arity: 1,
        call: |args| {
            let date_str = require_string(&args[0])?;
            let ts = parse_to_timestamp(&date_str)?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.month() as f64))
        },
    }
}

/// day(date_str) -> Number
/// คืนวันที่จากวันที่/เวลา string (1-31)
pub fn day() -> BuiltinFunction {
    BuiltinFunction {
        name: "day".to_string(),
        arity: 1,
        call: |args| {
            let date_str = require_string(&args[0])?;
            let ts = parse_to_timestamp(&date_str)?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.day() as f64))
        },
    }
}

/// date_diff(date_str1, date_str2, unit) -> Number
/// คืนจำนวนวันระหว่างสองวันที่ (หน่วยวัน, ไม่สนใจ unit)
pub fn date_diff() -> BuiltinFunction {
    BuiltinFunction {
        name: "date_diff".to_string(),
        arity: 3,
        call: |args| {
            let date_str1 = require_string(&args[0])?;
            let date_str2 = require_string(&args[1])?;
            // Ignore unit for now

            let t1 = parse_to_timestamp(&date_str1)?;
            let t2 = parse_to_timestamp(&date_str2)?;
            let span = t2 - t1; // jiff::Span
            let days = span.total(jiff::Unit::Day).map_err(|_| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E010",
                    "Failed to calculate date difference",
                    None,
                )
            })?;
            Ok(Value::Number(days))
        },
    }
}

// -- Helpers --

fn require_string(value: &Value) -> Result<String, FormulaError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => Err(FormulaError::new(
            ErrorKind::FunctionError,
            "E006",
            "ต้องการข้อความ",
            None,
        )),
    }
}

fn require_number(value: &Value) -> Result<f64, FormulaError> {
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(FormulaError::new(
            ErrorKind::FunctionError,
            "E006",
            "ต้องการตัวเลข",
            None,
        )),
    }
}

fn parse_to_timestamp(s: &str) -> Result<jiff::Timestamp, FormulaError> {
    jiff::Timestamp::from_str(s)
        .or_else(|_| {
            jiff::civil::Date::from_str(s)
                .and_then(|d| d.to_zoned(jiff::tz::TimeZone::UTC).map(|z| z.timestamp()))
        })
        .map_err(|_| {
            FormulaError::new(
                ErrorKind::FunctionError,
                "E010",
                "Invalid date/time string",
                None,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_fn(f: BuiltinFunction, args: Vec<Value>) -> Result<Value, FormulaError> {
        (f.call)(&args)
    }

    #[test]
    fn test_now() {
        let result = call_fn(now(), vec![]).unwrap();
        match result {
            Value::String(s) => {
                // Basic check that it returns a string and looks like a timestamp
                assert!(s.len() > 0);
                assert!(s.contains('T'));
            }
            _ => panic!("expected string"),
        }
    }

    #[test]
    fn test_date() {
        let result = call_fn(
            date(),
            vec![
                Value::Number(2025.0),
                Value::Number(6.0),
                Value::Number(15.0),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::String("2025-06-15".to_string()));
    }

    #[test]
    fn test_year() {
        let result = call_fn(year(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(2023.0));
    }

    #[test]
    fn test_month() {
        let result = call_fn(month(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_day() {
        let result = call_fn(day(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(15.0));
    }

    #[test]
    fn test_date_diff() {
        let result = call_fn(
            date_diff(),
            vec![
                Value::String("2023-01-01".to_string()),
                Value::String("2023-01-05".to_string()),
                Value::String("days".to_string()), // ignored
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));
    }
}
