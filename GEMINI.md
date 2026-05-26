## Architects Mandates (CRITICAL)

### 1. Context & Tool Efficiency
- **Token Cap**: Stop and report plan if a task exceeds 10 tool calls in a single Turn.
- **Surgical Read**: `read_file` MUST NOT exceed 50 lines. Use start/end lines.
- **Search Scope**: `grep`/`find` MUST include `--max-depth`, `--max-matches`, and specific patterns. NEVER search root.
- **State Check**: ALWAYS read `gemini-curator/SESSION_STATE.md` at the start of a session.

### 2. Communication Standards
- **Zero Filler**: No "I see", "I will now", "I have finished", or apologies.
- **Technical Only**: Responses must focus on intent and technical rationale.
- **No Repetition**: Do not explain what was just done if the tool output is visible.

### 3. Verification
- All changes must be verified with evidence (logs/test results).
- Failure to verify = Task Incomplete.

---

## Technical Style (Good vs Bad)

**Bad**: "I see you want me to find the file. I will now search for it using the find tool. I have found it here: /path/to/file. I hope this helps!"
**Good**: "Locating team-cli-source. Found at /data/data/com.termux/files/home/bl1z/gemini-curator/team-cli-source/"
