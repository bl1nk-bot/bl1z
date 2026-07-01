---
name: git_merge_conflict_powershell
description: Handle git merge conflicts on gitignored files (like snapshots) using force-stage, and avoid using && operators in Windows PowerShell.
globs: "*"
---

# Staging Gitignored Conflicts and PowerShell Syntax

## Context
When git merge conflicts occur on files that are matched by new or updated `.gitignore` patterns (such as `*.snap` or `tests/snapshots/`), standard staging (`git add <file>`) fails with a warning/error about ignored files. Furthermore, executing chained terminal commands with `&&` in Windows PowerShell results in a syntax error.

## Pattern / Rule
- To stage files that are matched by gitignore patterns during merge conflict resolution, use the force flag: `git add -f <file>`.
- Avoid using the `&&` operator when proposing or running commands on Windows PowerShell systems. Instead, separate the commands with a semicolon (`;`) or propose them as separate lines/invocations.
