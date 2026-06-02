---
name: lazy_evaluation_conditionals
description: Ensure conditional functions (e.g., if) in formula evaluation engines use lazy (short-circuiting) evaluation rather than eager evaluation, preventing infinite recursion in user-defined functions.
globs: "**/src/eval.rs"
---

# Lazy Evaluation for Conditionals in bl1z

## Context
When implementing conditional functions (like `if(cond, then_branch, else_branch)`) in formula evaluation engines that support user-defined recursive functions, eager evaluation of all function arguments will cause infinite recursion and overflow the recursion limit.

## Pattern / Rule
- Intercept the conditional function (e.g., `if`) before evaluating its arguments.
- Evaluate only the condition argument first.
- Based on the condition's boolean value, evaluate and return only the matching branch (`then_branch` or `else_branch`).
- Do not evaluate the other branch. This prevents recursive base cases from evaluating the recursive path endlessly.
