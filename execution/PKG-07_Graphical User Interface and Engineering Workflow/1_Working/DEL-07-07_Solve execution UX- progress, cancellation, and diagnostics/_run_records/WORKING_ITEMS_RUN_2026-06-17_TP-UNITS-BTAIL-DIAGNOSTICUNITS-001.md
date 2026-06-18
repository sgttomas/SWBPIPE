# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-DIAGNOSTICUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-07 - Solve execution UX: progress, cancellation, and diagnostics

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail diagnostic review-surface slice while C5.7 remains
human-execution gated. The tranche exposes unit traceability for result rows
linked to a selected diagnostic in the desktop Diagnostic Detail panel.

## Changes

- Added first-class `unit` and `unit_source` fields to diagnostic linked-result
  interpretation records.
- Added `data-testid="diagnostic-unit-context"` to the Diagnostic Detail panel.
- The unit-context line reports linked result count, linked result units,
  `source=result_envelope`, and `conversion=false`.
- Added focused App and Playwright assertions for
  `HIGH_DISPLACEMENT_REVIEW`, where the selected node-level diagnostic links
  21 result rows with units `mm,rad`.
- Updated `apps/desktop/SMOKE.md` TP-MAC-224, the completion log, and the
  active completion-plan B-tail pointer.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "links selected diagnostics to affected result and model context"`
  - 1/1 selected test passed.
- `npx playwright test e2e/r2-smoke.spec.ts -g "diagnostic detail exposes linked result unit context"`
  - 2/2 focused Playwright tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npx playwright test e2e/r2-smoke.spec.ts`
  - 18/18 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.

Command errors excluded from validation evidence:

- `npm run test --workspace apps/desktop -- --runInBand apps/desktop/src/App.test.tsx -t "links selected diagnostics to affected result and model context"`
  failed before test execution because Vitest does not support the Jest
  `--runInBand` option.
- `npm run test --workspace apps/desktop -- apps/desktop/src/App.test.tsx -t "links selected diagnostics to affected result and model context"`
  failed before test execution because the path was not package-local.

## Boundary

This tranche changes diagnostic review UI evidence and interpretation metadata
only. It does not change diagnostic schemas, solver behavior, result values,
unit conversion APIs, DEC-018 catalog constants, schema dimension enums,
protected standards content, private payloads, lifecycle state, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-07-07 and DEL-02-02. DEC-025 sweep evidence remains to be recorded during
git closeout.
