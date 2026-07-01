---
name: gh_aw_expression_limitations
description: Avoid using unauthorized GitHub expressions in gh-aw frontmatter. Offload complex lookup logic to agent instructions using basic event parameters and tools.
globs: "**/.github/workflows/*.md"
---

# GitHub Agentic Workflows (`gh aw`) Expression Limitations

## Context
When writing agentic workflows (`gh aw`), compiling the Markdown definition to a GitHub Actions workflow using `gh aw compile` checks expression security. Using complex expressions such as `toJson(github.event.workflow_run.pull_requests)` fails validation because they are not in the allowed list of expressions.

## Pattern / Rule
- Stick strictly to allowed expressions (e.g., `github.event.workflow_run.id`, `github.event.workflow_run.head_sha`, `github.event.workflow_run.conclusion`) in workflow frontmatter configuration.
- Instead of extracting complex payloads inside YAML expressions, pass basic identifiers (like ID or SHA) and write instructions instructing the AI agent to use GitHub APIs or CLI tools to query the rest of the information dynamically.
