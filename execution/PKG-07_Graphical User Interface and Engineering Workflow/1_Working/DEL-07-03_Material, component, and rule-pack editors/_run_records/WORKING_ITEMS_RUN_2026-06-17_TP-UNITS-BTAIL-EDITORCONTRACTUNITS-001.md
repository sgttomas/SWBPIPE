---
doc_id: WORKING-ITEMS-RUN-2026-06-17-TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001-DEL-07-03
doc_kind: working_items.run_record
status: completed
created: 2026-06-17
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001
smoke_id: TP-MAC-227
deliverable_id: DEL-07-03
package_id: PKG-07
role: primary
---

# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001

## Scope

Bounded Phase B-tail editor-contract unit-contract slice while C5.7 remains
human-execution gated. The tranche makes the existing DEL-02-02 unit contract
visible in the DEL-07-03 Editor Contract review panel.

## Implementation

- Added `editor-contract-unit-contract` to
  `apps/desktop/src/features/editor-contract/EditorContractPanel.tsx`.
- The visible row reports the exported packet's existing `unit_contract`
  fields:
  - `contract=DEL-02-02`;
  - `schema=schemas/units.schema.yaml#/$defs/DimensionId`;
  - `policy=unit_bearing_values_require_explicit_unit_metadata`;
  - `missing=diagnostic_blocking`.
- Added App-level assertions that the visible row and downloaded Editor
  Contract JSON packet agree.
- Added a focused Chromium desktop journey assertion for the same visible row.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- `npx playwright test e2e/r2-smoke.spec.ts -g "guided workbench shell keeps journey steps, details, and compact status reachable" --project=chromium-desktop` passed 1/1 focused Chromium desktop test.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary Review

No editor persistence, accepted model-state mutation, operation-applier
behavior, solver behavior, unit conversion API, DEC-018 catalog constant,
schema dimension enum, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.
