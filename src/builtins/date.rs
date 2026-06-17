//! ฟังก์ชันเกี่ยวกับวันที่และเวลา (Phase 6.3 + Phase 11.3/11.4)

use crate::error::{ErrorKind, FormulaError};
use crate::functions::BuiltinFunction;
use crate::value::Value;
use std::str::FromStr;

// ── Helper: แปลง Value เป็น jiff::Timestamp ────────────────────────────────

/// แปลงค่าที่อาจเป็น Value::DateTime หรือ Value::String เป็น jiff::Timestamp
/// - Value::DateTime(ts) → คืน ts โดยตรง
/// - Value::String(s) → พยายาม parse เป็น Timestamp หรือ civil::Date → Timestamp
/// - อื่นๆ → คืน Error
fn require_datetime_or_string(value: &Value) -> Result<jiff::Timestamp, FormulaError> {
    match value {
        Value::DateTime(ts) => Ok(*ts),
        Value::String(s) => parse_to_timestamp(s),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E501",
            &format!(
                "ต้องการ DateTime หรือ String รูปแบบ ISO 8601 (เช่น '2024-01-15T10:30:00Z') แต่ได้ {}",
                type_name(value)
            ),
            None,
        )),
    }
}

/// แปลง String เป็น jiff::Timestamp — ลอง Timestamp ก่อน แล้ว civil::Date
fn parse_to_timestamp(s: &str) -> Result<jiff::Timestamp, FormulaError> {
    jiff::Timestamp::from_str(s)
        .or_else(|_| {
            jiff::civil::Date::from_str(s)
                .and_then(|d| d.to_zoned(jiff::tz::TimeZone::UTC).map(|z| z.timestamp()))
        })
        .map_err(|_| {
            FormulaError::new(
                ErrorKind::FunctionError,
                "E301",
                &format!(
                    "รูปแบบวันที่ไม่ถูกต้อง — ต้องใช้ ISO 8601 (เช่น '2024-01-15' หรือ '2024-01-15T10:30:00Z') แต่ได้ '{}'",
                    s
                ),
                None,
            )
        })
}

/// คืนชื่อชนิด Value สำหรับข้อความ error
fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Bool(_) => "Bool",
        Value::Null => "Null",
        Value::Array(_) => "Array",
        Value::Map(_) => "Map",
        Value::Lambda(_, _, _, _) => "Lambda",
        Value::DateTime(_) => "DateTime",
        Value::Duration(_) => "Duration",
        Value::Set(_) => "Set",
        Value::Range { .. } => "Range",
    }
}

fn require_number(value: &Value) -> Result<f64, FormulaError> {
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E501",
            &format!("ต้องการตัวเลข (Number) แต่ได้ {}", type_name(value)),
            None,
        )),
    }
}

fn require_string(value: &Value) -> Result<String, FormulaError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => Err(FormulaError::new(
            ErrorKind::TypeError,
            "E501",
            &format!("ต้องการข้อความ (String) แต่ได้ {}", type_name(value)),
            None,
        )),
    }
}

// ── ฟังก์ชันหลัก ─────────────────────────────────────────────────────────

/// now() → DateTime
/// คืนวันที่และเวลาปัจจุบันเป็น Value::DateTime
pub fn now() -> BuiltinFunction {
    BuiltinFunction {
        name: "now".to_string(),
        arity: 0,
        call: |_, _| {
            let now = jiff::Timestamp::now();
            Ok(Value::DateTime(now))
        },
    }
}

/// date(year, month, day) → DateTime
/// สร้างวันที่และคืนเป็น Value::DateTime (เวลา 00:00:00 UTC)
pub fn date() -> BuiltinFunction {
    BuiltinFunction {
        name: "date".to_string(),
        arity: 3,
        call: |args, _| {
            let year = require_number(&args[0])? as i16;
            let month = require_number(&args[1])? as i8;
            let day = require_number(&args[2])? as i8;

            let d = jiff::civil::Date::new(year, month, day).map_err(|_| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E301",
                    &format!(
                        "ไม่สามารถสร้างวันที่จาก year={}, month={}, day={} ได้ — ค่าอยู่นอกช่วงที่ยอมรับ",
                        year, month, day
                    ),
                    None,
                )
            })?;

            // แปลง civil::Date → Timestamp (เวลา 00:00:00 UTC)
            let ts = d
                .to_zoned(jiff::tz::TimeZone::UTC)
                .map_err(|_| {
                    FormulaError::new(
                        ErrorKind::FunctionError,
                        "E301",
                        &format!("ไม่สามารถแปลงวันที่ {} เป็น timestamp ได้", d),
                        None,
                    )
                })?
                .timestamp();
            Ok(Value::DateTime(ts))
        },
    }
}

/// year(dt) → Number
/// คืนปีจาก DateTime หรือ String
pub fn year() -> BuiltinFunction {
    BuiltinFunction {
        name: "year".to_string(),
        arity: 1,
        call: |args, _| {
            let ts = require_datetime_or_string(&args[0])?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.year() as f64))
        },
    }
}

/// month(dt) → Number
/// คืนเดือนจาก DateTime หรือ String (1-12)
pub fn month() -> BuiltinFunction {
    BuiltinFunction {
        name: "month".to_string(),
        arity: 1,
        call: |args, _| {
            let ts = require_datetime_or_string(&args[0])?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.month() as f64))
        },
    }
}

/// day(dt) → Number
/// คืนวันที่จาก DateTime หรือ String (1-31)
pub fn day() -> BuiltinFunction {
    BuiltinFunction {
        name: "day".to_string(),
        arity: 1,
        call: |args, _| {
            let ts = require_datetime_or_string(&args[0])?;
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            Ok(Value::Number(zdt.day() as f64))
        },
    }
}

/// date_add(dt, days) → DateTime
/// เพิ่มจำนวนวันให้กับ DateTime/String แล้วคืนเป็น Value::DateTime
pub fn date_add() -> BuiltinFunction {
    BuiltinFunction {
        name: "date_add".to_string(),
        arity: 2,
        call: |args, _| {
            let ts = require_datetime_or_string(&args[0])?;
            let days = require_number(&args[1])?;

            let days_i64 = days as i64;
            let span = jiff::Span::new().days(days_i64);

            // Timestamp ไม่รองรับ calendar units โดยตรง ต้องแปลงเป็น Zoned ก่อน
            let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
            let new_zdt = zdt.checked_add(span).map_err(|_| {
                FormulaError::new(
                    ErrorKind::FunctionError,
                    "E301",
                    &format!(
                        "ไม่สามารถเพิ่ม {} วันจากวันที่ {} ได้ — การคำนวณล้มเหลว",
                        days_i64, ts
                    ),
                    None,
                )
            })?;

            Ok(Value::DateTime(new_zdt.timestamp()))
        },
    }
}

/// date_diff(dt1, dt2, unit) → Number
/// คืนผลต่างระหว่างสองวันที่ในหน่วยที่ระบุ
/// unit: 'days' (default), 'hours', 'minutes', 'months', 'years'
pub fn date_diff() -> BuiltinFunction {
    BuiltinFunction {
        name: "date_diff".to_string(),
        arity: 3,
        call: |args, _| {
            let ts1 = require_datetime_or_string(&args[0])?;
            let ts2 = require_datetime_or_string(&args[1])?;
            let unit_str = require_string(&args[2])?;

            let span = ts2 - ts1;

            let result = match unit_str.to_lowercase().as_str() {
                "days" | "day" => {
                    let rel = jiff::SpanRelativeTo::days_are_24_hours();
                    span.total((jiff::Unit::Day, rel)).map_err(|_| {
                        FormulaError::new(
                            ErrorKind::FunctionError,
                            "E301",
                            &format!("ไม่สามารถคำนวณผลต่างเป็นจำนวนวันระหว่าง {} และ {} ได้", ts1, ts2),
                            None,
                        )
                    })?
                }
                "hours" | "hour" => span.total(jiff::Unit::Hour).map_err(|_| {
                    FormulaError::new(
                        ErrorKind::FunctionError,
                        "E301",
                        &format!(
                            "ไม่สามารถคำนวณผลต่างเป็นจำนวนชั่วโมงระหว่าง {} และ {} ได้",
                            ts1, ts2
                        ),
                        None,
                    )
                })?,
                "minutes" | "minute" => span.total(jiff::Unit::Minute).map_err(|_| {
                    FormulaError::new(
                        ErrorKind::FunctionError,
                        "E301",
                        &format!("ไม่สามารถคำนวณผลต่างเป็นจำนวนนาทีระหว่าง {} และ {} ได้", ts1, ts2),
                        None,
                    )
                })?,
                "months" | "month" => {
                    let zdt1 = ts1.to_zoned(jiff::tz::TimeZone::UTC);
                    span.total((jiff::Unit::Month, &zdt1)).map_err(|_| {
                        FormulaError::new(
                            ErrorKind::FunctionError,
                            "E301",
                            &format!(
                                "ไม่สามารถคำนวณผลต่างเป็นจำนวนเดือนระหว่าง {} และ {} ได้",
                                ts1, ts2
                            ),
                            None,
                        )
                    })?
                }
                "years" | "year" => {
                    let zdt1 = ts1.to_zoned(jiff::tz::TimeZone::UTC);
                    span.total((jiff::Unit::Year, &zdt1)).map_err(|_| {
                        FormulaError::new(
                            ErrorKind::FunctionError,
                            "E301",
                            &format!("ไม่สามารถคำนวณผลต่างเป็นจำนวนปีระหว่าง {} และ {} ได้", ts1, ts2),
                            None,
                        )
                    })?
                }
                _ => {
                    return Err(FormulaError::new(
                        ErrorKind::FunctionError,
                        "E301",
                        &format!(
                            "หน่วย '{}' ไม่ถูกต้อง — ใช้ได้ 'days', 'hours', 'minutes', 'months', หรือ 'years'",
                            unit_str
                        ),
                        None,
                    ));
                }
            };

            Ok(Value::Number(result))
        },
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn call_fn(f: BuiltinFunction, args: Vec<Value>) -> Result<Value, FormulaError> {
        let registry = crate::functions::FunctionRegistry::new();
        (f.call)(&args, &registry)
    }

    // -- now() --

    #[test]
    fn test_now_returns_datetime() {
        let result = call_fn(now(), vec![]).unwrap();
        match result {
            Value::DateTime(_) => {} // OK
            other => panic!("expected DateTime, got {:?}", other),
        }
    }

    // -- date(y, m, d) --

    #[test]
    fn test_date_returns_datetime() {
        let result = call_fn(
            date(),
            vec![
                Value::Number(2025.0),
                Value::Number(6.0),
                Value::Number(15.0),
            ],
        )
        .unwrap();
        match result {
            Value::DateTime(ts) => {
                let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
                assert_eq!(zdt.year(), 2025);
                assert_eq!(zdt.month() as i32, 6);
                assert_eq!(zdt.day() as i32, 15);
            }
            other => panic!("expected DateTime, got {:?}", other),
        }
    }

    #[test]
    fn test_date_invalid_values() {
        let result = call_fn(
            date(),
            vec![
                Value::Number(2025.0),
                Value::Number(13.0),
                Value::Number(1.0),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("ไม่สามารถสร้างวันที่"));
    }

    // -- year(dt) --

    #[test]
    fn test_year_with_string() {
        let result = call_fn(year(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(2023.0));
    }

    #[test]
    fn test_year_with_datetime() {
        let ts = jiff::Timestamp::from_str("2024-12-25T15:30:00Z").unwrap();
        let result = call_fn(year(), vec![Value::DateTime(ts)]).unwrap();
        assert_eq!(result, Value::Number(2024.0));
    }

    #[test]
    fn test_year_type_error() {
        let result = call_fn(year(), vec![Value::Number(2023.0)]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("ต้องการ DateTime หรือ String"));
        assert!(err.message.contains("Number"));
    }

    // -- month(dt) --

    #[test]
    fn test_month_with_string() {
        let result = call_fn(month(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_month_with_datetime() {
        let ts = jiff::Timestamp::from_str("2024-12-25T15:30:00Z").unwrap();
        let result = call_fn(month(), vec![Value::DateTime(ts)]).unwrap();
        assert_eq!(result, Value::Number(12.0));
    }

    // -- day(dt) --

    #[test]
    fn test_day_with_string() {
        let result = call_fn(day(), vec![Value::String("2023-05-15".to_string())]).unwrap();
        assert_eq!(result, Value::Number(15.0));
    }

    #[test]
    fn test_day_with_datetime() {
        let ts = jiff::Timestamp::from_str("2024-12-25T15:30:00Z").unwrap();
        let result = call_fn(day(), vec![Value::DateTime(ts)]).unwrap();
        assert_eq!(result, Value::Number(25.0));
    }

    // -- date_add(dt, days) --

    #[test]
    fn test_date_add_with_string() {
        let result = call_fn(
            date_add(),
            vec![Value::String("2023-01-01".to_string()), Value::Number(10.0)],
        )
        .unwrap();
        match result {
            Value::DateTime(ts) => {
                let zdt = ts.to_zoned(jiff::tz::TimeZone::UTC);
                assert_eq!(zdt.year(), 2023);
                assert_eq!(zdt.month() as i32, 1);
                assert_eq!(zdt.day() as i32, 11);
            }
            other => panic!("expected DateTime, got {:?}", other),
        }
    }

    #[test]
    fn test_date_add_with_datetime() {
        let ts = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = call_fn(date_add(), vec![Value::DateTime(ts), Value::Number(30.0)]).unwrap();
        match result {
            Value::DateTime(ts2) => {
                let zdt = ts2.to_zoned(jiff::tz::TimeZone::UTC);
                assert_eq!(zdt.year(), 2023);
                assert_eq!(zdt.month() as i32, 1);
                assert_eq!(zdt.day() as i32, 31);
            }
            other => panic!("expected DateTime, got {:?}", other),
        }
    }

    #[test]
    fn test_date_add_type_error() {
        let result = call_fn(date_add(), vec![Value::Number(123.0), Value::Number(5.0)]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("ต้องการ DateTime หรือ String"));
    }

    // -- date_diff(dt1, dt2, unit) --

    #[test]
    fn test_date_diff_days_string() {
        let result = call_fn(
            date_diff(),
            vec![
                Value::String("2023-01-01".to_string()),
                Value::String("2023-01-05".to_string()),
                Value::String("days".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn test_date_diff_days_datetime() {
        let ts1 = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let ts2 = jiff::Timestamp::from_str("2023-01-05T00:00:00Z").unwrap();
        let result = call_fn(
            date_diff(),
            vec![
                Value::DateTime(ts1),
                Value::DateTime(ts2),
                Value::String("days".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn test_date_diff_hours() {
        let ts1 = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let ts2 = jiff::Timestamp::from_str("2023-01-01T06:30:00Z").unwrap();
        let result = call_fn(
            date_diff(),
            vec![
                Value::DateTime(ts1),
                Value::DateTime(ts2),
                Value::String("hours".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(6.5));
    }

    #[test]
    fn test_date_diff_minutes() {
        let ts1 = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let ts2 = jiff::Timestamp::from_str("2023-01-01T01:30:00Z").unwrap();
        let result = call_fn(
            date_diff(),
            vec![
                Value::DateTime(ts1),
                Value::DateTime(ts2),
                Value::String("minutes".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(90.0));
    }

    #[test]
    fn test_date_diff_months() {
        let ts1 = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let ts2 = jiff::Timestamp::from_str("2023-06-15T00:00:00Z").unwrap();
        let result = call_fn(
            date_diff(),
            vec![
                Value::DateTime(ts1),
                Value::DateTime(ts2),
                Value::String("months".to_string()),
            ],
        )
        .unwrap();
        // ~5.5 months
        match result {
            Value::Number(n) => assert!((n - 5.47).abs() < 0.1),
            other => panic!("expected Number, got {:?}", other),
        }
    }

    #[test]
    fn test_date_diff_years() {
        let ts1 = jiff::Timestamp::from_str("2023-01-01T00:00:00Z").unwrap();
        let ts2 = jiff::Timestamp::from_str("2025-01-01T00:00:00Z").unwrap();
        let result = call_fn(
            date_diff(),
            vec![
                Value::DateTime(ts1),
                Value::DateTime(ts2),
                Value::String("years".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_date_diff_invalid_unit() {
        let result = call_fn(
            date_diff(),
            vec![
                Value::String("2023-01-01".to_string()),
                Value::String("2023-01-05".to_string()),
                Value::String("decades".to_string()),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("ไม่ถูกต้อง"));
        assert!(err.message.contains("decades"));
    }

    #[test]
    fn test_date_diff_type_error() {
        let result = call_fn(
            date_diff(),
            vec![
                Value::Number(1.0),
                Value::String("2023-01-05".to_string()),
                Value::String("days".to_string()),
            ],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("ต้องการ DateTime หรือ String"));
    }

    // -- parse_to_timestamp error messages --

    #[test]
    fn test_parse_error_message() {
        let result = call_fn(year(), vec![Value::String("not-a-date".to_string())]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("รูปแบบวันที่ไม่ถูกต้อง"));
        assert!(err.message.contains("not-a-date"));
    }
}
