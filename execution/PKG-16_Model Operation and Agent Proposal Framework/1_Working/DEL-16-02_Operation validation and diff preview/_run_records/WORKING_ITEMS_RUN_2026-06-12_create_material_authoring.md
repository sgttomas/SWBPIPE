# WORKING_ITEMS Run Record - TP-APP-R2-CREATEMATERIAL-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-02 Operation validation and diff preview
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
add the second missing from-scratch entity creation operation,
`create_material`, to the structured operation validation/apply seam.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
- `fixtures/model_operations/contract_corpus/case_46_accept_create_material.json`
- `fixtures/model_operations/contract_corpus/README.md`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/services/operationContractCorpus.test.ts`

## Changes

- Added `create_material` as a supported create-kind operation.
- Implemented validation for explicit material payloads:
  matching material id, non-empty label/provenance, positive elastic and
  shear modulus quantities in the project pressure unit, optional finite
  thermal-expansion quantity in `1/{project_temperature_unit}`,
  duplicate-id blocking, unit `project.units.pressure`, and dimension
  `stress`.
- Added `apply_created_material` so successful apply returns a new model
  document and never mutates the input model in place.
- Added engine unit tests for accepted material creation, duplicate-id block,
  and invalid quantity payload blocking.
- Added contract corpus case
  `case_46_accept_create_material.json` and updated both Rust and browser
  corpus coverage floors to require an accepted `create_material` apply.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml`
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  - 38 unit tests passed
  - canonical hash parity passed
  - 46-case contract corpus passed
- `CORPUS_BLESS=1 cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml --test contract_corpus`
  - regenerated the new case expectation from the Rust reference engine
- `npm run build:wasm:desktop`
- `npm test --workspace apps/desktop -- src/services/operationContractCorpus.test.ts`
  - 95/95 passed across browser adapter and direct wasm lanes
- `npm test --workspace apps/desktop`
  - 180/180 passed
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

A10 remains open for section creation. A11 deletion coverage remains open.
Lifecycle state is unchanged.
