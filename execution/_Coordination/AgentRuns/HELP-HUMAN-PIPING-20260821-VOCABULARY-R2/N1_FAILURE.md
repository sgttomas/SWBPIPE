# N1 terminal failure — integrated review

- Node: `N1` residual component kinds.
- Terminal verdict: `FAILED_INTEGRATED_REVIEW_STOPPED`.
- Accepted basis: branch base `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`; bend seam `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`.
- Manager returns: `N1_ENGINE_RETURN.md` and `N1_UI_RETURN.md`; both focused implementations/checks completed.
- Terminal gate: `N1_INTEGRATED_REVIEW.md` reviewed 100% of the five-file product diff and returned `FAIL` with two actionable findings.

## Findings and disposition

1. New tee/rigid UI drafts infer pipe-role selections from model array order rather than requiring explicit user selection; for tee this silently assigns header/branch engineering roles.
2. UI rejects equal tee header/branch references while the authoritative operation applier accepts and persists them, leaving acceptance behavior inconsistent across the seam.

The current owner steer requires a failed node to land only what passed, record the failure, and stop. A repair dispatch was interrupted before any repair write occurred. The reviewed product hashes remain exactly those frozen in `N1_INTEGRATED_REVIEW.md`. No N1 product file, deliverable status, coverage row, or lifecycle state is authorized for commit in this run.

Row 15 remains `PARTIAL / NOT_CLOSED / PARTIALLY_LANDED`: bend remains the sole landed component kind; tee, reducer, valve, and flange remain. The exact minimum repair/re-review matrix is preserved in `N1_INTEGRATED_REVIEW.md` for a later separately directed iteration.
