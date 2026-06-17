/// เก็บตำแหน่งในข้อความสูตร (บรรทัด, คอลัมน์)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Position {
    pub line: usize,   // หมายเลขบรรทัด (เริ่มที่ 1)
    pub column: usize, // หมายเลขคอลัมน์ (เริ่มที่ 1)
}

/// ช่วงตำแหน่งตั้งแต่ start ถึง end
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// สร้าง span จากตำแหน่งเริ่มและจบ
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
