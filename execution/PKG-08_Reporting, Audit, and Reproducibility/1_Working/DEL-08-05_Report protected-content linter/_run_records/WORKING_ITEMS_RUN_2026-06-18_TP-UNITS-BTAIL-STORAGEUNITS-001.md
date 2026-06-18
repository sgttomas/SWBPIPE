# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-STORAGEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-02-05 - Project persistence and round-trip serialization

Supporting deliverable: DEL-08-05 - Report protected-content linter

Additional supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Supporting DEL-08-05 evidence for a bounded Phase B-tail Project Storage
Audit unit-policy slice. Because the storage audit adds a public unit-policy
surface, the Report Content Lint inventory was updated to include that
surface.

## Changes

- Added `apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx`
  to the explicit public report-lint target list.
- Added `project-storage-unit-round-trip` to the report-lint unit-policy
  surface markers.
- Report-lint visible and exported unit-policy target count increased from
  17 to 18.
- Target-format conversion-witness count remains two and the linter still
  reports `lint_conversion=false`.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
  - 1/1 selected test passed.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --project=chromium-desktop`
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

This tranche changes report-lint public-surface inventory evidence only. It
does not change report-linter protected-content semantics, legal clearance,
redaction controls, target writer compatibility, unit-conversion APIs,
DEC-018 catalog constants, schema dimension enums, protected standards
content, private payloads, lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This supporting report-lint inventory evidence is complete for the
project-storage audit slice. DEC-025 sweep evidence remains to be recorded
during git closeout.
