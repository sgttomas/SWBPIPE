# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SOLVEJOBUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-07 - Solve execution UX: progress, cancellation, and diagnostics

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail solve-job audit slice while C5.7 remains
human-execution gated. The tranche exposes unit-policy evidence for the
Solve execution panel's solve-job audit row and downloaded JSON packet.

## Changes

- Added `data-testid="solve-job-unit-policy"` to the Solve execution audit
  list.
- Added `unit_policy_evidence` to the solve-job JSON packet.
- The visible row and packet report sorted model units, solved result units
  or `results=none`, result row count, DEC-018 and DEL-02-02/DEL-07-07
  basis refs, storage convention, analysis-run ref, and
  `conversion=false`.
- Added focused App assertions for not-started and solved solve-job states.
- Added R2 Playwright assertions for solved preview output and from-blank
  zero-result output.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
  - 1/1 selected test passed.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
  - 2/2 focused Playwright tests passed.
- `git diff --check`
  - Passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes solve-job audit UI/export evidence only. It does not
change solve execution behavior, solver backend seam, result values,
analysis-run hashes, cancellation behavior, unit-conversion APIs, DEC-018
catalog constants, schema dimension enums, protected standards content,
private payloads, lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-07-07 and DEL-02-02. DEC-025 sweep evidence remains to be recorded during
git closeout.
