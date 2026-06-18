# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverables:

- DEL-02-05 - Project persistence and round-trip serialization
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Report Content Lint inventory slice while C5.7 remains
human-execution gated. The tranche records the Project Validation Preflight
public unit-policy surface in the Report Content Lint explicit public-surface
inventory.

## Changes

- Added `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`
  to the report-lint public-surface roots and explicit target list.
- Added `project-validation-unit-policy` to
  `unit_policy_evidence.target_refs`.
- Increased the visible and exported report-lint unit-policy target count
  from 19 to 20 while leaving target-format conversion-witness count at two.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - Initially caught the missing explicit lint target entry, then passed 1/1
    selected test after the target list was updated.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
  - 1/1 focused Chromium desktop test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
  - Passed 1/1 selected test after an over-broad export-review metadata count
    assertion was restored.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes the Report Content Lint inventory over existing Project
Validation Preflight unit-policy evidence only. It does not change
validation-preflight packet semantics, project persistence semantics,
report-linter protected-content semantics, legal clearance, redaction
controls, target-writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private data,
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-08-05, DEL-02-05, and DEL-02-02. DEC-025 sweep evidence remains to be
recorded during git closeout.
