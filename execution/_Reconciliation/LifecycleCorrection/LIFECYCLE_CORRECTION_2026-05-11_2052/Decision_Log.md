# Decision Log

## Human Ruling

Human project authority clarified that CHECKING was incorrectly allowed through: the deliverables are not ready for full review and still require substantial development.

## Correction Policy

- `PKG-00` architecture-basis deliverables remain `SEMANTIC_READY`.
- Every non-`PKG-00` deliverable previously in `CHECKING` is corrected to `IN_PROGRESS`.
- No deliverable is promoted to `CHECKING`, `ISSUED`, or any release/professional acceptance state.
- Prior history is preserved; a new correction entry is appended to each corrected `_STATUS.md`.

## Rationale

`CHECKING` must mean genuinely ready for review against deliverable acceptance criteria. The observed state used `CHECKING` for bounded implementation-evidence closeout, deferred scope, pending human review, or committed evidence reconciliation. That overstated maturity.
