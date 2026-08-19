# TASK-REVIEW-004 Return

RUN_STATUS: `FAILED`

ControlSurface: `INLINE`

TaskProfile: `NONE`

TaskSkill: `software-code-review`

ScopePath: `/Users/ryan/.codex/worktrees/4918/chirality/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline`

ResolvedSkillPath: `/Users/ryan/.codex/worktrees/4918/chirality/skills/software-code-review`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

AllowedTools: `python3 tools/software_workflow/validate_change_scope.py:*`, `python3 tools/software_workflow/select_affected_checks.py:*`, `python3 tools/software_workflow/compare_structured.py:*`, `python3 tools/software_workflow/verify_generated_manifest.py:*`, plus brief-authorized read-only shell/Git inspection

RuntimeOverrides: none

WriteAuthorization: `RUN_RECORD_ONLY` — only this managed `RETURN.md` and child `STATUS.json` were written.

ReviewVerdict: `FAIL`

FanInValidity: `INVALID` until the blocking finding is remediated, a new integrated manifest is frozen, and a fresh independent review returns PASS.

## Blocking findings

1. **A contradictory dirty-path record is accepted as clean evidence by both downstream consumers.** `_validate_git_state()` validates only that `git.dirty_paths` is an array of strings (`projects/chirality-piping/tools/release/run_evidence_sweep.py:683-709`); it never requires `git.working_tree_dirty` to equal `bool(git.dirty_paths)` when status capture succeeds. Consequently, an otherwise valid full summary with `status_capture_failed: false`, `working_tree_dirty: false`, and `dirty_paths: ["modified-file"]` passes `is_complete_sweep_summary()`. The release-gate selector then classifies it as clean because `_is_clean_commit_bound()` checks only the false dirty flag (`projects/chirality-piping/tools/release/run_release_gate_records.py:660-688`), and release packaging likewise checks only the false dirty flag before reaching `CHAIN_VERIFIED` (`projects/chirality-piping/tools/release/package_release_artifact.py:348-368`). **Impact:** malformed or tampered sweep evidence that explicitly names worktree deltas can still be selected as clean commit-bound gate evidence and can verify the packaging authenticity chain, violating the invalid/dirty-evidence exclusion required by this brief. **Remediation:** make `_validate_git_state()` enforce the emitted invariant `working_tree_dirty == bool(dirty_paths)` whenever `status_capture_failed` is false (and preferably require `dirty_paths == []` when capture failed); add validator, release-gate-selection, and packaging regressions for the false-flag/non-empty-path contradiction. This is compatible with the committed v2 corpus: a read-only audit found no v2 artifact violating either invariant.

## Non-blocking findings

None.

## Scope and evidence audit

- Frozen basis: base `219f695d348f1d83ba904ef4dd38781636b423a6`; all 28 SHA-256 values in `N1_DIFF_MANIFEST_V4.md` independently matched exactly before substantive review (`28/28`, no missing files or mismatches).
- Coverage: 100% of every frozen input was inspected: all eight tracked diffs (998 insertions, 42 deletions) and all 20 listed untracked files in full. `N1_DIFF_MANIFEST_V4.md` and TASK-REVIEW-004's own launch/status/return are post-freeze review evidence, not integrated-diff inputs.
- Scope: `python3 tools/software_workflow/validate_change_scope.py` returned `PASS` for all 28 paths against the manager launch brief plus V2/V3 amendments; no write-scope drift was found.
- Authority/contracts read: complete root and project `AGENTS.md`; complete `agents/AGENT_TASK.md`; all four `software-code-review` contracts; D-65/DEC-093; manager launch brief and V2/V3 amendments; live workflow and Playwright viewport contracts; and the accepted prior-review/remediation evidence. TASK-REVIEW-002's required frozen files were read only to satisfy 100% manifest coverage and its no-verdict disposition; no TASK-REVIEW-002 context or result was reused or inferred.
- The D-65/DEC-093 emitter path is otherwise correctly implemented: closed exact binding fields, fixed workflow path, positive run ID/attempt, full lowercase head SHA, success-only conclusion, affirmative registered-spec execution, exact dual viewport list, pre-surface and pre-write SHA matching, same-SHA higher attempts, and unchanged default host capability/commands.
- Historical v2 and current v3 structural validation, valid partial-diagnostic preservation, full-evidence partial exclusion, Git-unverified exclusion, malformed nested-Git rejection, and parseable UTC `started_utc` sorting are present, subject to the contradictory dirty-state bypass above. All committed v2 files were read-only checked for the proposed dirty-state invariant, with no incompatibility found.
- Affected-check selection requires `evidence-sweep`, `piping-pytest`, `harness-pytest`, and `harness-self-check`. The supplied evidence reports 52/52 evidence-sweep tests (including all 284 committed v2 summaries), 23/23 selected release-gate tests with one jsonschema-only case deselected, 18/18 packaging tests, combined 93 PASS, `py_compile` PASS, and `git diff --check` PASS. I independently confirmed the frozen tracked diff passes `git diff --check`; per the sealed exclusion, I did not rerun tests or builds.

## Residual risk

- Review was static and read-only. The CI binding is caller-supplied evidence; external Actions provenance remains for the governing closeout process to authenticate.
- The reported jsonschema-dependent release-gate case remains unavailable in the stated Python 3.13 environment.
- Full registered checks and committed-HEAD DEC-025 evidence remain outstanding by design.

## Tools Used

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `shasum`, `awk`, `jq`, `rg`, `sed`, `nl`, `wc`, `ls`, `find`, and shell inspection

`python3 tools/software_workflow/compare_structured.py` and `python3 tools/software_workflow/verify_generated_manifest.py` were not used because no structured before/after artifact or generated manifest was declared.

## Tool Policy Compliance

`PASS` — scope validation was the first registered workflow tool used; only effective-allowlist tools and brief-authorized read-only shell/Git inspection were used. No test/build command, install, implementation/deliverable edit, network access, host capability, delegation, commit, push, PR, receipt, or lifecycle/release/publication action occurred.

## Outputs Produced

- This `RETURN.md`.
- Final child `STATUS.json`.

## Missing

- Remediation of the blocking dirty-state-consistency finding.
- A newly frozen integrated manifest after remediation.
- Fresh independent PASS review.
- Deferred full registered closeout checks.

## Needs Human Ruling

None; the finding is an implementation defect inside the existing authorization fence.

## Dependency Notes

None.

## Proposed Changes

- Enforce Git dirty-flag/path consistency in the summary validator, add focused downstream regressions, regenerate evidence, freeze a new manifest, and dispatch a fresh read-only review.

## Model attribution

Inherited GPT-5-based Codex runtime; freshly instantiated independently of the implementer and prior reviewers; no model substitution recorded.

No lifecycle, release, publication, or professional-reliance act is performed by this review.
