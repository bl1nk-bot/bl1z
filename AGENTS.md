# bl1z Agent Instructions

| CI Status | Code Quality | Security |
|:---:|:---:|:---:|
| [![CI](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci.yml) | [![CodeQL](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/codeql.yml) | [![CI Failure Handler](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml/badge.svg)](https://github.com/bl1nk-bot/bl1z/actions/workflows/ci-fail.yml) |

@SPEC.md
@PLAN.md
@STYLE.md
@TODO.md
@REVIEW.md

## Context Management & Navigation
| File | Role | When to Read | When to Update |
|------|------|--------------|----------------|
| **TODO.md** | Active Tasks | Start of every session/task. | After EVERY completed task. |
| **PLAN.md** | Roadmap | Before starting a new Phase. | When a Phase starts or finishes. |
| **SPEC.md** | Tech Specs | When implementing logic/types. | When architecture/syntax changes. |
| **STYLE.md** | Standards | During code implementation. | If team conventions change. |
| **REVIEW.md**| Review Guide | Before reviewing any PR/code. | If review standards evolve. |

## Communication & Token Efficiency
- **English-Only Headers:** Titles/Headers MUST be English-only. DO NOT write redundant bilingual headers (e.g., never use `หัวข้อ (Header)` or `ขั้นตอน (Step)`).
- **Thai Body Content:** Use Thai for standard chat, logic explanations, and project-specific communications.

## Operational Mandates
- **No Apologies:** Do not apologize for errors or limitations. Instead, provide an immediate technical fix or identify missing requirements.
- **Review Before Merge:** Always read all review comments (especially from other bots or human reviewers) and address them before merging any Pull Request.
- **Pre-Commit Checks:** Execution of project-specific formatters (e.g., `cargo fmt`) and linters (e.g., `cargo clippy`) is a mandatory prerequisite for every commit.
- **Commit Standards:** Strictly enforce Conventional Commits. Header length must not exceed 50 characters.
- **Issues-First:** Every task MUST have a GitHub Issue. The "Originating Prompt" from the human must be preserved in the Issue description or comments.
- **Verification:** Every change MUST be verified with logs or test results. Failure to verify means the task is incomplete.
- **Zero Warnings:** Clippy and tests must pass with zero warnings (-D warnings).
- **Zero Unsafe:** Strictly adhere to the zero-unsafe policy defined in SPEC.md and STYLE.md.
- **Automated Release:** NEVER release manually. Documentation (CHANGELOG, TODO, PLAN) MUST be 100% updated and all tests passed before triggering the release workflow via `git tag` or `gh release`.

## Compact Instructions
Preserve during /compact: [English-Only Headers rule], [MCP Tool Preferences], [Hard Boundaries], [Error Code taxonomy]. Drop redundant explanations first.

## MCP Tool Preferences
- Use `hex-line` first for repository text reads/search/edits.
- Use `hex-graph` first for semantic code questions (symbol identity, architecture).
- Use built-in tools or shell ONLY when MCP is unavailable or the task is shell-native.
