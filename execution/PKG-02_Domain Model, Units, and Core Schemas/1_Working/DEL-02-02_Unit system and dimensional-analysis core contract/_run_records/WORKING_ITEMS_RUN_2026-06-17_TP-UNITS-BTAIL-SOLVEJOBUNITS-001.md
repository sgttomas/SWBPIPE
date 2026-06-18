# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SOLVEJOBUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-07 - Solve execution UX: progress, cancellation, and diagnostics

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Supporting DEL-02-02 evidence for a bounded Phase B-tail solve-job audit
slice. The tranche exposes the unit-system and unit-preservation basis already
available in model/result metadata through the Solve execution solve-job
audit surface.

## Changes

- The Solve execution panel now includes `solve-job-unit-policy`.
- The solve-job JSON packet now includes `unit_policy_evidence`.
- Evidence records DEC-018 unit-system reference, sorted model units, solved
  result units or `results=none`, result row count, storage convention,
  analysis-run ref, DEL-02-02/DEL-07-07 basis refs, and
  `conversion_performed=false`.

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

This tranche does not change DEC-018 catalog constants, schema dimension
enums, unit-conversion APIs, solve execution behavior, solver backend seam,
result values, analysis-run hashes, cancellation behavior, protected
standards content, private payloads, lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This supporting unit evidence is complete for the solve-job audit slice.
DEC-025 sweep evidence remains to be recorded during git closeout.
