# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverable: DEL-07-07 - Solve execution UX: progress,
cancellation, and diagnostics

## Scope

Supporting DEL-07-07 evidence for the bounded Phase B-tail Report Content Lint
inventory slice. The tranche records the existing Solve execution
`solve-job-unit-policy` surface in the DEL-08-05 public lint inventory.

## Changes

- Added `apps/desktop/src/features/solve/SolvePanel.tsx` to the report-lint
  public-surface roots and target list.
- Added `solve-job-unit-policy` to the lint packet
  `unit_policy_evidence.target_refs`.
- Left solve execution behavior, analysis-run hash generation,
  cancellation/progress semantics, backend job behavior, and solve-job unit
  evidence behavior unchanged.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  - 2/2 focused configured-project tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files and 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.

## Boundary

Report-lint inventory over existing Solve execution solve-job unit-policy
evidence only. No solve execution behavior, analysis-run hash generation,
cancellation/progress semantics, backend job behavior, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, protected standards content,
private data, lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance posture changed.

## Handoff

C5.7 remains human-execution gated. This supporting DEL-07-07 evidence can be
consumed by DEL-08-05 and DEL-02-02 inventory/units review. DEC-025 sweep
evidence remains to be recorded during git closeout.
