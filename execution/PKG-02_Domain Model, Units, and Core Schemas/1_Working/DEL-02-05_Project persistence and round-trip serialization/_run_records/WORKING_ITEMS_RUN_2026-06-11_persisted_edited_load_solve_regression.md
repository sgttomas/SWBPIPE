# WORKING_ITEMS Run Record - Persisted Edited-Load Solve Regression

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-PERSISTEDSOLVE-001`, completion plan Phase A5 second
  sub-slice.
- Deliverable context: DEL-02-05 (project persistence and round-trip
  serialization); related contexts DEL-07-07 and DEL-14-02.

## What Changed

- `apps/desktop/src-tauri/src/lib.rs` now includes a backend regression for
  persisted edited-load data. It applies a structured `update_load` operation
  to `load:L-100` primitive `primitive_loads.1.magnitude.value`, changing the
  explicit force from `350` to `425` N.
- The test persists the edited model document through the local SQLite store
  helpers, reloads `project:edited-load-roundtrip`, verifies the restored
  model retains the edited load magnitude, and confirms the persisted model
  hash carrier is restored.
- This is a backend persistence/solve regression only. It does not modify the
  runtime create/save/open UI or complete the later packaged-Tauri GUI smoke.

## Validation Evidence

- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` passed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml saved_edited_load_model_round_trips_and_solves_from_restored_payload`
  passed with 1/1 targeted test.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with
  27/27 Rust tests.

## Boundary Review

- The test uses an in-memory SQLite store and invented bundled fixture data;
  no repository-default private data or user project data is written.
- No schema migration policy, model-document version, physical project
  container, lifecycle state, release claim, professional approval,
  certification, sealing, authentication, or code-compliance posture changed.
- DEL-02-05 `_STATUS.md` remains `CHECKING`.

## Residual

A5 still needs full packaged-Tauri GUI smoke over a saved edited project
snapshot. DEL-02-05 residuals from DEC-019 also remain: compatibility-window
size, explicit "Migrate project" operation, sibling JSON-slot coverage, and
physical container semantics.
