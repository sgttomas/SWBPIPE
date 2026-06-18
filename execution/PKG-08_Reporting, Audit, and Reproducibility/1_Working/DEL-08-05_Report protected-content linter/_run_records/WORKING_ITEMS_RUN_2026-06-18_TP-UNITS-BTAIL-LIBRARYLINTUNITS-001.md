# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-LIBRARYLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverables:

- DEL-07-03 - Material, component, and rule-pack editors
- DEL-03-01 - Material library schema with provenance
- DEL-03-02 - Pipe section and component library schema
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Report Content Lint inventory slice while C5.7 remains
human-execution gated. The tranche records the existing Library Manager
material, section, and component draft unit-helper surfaces in the Report
Content Lint explicit public-surface inventory.

## Changes

- Added `apps/desktop/src/features/library/LibraryManagerPanel.tsx` to the
  report-lint explicit target list.
- Added `library-unit-helper-surfaces` to
  `unit_policy_evidence.target_refs`.
- Increased the visible and exported report-lint unit-policy target count from
  37 to 38 while leaving target-format conversion-witness count at two.
- Increased static report-lint target count from 41 to 42; solved report
  packet target count moves from 42 to 43 because the generated preview-report
  JSON target is still appended only after a solve.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- LibraryManagerPanel.test.tsx -t "unit|library|desktop"`
  - 9/9 selected tests passed.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|library manager"`
  - 4/4 focused configured-project tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files and 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.
- `git diff --check`
  - Passed.

## Boundary

This tranche changes the Report Content Lint inventory over existing Library
Manager draft unit-helper surfaces only. It does not change library schemas,
library import storage, private-library payload handling, rule-pack behavior,
report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private data,
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-08-05, DEL-07-03, DEL-03-01, DEL-03-02, and DEL-02-02. DEC-025 sweep
evidence remains to be recorded during git closeout.
