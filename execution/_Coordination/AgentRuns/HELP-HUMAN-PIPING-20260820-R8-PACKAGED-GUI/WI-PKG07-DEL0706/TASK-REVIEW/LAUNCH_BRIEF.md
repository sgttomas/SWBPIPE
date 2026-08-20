# Managed read-only TASK brief — R8 attempt-2 integrated code review

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW`
TaskProfile: NONE
TaskSkill: software-code-review
ApplyEdits: false
AllowedWriteTargets: none
AllowedTools:
- `tools/software_workflow/validate_change_scope.py`
- `tools/software_workflow/select_affected_checks.py`

ImplementationBrief:
- Review the complete R8 attempt-2 N1 integrated snapshot that closes the
  DEL-07-06 packaged saved-project edited-load GUI residual.
- Product change is bounded to an always-visible Solve proof status row backed
  by existing `solveJob.backend_job_seam`, `model.project.id`,
  `result.model_ref`, and result-row count; no contract change is intended.
- Vitest and Playwright assertions cover browser-fixture identity; actual
  rebuilt packaged Tauri AX evidence covers `tauri_backend_job`, matching
  identity, `MECHANICS_SOLVED`, and 830 rows after reopen.

AcceptedBasis:
- Branch `codex/piping-product-20260820`.
- Source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, current target R5.
- Owner-authorized retry and in-node observability repair recorded in
  `AMENDMENT_1.md` and `AMENDMENT_2.md`.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  listed in `FROZEN_NODE_DIFF.json`.
- Verify every listed SHA-256 before review; review 100% of listed paths.
- Treat this launch brief as control metadata; it is hash-bound by the frozen
  manifest. The manifest cannot hash itself and is checked structurally.

VerificationEvidence:
- Focused App Vitest PASS: 1 selected, 59 skipped.
- Focused Playwright PASS: 2/2 desktop and compact Chromium profiles.
- Registered desktop test PASS: 29 files, 526 tests.
- Registered desktop build PASS.
- Fresh `npm run tauri -- build --bundles app` PASS.
- Final executable SHA-256:
  `e5260300876e635be42bbd50c60999d1e45c18f006967dfac375699972d2652e`.
- Actual packaged GUI AX predicate:
  `seam=tauri_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830`, alongside
  `MECHANICS_SOLVED`.
- Harness self-check PASS execution; existing repository-wide findings only.
- Harness pytest PASS: 350 tests.
- `git diff --check` and `git diff HEAD --check` PASS before freeze.

PROFILE_PATH:
- `{REPO_ROOT}/docs/SOFTWARE_WORKFLOW_PROFILE.md`

Tasks:
- Load `agents/AGENT_TASK.md` and the complete
  `skills/software-code-review/{SKILL.md,BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md}`.
- Validate changed-path containment and affected-check selection first using
  the skill-declared tools.
- Verify all frozen hashes and inspect 100% of product, tests, evidence,
  deliverable status/memory/run record, and manager coordination diff.
- Trace the StatusBar prop/helper through result lifecycle and confirm the row
  cannot present stale or mismatched proof as a match.
- Check correctness, regressions, contract/scope discipline, privacy/claim
  boundaries, maintainability, and sufficiency/truthfulness of verification.
- Return findings only when actionable, with severity, exact path/line, impact,
  evidence, and remediation direction.

ExpectedReturn:
- `PASS` with no actionable findings, or `FAIL` with actionable findings.
- Explicit hash-verification count, changed-path containment verdict,
  affected-check verdict, 100% coverage statement, residual risk, and whether
  the return is valid for WORKING_ITEMS fan-in.

EXCLUSIONS:
- No writes, installs, commits, pushes, PRs, receipts, lifecycle actions, or
  release/professional-reliance claims.
- Do not rerun the packaged GUI or substitute browser/headless evidence.
- Do not delegate.

RuntimeAttribution:
- Fresh read-only Agent 2 TASK instance on Codex; non-delegating.
