# WORKING_ITEMS Run Record - TP-UNITS-B2-INSPECTOREDITUNITS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract

## Scope

Extended B2 unit-aware app I/O into existing Property Inspector quantity edits:

- Added selected-unit state and visible unit selectors for material and
  pipe-section sibling-unit quantity fields.
- Browser preview keeps a single model-metadata option and does not synthesize
  a fallback catalog.
- Desktop/Tauri mode can derive options from accepted DEC-018 catalog entries.
- The operation seam accepts generic atomic `{ value, unit }` sibling-quantity
  edit payloads and preserves entered units by updating the sibling `.unit`
  field.
- Node coordinate edits remain value-only; changing project length-unit basis
  is outside this tranche.

## Evidence

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/SMOKE.md` TP-MAC-138
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed: 54 unit tests plus canonical-hash and contract-corpus tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed:
  32 tests, 0 doctests.
- `npm run build:wasm --workspace apps/desktop` passed.
- `npm test --workspace apps/desktop -- --run src/App.test.tsx src/services/unitCatalogService.test.ts src/services/operationContractCorpus.test.ts`
  passed: 165 tests.
- `npm test --workspace apps/desktop -- --run` passed: 216 tests.
- `npm run build --workspace apps/desktop` passed with the pre-existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  passed: 2 tests after wasm engine build.

## Boundaries

This is local technical-preview app-integration evidence only. It does not
create a project-wide unit-system picker, mutate project unit metadata, perform
hidden unit conversion, change import/export unit handling, add rule-pack unit
I/O, add browser fallback catalog data, introduce protected standards content,
ingest private project data, create release-readiness evidence, or make a
professional approval, certification, sealing, authentication, or
code-compliance claim.

## Residual Handoff

B2 still owns broader app unit entry/pickers outside material/section and
primitive-load create/edit forms, imports/exports, and rule-pack unit I/O. B3
still owns broader mixed-unit round-trip and D-04/DEC-026 tolerance corpus
coverage outside the witnesses landed here.
