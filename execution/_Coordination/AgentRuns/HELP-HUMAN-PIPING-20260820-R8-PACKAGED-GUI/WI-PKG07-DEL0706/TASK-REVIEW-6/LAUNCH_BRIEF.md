# Managed read-only TASK brief — R8 terminal closeout review 6

ControlSurface: INLINE
RequestedBy: WORKING_ITEMS `WI-PKG07-DEL0706`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-6`
TaskProfile: NONE
TaskSkill: software-code-review
ApplyEdits: false
AllowedWriteTargets: none
AllowedTools:
- `tools/software_workflow/validate_change_scope.py`
- `tools/software_workflow/select_affected_checks.py`

ImplementationBrief:
- Review the complete current terminal R8 N1 tree after reviewer-5 PASS and
  mechanical closeout reconciliation.
- Reviewer 5 froze 47 paths before its own RETURN/STATUS and terminal manager,
  handoff, graph, run-record, and telemetry wording existed. CHANGE correctly
  detected that the DEL run record now differs from the reviewer-5 freeze.
- The exact post-review DEL run-record mutation is limited to changing the
  reviewer-5 row from PENDING to its observed PASS facts and appending the
  reviewer-5 closure/non-blocking-residual paragraph.
- Other post-review changes are reviewer-5 RETURN/STATUS, manager/handoff/graph
  terminal status, review/manager terminal telemetry, and the regenerated PASS
  runtime summary. No product source, test, screenshot, executable, GUI proof
  predicate, deliverable status/memory, or acceptance claim changed after the
  reviewer-5 product freeze.
- The frozen set is exactly the 50 current changed paths that existed before
  this review-6 brief and manifest. The review-6 control files are explicitly
  outside the 50-member set and must not be treated as product divergence.

AcceptedBasis:
- Branch `codex/piping-product-20260820`; source-tree base HEAD
  `91b92f949b8d808b61268baca9a54f4f716c99ff`.
- Receipt-118, DAG-009, target R5, owner retry, Amendments 1-6, reviewers 1-4
  preserved FAILs/remediation, and reviewer-5 PASS.

DiffBasis:
- `git diff 91b92f949b8d808b61268baca9a54f4f716c99ff --` plus every untracked path
  in `FROZEN_NODE_DIFF.json`.
- Verify all 50 hashes and exact membership. Current working state should equal
  those 50 frozen members plus only this review-6 brief and its self-excluded
  manifest before reviewer outputs are recorded.

VerificationEvidence:
- Focused App Vitest PASS: 7 selected / 60 skipped.
- Registered desktop test PASS: 29 files / 533 tests.
- Registered desktop build PASS.
- Focused Playwright PASS: 2/2 desktop and compact profiles, happy path only.
- Tauri `.app` build PASS; executable SHA-256
  `3a95da3d2269b124734bc22a0e46d820e3fdb6870630159757ff8bab30507a8c`.
- Final V6 package reopened zero pending/applied/editor/proposal and zero
  result rows, then exposed `MECHANICS_SOLVED`, `tauri_backend_job`, matching
  identity, 830 rows, generation 6, job ID, and exact model/input hashes.
- Harness self-check PASS execution with existing findings retained; harness
  pytest PASS 350 tests.
- Reviewer 5 PASS: 47/47 frozen hashes, exact 48-path containment at that
  stage, exact profile/check evidence, 100% review, all four prior findings
  closed, no actionable product finding.
- Terminal runtime summary PASS: 29 events, seven matched sessions.

PROFILE_PATH:
- `{WORKING_ROOT}/software-workflow.json`

Tasks:
- Load completely `agents/AGENT_TASK.md` and
  `skills/software-code-review/{SKILL.md,BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md}`.
- Verify the 50-member manifest, current exact membership, declared write-root
  containment, and affected checks using the exact JSON profile.
- Review 100% of the 50 frozen files including all product/tests, twelve
  screenshots, DEL status/memory/run record, Amendments 1-6, all five prior
  review packages including reviewer-5 RETURN/STATUS, manager/handoff/graph,
  terminal runtime events/summary, SMOKE, and all control metadata.
- Reconcile reviewer-5 frozen-versus-current divergence and confirm every
  post-review mutation is mechanical truthful closeout only.
- Recheck all previously reviewed concurrency/cancellation/proof logic enough
  to ensure no product member changed relative to reviewer-5 hashes and no
  terminal evidence overclaim was introduced.
- Check correctness, regressions, contracts, scope, security/privacy, claim
  discipline, maintainability, and evidence truthfulness.

ExpectedReturn:
- PASS with no actionable findings, or FAIL with exact actionable findings.
- Include 50/50 hash result, exact current-membership/containment verdict,
  100% coverage, exact post-review divergence classification, prior-review
  closure, residual risk, and terminal CHANGE recommendation.

EXCLUSIONS:
- No writes, installs, checks, GUI action, commits, pushes, PRs, receipts,
  delegation, lifecycle, release, or professional-reliance claims.

RuntimeAttribution:
- Sixth different fresh read-only Agent 2 TASK instance; non-delegating.
