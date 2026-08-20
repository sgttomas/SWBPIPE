# Handoff state — PKG-07 / DEL-07-06

- Run / instance: `HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI` /
  `WI-PKG07-DEL0706`.
- Accepted upstream: Git
  `fae5e38ee60fd4c8d4a52ac7f663036a83cdbd7d`, Receipt-118, approved DAG-009,
  target R5, and accepted R7 packaged-binary seam proof.
- Closure verdict: `NOT_CLOSED — HOST_COMPUTER_USE_STATE_CAPTURE_HUNG`.
- Authoritative truth: unchanged. The exact packaged-Tauri GUI smoke residual
  remains open; DEL-07-06 remains `IN_PROGRESS`; both PDU-045/PDU-046 holds
  remain unchanged.
- Product state: unchanged; no application source, test, fixture, package, or
  build artifact was modified.
- Observed host state: `list_apps` identified
  `org.openpipestress.technical-preview` as running. Both path and exact-bundle
  `get_app_state` attempts returned no AX tree/screenshot and hung until
  interrupted. No permission prompt or GUI predicate was observed.
- Derivative package: this R8 AgentRuns subtree is current for the failed host
  proof attempt and cites its accepted upstream; it is not a substitute for
  deliverable/decomposition truth.
- Rerun: require a host session where Computer Use returns a screenshot or AX
  tree for the exact bundle ID, then perform the frozen edit/apply/save/quit/
  relaunch/reopen/solve journey. Obtain confirmation if a macOS permission
  prompt appears. Do not substitute headless evidence.
- Remaining blocker: host UI-state capture only. No product defect is claimed.
- Next owner: HELP_HUMAN for fan-in disposition; CHANGE alone owns any Git
  commit/publication and receipt action.

## Attempt 2 current handoff — supersedes the blocked verdict

- Closure verdict: `PASS_VALIDATED_TERMINAL_FAN_IN`.
- Authoritative deliverable effect: the exact packaged edited-load GUI
  residual is removed after actual packaged observation. DEL-07-06 remains
  `IN_PROGRESS`; PDU-045/PDU-046 remain `VERIFIED_NOT_VALIDATED` with the
  measurable target still `TBD_by_human_project_authority`.
- Product state: one visible `Solve proof` row renders existing native seam,
  project/result identity, identity-match, row-count, and synchronous solve-
  generation values. No contract or cross-package authority changed.
- Actual packaged predicates: reopened `425 N`; no queued edit; zero pending/
  applied/editor/proposal state; no stale solve; post-reopen
  `MECHANICS_SOLVED`; `seam=tauri_backend_job`; matching
  `project:invented-loop-01` identity; 830 rows; generation 6.
- Final package executable SHA-256:
  `3a95da3d2269b124734bc22a0e46d820e3fdb6870630159757ff8bab30507a8c`.
- Checks: focused Vitest 7/7 PASS; focused Playwright 2/2 PASS; desktop Vitest
  29/29 files and 533/533 tests PASS; desktop build PASS; Tauri package PASS;
  harness self-check execution PASS; harness pytest 350/350 PASS.
- Derivative package: DEL run record, TP-MAC-286, and this exact R8 AgentRuns
  subtree cite the attempt-2 product/evidence snapshot and do not replace DAG
  or decomposition truth.
- Rerun requirement: any product/evidence change after the frozen snapshot
  requires affected checks and a fresh review. No host rerun remains if the
  independent review passes.
- Remaining gate: none within N1. Fresh reviewer 5 verified 47/47 hashes,
  exact containment and check selection, reviewed 100%, closed all four prior
  findings, and returned PASS. HELP_HUMAN owns fan-in; CHANGE owns commit/PR.
- Review history: reviewer 1 verified 16/16 hashes but returned an actionable
  FAIL for stale result/current-job pairing and incorrect PROFILE_PATH.
  `AMENDMENT_3.md` remediated both; corrected packaged proof now binds
  `backend-solve-job-1`, exact model and input-manifest SHA-256 values, matching
  project/result model, and 830 rows. Reviewer 2 verified 23/23 hashes but
  returned FAIL for an overlapping native-run callback race and an evidence
  overclaim. `AMENDMENT_4.md` adds a synchronous generation gate, rejects
  superseded same-ID callbacks, and narrows Playwright to happy-path evidence.
  Reviewer 3 verified 31/31 hashes and returned FAIL for post-commit model
  invalidation, lost pre-start cancellation, and non-integrated callback tests.
  `AMENDMENT_5.md` makes invalidation pre-commit, persists ordinary immediate
  cancel, and adds rendered deferred native-event coverage. Reviewer 4
  verified 40/40 hashes but found a model/open transition could detach a late
  start receipt from its requested cancel. `AMENDMENT_6.md` carries a
  generation-bound cancellation tombstone to that receipt, dispatches exactly
  once, and keeps stale UI callbacks inert. Reviewer 5 passed the final
  snapshot with no actionable finding.
