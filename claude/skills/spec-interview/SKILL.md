---
name: spec-interview
description: >
  Conduct deep-dive interviews on a minimal spec using AskUserQuestion to uncover
  hidden assumptions, constraints, and priorities, then generate a detailed spec.
---

Read $ARGUMENTS and interview me in detail using the AskUserQuestionTool about literally anything: technical implementation, UI & UX, concerns, tradeoffs, etc. but make sure the questions are not obvious.

If $ARGUMENTS is empty, read this @SPEC.md.

Be very in-depth and continue interviewing me continually until it's complete, then write the spec to the file.

---
References:
- https://x.com/trq212/status/2005315275026260309/
