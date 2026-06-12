# WORKING_ITEMS Run Record - TP-APP-R2-DELNODE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add explicit node deletion, `delete_node`, to the structured operation
validation/apply seam and close the remaining A11 entity-deletion family.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_56_accept_delete_node.json`
- `fixtures/model_operations/contract_corpus/case_57_block_delete_node_referenced.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_node` as a supported delete-kind operation.
- Implemented validation for whole-node deletion: target `object_type=Node`,
  `field_path=nodes`, `after=not_present`, current node
  label/x/y/z before-state display, unit `none`, and dimension
  `dimensionless`.
- Added reference blocking for nodes still used by pipe endpoints, supports,
  component symbols, or primitive nodal loads, emitting
  `OP-NODE-DELETE-REFERENCED` with deterministic affected refs.
- Added `apply_deleted_node` so successful apply removes exactly one
  unreferenced node and returns a new model document without mutating the
  input model in place.
- Added engine unit tests for accepted node deletion, stale before-value
  blocking, missing-target blocking, and referenced-entity blocking.
- Added contract corpus cases `case_56_accept_delete_node.json` and
  `case_57_block_delete_node_referenced.json`, then updated both Rust and
  browser corpus coverage floors to require accepted `delete_node` and
  `OP-NODE-DELETE-REFERENCED` block coverage.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
- Focused Rust test:
  `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml delete_node`
  - 2/2 focused node-deletion tests passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the two new case expectations from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - 57-case contract corpus passed
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- operationContractCorpus`
  - 117/117 passed across browser adapter and direct wasm lanes
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 49 unit tests passed
  - canonical hash parity passed
  - 57-case contract corpus passed
- `npm test --workspace apps/desktop`
  - 213/213 passed
- `npm run build --workspace apps/desktop`
  - passed
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts`
  - Playwright R2 smoke 1/1 passed with node delete preview coverage
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - created `node:N-160`
  - applied `op:delete-node-node:N-160`
  - deleted node row count became 0
  - local wasm receipt retained session-only persistence and no professional
    approval
  - browser console errors: 0

## Boundary

All data and ids are invented local preview data. The operation seam remains
structured-operation-only: no direct accepted-model mutation, no hidden
engineering defaults, no unit conversion, no protected standards content, no
private project data, no hidden pipe/support/load/component cascade, and no
professional approval, certification, sealing, authentication,
release-readiness, or code-compliance claim.

## Handoff

A11 entity deletion coverage is complete: support, primitive-load,
full-combination, load-case, pipe-run, node, and the previously landed
combination-term deletion all route through the structured operation seam.
Next unblocked plan item is A12 from-blank create -> solve -> report
rehearsal, followed by A8 journey automation as the R2 exit-evidence
backbone. Lifecycle state is unchanged.
