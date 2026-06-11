# WORKING_ITEMS Run Record — Solve Cancellation Backend Job Seam

- Date: 2026-06-10
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: replace the UI-only solve cancellation record
  (`backend_cancellation_token: "TBD"`) with a real asynchronous solve-job
  seam in the Tauri backend, with cooperative checkpoint cancellation and an
  honest frontend audit trail.
- Deliverable context: DEL-07-07 (solve execution UX: progress, cancellation,
  diagnostics); related context DEL-00-03 (command-query-job model),
  DEL-04-06 (solver diagnostics), DEL-14-02 (analysis run records).

## What changed

Backend (`apps/desktop/src-tauri/src/lib.rs`):

- Added a `SolveJobRegistry` (mutex-guarded job table managed as Tauri state)
  with three new commands registered in the invoke handler:
  `start_preview_mechanics_job`, `poll_preview_mechanics_job`, and
  `cancel_preview_mechanics_job`.
- `start_preview_mechanics_job` registers a job, issues a per-job
  `backend_cancellation_token`, and executes the preview solve on a background
  thread. `run_preview_mechanics` remains available and now shares the same
  `solve_preview_mechanics` helper.
- Cancellation is cooperative at backend checkpoints
  (`cooperative_checkpoints_not_preemptive`): the executor checks the
  cancellation flag before solver start and again before result publication;
  a cancelled job discards its result and never publishes it. No preemptive
  mid-solve interruption is claimed.
- Cancellation receipts are honest: invalid tokens are rejected without
  flagging the job; requests after a terminal state report
  `request_after_terminal_state_*_no_cancellation_performed`; receipts always
  carry `cancellation_success_claimed: false`.

Frontend (`apps/desktop/src`):

- `previewService.ts` exposes `startPreviewMechanicsJob`,
  `pollPreviewMechanicsJob`, and `cancelPreviewMechanicsJob`; browser fixture
  mode (no Tauri runtime) reports
  `mode: "browser_fixture_no_backend_job"` instead of pretending a job exists.
- `types.ts` widens `backend_cancellation_token` from the `"TBD"` literal to a
  string and adds `backend_job_seam` / `backend_job_id`; the solve job event
  state union gains `"cancelled"`.
- `App.tsx` runs the solve through the backend job seam when available
  (start, poll to terminal state, bind result), records backend cancellation
  receipts and failures as audit events, and keeps the fixture path for
  browser mode. Cancellation status values name what actually happened
  (e.g. `request_sent_to_backend_job_awaiting_receipt`,
  `request_recorded_no_backend_job_in_browser_fixture_mode`,
  `cancelled_before_result_publication_result_discarded`).
- `SolvePanel.tsx` solve-job audit packet now reports `backend_job_seam`,
  `backend_job_id`, the real token value, and `cancellation_scope`;
  `HeadlessRunnerPanel.tsx` maps the new `cancelled` state.

## Validation evidence

- `cargo test` (apps/desktop/src-tauri): 12 passed, 0 failed; includes six new
  solve-job tests covering completion, cancel-before-start, cancel-mid-run
  result discard at the publication checkpoint, post-completion cancellation
  rejection without success claim, invalid-token rejection, and failed-solve
  error recording. `cargo fmt` applied.
- `npm test --workspace apps/desktop` (Vitest): 13 passed, 0 failed, with
  updated expectations for the new seam/token audit values.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite build
  green.
- Browser preview (Vite dev server, fixture mode): initial solve panel shows
  `seam=no_job_started; token=none_no_active_backend_job`; after a run the
  panel shows `state=completed; result_rows=647` with
  `seam=browser_fixture_no_backend_job` and
  `token=unavailable_no_backend_job_browser_fixture_mode`. The Tauri backend
  path is exercised by the Rust unit tests; no browser-mode claim is made for
  it.

## Boundary review

- Local-only invented preview data; no network, telemetry, daemon, or cloud
  paths added. The cancellation token is a local in-process job handle, not an
  authentication or security credential.
- No release, professional, certification, sealing, authentication, or
  code-compliance claims. Cancellation success is never claimed; only
  checkpoint outcomes are reported.
- Residual gaps (unchanged, still TBD): no backend percent-progress stream
  (`backend_percent_stream_available=false`); no preemptive mid-solve
  interruption; headless-runner CLI/process seams remain panel-level
  representations.

## Scope and external noise

- Write scope: `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src/**`
  (App, SolvePanel, HeadlessRunnerPanel, previewService, types, App.test),
  this run record, and the DEL-07-07 `MEMORY.md` addendum.
- External-scope noise recorded and bypassed: untracked `init/init-prompt.md`
  at the chirality repository root (outside this project); not staged.
- No lifecycle state change: DEL-07-07 `_STATUS.md` remains `CHECKING`.
