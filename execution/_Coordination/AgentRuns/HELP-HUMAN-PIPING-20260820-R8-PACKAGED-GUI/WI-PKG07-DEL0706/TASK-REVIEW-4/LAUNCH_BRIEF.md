# Managed read-only TASK brief — R8 synchronous-transition review 4

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-4`
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
- Three prior review FAIL returns and Amendments 1-5 are preserved.
- The current repair acquires solve generation before any await, rejects
  overlap, invalidates synchronously before every accepted model commit with a
  layout-effect backstop, binds publication/finally to the generation/job, and
  retains immediate cancel intent until it can dispatch exactly once to a
  backend receipt.
- Rendered-App native-event deferred tests execute late completed, failed, and
  cancelled terminals after open plus immediate pre-start cancel. Playwright is
  happy-path visibility only.
- No schema, backend, persistence, solver, operation, or result contract change
  is intended.

AcceptedBasis:
- Branch `codex/piping-product-20260820`; source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, target R5, owner retry, Amendments 1-5, and all prior
  reviewer histories.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  in `FROZEN_NODE_DIFF.json`.
- Verify all hashes, 100% of listed paths, six-plus screenshot history, and the
  manifest structure; manifest self-hash is explicitly N/A.

VerificationEvidence:
- Focused App Vitest PASS: 6 selected / 60 skipped.
- Registered desktop test PASS: 29 files / 532 tests.
- Registered desktop build PASS.
- Focused Playwright PASS: 2/2 desktop and compact profiles (happy path only).
- Fresh Tauri `.app` build PASS; executable SHA-256
  `f8ababde399eac15d130478c26ce135005cabb2dd14dd5a721741c84f61747dc`.
- Final package reopened zero pending/applied/editor/proposal and zero result
  rows, then exposed `MECHANICS_SOLVED`, `tauri_backend_job`, matching identity,
  830 rows, `generation=6`, `job=backend-solve-job-1`, model SHA-256
  `sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5`,
  and input-manifest SHA-256
  `0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3`.
- V5 screenshots: reopened-unsolved SHA-256
  `3ec4ebb977f9e147ad4c9cdbb2ab319732449ad1dc7425a1dc0ece58f4ed9e47`;
  solved-proof SHA-256
  `a585e26d8ecf58c155c3e724087377f50bf1e1a60e42b71d5ee501037fa3c23f`.
- Harness self-check PASS execution; existing repository-wide findings only.
- Harness pytest PASS: 350 tests. Diff checks PASS.

PROFILE_PATH:
- `{WORKING_ROOT}/software-workflow.json`

Tasks:
- Load completely `agents/AGENT_TASK.md` and
  `skills/software-code-review/{SKILL.md,BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md}`.
- Validate containment/check selection with the exact JSON PROFILE_PATH.
- Verify every hash and review 100% of product/tests/Playwright/SMOKE, all
  screenshots, DEL status/memory/run record, Amendments 1-5, all prior review
  artifacts, telemetry/work graph, manager return/handoff, and control metadata.
- Trace native/DOM run and cancel before/during/after backend start; repeated
  cancel; same ID; completed/failed/cancelled terminal; cancellation receipt/
  failure; apply/undo/redo/blank/open transition; publication; and finally.
- Confirm model commit invalidates before the semantic transition and no old
  callback can alter current model result/manifest/analysis/job/proof/running.
- Check correctness, regressions, contracts, scope, security/privacy, claim
  discipline, maintainability, and evidence truthfulness.

ExpectedReturn:
- PASS with no actionable findings, or FAIL with exact actionable findings.
- Include hash count, containment, check coverage, 100% statement, closure of
  all three prior review findings, residual risk, and fan-in validity.

EXCLUSIONS:
- No writes, installs, product checks, GUI action, commits, pushes, PRs,
  receipts, delegation, lifecycle, release, or professional-reliance claims.

RuntimeAttribution:
- Fourth different fresh read-only Agent 2 TASK instance; non-delegating.
