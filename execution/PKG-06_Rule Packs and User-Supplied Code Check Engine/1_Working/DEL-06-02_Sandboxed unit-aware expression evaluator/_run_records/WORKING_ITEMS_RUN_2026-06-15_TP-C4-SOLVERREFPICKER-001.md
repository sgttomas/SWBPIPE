---
run-id: WORKING_ITEMS_RUN_2026-06-15_TP-C4-SOLVERREFPICKER-001
timestamp: 2026-06-15T14:40:18-0600
completed: 2026-06-15T14:40:18-0600
run-status: SUCCESS
closeout: VALIDATED_PENDING_CHANGE
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-SOLVERREFPICKER-001 — run-panel preview for authored `solver_result_ref`

## Tranche and authority basis

- Tranche: completion-plan Phase C4 residual, "a run-panel resolution preview
  for `solver_result_ref` (mirrors `TP-C3-LIBREFPICKER-001`)."
- Selection: earliest unblocked R3/Phase C follow-up after
  `TP-C4-SOLVERREFAUTHOR-001`. Unblocked because `solver_result_ref` was already
  ratified by `DEC-039` and backend resolution landed in `TP-C4-SOLVERREF-001`.
- Scope: frontend-only app integration in the run-rule-checks panel and its
  tests. No schema, Rust backend, Python, lifecycle, issuance, or decision
  register changes.

## Changes

- `apps/desktop/src/services/ruleCheckService.ts`
  - `deriveRuleCheckBindingPlan` now preserves optional
    `solver_result_ref: { result_id }` on `solver_result` required inputs.
  - Added pure `classifySolverResultReference`, returning `resolves`,
    `result_missing`, or `no_result_rows` against a solved envelope's result
    rows.
- `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`
  - A solver input with authored `solver_result_ref` now renders a read-only
    preview block instead of the manual result-row selector.
  - The preview checks whether the authored `result_id` exists in the current
    solved envelope, lists available row ids when rows exist, and surfaces that
    unresolved references block at `RULE_INPUTS_INCOMPLETE`.
  - `buildSolverBindings` omits caller-supplied selector bindings for inputs
    whose in-pack `solver_result_ref` is canonical. Packs without the member keep
    the previous selector behavior.
- `apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx`
  - Added coverage for a resolving authored reference, a missing authored
    reference, and the backend invocation shape with no fallback selector.
- `apps/desktop/e2e/r2-smoke.spec.ts`
  - Extended the run-rule-checks browser smoke to assert the visible preview
    path and the honest `no_result_rows` browser-preview state before a solve is
    available. The resolving solved-envelope case is covered in component tests.

## Evidence

- `npm test --workspace apps/desktop -- RuleCheckRunPanel ruleCheckService`:
  **27/27 pass**.
- `npm test --workspace apps/desktop`: **378/378 pass**.
- `npm run build --workspace apps/desktop`: clean (`tsc -b` + Vite production
  build; pre-existing chunk-size advisory only).
- `npm run test:e2e --workspace apps/desktop -- -g "run-rule-checks panel"`:
  first attempt failed because the test expected `resolves` in browser preview;
  the product state actually had no solved rows, so the preview correctly
  emitted `no_result_rows`. The test was corrected to assert that honest state.
  Rerun: **2/2 pass** across configured desktop and compact Chromium viewports.

## Boundary compliance

- Local-only frontend behavior; no network, daemon, telemetry, or repository
  private-data writes.
- The preview carries only a result-row id and current-envelope row metadata; it
  never embeds a solver result value into the rule pack and never mutates the
  pack.
- Status-vocabulary-only: unresolved references block at
  `RULE_INPUTS_INCOMPLETE`; no compliance, certification, sealing,
  authentication, approval, code-compliance, release-readiness, or professional
  acceptance claim is made.
- Deliverables remain `CHECKING`.

## Residuals and hand-offs

- C4's named `solver_result_ref` preview residual is closed.
- Remaining R3-stage blockers for exit review are unchanged: F-4 completed
  human packaged-GUI journey and the A3 authoring-journey usability residual.
- Next ordinary unblocked plan item after Phase C closure is the in-stage
  parallel lane already named by coordination: continue the A3 usability lane or
  the Phase B B2/B3 unit-aware I/O remainder, subject to the next session's
  state discovery.

## Open decisions awaiting human ruling

- None introduced by this slice. Standing not-yet-prepared decision packets are
  unchanged: `D-06`, `D-11`, `D-12`, plus deferred `D-04b`, `D-05b`, `D-07b`,
  and `D-10b`.
