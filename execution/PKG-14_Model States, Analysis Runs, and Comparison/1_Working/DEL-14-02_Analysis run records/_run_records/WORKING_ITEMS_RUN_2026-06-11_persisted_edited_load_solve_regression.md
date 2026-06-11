# WORKING_ITEMS Run Record - Persisted Edited-Load Solve Regression

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-PERSISTEDSOLVE-001`, completion plan Phase A5 second
  sub-slice.
- Deliverable context: DEL-14-02 (analysis-run records and result binding);
  related contexts DEL-02-05 and DEL-07-07.

## What Changed

- `apps/desktop/src-tauri/src/lib.rs` now includes backend evidence that a
  restored saved model, not the original bundled fixture, supplies the solve
  payload after an edited load-data operation.
- The regression checks the restored solve binds to
  `project:edited-load-roundtrip`, emits `MECHANICS_SOLVED`, and produces a
  changed `result:disp:node-N-140` value relative to the original fixture
  solve.
- No analysis-run schema, hash policy, or persisted analysis-run payload shape
  changed in this tranche.

## Validation Evidence

- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` passed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml saved_edited_load_model_round_trips_and_solves_from_restored_payload`
  passed with 1/1 targeted test.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with
  27/27 Rust tests.

## Boundary Review

- This run records evidence only. It does not alter lifecycle state, review
  disposition, protected/private data posture, release status, professional
  approval, certification, sealing, authentication, or code-compliance
  posture.
- DEL-14-02 `_STATUS.md` remains `CHECKING`.

## Residual

A5 still needs full packaged-Tauri GUI smoke over a saved edited project
snapshot and broader persisted solve/run evidence as new authoring surfaces
grow.
