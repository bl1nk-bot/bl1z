---
name: user-intent-translation
description: Explain project rules and code comments as user-facing meaning before discussing implementation.
---

# User-intent translation

Use this for requests about code comments, rules, requirements, project context, or why code is defined a certain way.

## Rule

The user's instruction is authoritative. Code and scripts implement that instruction; they do not redefine it.

Treat comments that state a requirement as shared explanations of intent. Explain them in Thai, plainly and in this order:

1. What the rule says.
2. Why the rule exists.
3. What it makes the system, user, or future maintainer able to expect.
4. What would go wrong or become unclear without it.

Do this before discussing whether current code follows the rule. Only inspect or change code when the user asks for that separately.

## Communication

- The user learns while directing real project work and may not write code.
- Translate English comments and implementation terms; do not assume programming knowledge.
- Do not make the user reverse-engineer technical terms or infer missing context.
- Separate the user's requested work, project context, and implementation details. Do not let a script's input format stand in for understanding the user's instruction.
- State uncertainty plainly instead of inventing a reason.
