# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVIEWSTORAGEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverables:

- DEL-02-05 - Project persistence and round-trip serialization
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Export Safety Review matrix cleanup while C5.7 remains
human-execution gated. The tranche reclassifies the Project Storage Audit and
Project Validation Preflight export records as unit-evidence-required in the
Export Safety Review manifest because those source packets already carry
DEC-018 unit-policy evidence.

## Changes

- Added `project_storage_audit` and `project_validation_preflight` to the
  export-review manifest's unit-evidence-required export id set.
- Updated the visible `export-review-units` line and downloaded manifest
  assertions from `covered=14/14` to `covered=16/16`.
- Added assertions that both storage/preflight rows carry
  `unit_evidence_status=covered_by_target_panel_or_export_packet`.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes the Export Safety Review unit-evidence inventory only. It
does not change project persistence semantics, validation-preflight semantics,
target-specific writers, manifest-level unit conversion, runtime redaction
rules, public transport commitments, protected standards content, private
payloads, lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-12-02, DEL-02-05, and DEL-02-02. DEC-025 sweep evidence remains to be
recorded during git closeout.
