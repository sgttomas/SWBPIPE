# WORKING_ITEMS Run Record - TP-UNITS-B2-INSPECTOREDITUNITS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-03-01 Material library schema with provenance

## Scope

Bound the Property Inspector material quantity edit surface to B2 unit-aware
I/O:

- Existing material elastic modulus, shear modulus, and thermal expansion
  coefficient edits now expose a selected unit.
- The UI queues atomic `{ value, unit }` payloads for sibling-unit quantity
  fields.
- The operation seam preserves compatible entered material units in the
  returned session model.
- Native regression coverage includes material modulus edited to `MPa` and
  incompatible `mm` rejection for stress.

## Evidence

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/SMOKE.md` TP-MAC-138
- `plans/PLAN_COMPLETION_LOG.md`

## Validation

- Operation-applier cargo suites passed: 54 unit tests plus canonical-hash and
  contract-corpus tests.
- Tauri Rust tests passed: 32/32.
- Focused desktop Vitest passed: 165/165.
- Full desktop Vitest passed: 216/216.
- Desktop build passed with the pre-existing Vite chunk-size warning.
- Playwright R2 smoke passed: 2/2 after wasm engine build.

## Boundaries

No material source catalog, public material data, protected standards content,
code-specific material values, private project data, lifecycle state, review
finding status, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim was changed.

## Residual Handoff

Material library source/catalog policy, private-library management, and
broader B2 import/export/rule-pack unit I/O remain outside this tranche.
