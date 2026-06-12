# WORKING_ITEMS Run Record - TP-APP-R2-DELLOADCASE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add whole load-case deletion, `delete_load_case`, to the structured operation
validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_52_accept_delete_load_case.json`
- `fixtures/model_operations/contract_corpus/case_53_block_delete_load_case_referenced.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `delete_load_case` as a supported delete-kind operation.
- Implemented validation for whole-load-case deletion: target
  `object_type=Load`, `field_path=load_cases`, `after=not_present`, current
  load-case id/label/kind/status/primitive-count before-state display, unit
  `none`, and dimension `dimensionless`.
- Added reference blocking for load cases still used by combination terms,
  emitting `OP-LOAD-CASE-DELETE-REFERENCED` and affected refs such as
  `combination:C-OPER-ALT.terms.0`.
- Added `apply_deleted_load_case` so successful apply removes exactly one
  unreferenced load case and returns a new model document without mutating the
  input model in place.
- Added engine unit tests for accepted load-case deletion, stale before-value
  blocking, and referenced-load blocking.
- Added contract corpus cases
  `case_52_accept_delete_load_case.json` and
  `case_53_block_delete_load_case_referenced.json`, then updated both Rust and
  browser corpus coverage floors to require accepted `delete_load_case` and
  `OP-LOAD-CASE-DELETE-REFERENCED` block coverage.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- Focused Rust test:
  `explicit_delete_load_case_removes_unreferenced_case_only` passed
- `npm run build:wasm:desktop`
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the two new case expectations from the Rust reference engine
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - 53-case contract corpus passed
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 109/109 passed across browser adapter and direct wasm lanes
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 45 unit tests passed
  - canonical hash parity passed
  - 53-case contract corpus passed
- `npm test --workspace apps/desktop`
  - 201/201 passed
- `npm run build --workspace apps/desktop`
  - passed
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- `npm run test:e2e --workspace apps/desktop`
  - Playwright R2 smoke 1/1 passed with load-case delete preview coverage
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - created then deleted `load:L-300`
  - final summary returned to `2 load cases; 7 primitive loads; 1 combinations`
  - local wasm receipt retained session-only persistence and no professional approval
  - browser console errors: 0

## Boundary

All data and ids are invented local preview data. The operation seam remains
structured-operation-only: no direct accepted-model mutation, no hidden
engineering defaults, no unit conversion, no protected standards content, no
private project data, no hidden combination-term cascade, and no professional
approval, certification, sealing, authentication, release-readiness, or
code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, full-combination
deletion, and load-case deletion. Remaining A11 entity deletion families are
node and pipe-run deletion; `delete_combination_term` remains the existing
term-level deletion operation. A12 should rehearse the full from-blank create
-> solve -> report path, and A8 journey automation should carry that route as
the R2 exit-evidence backbone. Phase B1 units work has been split into a
separate crate-side worker and is not part of this A11 closeout. Lifecycle
state is unchanged.
