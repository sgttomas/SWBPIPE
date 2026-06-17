---
run_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-SUPPORTUNITS-001
supporting_deliverable: DEL-02-02
package: PKG-02
status: SUCCESS
created: 2026-06-17
---

# WORKING_ITEMS Supporting Run Record - TP-UNITS-BTAIL-SUPPORTUNITS-001

## Scope

Supporting DEL-02-02 evidence for the support linear-stiffness unit I/O slice.
The primary app-integration record lives under DEL-07-02.

## Contract Change

- `core/model_operations/operation_applier` support creation now accepts two
  explicit operation metadata shapes:
  - bare support restraint creation: `unit=none`,
    `dimension=dimensionless`, no `properties.linear_stiffness`;
  - support stiffness creation: an accepted DEC-018 linear-stiffness unit,
    `dimension=linear_stiffness`, and a positive
    `properties.linear_stiffness` quantity.
- Invalid or mismatched stiffness payloads block with explicit diagnostics.
- Entered units are preserved in the applied local session model.

## Validation

Passed:

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 61 unit tests passed
  - canonical-hash parity test passed
  - contract-corpus tests passed
- `npm run -w apps/desktop build:wasm`
- `npm run -w apps/desktop test -- --run src/App.test.tsx`
  - 55 tests passed
- `npm test --workspace apps/desktop`
  - 18 test files passed
  - 390 tests passed
- `npm run build --workspace apps/desktop`
  - passed with the existing Vite large-chunk warning

## Boundary

This supporting slice does not introduce a new DEC-018 catalog constant,
dimension enum, hidden unit fallback, tolerance policy, protected standards
content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim.
