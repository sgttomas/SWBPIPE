# WORKING_ITEMS Run Record — Edited-Model Solve Binding Guard

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-SOLVEBOUND-001`, completion plan Phase A5 first
  sub-slice.
- Deliverable context: DEL-07-07 (solve execution UX: progress,
  cancellation, diagnostics); related contexts DEL-14-02 (analysis-run/result
  binding) and DEL-04-06 (solver diagnostics and incomplete-model gating).

## What Changed

- `apps/desktop/src/services/previewService.ts` now routes browser fallback
  mechanics through a model-aware helper. No model, or the unchanged bundled
  model, still returns the solved invented fixture.
- For edited browser-session models, browser fixture mode returns a
  `MechanicsResult` bound to the edited `project.id` with
  `MODEL_INCOMPLETE`, `RULE_INPUTS_INCOMPLETE`, zero result rows, current
  model counts, and blocking diagnostic
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`.
- `apps/desktop/src/App.test.tsx` now verifies that an applied inspector edit
  followed by a browser-mode preview run completes with zero result rows and
  the backend-required diagnostic instead of stale fixture solve rows.
- `apps/desktop/src-tauri/src/lib.rs` now has explicit unit coverage that
  `run_preview_mechanics(Some(model))` and the solve-job registry use the
  supplied edited model payload and publish solved results bound to the edited
  `project.id`.

## Validation Evidence

- `npm test --workspace apps/desktop`: 31 passed, 0 failed.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`: 26 passed,
  0 failed.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed.
- `git diff --check -- . ':!init/init-prompt.md'`: passed for the touched
  scope; `init/init-prompt.md` remains unrelated pre-existing workspace noise.
- Browser smoke on `http://127.0.0.1:5174/`: selected
  `material:invented-carbon-steel`, applied `elastic_modulus.value =
  195000000000`, ran mechanics preview, and observed `MODEL INCOMPLETE`,
  `0 computed result rows; model incomplete`, `state=completed;
  result_rows=0`, and visible diagnostic
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. The solve-job JSON
  carried `result_row_count=0` and diagnostics `RULE_INPUTS_MISSING` plus
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. Browser console output
  contained only Vite debug and React DevTools info messages.

## Boundary Review

- This is an honesty guard for browser fixture mode, not a browser solver.
  Edited-model solving is proven through the Tauri backend command/job path.
- No network, telemetry, cloud, daemon, protected standards text, private
  project data, code-specific defaults, release claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was added.
- Residual Phase A5 work: packaged-Tauri GUI smoke over a saved edited project
  snapshot, broader non-fixture persisted solve coverage, and UI copy polish
  for incomplete-model diagnostics.
- No lifecycle state change: DEL-07-07 `_STATUS.md` remains `CHECKING`.
