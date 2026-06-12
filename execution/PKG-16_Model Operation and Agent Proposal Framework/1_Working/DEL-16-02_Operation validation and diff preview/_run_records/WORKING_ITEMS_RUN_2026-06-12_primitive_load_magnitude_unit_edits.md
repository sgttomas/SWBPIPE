# WORKING_ITEMS Run Record - TP-UNITS-B2-PRIMEDITUNITS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-16-02 Operation validation and diff preview

## Scope

Extended the structured operation seam for existing primitive-load magnitude
edits:

- `primitive_loads.N.magnitude.value` still accepts legacy numeric-string
  edits for existing corpus compatibility.
- The same path now accepts `{ value, unit }` JSON payloads from the Load Cases
  manager.
- Compatible entered units are checked against the declared primitive-load
  dimension through DEC-018 catalog bindings.
- Apply writes both `magnitude.value` and sibling `magnitude.unit` on the
  returned session model.
- Incompatible units remain blocking with
  `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.

## Evidence

- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md` TP-MAC-137
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed: 53 unit tests plus canonical-hash and contract-corpus tests.
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
mutate the input model, add hidden cascades, persist accepted model truth,
perform import/export conversion, introduce protected/private data, or create
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual Handoff

Broader unit I/O, import/export conversion, rule-pack unit I/O, and
tolerance-corpus expansion remain outside this tranche.
