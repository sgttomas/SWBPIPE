# WORKING_ITEMS Run Record - TP-UNITS-B2-INSPECTOREDITUNITS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-16-02 Operation validation and diff preview

## Scope

Generalized the structured operation seam for sibling-unit quantity edits:

- Legacy numeric-string quantity edits remain valid when the intent unit equals
  the stored unit.
- Sibling-unit quantity paths now also accept `{ value, unit }` JSON payloads.
- The payload unit must match the intent unit.
- Compatible non-stored units are accepted only when the same validated edit
  writes the sibling `.unit` field.
- Apply writes both the target `.value` and sibling `.unit` on the returned
  session model.
- Incompatible units remain blocking with
  `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.

## Evidence

- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md` TP-MAC-138
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed: 54 unit tests plus canonical-hash and contract-corpus tests.
- `npm run build:wasm --workspace apps/desktop` passed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed:
  32 tests, 0 doctests.
- Focused desktop Vitest passed: 165 tests.
- Full desktop Vitest passed: 216 tests.
- Desktop build passed with the pre-existing Vite chunk-size warning.
- Playwright R2 smoke passed: 2 tests after wasm engine build.

## Boundaries

This is local technical-preview operation-seam evidence only. It does not
mutate the input model, create hidden fallback units, mutate project unit
metadata, persist accepted model truth, perform import/export conversion,
introduce protected/private data, or create release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Residual Handoff

Broader unit I/O, import/export conversion, rule-pack unit I/O, and
tolerance-corpus expansion remain outside this tranche.
