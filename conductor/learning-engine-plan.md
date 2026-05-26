# 🧠 Semantic Learning & Pattern Detection Engine

## Objective
To transform the agent from a "mechanical executor" into a "learning entity." The system will extract qualitative lessons from failures (Post-Mortems), store them persistently, and employ an automated pattern-detection mechanism to ensure these mistakes are never repeated. If a mistake recurs, the system forces a refinement of the lesson.

## Architecture

### 1. The Knowledge Base (`gemini-curator/memory/POST_MORTEMS.md`)
A persistent file storing extracted lessons, formatted strictly with `---` section separators.
**Format per section:**
- **Failure**: What went wrong.
- **Root Cause**: Why it happened (the cognitive or technical error).
- **Anti-Pattern**: The observable action in the logs (e.g., `tool: run_shell_command`, `args: find /`).
- **The Lesson (Mandate)**: The strict rule to prevent recurrence.
- **Recurrence Count**: Integer tracking how many times this was repeated after the lesson was learned.

### 2. The Pattern Detector (`gemini-curator/scripts/pattern_detector.py`)
An automated script triggered by the `AfterAgent` hook.
- **Function**: Reads the most recent turn from `logs.json`.
- **Analysis**: Compares the agent's actions against the `Anti-Pattern` definitions in `POST_MORTEMS.md`.
- **Action on Detection**:
  - If a match is found: Increments the `Recurrence Count` in `POST_MORTEMS.md`.
  - Injects a `[CRITICAL_WARNING]` directly into `SESSION_STATE.md`, forcing the agent to read it on the next turn and refine its approach.

## Implementation Steps

1. **Create Base Structure**:
   - Initialize `gemini-curator/memory/POST_MORTEMS.md` with the first lesson (The 95-Turn Token Burn failure).
2. **Develop Detector Logic**:
   - Write `pattern_detector.py` to parse logs and match regex/heuristic rules defined in the Post-Mortems.
3. **Hook Integration**:
   - Update `gemini-extension.json` `AfterAgent` hook array to run `pattern_detector.py` before `auditor.sh`.
4. **Context Injection**:
   - Update `SessionStart` hook to quickly summarize active Anti-Patterns from `POST_MORTEMS.md` into the active context, ensuring the agent is "aware" of them before acting.

## Verification
- Deliberately execute an anti-pattern (e.g., searching the root directory without limits).
- Verify that `pattern_detector.py` catches it, increments the counter, and flags `SESSION_STATE.md`.