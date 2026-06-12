# WORKING_ITEMS Run Record - TP-APP-R2-DELCOMBINATION-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add full load-combination deletion, `delete_combination`, to the structured
operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_51_accept_delete_combination.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_combination` as a supported delete-kind operation.
- Implemented validation for whole-combination deletion: target
  `object_type=Combination`, `field_path=combinations`, `after=not_present`,
  current combination before-state display, unit `none`, and dimension
  `dimensionless`.
- Added `apply_deleted_combination` so successful apply removes exactly one
  combination and returns a new model document without mutating the input
  model in place.
- Added engine unit tests for accepted combination deletion, stale
  before-value blocking, and missing-target blocking.
- Added contract corpus case `case_51_accept_delete_combination.json` and
  updated both Rust and browser corpus coverage floors to require accepted
  `delete_combination` coverage.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- Focused Rust test:
  `explicit_delete_combination_removes_one_combination_only` passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectation from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - 51-case contract corpus passed
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 105/105 passed across browser adapter and direct wasm lanes
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 44 unit tests passed
  - canonical hash parity passed
  - 51-case contract corpus passed
- `npm test --workspace apps/desktop`
  - 195/195 passed
- `npm run build --workspace apps/desktop`
  - passed
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- `npm run test:e2e --workspace apps/desktop`
  - Playwright R2 smoke 1/1 passed with whole-combination delete preview coverage
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - full-combination delete apply passed with zero console errors

## Boundary

All data and ids are invented local preview data. The operation seam remains
structured-operation-only: no direct accepted-model mutation, no hidden
engineering defaults, no unit conversion, no protected standards content, no
private project data, and no professional approval, certification, sealing,
authentication, release-readiness, or code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, and full
combination deletion. Remaining A11 entity deletion families are node, pipe
run, and load case deletion; `delete_combination_term` remains the existing
term-level deletion operation. Lifecycle state is unchanged.
