# WORKING_ITEMS manager return — PKG-07 / DEL-07-06

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI` |
| InstanceID | `WI-PKG07-DEL0706` |
| Node | `N1` |
| Accepted basis | `fae5e38ee60fd4c8d4a52ac7f663036a83cdbd7d`; Receipt-118; DAG-009; R5 |
| Outcome | `PASS_CHECKS_AWAITING_REVIEW_8` |
| Product effect | Generation-bound packaged solve proof row and detached exact-once cancellation; viewport-budgeted desktop workspace layout; adverse Vitest and real-click Playwright coverage |
| Deliverable truth effect | Exact packaged edited-load smoke residual closed; PDU-045/PDU-046 holds and lifecycle unchanged |

## Attempt 1 — preserved blocked history

The run attempted only the selected packaged-Tauri GUI residual. The accepted
R7 packaged binary exists at:

`projects/chirality-piping/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`

Its executable SHA-256 was confirmed as
`28e2effdc2203437f0fb7ef02339f78a2f6e1ad1ccf775bb0053edba858669ca`,
matching accepted R7 evidence.

Computer Use observations:

1. A path-targeted `sky.get_app_state` returned no accessibility tree,
   screenshot, or other state before interruption after approximately 321
   seconds.
2. After resetting the Node REPL, `sky.list_apps` completed and returned the
   packaged application as `OpenPipeStress Technical Preview`, exact bundle
   identifier `org.openpipestress.technical-preview`, with `isRunning: true`.
3. A bundle-identifier-targeted `sky.get_app_state` requested with a 30-second
   tool timeout again returned no accessibility tree or screenshot and hung
   until interruption after approximately 154 seconds.

The skill's screenshot/coordinate fallback was unavailable because neither
state call returned a screenshot. No macOS permission prompt was observed or
available in returned state. No GUI action beyond application launch/state
inspection was possible, and no load value, pending state, save, reopen, solve
status, backend route, identity, or result row was observed.

## Fan-in disposition

The fan-in gate fails at the host UI-control prerequisite. The exact
DEL-07-06 Remaining item is not closed. R7's headless packaged-binary proof was
not substituted. No in-scope product defect was diagnosed because the host
never exposed application UI state.

No independent code review was triggered: no product code or product behavior
changed. No deliverable status, memory, or run-record surface changed.

## Changed paths

- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/ORCHESTRATION_PLAN.md`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WORK_GRAPH.json`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/ACTIVATION_BRIEF.md`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/RUNTIME_EVENTS.jsonl`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/RUNTIME_SUMMARY.json`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/MANAGER_RETURN.md`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/HANDOFF_STATE.md`

## Checks, blockers, and reruns

- Host packaged GUI journey: `BLOCKED`; state capture hung twice and returned
  no screenshot/AX tree.
- `harness-self-check`: PASS execution (exit 0); reported the existing
  repository-wide generated-view findings, with no R8-specific failure.
- `harness-pytest`: PASS, 350 tests.
- Desktop test/build: not triggered; no desktop code changed.
- Focused product checks: not triggered; no product behavior changed.
- Independent code review: not triggered; frozen integrated diff contains
  coordination records only.

Rerun requirement: repeat the exact packaged GUI journey only when Computer Use
can return either a usable accessibility tree or screenshot for
`org.openpipestress.technical-preview`. If macOS exposes an Accessibility or
Screen Recording prompt, obtain the owner's confirmation at action time and
then retry. Do not substitute the accepted headless self-test.

## Derivative status and requested Agent 0 action

This AgentRuns package is derivative coordination/evidence only and cites the
accepted basis above; it does not amend authoritative decomposition or
deliverable truth. Requested action: Agent 0 should accept the truthful host
blocker, preserve the DEL-07-06 residual unchanged, and route only these
coordination records to CHANGE if the iteration is to land as a blocked-proof
receipt. No lifecycle, release, professional-reliance, accessibility-
conformance, certification, sealing, authentication, or code-compliance claim
is made.

Runtime telemetry is terminal and internally matched at
`RUNTIME_SUMMARY.json`: 4 events, 1 session, final outcome `BLOCKED`, summary
integrity status `PASS`; token/context occupancy was unavailable.

## Attempt 2 — owner-invited retry and product proof

Owner invitation reopened the same frozen N1 node under `AMENDMENT_1.md`.
Computer Use returned a full AX tree and screenshot for exact bundle
`org.openpipestress.technical-preview`. The actual packaged GUI journey then:

1. selected `load:L-100/load:L-100-Y` and observed `350 N`;
2. entered `425`, queued the magnitude operation, and applied it through
   `tauri_backend_apply`;
3. created/saved the local project, quit, relaunched, listed, and explicitly
   reopened `project:invented-loop-01`;
4. observed persisted `425 N`, no changed magnitude queued, zero pending/
   applied/editor/proposal state, and `not started; result rows=0`; and
5. ran mechanics to `MECHANICS_SOLVED` with 830 rows and an available viewport.

The backend route/model-result identity existed in state but was not reliably
visible. `AMENDMENT_2.md` authorized a bounded in-node observability repair.
`apps/desktop/src/App.tsx` now renders a global `Solve proof` row from existing
`solveJob.backend_job_seam`, `model.project.id`, `result.model_ref`, and result
row count. No schema, solver, persistence, backend, or result contract changed.

After rebuilding the `.app`, the post-reopen solve was repeated. The fresh AX
tree exposed exactly:

`seam=tauri_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830; generation=6; job=backend-solve-job-1`

alongside `MECHANICS_SOLVED`. Final executable SHA-256:
`3a95da3d2269b124734bc22a0e46d820e3fdb6870630159757ff8bab30507a8c`.
Screenshots are `PACKAGED_REOPENED_UNSOLVED_V6.jpeg` and
`PACKAGED_SOLVED_PROOF_V6.jpeg` in this instance root; earlier attempts and
reviewer-1/reviewer-2/reviewer-3/reviewer-4 FAIL returns remain preserved.

### Attempt-2 checks

- Focused App Vitest: PASS, 7 selected / 60 skipped.
- Focused Playwright: PASS, 2/2 desktop and compact profiles.
- Registered desktop test: PASS, 29 files / 533 tests.
- Registered desktop build: PASS.
- Fresh Tauri `.app` build: PASS.
- Actual packaged Computer Use journey: PASS.
- Harness self-check: PASS execution; existing repository-wide findings only.
- Harness pytest: PASS, 350 tests.
- Mandatory fresh 100% frozen-diff software-code-review 1: FAIL; found stale
  result/current-job pairing and incorrect PROFILE_PATH.
- Reviewer 2: FAIL after verifying 23/23 hashes; found the packaged native
  Analyze overlap race and overstated adverse Playwright wording.
- Reviewer 3: FAIL after verifying 31/31 hashes; found post-commit model
  invalidation, lost pre-start cancellation, and gate-only coverage overclaim.
- Reviewer 4: FAIL after verifying 40/40 hashes; found model/open invalidation
  dropped a requested pre-start cancellation before its delayed backend receipt.
- Detached-cancellation remediation: PASS focused/full/package checks and
  corrected packaged AX proof; review 5 PASS after 47/47 hashes, exact
  containment, exact profile/check evidence, and 100% frozen-diff review.

The attempt-2 product/deliverable/evidence snapshot is frozen for review. No
Git commit, push, PR, receipt, register, lifecycle transition, release claim,
or professional-reliance claim is authorized or performed by this manager.

Reviewer 1's FAIL is preserved under `TASK-REVIEW/`. `AMENDMENT_3.md` bound the
proof to a completed matching job, exact current model SHA-256, exact input-
manifest SHA-256, result run/model, and row count, while rejecting late model-
revision completion. The corrected packaged AX predicate includes
`job=backend-solve-job-1`, model SHA-256
`sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5`,
and input-manifest SHA-256
`0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3`.
`AMENDMENT_4.md` then added the synchronous generation gate after reviewer 2;
the final packaged predicate includes `generation=6`. `AMENDMENT_5.md` makes
model replacement invalidate before commit, retains immediate cancellation
until a backend receipt exists, and adds rendered native-event deferred tests
for real completed/failed/cancelled terminals and pre-start cancellation.
`AMENDMENT_6.md` retains a detached cancellation tombstone across model/open
invalidation until the delayed receipt and dispatches backend cancellation
exactly once without stale UI publication. Playwright remains a happy-path
visibility check only.

Fresh reviewer 5 found no actionable frozen-scope issue, closed all four prior
review findings, and returned `PASS / VALID_FOR_TERMINAL_FAN_IN`. Non-blocking
residuals are limited to sequential screenshots versus mocked timing adversity,
the narratively observed `425 N` not appearing in the final screenshot pair,
and an already-started invalidated backend poll continuing to terminal while
all stale publication/finalization is gated.

Requested Agent 0 action: accept N1 validated fan-in and route the complete
product/evidence tranche to CHANGE for its dependency-ordered commit and
closeout. No manager-owned Git, receipt, register, lifecycle, release, or
professional-reliance action is authorized or performed.

Attempt-2 terminal runtime telemetry integrity was PASS: 29 events across seven
matched manager/review sessions; attempt 2 final outcome was PASS. Native
token/context occupancy remained unavailable and is not inferred.

The exact terminal changed-path set is 50 paths: the 47 paths enumerated in
`TASK-REVIEW-5/FROZEN_NODE_DIFF.json`, that self-excluded manifest, and the
post-review control outputs `TASK-REVIEW-5/RETURN.md` and
`TASK-REVIEW-5/STATUS.json`. All are within the five declared graph write
surfaces; no receipt, register, DAG/decomposition, root-governance, Git, or PR
path is included.

## Post-node clean-sweep remediation — Amendment 7

CHANGE committed the validated node as
`a9b1fbef90f3bb9a894054c22f6fc77572fedd0d`, then stopped before receipt/PR
closeout when its mandatory clean DEC-025 sweep exposed a desktop-only
hit-testing regression. The commit-bound sweep
`SWEEP_20260820T214858Z_a9b1fbef90f3.json` passed Cargo, Python 902 tests, and
desktop Vitest 533 tests, then returned 20 Playwright passes / 2 failures. At
1440x920, the result-row and Render report clicks were intercepted by the
later workspace-status row. The compact variants passed. The earlier
`SWEEP_20260820T214752Z_a9b1fbef90f3.json` is preserved as environment-attempt
history only: bare Python lacked `jsonschema`.

Reviewer 6 had already returned `PASS / VALID_FOR_CHANGE_STAGING` over the
50-member terminal closeout snapshot before CHANGE created the node commit;
that review and reviewers 1-5 remain preserved as immutable history.

The root cause was layout overflow, not a control defect: the fixed-height app
grid could not satisfy the modeling core's 540px floor, the dock's 210px
floor, and the persistent chrome/status/footer at the desktop test height.
Targets scrolled underneath the later status row. `apps/desktop/src/styles.css`
now gives the modeling core a viewport-budgeted
`min-height: clamp(230px, 40vh, 540px)`. Status controls remain interactive;
no pointer-event bypass exists. `apps/desktop/e2e/r2-smoke.spec.ts` now asserts
that each formerly covered target has no rectangle overlap with the status
row, then performs the same real click and downstream assertions.

Adjacent-remediation checks against the node commit:

- exact focused Chromium desktop cases: PASS, 2/2;
- registered desktop Vitest: PASS, 29 files / 533 tests;
- registered desktop build: PASS, existing chunk-size warning only;
- full Playwright surface: PASS, 22/22 desktop and compact cases;
- registered harness pytest: PASS, 350 tests;
- registered always-on harness self-check: PASS execution, with only the
  existing repository-wide generated-view findings.

Exact profile selection over the adjacent tree additionally names
`piping-pytest` and `evidence-sweep` because the two immutable clean-sweep
summaries are new under `validation/evidence/**`. The accepted clean
node-commit sweep already records the Python surface PASS (902 tests); no
Python implementation changed. `evidence-sweep` is intentionally
`DEFERRED_TO_CHANGE_COMMIT_BOUND_PROOF`: HELP_HUMAN assigned the next clean
DEC-025 run to CHANGE after the adjacent commit, and an uncommitted sweep cannot
substitute for it. `ADJACENT_REMEDIATION_AFFECTED_CHECKS.json` freezes these
dispositions.

Runtime telemetry now summarizes 38 events across ten matched sessions with
status PASS; the original layout remediation, review-7 FAIL, and Amendment-8
control remediation are complete sessions.

Reviewer 7 verified 14/14 hashes, exact membership, exact six-check profile
selection, and 100% substantive coverage, with no product-code finding. Its
terminal `FAIL / ACTIONABLE_FINDINGS` is preserved: the graph omitted the two
exact Amendment-7 sweep-summary paths, so containment was only 12/14.
`AMENDMENT_8.md` adds only those two files to the graph's write targets; it does
not broaden a directory root or change product/tests/proof.

This manager return intentionally refreezes with reviewer 8 pending. A
different fresh read-only non-delegating reviewer must verify 100% of the
complete adjacent remediation diff against
`a9b1fbef90f3bb9a894054c22f6fc77572fedd0d`. If and only if reviewer 8 returns
PASS with hashes, exact membership/containment/profile, check evidence, and no
actionable finding, this section constitutes
`PASS_VALIDATED_TERMINAL_FAN_IN` without a post-review mutation. On that
condition, CHANGE may create the adjacent proof-loop fix commit and rerun clean
DEC-025. Otherwise CHANGE must remain stopped.
