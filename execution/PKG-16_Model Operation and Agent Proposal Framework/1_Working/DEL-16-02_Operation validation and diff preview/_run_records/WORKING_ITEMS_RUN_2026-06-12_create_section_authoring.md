# WORKING_ITEMS Run Record - TP-APP-R2-CREATESECTION-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
add the third missing from-scratch entity creation operation,
`create_section`, to the structured operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_47_accept_create_section.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `create_section` as a supported create-kind operation.
- Implemented validation for explicit pipe-section payloads:
  matching section id, non-empty name/provenance, `section_type=pipe`,
  positive outside diameter and wall thickness in the project length unit,
  wall thickness less than outside-diameter radius, duplicate-id blocking,
  unit `project.units.length`, and dimension `length`.
- Added `apply_created_section` so successful apply returns a new model
  document and never mutates the input model in place.
- Added engine unit tests for accepted section creation, duplicate-id block,
  and invalid pipe geometry payload blocking.
- Added contract corpus case `case_47_accept_create_section.json` and updated
  both Rust and browser corpus coverage floors to require an accepted
  `create_section` apply.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 40 unit tests passed
  - canonical hash parity passed
  - 47-case contract corpus passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectation from the Rust reference engine
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 97/97 passed across browser adapter and direct wasm lanes
- `npm test --workspace apps/desktop`
  - 183/183 passed
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

A10 is landed for support/material/section creation. A11 deletion coverage
and A12 full from-blank solve/report rehearsal remain open. Optional pipe-form
reuse of standalone section refs is not claimed. Lifecycle state is unchanged.
