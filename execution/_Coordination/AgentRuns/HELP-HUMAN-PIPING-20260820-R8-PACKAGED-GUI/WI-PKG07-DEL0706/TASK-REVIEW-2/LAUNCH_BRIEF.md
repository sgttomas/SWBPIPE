# Managed read-only TASK brief — R8 remediated integrated code review 2

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-2`
TaskProfile: NONE
TaskSkill: software-code-review
ApplyEdits: false
AllowedWriteTargets: none
AllowedTools:
- `tools/software_workflow/validate_change_scope.py`
- `tools/software_workflow/select_affected_checks.py`

ImplementationBrief:
- Review the complete remediated R8 attempt-2 N1 snapshot closing only the
  DEL-07-06 packaged saved-project edited-load GUI residual.
- Reviewer 1's 16/16-hash FAIL is preserved in `TASK-REVIEW/RETURN.md`.
- The corrected product row is visible only when a current completed solve's
  job ID/backend seam, exact current model SHA-256, input-manifest SHA-256,
  result run/model identity, and row count all match. It is cleared on rerun or
  model revision; late solve completion after model/open change is rejected.
- No schema, backend, persistence, solver, operation, or result contract change
  is intended.

AcceptedBasis:
- Branch `codex/piping-product-20260820`.
- Source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, target R5, owner-authorized retry, and Amendments 1-3.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  listed in `FROZEN_NODE_DIFF.json`.
- Verify every listed SHA-256, review 100% of listed paths, and review the
  manifest itself structurally. Manifest self-hash is explicitly N/A.

VerificationEvidence:
- Focused App Vitest PASS: 2 selected / 59 skipped, covering completed,
  running, cancelling, cancelled, failed, job mismatch, changed model hash,
  and late/different result run.
- Focused Playwright PASS: 2/2 desktop and compact Chromium profiles; asserts
  job, model SHA-256, input-manifest SHA-256, identity, and rows.
- Registered desktop test PASS: 29 files / 527 tests.
- Registered desktop build PASS.
- Fresh Tauri `.app` build PASS.
- Final executable SHA-256:
  `2fe1e2af60b8f355507826c78a752a7f989be01a6eb8f15cb9c92d84fca868e0`.
- Fresh packaged relaunch/reopen: SQLite project opened with zero pending,
  applied, editor, or proposal state and `not started; result rows=0`.
- Fresh packaged solve: `MECHANICS_SOLVED`, `tauri_backend_job`, matching
  project/result identity, 830 rows, `job=backend-solve-job-1`, model SHA-256
  `sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5`,
  input-manifest SHA-256
  `0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3`.
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
  Playwright, SMOKE, both generations of screenshots, DEL status/memory/run
  record, Amendments 1-3, reviewer-1 FAIL history, telemetry/work graph,
  manager return/handoff, and this control metadata.
- Trace solve start/rerun/cancel/fail/model-change/open-project/completion paths.
  Confirm proof cannot combine an old result with a different current job or
  same-ID changed model, and late completion cannot republish stale proof.
- Check correctness, regressions, contracts, scope, security/privacy, claim
  discipline, maintainability, and evidence sufficiency/truthfulness.
- Return only actionable findings with severity, exact path/line, impact,
  evidence, and remediation direction.

ExpectedReturn:
- `PASS` with no actionable findings, or `FAIL` with actionable findings.
- Include verified hash count, containment, affected-check coverage, 100%
  review statement, reviewer-1 closure verdict, residual risk, and fan-in
  validity.

EXCLUSIONS:
- No writes, installs, checks, GUI action, commits, pushes, PRs, receipts,
  delegation, lifecycle actions, or release/professional-reliance claims.
- No browser/headless substitution for recorded packaged evidence.

RuntimeAttribution:
- Different fresh read-only Agent 2 TASK instance on Codex; non-delegating.
