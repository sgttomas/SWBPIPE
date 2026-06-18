---
run_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
tranche_id: TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
smoke_id: TP-MAC-197
agent: WORKING_ITEMS
primary_deliverable: DEL-03-01
supporting_deliverables:
  - DEL-07-03
  - DEL-02-02
status: passed
created: 2026-06-17
---

# TP-UNITS-BTAIL-MATLIBFIELDUNITS-001 — Material Schema-Native Property Unit Drafts

## Scope

Primary data-model evidence for the DEL-07-03 material-library helper. The
work binds the Private Library Manager's draft helper to the existing
material-schema property shape without changing the schema itself.

## Changes

- The helper writes a `material_records[0].properties[]` entry with
  `property_kind`, `value_status=private_user_supplied`,
  `required_for=mechanics_solve`, and a unit-bearing `value` object.
- The value object carries `magnitude`, `unit_ref={ref_type:"Unit", ref_id}`,
  `dimension_id`, `quantity_kind=unit_bearing`, `unit_required=true`,
  `missing_unit_behavior=diagnostic_blocking`, and provenance.
- Default property presets cover density, elastic modulus, and thermal
  expansion coefficient using existing material dimension vocabulary and
  DEC-018 unit refs.

## Validation

Passed:

- `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx` — 13/13.
- `npm test --workspace apps/desktop` — 18/18 files, 395/395 tests.
- `npm run build --workspace apps/desktop` — passed with existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"` —
  2/2 Playwright tests.

## Boundary Review

No public material source/catalog authority, material engineering allowable,
schema enum change, protected-content or redistribution review disposition,
lifecycle promotion, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
