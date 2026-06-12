# WORKING_ITEMS Run Record - TP-APP-R2-CREATESUPPORT-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
add the first missing from-scratch entity creation operation,
`create_support`, to the structured operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_45_accept_create_support.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `create_support` as a supported create-kind operation.
- Implemented validation for explicit support payloads:
  matching support id, non-empty label/node/provenance, existing node
  reference, supports collection present, duplicate-id blocking, unit
  `none`, dimension `dimensionless`, and restraint-token vocabulary
  `UX/UY/UZ/RX/RY/RZ`.
- Added `apply_created_support` so successful apply returns a new model
  document and never mutates the input model in place.
- Shared restraint-token normalization between support edit and support
  creation paths.
- Added engine unit tests for accepted support creation, duplicate-id block,
  and missing-node block.
- Added contract corpus case
  `case_45_accept_create_support.json` and updated both Rust and browser
  corpus coverage floors to require an accepted `create_support` apply.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 36 unit tests passed
  - canonical hash parity passed
  - 45-case contract corpus passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectation from the Rust reference engine
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 93/93 passed across browser adapter and direct wasm lanes
- `npm test --workspace apps/desktop`
  - 177/177 passed
- `npm run build --workspace apps/desktop`
  - passed
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- `npm run test:e2e --workspace apps/desktop`
  - Playwright R2 smoke 1/1 passed

## Boundary

All data and ids are invented local preview data. The operation seam remains
structured-operation-only: no direct accepted-model mutation, no hidden
engineering defaults, no unit conversion, no protected standards content, no
private project data, and no professional approval, certification, sealing,
authentication, release-readiness, or code-compliance claim.

## Handoff

A10 remains open for material creation and section creation. A11 deletion
coverage remains open. Lifecycle state is unchanged.
