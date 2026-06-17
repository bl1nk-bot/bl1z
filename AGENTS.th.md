# คำแนะนำสำหรับ bl1z Agent (bl1z Agent Instructions)

| สถานะ CI (CI Status) | คุณภาพโค้ด (Code Quality) | ความปลอดภัย (Security) |
|:---:|:---:|:---:|
| [![CI](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml) | [![CodeQL](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml) | [![CI Failure Handler](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml) |

@SPEC.th.md
@PLAN.th.md
@STYLE.th.md
@TODO.th.md
@REVIEW.th.md

## การจัดการบริบทและการนำทาง (Context Management & Navigation)
| ไฟล์ (File) | บทบาท (Role) | เมื่อใดที่ควรอ่าน (When to Read) | เมื่อใดที่ควรบันทึก (When to Update) |
|------|------|--------------|----------------|
| **TODO.th.md** | งานที่กำลังดำเนินการ | เริ่มต้นทุกเซสชัน/งาน | หลังเสร็จสิ้นทุกงาน |
| **PLAN.th.md** | แผนงานหลัก | ก่อนเริ่มเฟสใหม่ | เมื่อเฟสเริ่มต้นหรือสิ้นสุด |
| **SPEC.th.md** | ข้อมูลทางเทคนิค | เมื่อติดตั้งลอจิก/ชนิดข้อมูล | เมื่อสถาปัตยกรรม/ไวยากรณ์เปลี่ยน |
| **STYLE.th.md** | มาตรฐานโค้ด | ในระหว่างการเขียนโค้ด | หากข้อกำหนดของทีมเปลี่ยน |
| **REVIEW.th.md**| แนวทางการรีวิว | ก่อนรีวิว PR หรือโค้ดใด ๆ | หากมาตรฐานการรีวิวเปลี่ยน |

## การสื่อสารและประสิทธิภาพของโทเค็น (Communication & Token Efficiency)
- **English-Only Headers:** หัวข้อและส่วนหัว (Headers) ต้องเป็นภาษาอังกฤษเท่านั้น ห้ามใช้ส่วนหัวสองภาษาที่ซ้ำซ้อน (เช่น ห้ามใช้ `หัวข้อ (Header)` หรือ `ขั้นตอน (Step)`)
- **Thai Body Content:** ใช้งานภาษาไทยสำหรับการแชททั่วไป การอธิบายตรรกะ และการสื่อสารเฉพาะเจาะจงในโปรเจกต์

## ข้อบังคับในการปฏิบัติงาน (Operational Mandates)
- **ห้ามขอโทษ (No Apologies):** ห้ามขอโทษสำหรับข้อผิดพลาดหรือข้อจำกัด ให้ระบุแนวทางแก้ไขทางเทคนิคทันทีหรือระบุความต้องการที่ขาดหายไป
- **รีวิวก่อนรวมโค้ด (Review Before Merge):** อ่านคอมเมนต์การรีวิวทั้งหมดเสมอ (โดยเฉพาะจากบอทตัวอื่นหรือผู้รีวิวที่เป็นมนุษย์) และแก้ไขให้เรียบร้อยก่อนรวม Pull Request
- **การตรวจสอบก่อน Commit (Pre-Commit Checks):** การรันเครื่องมือจัดรูปแบบ (เช่น `cargo fmt`) และการตรวจสอบคุณภาพโค้ด (เช่น `cargo clippy`) เป็นขั้นตอนบังคับก่อนการ commit ทุกครั้ง
- **มาตรฐาน Commit (Commit Standards):** บังคับใช้รูปแบบ Conventional Commits อย่างเคร่งครัด ความยาวหัวข้อต้องไม่เกิน 50 ตัวอักษร
- **Issues-First:** ทุกงานต้องมี GitHub Issue เสมอ ข้อความต้นฉบับจากมนุษย์ต้องถูกรักษาไว้ในรายละเอียดหรือคอมเมนต์ของ Issue
- **การยืนยัน (Verification):** ทุกการเปลี่ยนแปลงต้องได้รับการยืนยันด้วยบันทึกเหตุการณ์หรือผลการทดสอบ งานที่ไม่มีการยืนยันจะถือว่าไม่เสร็จสิ้น
- **Zero Warnings:** การตรวจสอบผ่าน Clippy และการทดสอบต้องไม่มีคำเตือนใด ๆ (-D warnings)
- **Zero Unsafe:** ปฏิบัติตามนโยบาย zero-unsafe ที่ระบุไว้ใน SPEC.th.md และ STYLE.th.md อย่างเคร่งครัด
- **การเปิดตัวอัตโนมัติ (Automated Release):** ห้ามทำการเปิดตัว (release) ด้วยตนเอง เอกสารประกอบ (CHANGELOG, TODO, PLAN) ต้องอัปเดตครบ 100% และการทดสอบทั้งหมดต้องผ่านก่อนเริ่มกระบวนการ release ผ่าน `git tag` หรือ `gh release`

## คำแนะนำแบบย่อ (Compact Instructions)
คงรักษาข้อมูลเหล่านี้ในระหว่างการใช้ /compact: [กฎ English-Only Headers], [ลำดับความสำคัญของเครื่องมือ MCP], [ขอบเขตที่เข้มงวด], [อนุกรมวิธานรหัสข้อผิดพลาด] ตัดคำอธิบายที่ซ้ำซ้อนออกก่อน

## ลำดับความสำคัญของเครื่องมือ MCP (MCP Tool Preferences)
- ใช้งาน `hex-line` เป็นอันดับแรกสำหรับการอ่าน/ค้นหา/แก้ไขไฟล์ข้อความใน repository
- ใช้งาน `hex-graph` เป็นอันดับแรกสำหรับคำถามเชิงตรรกะของโค้ด (อัตลักษณ์ของสัญลักษณ์, สถาปัตยกรรม)
- ใช้งานเครื่องมือในตัวหรือ shell เฉพาะเมื่อไม่สามารถใช้งาน MCP ได้ หรืองานนั้นเหมาะกับ shell มากกว่า
