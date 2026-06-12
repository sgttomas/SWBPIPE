# WORKING_ITEMS Run Record - TP-APP-R2-DELSUPPORT-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add the first entity-deletion operation, `delete_support`, to the structured
operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_48_accept_delete_support.json`
- `fixtures/model_operations/contract_corpus/case_49_block_delete_support_referenced.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_support` as a supported delete-kind operation.
- Implemented validation for explicit support deletion: target
  `object_type=Support`, `field_path=supports`, `after=not_present`, unit
  `none`, dimension `dimensionless`, existing support label before-state
  matching, missing-target blocking, and empty-label payload blocking.
- Added reference-integrity blocking for support-targeted primitive loads via
  `OP-SUPPORT-DELETE-REFERENCED`.
- Added `apply_deleted_support` so successful apply returns a new model
  document and never mutates the input model in place.
- Added engine unit tests for accepted unreferenced support deletion, stale
  before-value blocking, and imposed-displacement reference blocking.
- Added contract corpus cases `case_48_accept_delete_support.json` and
  `case_49_block_delete_support_referenced.json`, and updated both Rust and
  browser corpus coverage floors.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- Focused Rust tests:
  - `explicit_delete_support_removes_unreferenced_support_only` passed
  - `delete_support_blocks_imposed_displacement_reference` passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectations from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - 49-case contract corpus passed
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 42 unit tests passed
  - canonical hash parity passed
  - 49-case contract corpus passed
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 101/101 passed across browser adapter and direct wasm lanes
- `npm test --workspace apps/desktop`
  - 189/189 passed
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

A11 has landed support deletion only. Remaining A11 entity deletion families
are node, pipe run, load case, primitive load, and full combination deletion;
`delete_combination_term` remains the existing term-level deletion operation.
Lifecycle state is unchanged.
