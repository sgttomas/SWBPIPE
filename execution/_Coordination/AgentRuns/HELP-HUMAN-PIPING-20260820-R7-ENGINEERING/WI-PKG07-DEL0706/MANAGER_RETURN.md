# WORKING_ITEMS Manager Return — PKG-07 / DEL-07-06

## Verdict

`VALIDATED_PARTIAL_PRODUCT_PROGRESS / PACKAGED_GUI_HOST_BLOCKED`

The actual packaged executable now contains and passes a durable saved
edited-load self-test. The narrower GUI-control journey could not be observed
because the host exposed no usable accessibility state; its exact Remaining
bullet is preserved. No lifecycle transition or authority claim is made.

## Changed files

- `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs`
- `projects/chirality-piping/apps/desktop/src-tauri/src/main.rs`
- `projects/chirality-piping/apps/desktop/SMOKE.md`
- `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_STATUS.md`
- `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/MEMORY.md`
- `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/WORKING_ITEMS_RUN_2026-08-20_R7_PACKAGED_EDITED_LOAD.md`
- this instance subtree under
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R7-ENGINEERING/WI-PKG07-DEL0706/`.

## Product behavior and evidence

- New actual-binary flag: `--self-test-saved-edited-load`.
- Structured operation: `load:L-100/load:L-100-Y`, `350 N -> 425 N`, no
  professional approval.
- Persistence: unique disposable file-backed SQLite store, schema 11, writer
  connection closed before a second-connection restore, temp store removed.
- Restored solve: `425 N`, `MECHANICS_SOLVED`, matching project/model identity,
  830 result rows, edited displacement differs from baseline.
- Fresh bundle executable SHA-256:
  `28e2effdc2203437f0fb7ef02339f78a2f6e1ad1ccf775bb0053edba858669ca`.

## Validation

- Focused Vitest: PASS 68/68.
- New Rust self-test: PASS 1/1.
- Existing saved edited-load Rust test: PASS 1/1.
- Rust format check: PASS.
- Registered desktop-test: PASS, 29 files / 526 tests.
- Registered desktop-build: PASS.
- Fresh `.app` build and actual bundled executable self-test: PASS.
- Registered harness self-check: executed successfully with pinned dependencies
  under Python 3.13; reported only pre-existing repository findings.
- Harness pytest: PASS, 350 tests.
- Fresh independent review attempt 1 returned actionable P2: restored and
  solved identity was recorded but not fail-closed. The manager remediated it,
  reran focused/full/package checks and the actual bundle proof, and froze v2.
  Agent 0 strengthened the gate to require one terminal
  read-only, non-delegating `software-code-review` over 100% of the complete
  remediated node diff. The authoritative terminal verdict is persisted at
  `TASK-REVIEW-2/RETURN.md`; a PASS with no actionable finding is required
  before this return may be accepted.
- Fresh review attempt 2 returned PASS with no actionable findings. It verified
  18/18 frozen hashes, 100% of five tracked diffs and 13 untracked artifacts,
  P2 closure, bundle SHA-256, containment, registered-check selection, JSON/
  JSONL parsing, diff hygiene, and claim consistency.
- CHANGE then detected `new blank line at EOF` in five new AgentRuns Markdown
  files during `git diff --cached --check`. Amendment 2 normalizes only those
  endings. A third different fresh reviewer returned PASS with no actionable
  findings over 24/24 v3 hashes and 100% of the corrected snapshot;
  authoritative result: `TASK-REVIEW-3/RETURN.md`.
- Corrected working-tree `git diff --check` and `git diff HEAD --check`: PASS.
  The untouched index still holds CHANGE's five pre-normalization bytes, so
  CHANGE must restage before rerunning `git diff --cached --check`.
- Runtime telemetry: terminal summary PASS at 16 events / 5 sessions, with no
  unmatched sessions.

## Host blocker and residual

`open` launched the packaged app and the process was visible. Two bounded
Computer Use state calls and one exact read-only System Events UI query yielded
no accessibility tree, screenshot, window controls, or usable state. The
System Events query was terminated after 20 seconds. This is a host automation
blocker, not a product pass or defect verdict.

The exact `_STATUS.md ## Remaining` bullet “Run a packaged-Tauri smoke over a
saved project with edited load data” remains unchanged. Rerun the GUI journey
on a host with working Accessibility control. The PDU-045/PDU-046 holds remain
unchanged.

## Derivative and requested Agent 0 action

The AgentRuns package, SMOKE entry, and deliverable run record are derivative
evidence bound to the named accepted basis and product diff. They do not replace
decomposition/DAG/deliverable truth.

Requested Agent 0 action: validate fan-in and route the uncommitted scoped diff
to CHANGE for the node commit. Preserve the GUI residual and host rerun
requirement. The final clean-commit DEC-025 sweep remains for Agent 0/CHANGE
because this manager return is necessarily uncommitted and the sweep binds Git
state.
