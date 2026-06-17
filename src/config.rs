/// ตั้งค่าข้อจำกัดของ engine (Phase 15)
///
/// `EngineConfig` ใช้ควบคุมข้อจำกัดด้านความปลอดภัย เช่น
/// ความยาวสูตรสูงสุด, ความลึก recursion สูงสุด, และ timeout
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// ความยาวสูตรสูงสุดที่ยอมรับ (จำนวนตัวอักษร)
    /// ถ้าสูตรยาวเกินนี้ จะคืน Err ก่อนเข้า tokenizing
    /// ค่าเริ่มต้น: 10,000
    pub max_formula_length: usize,

    /// ความลึก recursion สูงสุด
    /// ถ้า eval ซ้อนลึกเกินนี้ จะคืน Err
    /// ค่าเริ่มต้น: 100
    pub max_depth: usize,

    /// จำกัดเวลา eval เป็นมิลลิวินาที
    /// ถ้าเป็น None ไม่มีการจำกัดเวลา
    /// ค่าเริ่มต้น: None
    pub max_time_ms: Option<u64>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_formula_length: 10_000,
            max_depth: 100,
            max_time_ms: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.max_formula_length, 10_000);
        assert_eq!(config.max_depth, 100);
        assert_eq!(config.max_time_ms, None);
    }

    #[test]
    fn test_custom_config() {
        let config = EngineConfig {
            max_formula_length: 500,
            max_depth: 50,
            max_time_ms: Some(100),
        };
        assert_eq!(config.max_formula_length, 500);
        assert_eq!(config.max_depth, 50);
        assert_eq!(config.max_time_ms, Some(100));
    }

    #[test]
    fn test_config_clone() {
        let config = EngineConfig {
            max_formula_length: 2000,
            max_depth: 75,
            max_time_ms: Some(500),
        };
        let cloned = config.clone();
        assert_eq!(config.max_formula_length, cloned.max_formula_length);
        assert_eq!(config.max_depth, cloned.max_depth);
        assert_eq!(config.max_time_ms, cloned.max_time_ms);
    }
}
