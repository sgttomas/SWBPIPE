# WORKING_ITEMS Run Record - Persisted Edited-Load Solve Regression

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-PERSISTEDSOLVE-001`, completion plan Phase A5 second
  sub-slice.
- Deliverable context: DEL-07-07 (solve execution UX: progress,
  cancellation, diagnostics); related contexts DEL-02-05 and DEL-14-02.

## What Changed

- `apps/desktop/src-tauri/src/lib.rs` now proves the backend solve boundary
  can consume a model restored from local project persistence after a
  structured load-data edit.
- The regression applies an explicit `update_load` operation, persists the
  edited model, reloads it, and calls `solve_preview_mechanics` on the
  restored payload.
- The restored solve reports `model_ref=project:edited-load-roundtrip`,
  `MECHANICS_SOLVED`, non-empty result rows, and a changed
  `result:disp:node-N-140` value versus the original fixture solve.
- No frontend solve controls, progress UI, or cancellation behavior changed.

## Validation Evidence

- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` passed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml saved_edited_load_model_round_trips_and_solves_from_restored_payload`
  passed with 1/1 targeted test.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with
  27/27 Rust tests.

## Boundary Review

- This tranche is backend regression evidence for the saved edited-load solve
  path, not a packaged GUI smoke or browser solver.
- No network, telemetry, cloud, daemon, protected standards content, private
  project data, release readiness, professional approval, certification,
  sealing, authentication, or code-compliance claim was added.
- DEL-07-07 `_STATUS.md` remains `CHECKING`.

## Residual

A5 still needs full packaged-Tauri GUI smoke over a saved edited project
snapshot, UI polish for incomplete-model diagnostics, and broader persisted
solve coverage as new authoring surfaces grow.
