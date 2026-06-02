# AGENT LEARNING LOG

- Session: 2026-06-02
- Status: Critical Failure in Protocol Adherence

## Rules & Constraints
- ห้ามขอโทษ (No Apologies): สิ้นเปลืองโทเคนและไร้ประโยชน์
- การแก้ไข (Correction): เมื่อทำผิด ให้ลงมือแก้ไขทันทีในเทิร์นถัดไปแทนการกล่าวโทษ
- การจัดรูปแบบ (Formatting): ห้ามใช้ตัวหนา (**) เด็ดขาด ใช้เฉพาะ #, ##, -, ---, >
- ภาษา (Language): Header ต้องเป็นภาษาอังกฤษล้วนและสั้นกระชับ
- การบันทึก (Persistence): ต้องบันทึกบทเรียนลงใน LEARN.md พร้อมวันที่และ Session ห้ามรายงานซ้ำซากในแชท
- Gemini.md Update: เขียนเฉพาะ "ต้องทำ" หรือ "ห้ามทำ" ให้ชัดเจน ตัดคำอธิบายที่ไม่จำเป็นออก

## Technical Failures
- Git Auth: ต้องตรวจสอบสิทธิ์ PAT (repo, workflow) และแจ้งวิธีแก้ (gh auth refresh) ก่อนเริ่มงาน push/merge
- Token Efficiency: ลดการใช้คำฟุ่มเฟือยเพื่อรักษา Context Window
