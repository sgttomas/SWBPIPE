# WORKING_ITEMS Run Record - TP-UNITS-B2-UNITPICKERS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-16-02 Operation validation and diff preview

## Scope

Extended the structured operation applier's create-section and create-material
validation to accept DEC-018-compatible entered units:

- create-section validates length quantities through `core/units` and checks
  wall-thickness geometry after converting both values to the project length
  basis for comparison.
- create-material validates stress and thermal-expansion coefficient
  quantities through `core/units`.
- Applied records preserve the entered units instead of forcing project-unit
  storage.
- Incompatible dimensions still block with explicit operation diagnostics.

## Evidence

- `core/model_operations/operation_applier/Cargo.toml`
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/Cargo.lock`
- `apps/desktop/src-tauri/Cargo.lock`
- Corresponding DEL-02-02, DEL-07-02, and DEL-16-03 run records.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed: 51 unit tests, canonical hash parity, contract corpus, 0 doctests.
- `npm run build:wasm --workspace apps/desktop` passed.
- Focused desktop Vitest passed 165/165.
- Full desktop Vitest passed 216/216.
- Playwright R2 smoke passed 2/2 after wasm engine build.

## Boundaries

The seam still rejects incompatible units and does not invent defaults,
perform protected-content lookup, persist accepted model state by itself, or
make release, professional, certification, sealing, authentication, approval,
or code-compliance claims.
