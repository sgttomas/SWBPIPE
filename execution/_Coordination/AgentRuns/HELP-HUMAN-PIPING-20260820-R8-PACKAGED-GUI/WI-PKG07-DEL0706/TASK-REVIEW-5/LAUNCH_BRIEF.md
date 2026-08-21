# Managed read-only TASK brief — R8 detached-cancellation review 5

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-5`
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
- Four prior review FAIL returns and Amendments 1-6 are preserved.
- The current repair retains a detached generation-bound cancellation
  tombstone after a pre-start Cancel followed by model/open invalidation. When
  the delayed start receipt creates a backend job, cancellation dispatches
  exactly once without restoring any stale visible job/result/proof state.
- The prior generation/job/model/hash/publication gates and synchronous
  pre-commit invalidation remain intact. Rendered native-event tests now cover
  the exact Run -> Cancel -> Open -> delayed receipt sequence in addition to
  late completed/failed/cancelled terminals and ordinary immediate cancel.
- Playwright is happy-path visibility only. No schema, backend, persistence,
  solver, operation, or result contract change is intended.

AcceptedBasis:
- Branch `codex/piping-product-20260820`; source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, target R5, owner retry, Amendments 1-6, and all prior
  reviewer histories.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  in `FROZEN_NODE_DIFF.json`.
- Verify all hashes, 100% of listed paths, twelve-screenshot history, and the
  manifest structure; manifest self-hash is explicitly N/A.

VerificationEvidence:
- Focused App Vitest PASS: 7 selected / 60 skipped.
- Registered desktop test PASS: 29 files / 533 tests.
- Registered desktop build PASS.
- Focused Playwright PASS: 2/2 desktop and compact profiles (happy path only).
- Fresh Tauri `.app` build PASS; executable SHA-256
  `3a95da3d2269b124734bc22a0e46d820e3fdb6870630159757ff8bab30507a8c`.
- Final package reopened zero pending/applied/editor/proposal and zero result
  rows, then exposed `MECHANICS_SOLVED`, `tauri_backend_job`, matching identity,
  830 rows, `generation=6`, `job=backend-solve-job-1`, model SHA-256
  `sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5`,
  and input-manifest SHA-256
  `0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3`.
- V6 screenshots: reopened-unsolved SHA-256
  `fc0cd500a2a5fc410febb106b867a5c155d23ea4bc0fa58f84599a77dd48c718`;
  solved-proof SHA-256
  `855738dabdd298fe4043f748003e4b292f1fc08122fa0b7bd0468a55e85b1996`.
- Harness self-check PASS execution; existing repository-wide findings only.
- Harness pytest PASS: 350 tests. Diff checks PASS.

PROFILE_PATH:
- `{WORKING_ROOT}/software-workflow.json`

Tasks:
- Load completely `agents/AGENT_TASK.md` and
  `skills/software-code-review/{SKILL.md,BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md}`.
- Validate containment/check selection with the exact JSON PROFILE_PATH.
- Verify every hash and review 100% of product/tests/Playwright/SMOKE, all
  screenshots, DEL status/memory/run record, Amendments 1-6, all prior review
  artifacts, telemetry/work graph, manager return/handoff, and control metadata.
- Trace native/DOM run and cancel before/during/after backend start; repeated
  cancel; Run -> Cancel -> apply/undo/redo/blank/open before delayed start
  receipt; detached receipt success/failure; same ID; completed/failed/cancelled
  terminal; cancellation receipt/failure; publication; and finally.
- Confirm detached cancellation is exactly once, bounded to its originating
  generation/job, and cannot alter current model result/manifest/analysis/job/
  proof/running. Confirm all four prior review findings are closed.
- Check correctness, regressions, contracts, scope, security/privacy, claim
  discipline, maintainability, and evidence truthfulness.

ExpectedReturn:
- PASS with no actionable findings, or FAIL with exact actionable findings.
- Include hash count, containment, check coverage, 100% statement, closure of
  all four prior review findings, residual risk, and fan-in validity.

EXCLUSIONS:
- No writes, installs, product checks, GUI action, commits, pushes, PRs,
  receipts, delegation, lifecycle, release, or professional-reliance claims.

RuntimeAttribution:
- Fifth different fresh read-only Agent 2 TASK instance; non-delegating.
