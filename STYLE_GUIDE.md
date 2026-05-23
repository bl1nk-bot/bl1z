# คู่มือสไตล์ (Style Guide)

คู่มือสไตล์นี้เป็นข้อตกลงในการพัฒนาสำหรับโปรเจกต์ `bl1z` ซึ่งเป็น Notion-like formula evaluation engine ในภาษา Rust โปรดปฏิบัติตามกฎและแนวทางเหล่านี้อย่างเคร่งครัดเพื่อให้โค้ดมีความเป็นระเบียบเรียบร้อยและรักษาคุณภาพของซอร์สโค้ดตามมาตรฐานของโครงการ

---

## 1. สไตล์ของโค้ด (Code Style)

### ข้อตกลงเฉพาะภาษา

**ภาษาหลัก:** Rust (Edition 2021 | MSRV 1.90.0)

| ข้อตกลง | กฎ |
|-----------|------|
| การย่อหน้า | 4 spaces (ตามมาตรฐาน `rustfmt` และ `cargo fmt`) |
| ความยาวบรรทัด | 100 ตัวอักษร |
| Semicolons | จำเป็นในแต่ละ statement ของ Rust (ยกเว้น statement สุดท้ายของบล็อกที่เป็น implicit return) |
| Quotes | Double quotes (`"`) สำหรับ strings และ Single quotes (`'`) สำหรับ characters ตามข้อกำหนดของภาษา Rust |
| Trailing commas | ใส่เครื่องหมาย comma (`,`) ท้ายบรรทัดเสมอในรายการที่มีการประกาศแบบหลายบรรทัด (เช่น struct, enum, arrays หรือ list parameters) |

### การตั้งชื่อ (Naming Conventions)

การตั้งชื่ออ้างอิงตามมาตรฐาน [Rust Style Guide](https://doc.rust-lang.org/style-guide/) และข้อกำหนดของโปรเจกต์ `bl1z`:

| องค์ประกอบ | ข้อตกลง | ตัวอย่าง |
|---------|-----------|---------|
| ตัวแปร | `snake_case` | `user_name`, `formula_error`, `expr_type` |
| ฟังก์ชัน / เมธอด | `snake_case` | `get_user_by_id`, `eval_formula`, `tokenize` |
| Structs / Enums / Traits | `PascalCase` | `Value`, `FormulaError`, `Context`, `Expr` |
| ค่าคงที่ (Constants) | `SCREAMING_SNAKE_CASE` | `MAX_RETRY_COUNT`, `DEFAULT_PRECISION` |
| โมดูล / ไฟล์ | `snake_case` | `lexer.rs`, `parser.rs`, `error_snapshots.rs` |
| Generic Type Parameters | `PascalCase` (มักใช้ตัวอักษรเดี่ยว เช่น `T`, `E` หรือชื่อสั้น ๆ) | `T`, `E`, `Ctx` |

---

## 2. สไตล์ของเอกสาร (Documentation Style)

- **อธิบาย "ทำไม" (Why) ไม่ใช่ "อะไร" (What)**: ในคอมเมนต์ปกติ (Inline comments) ให้อธิบายเหตุผลของแนวทางหรือความซับซ้อนของลอจิกนั้น ๆ แทนการอธิบายสิ่งปรกติที่โค้ดทำอยู่แล้ว
- **ใช้ Doc Comments เสมอ**: สำหรับ public APIs ทั้งหมดในโปรเจกต์ (เช่น ใน [lib.rs](file:///D:/dev/10/poe-sdk-rs-fix-pr10/src/lib.rs)) จะต้องมี doc comments (`///` หรือ `//!`) ที่ระบุข้อมูลดังต่อไปนี้ครบถ้วน:
  - `# Arguments` (รายละเอียดพารามิเตอร์)
  - `# Returns` (รายละเอียดค่าที่ส่งกลับ)
  - `# Examples` (ตัวอย่างการใช้งานโค้ด)
  - ตัวอย่าง:
    ```rust
    /// Evaluates a formula with the given context.
    ///
    /// # Arguments
    /// * `formula` - A string slice containing the formula to evaluate.
    /// * `ctx` - The context containing variable bindings.
    ///
    /// # Returns
    /// Returns a `Result` containing the evaluated `Value` or a `FormulaError`.
    ///
    /// # Examples
    /// ```
    /// use bl1z::{evaluate, Context, Value};
    /// let ctx = Context::new();
    /// let result = evaluate("1 + 2", &ctx);
    /// assert_eq!(result.unwrap(), Value::Number(3.0));
    /// ```
    pub fn evaluate(formula: &str, ctx: &Context) -> Result<Value, FormulaError> {
        // ...
    }
    ```
- **Error Messages ในภาษาไทย**: สำหรับข้อความแสดงข้อผิดพลาด (เช่น ใน [error.rs](file:///D:/dev/10/poe-sdk-rs-fix-pr10/src/error.rs)) จะต้องเขียนเป็นภาษาไทย และใช้รหัสข้อผิดพลาดตามกลุ่มโค้ด เช่น `E1xx` (Lex), `E2xx` (Parse), `E3xx` (Eval), `E4xx` (Type), `E5xx` (Function), `E6xx` (Context)
- **ลบโค้ดที่คอมเมนต์ไว้ออก**: ห้ามทิ้งโค้ดที่ไม่ได้ใช้งานหรือถูกคอมเมนต์ทิ้งไว้ ให้ลบออกทันทีและอาศัย Git (Version Control) ในการย้อนดูประวัติแทน

---

## 3. รูปแบบ Commit Message

โปรเจกต์นี้ใช้ข้อกำหนด [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<scope>): <สรุปสั้นๆ>

<เนื้อหาเพิ่มเติม: อธิบายว่าทำอะไรและทำไม ไม่ใช่ทำอย่างไร>
```

### ประเภทของ Type ที่ใช้งานบ่อย:
- `feat`: เพิ่มฟีเจอร์ใหม่ให้กับโปรเจกต์ (เช่น `feat(parser): add access chaining for object properties`)
- `fix`: แก้ไขข้อผิดพลาดหรือบั๊ก (เช่น `fix(eval): prevent division by zero in evaluator`)
- `docs`: ปรับปรุงหรือเพิ่มเอกสารต่าง ๆ (เช่น `docs(readme): update build instructions`)
- `style`: ปรับแต่งรูปแบบโค้ดที่ไม่มีผลต่อการทำงานของโปรเจกต์ เช่น การทำ formatting ด้วย `cargo fmt`
- `refactor`: ปรับปรุงโครงสร้างของโค้ดใหม่โดยไม่มีการเพิ่มฟีเจอร์หรือแก้บั๊ก
- `test`: เพิ่ม ย้าย หรือแก้ไขไฟล์ทดสอบ (เช่น `test(lexer): add unit tests for escape characters`)
- `chore`: งานอื่น ๆ เช่น การแก้ไขระบบ CI หรือ dependency (เช่น `chore(deps): update jiff dependency`)

---

## 4. ข้อห้ามเด็ดขาด (Hard Bans)

- **ห้ามใส่ Credentials หรือ Secrets**: ห้ามฮาร์ดโค้ดหรือฝัง Credentials, API keys, หรือข้อมูลลับใด ๆ ลงในซอร์สโค้ดหรือเก็บไว้ในเวอร์ชันคอนโทรลเด็ดขาด
- **ห้ามใช้โค้ด `unsafe`**: โครงการนี้มีนโยบาย **Zero unsafe code** ทั่วทั้ง Crate (ห้ามระบุบล็อก `unsafe` หรือเรียกใช้ฟังก์ชันที่ไม่ปลอดภัย)
- **ห้ามมีคอมเมนต์ `TODO` แบบลอย ๆ**: ห้ามใส่คอมเมนต์ `TODO` ไว้โดยไม่มีการระบุ Issue ลิงก์แนบไว้ หรือไม่มีข้อมูลผู้รับผิดชอบและรายละเอียดที่ชัดเจน
- **ห้ามใช้ Wildcard Imports**: หลีกเลี่ยงการใช้ `use module::*;` ยกเว้นในการนำเข้าสัญลักษณ์ในโมดูลทดสอบ (`#[cfg(test)]`) หรือกรณีที่เป็น `prelude` ของ Crate เท่านั้น
- **ห้ามข้ามคำเตือน Clippy โดยไม่มีเหตุผล**: ห้ามระบุ `#![allow(...)]` หรือ `#[allow(...)]` เพื่อปิด linting ของ Clippy โดยไม่มีการเขียนคอมเมนต์อธิบายเหตุผลความจำเป็นที่ชัดเจนกำกับไว้ เนื่องจาก CI ของโปรเจกต์จะปฏิเสธโค้ดที่มีคำเตือนใด ๆ ทั้งสิ้น (`-D warnings`)
