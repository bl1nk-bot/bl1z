# บันทึกการเปลี่ยนแปลง (Changelog)

การเปลี่ยนแปลงที่สำคัญทั้งหมดในโปรเจกต์นี้จะถูกบันทึกไว้ในไฟล์นี้

รูปแบบอ้างอิงตาม [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
และโปรเจกต์นี้ปฏิบัติตามกฎ [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## [0.2.15] - 2026-06-17

### เพิ่มเติม (Added)
- **Lambda & Higher-Order Functions**: เพิ่มการรองรับ lambda expression `(x) => x * 2` และฟังก์ชันต่างๆ เช่น `map`, `filter`, `reduce`, `sort`, `group_by`, `unique`
- **User-Defined Functions**: ไวยากรณ์ใหม่ `fn name(params) = expression` สำหรับนิยามฟังก์ชันที่นำกลับมาใช้ใหม่ได้ภายในบริบท (Context)
- **Advanced Data Types**: รองรับชนิดข้อมูลแบบดั้งเดิม (Native) สำหรับ `DateTime` (jiff), `Duration`, `Set` และ `Range`
- **DateTime Literals**: เพิ่มตัวดำเนินการ `@` สำหรับค่าคงที่วันที่ (เช่น `@2024-01-01`)
- **Set Operations**: เพิ่มฟังก์ชันภายในใหม่สำหรับการจัดการเซต: `set_union`, `set_intersection`, `set_difference` และ `set_in`
- **Math Extensions**: เพิ่มฟังก์ชันคณิตศาสตร์ `pi()`, `round()`, `ceil()`, `floor()`, `sqrt()`, `pow()`, `sin()`, `cos()`, `tan()` และ `random()`
- **String Extensions**: เพิ่มฟังก์ชันจัดการข้อความ `trim()`, `trim_start()`, `trim_end()`, `split()`, `replace()` และ `substring()`
- **Serialization**: เพิ่มการรองรับ `serde` สำหรับ `Value`, `Expr` และ `Context` ภายใต้ feature gate `serialization`
- **Context Snapshot**: `Context::to_json()` และ `Context::from_json()` สำหรับการบันทึกสถานะของระบบ
- **Caching**: เพิ่ม `FormulaCache` สำหรับการทำแคชแบบ LRU เพื่อประสิทธิภาพในการประมวลผลนิพจน์ที่ซ้ำกัน
- **Plugin SDK**: โครงสร้างพื้นฐานสำหรับปลั๊กอินภายนอกด้วย `Plugin` trait และ `PluginManager`
- **Sequence Expressions**: รองรับการระบุนิพจน์หลายรายการโดยคั่นด้วยเครื่องหมายอัฒภาค `;`

### เปลี่ยนแปลง (Changed)
- **Lexer**: เพิ่มโทเค็นสำหรับ `@`, `=>`, `fn`, `=` และ `;`
- **Value**: เพิ่มตัวเลือก (variant) สำหรับ `DateTime`, `Duration`, `Set` และ `Range`
- **Builtins**: จัดระเบียบโมดูลตามหมวดหมู่ (`math.rs`, `string.rs`, `sets.rs`, `higher_order.rs`)

### แก้ไข (Fixed)
- **Higher-Order Functions**: ปรับปรุงการส่งค่า registry ให้มีประสิทธิภาพมากขึ้นเพื่อป้องกันการโคลนข้อมูลโดยไม่จำเป็นในระหว่างการวนซ้ำ
- **Clippy**: แก้ไข `cloned_ref_to_slice_refs` warnings ด้วย `std::slice::from_ref`

### เปลี่ยนแปลง (Changed)
- **Date Builtins**: `now()` และ `date()` คืนค่า `Value::DateTime` แทน `Value::String`
- **Date Input**: `year()`, `month()`, `day()`, `date_add()`, `date_diff()` รับทั้ง `Value::DateTime` และ `Value::String`
- **Error Messages**: ทุก builtin แสดง expected vs received type (เช่น `abs ต้องการ Number แต่ได้ String`)
- **Version**: อัพจาก `0.2.0` เป็น `0.2.15`

### การเพิ่มประสิทธิภาพ (Phase 14)
- **AST Optimizer** (`src/optimizer.rs`): Constant folding, algebraic identities (`x+0`, `x*1`, `x*0`, `--x`), string concat folding, comparison folding
- **`evaluate_optimized()`**: Entry point ใหม่ที่รัน optimizer ก่อน evaluate
- **Benchmarks**: 11 criterion benchmarks ครอบคลุม arithmetic, complex expressions, arrays, nested functions, dates, maps, access chaining, HOF, UDF vs lambda, formula cache, และ advanced types

### การกู้คืนข้อผิดพลาด (Phase 15)
- **`parse_with_recovery()`**: เก็บทุก parse error แทน fail-fast, ข้ามไปsemicolon ถัดไป
- **`EngineConfig`**: กำหนดค่าขีดจำกัด — `max_formula_length` (default 10,000), `max_depth` (default 100), `max_time_ms` (optional timeout)
- **`evaluate_with_config()`** และ `parse_with_config()`: parsing/evaluation ที่รองรับ config
- **Timeout**: ใช้ `std::time::Instant` บังคับเวลา (ตรวจทุก 1,000 eval steps)
- **Error Code E901**: Recovery errors สำหรับ partial parse results

### เอกสาร
- **PGO Guide** (`docs/PGO.md`): แนวทาง Profile-Guided Optimization สำหรับ bl1z
- **Bilingual Docs**: เอกสารภาษาไทยสำหรับเอกสาร project-facing ทั้งหมด

## [0.2.0] - 2026-05-18

### เพิ่มเติม (Added)
- **Access Chaining**: รองรับการใช้งานแบบจุด (`obj.prop`) และการระบุดัชนีผ่านวงเล็บเหลี่ยม (`arr[0]`) สำหรับการเข้าถึงข้อมูลแบบลำดับชั้น
- **Chained Access**: รองรับการเชื่อมต่อการเข้าถึงข้อมูลอย่างเต็มรูปแบบ เช่น `users[0].name`, `config.db.host`, `a.b[0].c.d`
- **Context Scoping**: การค้นหาตัวแปรในลำดับชั้นแม่ (Parent chain) — ขอบเขตลูกจะสืบทอดตัวแปรจากแม่และสามารถประกาศทับ (shadowing) ได้โดยไม่กระทบต่อข้อมูลเดิม
- **Context Utilities**: เพิ่มเมธอด `Context::with_parent()`, `Context::get_all()`, `Context::depth()` สำหรับการจัดการขอบเขตแบบลำดับชั้น
- **Error Codes**: เพิ่มรหัสข้อผิดพลาด `E207` (PropertyNotFound), `E208` (IndexOutOfBounds) สำหรับข้อผิดพลาดในการเข้าถึงข้อมูล
- **Test Infrastructure**: แยกการทดสอบ 169 รายการไปยัง `src/lib_tests.rs` และเปลี่ยนชื่อตามมาตรฐาน `{subject}_{scenario}_{expected_outcome}`
- **pretty_assertions**: เพิ่ม dev-dependency สำหรับการแสดงผลความแตกต่าง (diff) เมื่อการทดสอบล้มเหลว

### เปลี่ยนแปลง (Changed)
- **Parser**: เปลี่ยนจากการใช้เทคนิคเชื่อมต่อข้อความแบบเดิมมาเป็นเมธอด `parse_postfix()` ที่รองรับการเข้าถึงข้อมูลแบบลำดับชั้นอย่างถูกต้อง
- **Evaluator**: เปลี่ยนจากการใช้ `name.split('.')` มาเป็นการประมวลผลผ่าน `PropertyAccess`/`IndexAccess` โดยเฉพาะ
- **Context**: เปลี่ยนโครงสร้างการจัดเก็บภายในจาก `HashMap` เป็น `BTreeMap` เพื่อให้การวนซ้ำข้อมูลมีความแน่นอน (deterministic)
- **Context**: ใช้งาน `Rc<Context>` สำหรับการแชร์ลำดับชั้นแม่ (ปลอดภัยในการโคลน, ไม่มีการคัดลอกข้อมูลซ้ำซ้อน)
- **FormulaError**: เพิ่มการใช้งาน `PartialEq` สำหรับการเปรียบเทียบในขั้นตอนการทดสอบ

### แก้ไข (Fixed)
- แก้ไขการทดสอบที่ล้มเหลวก่อนหน้า 4 รายการ: แก้ไขรหัสข้อผิดพลาดที่ไม่ตรงกัน (`E401` → `E501`) สำหรับข้อผิดพลาดเกี่ยวกับชนิดข้อมูลของฟังก์ชัน
- แก้ไข snapshot test 1 รายการ: อัปเดต `array_function_wrong_type` ให้ตรงกับรหัสข้อผิดพลาดจริง

### การเปลี่ยนแปลงที่ส่งผลกระทบ (Breaking Changes)
- **Context**: `Context` ไม่ได้สร้าง `Default` ผ่านการ derive อีกต่อไป (ใช้การติดตั้งแบบระบุชัดเจน) พฤติกรรมการโคลนเปลี่ยนไป — ข้อมูลที่โคลนจะแชร์แม่ผ่าน `Rc` แทนการคัดลอกข้อมูลทั้งหมด
- **Internal**: `Expr::Variable` ไม่รองรับการระบุแบบจุด (เช่น `user.score` จะกลายเป็น `PropertyAccess` แทนที่จะเป็น `Variable("user.score")`) ซึ่งจะส่งผลต่อโค้ดที่มีการตรวจสอบรูปแบบ (matching) บน `Expr` โดยตรงเท่านั้น

## [0.1.0] - 2026-05-07

### เพิ่มเติม (Added)
- **Core bl1z**: พัฒนา Lexer, Parser, AST และ Evaluator อย่างครบถ้วน
- **Data Types**: รองรับชนิดข้อมูล Number, String, Bool, Null, Array และ Map
- **Array Literals**: ไวยากรณ์ `[1, 2, 3]` พร้อมรองรับอาร์เรย์ซ้อนอาร์เรย์
- **Map Literals**: ไวยากรณ์ `{key: "value"}` สำหรับโครงสร้างข้อมูลแบบวัตถุ
- **Collection Functions**: ฟังก์ชัน `sum()`, `avg()`, `min()`, `max()`, `join()`, `count()` สำหรับอาร์เรย์
- **Date/Time Functions**: ฟังก์ชัน `now()`, `date()`, `date_diff()`, `year()`, `month()`, `day()` โดยใช้ไลบรารี jiff
- **String Functions**: ฟังก์ชัน `len()`, `upper()`, `lower()`, `contains()`, `starts_with()`, `ends_with()`
- **Math Functions**: ฟังก์ชัน `abs()`, `min()`, `max()` สำหรับตัวเลข
- **Logic Functions**: ฟังก์ชันเงื่อนไข `if()`
- **Performance Tools**: การทำ Benchmarking ด้วย Criterion และเครื่องมือสำหรับการวิเคราะห์โปรไฟล์ (profiling)
- **Error Handling**: ระบบรายงานข้อผิดพลาดที่ครอบคลุม พร้อมข้อมูลตำแหน่งในโค้ดและข้อความภาษาไทย
- **Testing**: การทดสอบระดับหน่วย (unit tests), ระดับรวม (integration tests) และ snapshot tests สำหรับรูปแบบข้อผิดพลาด
- **CI/CD**: ระบบ GitHub Actions สำหรับการทดสอบอัตโนมัติ การจัดรูปแบบโค้ด และการตรวจสอบคุณภาพโค้ด

### เปลี่ยนแปลง (Changed)
- **Date Library**: เปลี่ยนจาก `chrono` มาเป็น `jiff` (v0.2) ซึ่งเป็น pure Rust เพื่อประสิทธิภาพที่สูงขึ้นและไม่มีการพึ่งพาภาษา C
- **Build Optimization**: เพิ่ม release profile พร้อมการตั้งค่า LTO, codegen-units=1 และการตัดข้อมูลส่วนเกิน (stripping) เพื่อขนาดไฟล์ไบนารีที่เหมาะสมที่สุด

### ข้อมูลทางเทคนิค (Technical Details)
- **Zero-cost abstractions** พร้อมการประมวลผล AST ที่มีประสิทธิภาพ
- **Extensible architecture** ผ่านระบบการลงทะเบียนฟังก์ชัน (function registry)
- **Strong type safety** พร้อมการตรวจสอบในขณะรันไทม์ (runtime validation)
- **Memory safe** - ไม่มีการใช้โค้ดที่ไม่ปลอดภัย (unsafe code)
- **Performance focused** - ประมวลผลได้รวดเร็ว (<10μs สำหรับการประมวลผลทั่วไป)

### การพึ่งพา (Dependencies)
- `jiff = "0.2"` - ไลบรารีจัดการวันที่และเวลาแบบ Pure Rust
- `criterion = "0.5"` - โครงสร้างพื้นฐานสำหรับการทำ Benchmarking
- `insta = "1.34"` - เครื่องมือสำหรับการทำ Snapshot testing

### ความเข้ากันได้ (Compatibility)
- **Rust**: 1.70 ขึ้นไป (MSRV - Minimum Supported Rust Version)
- **Platforms**: รองรับทุกแพลตฟอร์มที่ Rust รองรับ
- **No C dependencies** - พัฒนาด้วยภาษา Rust ทั้งหมด

---

## ประเภทของการเปลี่ยนแปลง
- `Added`: สำหรับฟีเจอร์ใหม่
- `Changed`: สำหรับการเปลี่ยนแปลงฟังก์ชันการทำงานที่มีอยู่
- `Deprecated`: สำหรับฟีเจอร์ที่กำลังจะถูกยกเลิกในอนาคต
- `Removed`: สำหรับฟีเจอร์ที่ถูกนำออกแล้ว
- `Fixed`: สำหรับการแก้ไขบั๊ก
- `Security`: ในกรณีที่พบช่องโหว่ด้านความปลอดภัย
