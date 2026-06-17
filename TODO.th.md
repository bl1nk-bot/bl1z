# 📋 รายการงานที่ต้องทำ (Active TODO Checklist) — bl1z V2

## 🟢 Phase 10.5: ฟังก์ชันคณิตศาสตร์และข้อความที่ขาดหายไป 🆕 (2 สัปดาห์) ✅ เสร็จสมบูรณ์

ระยะเวลา: 2 สัปดาห์
> เป้าหมาย: พัฒนาฟีเจอร์ให้ครบถ้วนตาม SPEC.md

**คณิตศาสตร์ (Math):**
- [x] `pi()` → 3.14159...
- [x] `round(n)`, `ceil(n)`, `floor(n)`
- [x] `sqrt(n)`, `pow(base, exp)`, `abs(n)` ✅ (มีแล้ว)
- [x] `sin(n)`, `cos(n)`, `tan(n)` (ใช้ `libm` หรือ pure Rust implementation)
- [x] `random()` → สุ่มเลขทศนิยม 0-1

**ข้อความ (String):**
- [x] `trim(s)`, `trim_start(s)`, `trim_end(s)`
- [x] `split(s, delimiter)` → อาร์เรย์ของข้อความ
- [x] `replace(s, from, to)`
- [x] `substring(s, start, length)`

## 🟢 Phase 11: ชนิดข้อมูลขั้นสูง (Advanced Data Types) (2 สัปดาห์) ✅ เสร็จสมบูรณ์

ระยะเวลา: 2 สัปดาห์
> หมายเหตุ: ปรับปรุง (Refactor) ฟังก์ชันวันที่จากการห่อหุ้มข้อความ (string wrapping) เป็น DateTime/Duration แบบดั้งเดิม (native)

- [x] **11.1** เพิ่ม `Value::DateTime(jiff::Timestamp)` และ `Value::Duration(jiff::SignedDuration)`
- [x] **11.2** เพิ่ม `Value::Set(HashSet<Value>)` และ `Value::Range { start, end, step }`
- [x] **11.3** ปรับปรุงฟังก์ชันวันที่: `now()` → คืนค่าเป็น `Value::DateTime`, `date()` → วิเคราะห์ (parse) → `Value::DateTime`
- [x] **11.4** ปรับปรุง `date_add()`, `date_diff()` → ประมวลผลบนชนิดข้อมูลแบบดั้งเดิม
- [x] **11.5** เพิ่มตัวดำเนินการ @: `@2024-01-01` → DateTime literal
- [x] **11.6** การประมวลผลเซต (Set operations): `union`, `intersection`, `difference`, `in`
- [x] **11.7** การประมวลผลช่วง (Range operations): `range(1, 10)`, `range_to_array()`
- [x] **11.8** การทดสอบ (Test): กฎการแปลงชนิดข้อมูล (type coercion), การแสดงผลรูปแบบข้อมูลสำหรับชนิดข้อมูลขั้นสูง

## 🟢 Phase 12: การแปลงข้อมูลและการทำแคช (Serialization & Caching) (1.5 สัปดาห์) ✅ เสร็จสมบูรณ์

ระยะเวลา: 1.5 สัปดาห์

- [x] **12.1** ใช้งาน `#[derive(Serialize, Deserialize)]` บน `Value`, `Expr` (ภายใต้ feature gate `serde`)
- [x] **12.2** เพิ่ม Feature gate: `serialization` ในไฟล์ Cargo.toml
- [x] **12.3** `FormulaCache` — แคชแบบ LRU สำหรับนิพจน์ที่มีการใช้งานซ้ำ
- [x] **12.4** `Context::to_json()` / `Context::from_json()` — การจัดเก็บและดึงข้อมูลตัวแปรในรูปแบบ JSON
- [x] **12.5** การทดสอบ (Test): การประมวลผลแบบ round-trip serialization, อัตราการพบข้อมูลในแคช (cache hit/miss)

## 🔵 Phase 14: ประสิทธิภาพและการปรับแต่ง (Performance & Optimization) (2 สัปดาห์) 🔄 กำลังดำเนินการ

ระยะเวลา: 2 สัปดาห์

- [ ] **14.1** ขั้นตอนการปรับแต่ง Constant folding: `1 + 2` → `3` ในขั้นตอนวิเคราะห์หรือคอมไพล์
- [ ] **14.2** การปรับแต่ง AST: `if(true, X, Y)` → `X`, `if(false, X, Y)` → `Y`
- [ ] **14.3** เพิ่มการทดสอบประสิทธิภาพด้วย criterion: เปรียบเทียบกับฐานข้อมูล V1
- [x] **14.4** การทำ Memoization/Optimization สำหรับฟังก์ชันระดับสูง (Higher-order functions)
- [ ] **14.5** ใช้งาน `#[bench]` สำหรับทุกฟังก์ชันภายใน (builtin functions)
- [ ] **14.6** จัดทำเอกสารแนะนำการปรับแต่งประสิทธิภาพตามโปรไฟล์ (Profile-guided optimization)

## Phase 15: การกู้คืนข้อผิดพลาดและข้อจำกัดด้านความปลอดภัย (Error Recovery + Security Limits)

ระยะเวลา: 1 สัปดาห์

- [ ] **15.1** `parse_with_recovery()` — รวบรวมข้อผิดพลาดทั้งหมดแทนการหยุดทำงานทันที
- [ ] **15.2** กลยุทธ์การกู้คืนข้อผิดพลาด: ข้ามไปยังตัวคั่นถัดไป, เติมโทเค็นที่ขาดหายไป
- [ ] **15.3** `EngineConfig { max_formula_length, max_depth, max_time }`
- [ ] **15.4** `Evaluator::with_config(config)` — บังคับใช้ข้อจำกัด
- [ ] **15.5** การทดสอบ (Test): สูตรยาวเกินไป, ความลึกของการเรียกซ้ำเกินกำหนด, การหมดเวลาประมวลผล

---
**หมายเหตุ:** สำหรับประวัติของงานที่เสร็จสมบูรณ์แล้ว โปรดดูที่ [docs/achives/DONE_DETAILED.md](./docs/achives/DONE_DETAILED.md)
