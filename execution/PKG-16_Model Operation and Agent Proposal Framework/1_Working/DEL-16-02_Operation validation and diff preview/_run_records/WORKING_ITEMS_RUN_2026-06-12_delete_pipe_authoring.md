# WORKING_ITEMS Run Record - TP-APP-R2-DELPIPE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add explicit pipe-run deletion, `delete_pipe_run`, to the structured operation
validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_54_accept_delete_pipe_run.json`
- `fixtures/model_operations/contract_corpus/case_55_block_delete_pipe_run_referenced.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_pipe_run` as a supported delete-kind operation.
- Implemented validation for whole pipe-run deletion: target
  `object_type=Element`, `field_path=pipe_segments`, `after=not_present`,
  current pipe label/from/to/material before-state display, unit `none`, and
  dimension `dimensionless`.
- Added reference blocking for pipes still used by primitive loads, emitting
  `OP-PIPE-DELETE-REFERENCED` with affected refs for the pipe and the
  referencing primitive load ids.
- Added `apply_deleted_pipe` so successful apply removes exactly one
  unreferenced pipe segment and returns a new model document without mutating
  the input model in place.
- Added engine unit tests for accepted pipe deletion, stale before-value
  blocking, missing-target blocking, and referenced-primitive-load blocking.
- Added contract corpus cases
  `case_54_accept_delete_pipe_run.json` and
  `case_55_block_delete_pipe_run_referenced.json`, then updated both Rust and
  browser corpus coverage floors to require accepted `delete_pipe_run` and
  `OP-PIPE-DELETE-REFERENCED` block coverage.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
- Focused Rust test:
  `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml pipe_run`
  - 2/2 focused pipe-run tests passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the two new case expectations from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - 55-case contract corpus passed
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- operationContractCorpus`
  - 113/113 passed across browser adapter and direct wasm lanes
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 47 unit tests passed
  - canonical hash parity passed
  - 55-case contract corpus passed
- `npm test --workspace apps/desktop`
  - 207/207 passed
- `npm run build --workspace apps/desktop`
  - passed
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts`
  - Playwright R2 smoke 1/1 passed with pipe delete preview coverage
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - selected `pipe:P-130`
  - applied `op:delete-pipe-pipe:P-130`
  - deleted pipe row count became 0
  - local wasm receipt retained session-only persistence and no professional
    approval
  - browser console errors: 0

## Boundary

All data and ids are invented local preview data. The operation seam remains
structured-operation-only: no direct accepted-model mutation, no hidden
engineering defaults, no unit conversion, no protected standards content, no
private project data, no hidden primitive-load cascade, and no professional
approval, certification, sealing, authentication, release-readiness, or
code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, full-combination
deletion, load-case deletion, and pipe-run deletion. Remaining A11 entity
deletion scope is node deletion. A12 should rehearse the full from-blank
create -> solve -> report path after A11 closes, and A8 journey automation
should carry that route as the R2 exit-evidence backbone. Lifecycle state is
unchanged.
