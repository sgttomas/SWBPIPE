---
run-id: WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels
timestamp: 2026-06-12T03:12:16-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2-INSPECTORLABELS-001
package-id: PKG-02
deliverable-id: DEL-02-02
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/CONTRACT.md OPS-K-UNIT-1
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2
write-scope:
  - apps/desktop/src/services/unitCatalogService.ts
  - apps/desktop/src/services/unitCatalogService.test.ts
  - apps/desktop/src/features/model-tree/PropertyInspector.tsx
  - apps/desktop/src/App.test.tsx
  - apps/desktop/e2e/r2-smoke.spec.ts
  - apps/desktop/SMOKE.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/MEMORY.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/**
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/**
  - plans/PLAN_2026-06-10_prd_completion.md
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-UNITS-B2-INSPECTORLABELS-001

## Scope

Implemented the first visible B2 unit display retrofit in the desktop
Property Inspector. The tranche connects material and section authoring field
labels to catalog-aware unit basis display, while preserving the existing
model-unit payloads and deferring picker/conversion semantics.

## Changes

- Extended `unitCatalogService.ts` with `unitCatalogEntryForSymbol` and
  `describeUnitBasis`.
- The lookup matches accepted DEC-018 catalog entries by symbol and dimension
  and handles display-only dimension equivalents such as `stress` through
  pressure units. It reports catalog misses and browser-preview unavailability
  explicitly instead of normalizing or synthesizing units.
- Updated the Property Inspector to load the unit catalog route once and show
  a compact `Unit basis` status panel.
- Updated material and pipe-section creation labels to show their unit basis
  source, e.g. `m, model metadata` in browser preview or the DEC-018 catalog
  basis in desktop/Tauri mode.
- Added Vitest and Playwright assertions for the browser-preview visible unit
  basis behavior.

## Validation

- `npm test --workspace apps/desktop -- --run src/services/unitCatalogService.test.ts src/App.test.tsx`
  - PASS, 48 tests across 2 files.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  - PASS, 2 Playwright tests after wasm engine build.
- `npm test --workspace apps/desktop` - PASS, 216 tests across 9 files.
- `npm run build --workspace apps/desktop` - PASS with the pre-existing Vite
  chunk-size warning.

## Boundary Review

- This tranche does not add a unit picker, unit conversion, solver-boundary
  normalization, report renderer unit-system disclosure, import/export unit
  handling, or rule-pack unit I/O.
- Browser preview mode still does not synthesize a fallback unit catalog.
- Existing authoring payload units are preserved from the model document; no
  hidden unit default, engineering default, protected standards content,
  proprietary vendor data, private project data, material allowable, SIF, or
  flexibility factor was added.
- No lifecycle issuance, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim is made.

## Handoffs

- B2 still owns broader visible unit entry/pickers, solver-boundary
  normalization, report unit-system disclosures, imports/exports, and
  rule-pack unit I/O.
- B3 still owns broader mixed-unit round-trip, conversion-witness,
  incompatible-unit rejection, and D-04/DEC-026 tolerance corpus coverage.
