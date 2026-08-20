# Managed read-only TASK brief — R8 generation-remediated integrated code review 3

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-3`
TaskProfile: NONE
TaskSkill: software-code-review
ApplyEdits: false
AllowedWriteTargets: none
AllowedTools:
- `tools/software_workflow/validate_change_scope.py`
- `tools/software_workflow/select_affected_checks.py`

ImplementationBrief:
- Review the complete R8 attempt-2 N1 snapshot closing only the DEL-07-06
  packaged saved-project edited-load GUI residual.
- Reviewer 1's stale-result/current-job FAIL is preserved under `TASK-REVIEW/`.
- Reviewer 2's overlapping native-run callback/evidence-wording FAIL is
  preserved under `TASK-REVIEW-2/`.
- `AMENDMENT_4.md` adds a synchronous solve-generation gate before the first
  await, rejects overlapping DOM/native dispatch, invalidates on model/open
  revision, and makes superseded same-ID completion/cancellation/failure/
  finally callbacks inert. Proof visibility remains bound to completed job,
  native seam, exact model/input hashes, result run/model, rows, and generation.
- No schema, backend, persistence, solver, operation, or result contract change
  is intended.

AcceptedBasis:
- Branch `codex/piping-product-20260820`.
- Source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, target R5, owner-authorized retry, Amendments 1-4,
  and preserved reviewer-1/reviewer-2 FAIL history.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  listed in `FROZEN_NODE_DIFF.json`.
- Verify every listed SHA-256, review 100% of listed paths, and review the
  manifest itself structurally. Manifest self-hash is explicitly N/A.

VerificationEvidence:
- Focused App Vitest PASS: 2 selected / 60 skipped, covering proof mismatch,
  run overlap rejection, delayed same-ID completion/cancellation/failure,
  and model/open invalidation.
- Focused Playwright PASS: 2/2 desktop and compact Chromium profiles; this is
  happy-path completed bound-visibility evidence only.
- Registered desktop test PASS: 29 files / 528 tests.
- Registered desktop build PASS.
- Fresh Tauri `.app` build PASS.
- Final executable SHA-256:
  `f802ab5e15488f480e2bf030ef27be0667e687dea3efef711b40c394532aae05`.
- Fresh packaged relaunch/reopen: SQLite project opened with zero pending,
  applied, editor, or proposal state and `not started; result rows=0`.
- Fresh packaged solve: `MECHANICS_SOLVED`, `tauri_backend_job`, matching
  project/result identity, 830 rows, `generation=4`,
  `job=backend-solve-job-1`, model SHA-256
  `sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5`,
  input-manifest SHA-256
  `0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3`.
- V3 screenshot hashes: reopened-unsolved
  `42939278df7a9d734667d2885177d6a315259bb2bd0ad21f9c43a6b1129ca50f`;
  solved-proof
  `d18c50495b05fbbfa2000c51690c985d62b62edd723d5fefe6f18df0d0b82869`.
- Harness self-check PASS execution; existing repository-wide findings only.
- Harness pytest PASS: 350 tests.
- `git diff --check` and `git diff HEAD --check` PASS before freeze.

PROFILE_PATH:
- `{WORKING_ROOT}/software-workflow.json`

Tasks:
- Load completely `agents/AGENT_TASK.md` and
  `skills/software-code-review/{SKILL.md,BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md}`.
- Validate scope and affected-check selection first with the exact executable
  JSON PROFILE_PATH above.
- Verify every frozen hash and review 100% of product source, Vitest,
  Playwright, SMOKE, all screenshots, DEL status/memory/run record, Amendments
  1-4, both prior reviewer FAIL returns/status/manifests/briefs, telemetry/work
  graph, manager return/handoff, and this control metadata.
- Trace DOM/native solve dispatch, start, rerun, cancel receipt/failure,
  terminal completion/cancellation/failure, model/open invalidation, publication,
  and finally. Treat reused adapter job IDs and delayed older callbacks as
  adversarial cases.
- Confirm proof cannot combine an old result or callback with a current solve,
  and an older finally cannot clear a newer run's state.
- Check correctness, regressions, contracts, scope, security/privacy, claim
  discipline, maintainability, and evidence sufficiency/truthfulness.
- Return only actionable findings with severity, exact path/line, impact,
  evidence, and remediation direction.

ExpectedReturn:
- `PASS` with no actionable findings, or `FAIL` with actionable findings.
- Include verified hash count, containment, affected-check coverage, 100%
  review statement, reviewer-1/reviewer-2 closure verdict, residual risk, and
  fan-in validity.

EXCLUSIONS:
- No writes, installs, checks, GUI action, commits, pushes, PRs, receipts,
  delegation, lifecycle actions, or release/professional-reliance claims.
- No browser/headless substitution for recorded packaged evidence.

RuntimeAttribution:
- Third different fresh read-only Agent 2 TASK instance on Codex;
  non-delegating.
