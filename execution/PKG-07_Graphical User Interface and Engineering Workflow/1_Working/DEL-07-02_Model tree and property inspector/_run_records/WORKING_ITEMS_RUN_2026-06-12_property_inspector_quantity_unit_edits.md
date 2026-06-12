# WORKING_ITEMS Run Record - TP-UNITS-B2-INSPECTOREDITUNITS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-07-02 Model tree and property inspector

## Scope

Extended the Property Inspector's existing edit panel for sibling-unit
quantities:

- Added selected-unit state for editable material and pipe-section quantities.
- Added a visible `Unit` selector and value label basis for material modulus,
  material thermal expansion, pipe outside diameter, and pipe wall thickness.
- Queued atomic `{ value, unit }` payloads through the existing structured
  editor intent preview.
- Kept node coordinate edits value-only because project-unit-system mutation is
  outside this tranche.

## Evidence

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md` TP-MAC-138
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx src/services/unitCatalogService.test.ts src/services/operationContractCorpus.test.ts`
  passed: 165 tests.
- `npm test --workspace apps/desktop -- --run` passed: 216 tests.
- `npm run build --workspace apps/desktop` passed with the pre-existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  passed: 2 tests after wasm engine build.
- Adjacent Rust evidence: operation-applier cargo suites passed and Tauri Rust
  tests passed 32/32.

## Boundaries

This UI slice does not mutate accepted model state directly, create a
project-wide unit picker, mutate project unit metadata, synthesize browser
catalog data, persist private project data, introduce protected standards
content, or make release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claims.

## Residual Handoff

Broader app unit entry/pickers outside material/section/primitive-load
create/edit forms, imports/exports, and rule-pack unit I/O remain outside this
tranche.
