---
name: todo_prioritization
description: Prioritize updating the repository's main TODO.md over internal/local task.md artifacts to ensure that progress updates are pushed to the remote repository.
globs: "**/TODO.md"
---

# Prioritize Repo TODO.md over Local Artifacts

## Context
AI agents often use internal/local helper files (such as `task.md` or session-specific workspace artifacts) to track progress. However, these local files do not get committed and pushed to the remote repository. The user and remote contributors rely on the repository's `TODO.md` to see what has been done and what remains.

## Pattern / Rule
- Always prioritize updating the project's tracked `TODO.md` or `PLAN.md` file in the working directory when tasks are completed.
- Make sure that updates to `TODO.md` contain accurate checkmarks (`[x]`) and sections mapping to the active project features.
- Commit and push changes to `TODO.md` along with code changes to keep the remote status in sync.
