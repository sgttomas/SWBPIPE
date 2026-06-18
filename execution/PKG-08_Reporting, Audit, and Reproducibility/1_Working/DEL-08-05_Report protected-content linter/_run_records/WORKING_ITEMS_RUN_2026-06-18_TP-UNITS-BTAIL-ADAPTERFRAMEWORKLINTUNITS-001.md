# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverables:

- DEL-10-02 - Import-export adapter framework
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Report Content Lint inventory slice while C5.7 remains
human-execution gated. The tranche records the Adapter Framework public
unit-policy surface in the Report Content Lint explicit public-surface
inventory.

## Changes

- Added `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`
  to the report-lint public-surface roots and explicit target list.
- Added `adapter-framework-units` to `unit_policy_evidence.target_refs`.
- Increased the visible and exported report-lint unit-policy target count from
  24 to 25 while leaving target-format conversion-witness count at two.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  - 2/2 focused configured-project tests passed.
- `npm run test --workspace apps/desktop -- DeclarationsEditor.test.tsx -t "keeps an out-of-catalog stored unit visible as current instead of snapping it"`
  - 1/1 selected test passed after the first full-suite attempt surfaced this
    test as a transient timing failure.
- `npm run test --workspace apps/desktop`
  - Full rerun passed: 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes the Report Content Lint inventory over existing Adapter
Framework unit-policy evidence only. It does not change adapter-framework
packet semantics, schema, concrete external format list, public transport,
plugin runtime, permission persistence, package scripts, CI/release matrix,
report-linter protected-content semantics, legal clearance, redaction
controls, target-writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private data,
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-08-05, DEL-10-02, and DEL-02-02. DEC-025 sweep evidence remains to be
recorded during git closeout.
