# WORKING_ITEMS Run Record - TP-UNITS-B2-UNITPICKERS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract

## Scope

Retrofitted the first visible unit-entry controls onto app authoring fields
that already had B2 unit-basis labels:

- Property Inspector section creation now carries a `Length unit` selector.
- Property Inspector material creation now carries `Modulus unit` and
  `Thermal expansion unit` selectors.
- Browser preview keeps explicit model-metadata-only selectors and does not
  synthesize a fallback catalog.
- Desktop/Tauri mode derives options from accepted DEC-018 catalog entries.
- `core/model_operations/operation_applier` now accepts compatible DEC-018
  entered units for create-section length quantities and create-material
  stress / thermal-expansion quantities.

## Evidence

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/Cargo.toml`
- `apps/desktop/SMOKE.md` TP-MAC-134
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed: 51 unit tests, canonical hash parity, contract corpus, 0 doctests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed:
  32 tests, 0 doctests.
- `npm test --workspace apps/desktop -- --run src/App.test.tsx src/services/operationContractCorpus.test.ts src/services/unitCatalogService.test.ts`
  passed: 165 tests.
- `npm run build:wasm --workspace apps/desktop` passed.
- `npm test --workspace apps/desktop -- --run` passed: 216 tests.
- `npm run build --workspace apps/desktop` passed with the pre-existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  passed: 2 tests after wasm engine build.

## Boundaries

This is local technical-preview app-integration evidence only. It does not
create a project-wide unit-system picker, unit conversion for imports/exports,
rule-pack unit I/O, browser fallback unit catalog, protected standards
content, private project data, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim.

## Residual Handoff

B2 still owns broader app unit entry/pickers outside material/section create
forms, report renderer body expansion beyond packet disclosure,
imports/exports, and rule-pack unit I/O. B3 still owns broader mixed-unit
round-trip and D-04/DEC-026 tolerance corpus coverage.
