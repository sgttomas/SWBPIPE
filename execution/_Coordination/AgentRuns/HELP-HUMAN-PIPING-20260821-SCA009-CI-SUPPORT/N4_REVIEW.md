# N4 independent code review

- Reviewer: fresh `TASK + software-code-review` Agent 2
- Reviewer instance: `n4_independent_review`
- Model: current GPT-5.6 Codex runtime; no substitution
- Authorization: read-only; no files written
- Frozen diff: `N4_FROZEN_DIFF.md`
- Coverage: 100% of both hunks and all 77 changed lines
- Verdict: **PASS**
- Actionable findings: none

The reviewer confirmed that named `line_stop` and `vertical_support` preview
inputs emit solver `SupportFamily::LineStop` and
`SupportFamily::VerticalSupport`, respectively. The explicit restraint DOFs
are preserved, the existing six-DOF-to-Anchor and partial-to-Guide fallbacks
remain intact, and spring, nonlinear, and constant-effort routing is
unchanged. Solver validation continues to reject invalid named-family DOFs
rather than silently changing their meaning.

Residual risk is low and non-actionable: the focused tests exercise the
extracted mapping helper rather than a serialized full-preview request. The
parser and sole call site are unchanged apart from routing through that helper.
