---
run_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-SUPPORTUNITS-001
primary_deliverable: DEL-07-02
package: PKG-07
status: SUCCESS
created: 2026-06-17
---

# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SUPPORTUNITS-001

## Scope

Bounded Phase B-tail app-integration slice for support unit I/O while C5.7
remains human-execution gated. The tranche extends the existing Property
Inspector support creation form with explicit linear-stiffness unit/value
entry and verifies local-session apply/readback.

Primary deliverable: `DEL-07-02 Model tree and property inspector`.

Supporting deliverables/evidence:

- `DEL-02-02 Unit system and dimensional-analysis core contract`
- `core/model_operations/operation_applier`
- `apps/desktop/SMOKE.md` TP-MAC-191
- active completion plan Phase B-tail row
- PRD FR-002 unit-aware I/O coverage

## Files Changed

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/features/model-workspace/modelView.ts`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/App.test.tsx`
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/SMOKE.md`
- `plans/PLAN_2026-06-17_prd_completion.md`
- `plans/PLAN_COMPLETION_LOG.md`
- DEL-07-02 and DEL-02-02 `MEMORY.md`
- this run record and the DEL-02-02 supporting run record

## Implementation

- Added a `Linear stiffness unit` selector and `Linear stiffness` value input
  to the `Create support` intent panel.
- Browser preview uses model metadata (`N/m` derived from project force/length)
  and does not synthesize a DEC-018 fallback catalog.
- When the stiffness value is blank, support creation preserves the prior
  dimensionless support payload and operation metadata.
- When the stiffness value is entered, the queued `create_support` intent
  includes `properties.linear_stiffness: {value, unit}` with
  `dimension=linear_stiffness` and the entered unit.
- Selected support readback now displays the preserved `Linear stiffness`
  value/unit after local session apply.

## Validation

Passed:

- `npm run -w apps/desktop build:wasm`
- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 61 unit tests passed
  - canonical-hash parity test passed
  - contract-corpus tests passed
- `npm run -w apps/desktop test -- --run src/App.test.tsx`
  - 55 tests passed
- `npm test --workspace apps/desktop`
  - 18 test files passed
  - 390 tests passed
- `npm run build --workspace apps/desktop`
  - passed with the existing Vite large-chunk warning

## Evidence Notes

- `apps/desktop/SMOKE.md` adds TP-MAC-191.
- `plans/PLAN_COMPLETION_LOG.md` records the partial B-tail landing.
- `plans/PLAN_2026-06-17_prd_completion.md` keeps B-tail open and records
  this slice as partial.

## Boundary

Local preview support authoring only. This tranche does not change durable
persistence semantics, release tolerance policy, solver validation status,
protected standards content, private project data handling, network/telemetry
behavior, lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance status.
