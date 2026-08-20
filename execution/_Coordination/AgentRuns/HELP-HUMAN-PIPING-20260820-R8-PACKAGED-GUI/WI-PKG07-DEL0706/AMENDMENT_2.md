# Amendment 2 — packaged solve-proof observability remediation

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI` |
| InstanceID | `WI-PKG07-DEL0706` |
| Attempt | `2`, in-place remediation |
| Detection layer | actual packaged GUI post-reopen solve observation |
| Failure class | in-scope product observability gap |
| Reason code | `SOLVE_BACKEND_IDENTITY_NOT_RELIABLY_VISIBLE` |
| Status | `ACTIVE` |

The packaged GUI visibly proved restored `425 N`, zero pending edit, cleared
pre-solve state, `MECHANICS_SOLVED`, and 830 result rows. The explicit backend
job seam and result/model identity binding already exist in live application
state (`solveJob.backend_job_seam`, `model.project.id`, `result.model_ref`, and
`result.results.length`) but were not reliably visible or accessibility-
discoverable in the packaged journey because they were buried in the long Solve
audit surface.

Authorized remediation: without changing schemas, service contracts, solver
behavior, persistence, or cross-package truth, render one compact, always-
visible and accessibility-readable solve-proof status row in the global
workspace status after a result exists. It must name the backend seam, project
ID, result model ID, explicit identity match/mismatch, and result-row count.
Add durable Vitest coverage, run the registered desktop checks, rebuild the
packaged `.app`, and repeat the post-reopen solve/observation necessary to
capture every predicate.

Because this changes `apps/desktop/src/**` product behavior, the terminal
integrated diff must be frozen and reviewed by a fresh read-only non-delegating
`TASK + software-code-review` child over 100% of the product and evidence diff.
Any actionable finding requires remediation and a fresh re-review. Attempt-1
and pre-remediation attempt-2 observations remain preserved.

No change to the N1 objective, allowed write boundary, receipt/Git ownership,
CI proof surface, lifecycle, release, professional-reliance, accessibility-
conformance, certification, sealing, authentication, or code-compliance claims
is authorized. The new PR-label-gated unsigned-artifact CI surface is not
declared proof for this host-operated node and remains non-applicable.
