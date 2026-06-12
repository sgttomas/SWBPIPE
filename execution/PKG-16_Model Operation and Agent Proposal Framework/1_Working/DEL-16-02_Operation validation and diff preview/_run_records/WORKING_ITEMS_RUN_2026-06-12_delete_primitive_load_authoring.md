# WORKING_ITEMS Run Record - TP-APP-R2-DELPRIMLOAD-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add primitive-load deletion, `delete_primitive_load`, to the structured
operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_50_accept_delete_primitive_load.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_primitive_load` as a supported delete-kind operation.
- Implemented validation for indexed primitive-load deletion: target
  `object_type=Load`, `field_path=primitive_loads.N`, `after=not_present`,
  existing load case, existing indexed primitive load, current primitive
  before-state display, and matching primitive unit/dimension metadata.
- Added `apply_deleted_primitive_load` so successful apply removes exactly
  one primitive load and returns a new model document without mutating the
  input model in place.
- Added engine unit tests for accepted primitive-load deletion, stale
  before-value blocking, and out-of-range index blocking.
- Added contract corpus case `case_50_accept_delete_primitive_load.json` and
  updated both Rust and browser corpus coverage floors to require accepted
  `delete_primitive_load` coverage.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- Focused Rust test:
  `explicit_delete_primitive_load_removes_one_indexed_load_only` passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectation from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 43 unit tests passed
  - canonical hash parity passed
  - 50-case contract corpus passed
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 103/103 passed across browser adapter and direct wasm lanes
- `npm test --workspace apps/desktop`
  - 192/192 passed
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

A11 has landed support deletion and primitive-load deletion. Remaining A11
entity deletion families are node, pipe run, load case, and full combination
deletion; `delete_combination_term` remains the existing term-level deletion
operation. Lifecycle state is unchanged.
