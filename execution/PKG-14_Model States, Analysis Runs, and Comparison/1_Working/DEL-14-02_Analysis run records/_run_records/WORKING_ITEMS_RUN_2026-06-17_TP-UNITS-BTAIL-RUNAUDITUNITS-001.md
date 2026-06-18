# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RUNAUDITUNITS-001

- Agent: WORKING_ITEMS
- Date: 2026-06-17
- Package: PKG-14
- Deliverable: DEL-14-02 Analysis run records
- Tranche: Phase B-tail run-audit unit traceability evidence.
- Status: implemented and validated.

## Scope

Bounded Phase B-tail analysis-run audit evidence slice while C5.7 remains
human-execution gated. The desktop Run Audit panel now surfaces unit
traceability for the solved run by deriving a `Unit audit` line from the
already-bound model and result envelope.

## Changes

- Added `data-testid="run-audit-units"` to the Run Audit panel.
- The new audit line reports sorted model unit declarations, result row unit
  symbols in the established result display order, result row count,
  `source=result_envelope`, and `conversion=false`.
- Added focused App and R2 Playwright checks that verify the run-audit unit
  evidence after mechanics preview solve.
- Updated `apps/desktop/SMOKE.md` with TP-MAC-222.

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed 1/1
  file and 56/56 tests.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop --
  e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results,
  report, and viewport overlay"` passed 2/2 focused Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary

This tranche changed run-audit UI evidence only. It did not change the
analysis-run schema, hash canonicalization, solver behavior, result values,
unit conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle state,
release-readiness posture, professional approval, certification, sealing,
authentication, or code-compliance posture.

## Handoff

- Accepted upstream basis: DAG-006 current authority, DEL-14-02 analysis-run
  record contract, DEL-02-02 unit-system contract, and the app's existing
  mechanics result envelope.
- Derivative-package status: App UI evidence, SMOKE entry, and this run record
  are derivative evidence; they are not decomposition truth.
- Closure verdict: implementation and validation evidence recorded for this
  bounded tranche.
- Rerun requirements: rerun the focused App test, focused R2 smoke, full
  desktop Vitest, and desktop build after any change to run-audit, result
  unit ordering, or analysis-run/result envelope binding.
- Remaining blockers: C5.7 remains a human packaged-pass action; C5.8 remains
  dependent on C5.7.
