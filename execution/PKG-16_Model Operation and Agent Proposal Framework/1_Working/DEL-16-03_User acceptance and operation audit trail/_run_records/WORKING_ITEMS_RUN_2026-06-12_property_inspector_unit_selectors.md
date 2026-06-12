# WORKING_ITEMS Run Record - TP-UNITS-B2-UNITPICKERS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-16-03 User acceptance and operation audit trail

## Scope

Verified that selected units on material and section create intents flow
through the existing queue/apply acceptance route:

- The UI queues create intents with selected unit metadata in `change.unit`
  and the JSON payload.
- Applying accepted intents mutates only the local session model and records
  the existing user-initiated local-session acceptance basis.
- Entered units remain visible in the applied model, while incompatible unit
  dimensions remain blocking.

## Evidence

- `apps/desktop/src/App.test.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `core/model_operations/operation_applier/src/lib.rs`
- Corresponding DEL-02-02, DEL-07-02, and DEL-16-02 run records.

## Validation

- Focused desktop Vitest passed 165/165.
- Full desktop Vitest passed 216/216.
- Tauri Rust tests passed 32/32.
- Playwright R2 smoke passed 2/2 after wasm engine build.

## Boundaries

No direct durable persistence, professional approval, release-readiness,
certification, sealing, authentication, approval, code-compliance claim,
protected standards data, private project data, network path, or telemetry
path changed.
