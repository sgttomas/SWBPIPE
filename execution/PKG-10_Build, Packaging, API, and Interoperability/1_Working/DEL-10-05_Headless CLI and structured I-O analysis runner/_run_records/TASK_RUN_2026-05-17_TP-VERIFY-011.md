# TASK RUN - TP-VERIFY-011 Headless preview bridge regression

## Dispatch

- **Agent:** TASK
- **TaskProfile:** interop-build
- **DeliverableID:** DEL-10-05
- **PackageID:** PKG-10
- **Run timestamp:** 2026-05-17 10:27 MDT
- **Objective:** Fix the confirmed headless preview bridge regression where the deterministic preview bridge produced `MODEL_INCOMPLETE` instead of `MECHANICS_SOLVED`.

## Scope

- **ScopePath:** `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner`
- **Production/test surfaces touched:** `core/product_physics/src/lib.rs`; `core/product_physics/Cargo.lock`; `fixtures/product_preview/invented_preview_model.json`
- **Deliverable-local closeout touched:** this run record; `MEMORY.md`
- **Explicit exclusions honored:** `_STATUS.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `Review_Findings.csv`, DAG files, blocker queues, candidate rows, schemas, CI workflows, release records, acceptance records, and professional/code-compliance dispositions.

## Inputs Read

- `AGENTS.md`
- `agents/AGENT_TASK.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/_Registers/Deliverables.csv` row `DEL-10-05`
- `DEL-10-05` `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `MEMORY.md`
- `core/runner/headless/src/lib.rs`
- `core/product_physics/src/lib.rs`
- `fixtures/product_preview/invented_preview_model.json`
- `core/loads/primitive_loads/src/lib.rs`
- `schemas/model.schema.yaml`, `schemas/units.schema.yaml`, and PKG-02 vocabulary references as searched for `temperature_interval`

## Root Cause

The headless bridge failure was caused by the product-physics preview model returning a blocked mechanics envelope. Two stale metadata seams were involved:

- Product-physics preview parsing accepted `temperature_change` for thermal primitive loads, while the accepted PKG-02/schema vocabulary uses `temperature_interval`.
- The invented product-preview fixture used `dimension: "TBD"` for two distributed weight loads even though their magnitudes are explicit `N/m`; the strict primitive-load bridge requires `force_per_length` for those loads.

## Work Performed

- Updated `core/product_physics/src/lib.rs` so thermal primitive loads accept both the existing internal `temperature_change` value and the accepted schema-facing `temperature_interval` value.
- Preserved Cargo's product-physics lockfile dependency closure after validation reintroduced missing local dependency edges for `open_pipe_stress_stress_recovery`.
- Updated `fixtures/product_preview/invented_preview_model.json` so the invented distributed weight loads declare `force_per_length` instead of `TBD`.
- Preserved the runner boundary: no CLI syntax, package script, transport, adapter, CI, release, or filesystem/process policy was introduced.

## Validation

| Check | Result |
|---|---|
| `git status --short --branch` before edits | Clean on `main...origin/main`. |
| `cargo test --manifest-path core/runner/headless/Cargo.toml preview_bridge_executes_product_physics_with_deterministic_refs` before edits | Failed: expected `MECHANICS_SOLVED`, got `MODEL_INCOMPLETE`. |
| `cargo test --manifest-path core/product_physics/Cargo.toml valid_invented_model_solves_deterministically` before edits | Failed: expected `MECHANICS_SOLVED`, got `MODEL_INCOMPLETE`. |
| `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` | Pass. |
| `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check` | Pass. |
| `cargo test --manifest-path core/product_physics/Cargo.toml valid_invented_model_solves_deterministically` | Pass. |
| `cargo test --manifest-path core/runner/headless/Cargo.toml preview_bridge_executes_product_physics_with_deterministic_refs` | Pass. |
| `cargo test --manifest-path core/product_physics/Cargo.toml` | Pass; 23 tests. |
| `cargo test --manifest-path core/runner/headless/Cargo.toml` | Pass; 7 tests. |
| `python3 tests/test_headless_runner_contract.py` | Pass. |
| `python3 tests/test_model_schema.py` | Pass. |
| `python3 tests/product_preview/test_product_preview_service.py` | Pass. |

## Boundary Notes

- This run fixed a software regression in the headless preview bridge evidence path only.
- No lifecycle state, dependency status, review finding, candidate, blocker, DAG, release, acceptance, human disposition, professional adequacy, or code-compliance claim was changed.
- Follow-up parent fan-in remains required before commit or any broader release-gate interpretation.
