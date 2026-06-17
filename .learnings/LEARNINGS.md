# LEARNINGS LOG

## [LRN-20260602-001] Communication Protocol Correction

**Recorded at**: 2026-06-02T17:40:00Z
**Priority**: High
**Status**: Promoted
**Area**: behavior

### Summary
Agents must never apologize. Instead, they should immediately provide a technical fix or identify the required information to move forward.

### Details
Apologizing consumes tokens and provides zero technical value. The user explicitly demanded high-fidelity execution and immediate correction of errors without filler text.

### Suggested Action
Modify system prompts or instructions to enforce a "No Apology" rule and a "Immediate Technical Correction" pattern.

### Metadata
- Source: User Correction
- Files: .gemini/GEMINI.md, .agents/LEARN.md
- Promoted: SOUL.md, GEMINI.md

---

## [LRN-20260602-002] PR Merge Protocol (Review First)

**Recorded at**: 2026-06-02T17:40:00Z
**Priority**: Critical
**Status**: Pending
**Area**: infra | workflow

### Summary
Do not merge PRs blindly. Always read all review comments (especially from other bots or human reviewers) and address them before merging.

### Details
Blindly merging PR #25 led to missing critical feedback about error codes and regression in documentation that was previously addressed in other PRs. It also caused unnecessary conflicts in PR #26.

### Suggested Action
Check `gh pr view <num> --json reviews,comments` before any merge operation. Summarize the findings to the user and ensure all blockers are resolved.

### Metadata
- Source: Discovery
- Tags: workflow, git, quality-control

---
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
