# WORKING_ITEMS Run Record — Edited-Model Solve Binding Guard

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-SOLVEBOUND-001`, completion plan Phase A5 first
  sub-slice.
- Deliverable context: DEL-14-02 (analysis-run records and result binding);
  related contexts DEL-07-07 and DEL-04-06.

## What Changed

- Browser fixture-mode mechanics now preserves model-state/result binding
  honesty after an app-session edit. It emits a result envelope bound to the
  edited `project.id` with `MODEL_INCOMPLETE`, zero result rows, and blocking
  diagnostic `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` instead of
  carrying the bundled fixture's solved rows forward.
- The Tauri direct command and job-registry tests verify that the real solve
  path accepts an edited model payload and publishes solved mechanics results
  bound to the edited `project.id`.
- The existing analysis-run builder path remains unchanged; the app now
  receives either backend-solved edited results or an explicit incomplete
  browser-mode result envelope, rather than stale rows under a mismatched
  model state.

## Validation Evidence

- `npm test --workspace apps/desktop`: 31 passed, 0 failed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`: 26 passed,
  0 failed.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed.
- Browser smoke on `http://127.0.0.1:5174/` confirmed the edited-model
  solve-job JSON reports `result_row_count=0` with diagnostics
  `RULE_INPUTS_MISSING` and
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`; no browser warnings,
  errors, or page errors were observed.

## Boundary Review

- No schema, hash policy, analysis-run acceptance policy, lifecycle state,
  protected/private data posture, release status, professional approval,
  certification, sealing, authentication, or code-compliance posture changed.
- This run records evidence only. DEL-14-02 `_STATUS.md` remains `CHECKING`.
