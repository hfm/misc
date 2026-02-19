---
name: ask-user-question
description: Clarify ambiguous requirements through continuous AskUserQuestion-style interviews. Use when the user says "interview me", "ask me questions about this", or when constraints, priorities, risks, tradeoffs, and acceptance criteria are unclear before implementation.
---

# Ask User Question

Read the given spec or context, then conduct a structured interview with one high-impact question at a time.

## Rules

- Use AskUserQuestionTool when available.
- Ask non-obvious questions that surface hidden assumptions, priorities, constraints, and decision criteria.
- Avoid superficial or already-answered questions.
- Ask one question per turn by default and continue until understanding is sufficient.
- Provide 2 to 4 concrete options when helpful, with the recommended option first.
- If options do not fit, allow free-text input.

## Finish Criteria

Stop interviewing only when assumptions and decisions are clear enough to implement safely.

Then output:

1. Final consolidated spec.
2. Remaining open questions, if any.
3. Next implementation step.

## References

- https://x.com/trq212/status/2005315275026260309/
