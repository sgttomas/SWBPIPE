# TASK-REVIEW-003 Return

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

FanInValidity: `INVALID` until every blocking finding is remediated, a new integrated manifest is frozen, and a fresh independent review returns PASS.

## Blocking findings

1. **Release packaging can mark a Git-unverified sweep as `verified`.** `tools/release/run_evidence_sweep.py:683-709` deliberately treats `status_capture_failed: true`, `working_tree_dirty: null`, and a still-available full commit hash as a structurally valid diagnostic Git state; the sweep itself then says that such a summary is not commit-bound and exits nonzero at `run_evidence_sweep.py:1293-1299`. `is_complete_sweep_summary()` nevertheless returns true for that unselected five-surface diagnostic (`run_evidence_sweep.py:873-879`). In the packaging consumer, `package_release_artifact.py:334-348` validates completeness, checks whether the *current packaging record* Git state is unverified, but checks the sweep Git state only with the truthy dirty test at line 347. For the valid diagnostic state above, `null` is false, so a pass summary whose captured commit equals `record_commit` falls through to `CHAIN_VERIFIED` at lines 350-365. **Impact:** a sweep that explicitly failed its own DEC-025 commit-binding gate can satisfy the DEC-057 packaging authenticity chain. **Remediation:** reject `sweep_module.git_state_unverified(sweep["git"])` before commit/dirty evaluation (preferably with an explicit sweep-unverified chain status), retain the existing dirty rejection, and add a regression with `status_capture_failed=true`, `working_tree_dirty=null`, and a matching non-null commit.

2. **The release-gate consumer dereferences malformed Git state before invoking the strict summary validator.** `_load_json_artifacts()` admits every parsed object with the sweep artifact tag (`tools/release/run_release_gate_records.py:645-657`). `select_sweep_evidence()` then evaluates `(body.get("git") or {}).get("commit_hash")` at lines 676-681 before the new `is_complete_sweep_summary()` filter at lines 682-687. A parseable tagged artifact with a truthy non-object `git` value such as `"malformed"` therefore raises `AttributeError` instead of being rejected by `_validate_git_state()`. **Impact:** one invalid sweep JSON in the evidence directory can abort release-gate record generation, contrary to the new invalid-summary exclusion contract. **Remediation:** run `is_complete_sweep_summary()` before any nested Git dereference, then compute matching/clean candidates only from validated bodies; add a regression containing a correctly tagged artifact whose `git` member is a scalar or array.

3. **The validator leaves the downstream sort key untyped, so a summary it calls complete can still crash evidence selection.** `validate_summary()` at `tools/release/run_evidence_sweep.py:748-870` never validates `started_utc`, while `select_sweep_evidence()` sorts validated clean summaries directly on that value at `run_release_gate_records.py:688-690`. A structurally complete, clean summary with `started_utc` set to an object passes `is_complete_sweep_summary()`; when it is present with an ordinary string-timestamp artifact at the same commit, Python attempts to compare unlike sort-key types and raises `TypeError`. **Impact:** the purported strict validator does not make its downstream consumer safe, and one malformed timestamp can block gate-record generation. **Remediation:** require a string timestamp in the summary contract (and preferably validate its ISO-8601 shape) before declaring the summary complete, then add mixed valid/invalid timestamp selection coverage.

## Non-blocking findings

None.

## Scope and evidence audit

- Frozen basis: base `219f695d348f1d83ba904ef4dd38781636b423a6`; all 24 SHA-256 values in `N1_DIFF_MANIFEST_V3.md` independently matched exactly before substantive review.
- Coverage: 100% of all eight tracked diffs (920 insertions, 40 deletions) and all 16 listed untracked files were inspected. `N1_DIFF_MANIFEST_V3.md` and TASK-REVIEW-003's own launch/status/return are post-freeze evidence and were not treated as integrated-diff inputs.
- Scope: deterministic `validate_change_scope.py` evaluation returned PASS for every frozen path against the manager launch brief plus V2/V3 amendments. No scope violations were found.
- Authority/contracts read: complete root and project `AGENTS.md`; complete `agents/AGENT_TASK.md`; all four `software-code-review` contracts; D-65/DEC-093; manager launch brief and amendments; live workflow and Playwright viewport configuration; prior failed-review remediation basis; and TASK-REVIEW-002's preserved interrupted/no-verdict disposition. No TASK-REVIEW-002 result or context was reused as review judgment.
- The core D-65 emitter path is otherwise present: closed exact binding fields, fixed workflow path, positive run ID/attempt, full lowercase head SHA, success-only conclusion, affirmative registered-spec execution, exact dual viewport list, pre-surface and pre-write SHA matching, same-SHA higher attempts, non-success rejection, and unchanged default host capability/commands.
- Historical v2 and current v3 surface/status validation, valid partial diagnostic preservation, full-evidence exclusion in the release-gate selector, and invalid/partial packaging rejection are present, subject to the three failure paths above.
- Affected-check selection over the 24 project-relative paths requires `evidence-sweep`, `piping-pytest`, `harness-pytest`, and `harness-self-check`. The supplied evidence reports 51/51 evidence-sweep tests, 21/21 selected release-gate tests with one jsonschema-only case deselected, 18/18 packaging tests, combined 90 PASS, `py_compile` PASS, and `git diff --check` PASS. The full registered sweep and harness checks are explicitly deferred to governing closeout.
- Per the sealed exclusion, this reviewer did not run tests/builds, install anything, use network or host capability, authenticate an Actions run, edit implementation/deliverable state, delegate, commit, push, open a PR, or perform a lifecycle/release/publication act.

## Residual risk

- Review was static/read-only. The CI binding is caller-supplied evidence; external Actions provenance remains for the governing closeout process to authenticate.
- The reported jsonschema-dependent release-gate case remains unavailable in the stated Python 3.13 environment.
- Full registered checks and committed-HEAD DEC-025 evidence remain outstanding by design.

## Tools Used

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `shasum`, `awk`, `rg`, `sed`, `nl`, `wc`, and shell inspection

`python3 tools/software_workflow/compare_structured.py` and `python3 tools/software_workflow/verify_generated_manifest.py` were not used because no structured before/after artifact or generated manifest was declared.

## Tool Policy Compliance

`PASS` — scope validation was the first registered workflow tool used; only effective-allowlist tools and brief-authorized read-only shell/Git inspection were used. No test/build command or other excluded action was run.

## Outputs Produced

- This `RETURN.md`.
- Final child `STATUS.json`.

## Missing

- Remediation of the three blocking findings.
- A newly frozen integrated manifest after remediation.
- Fresh independent PASS review.
- Deferred full registered closeout checks.

## Needs Human Ruling

None; the findings are implementation defects inside the existing authorization fence.

## Dependency Notes

None.

## Proposed Changes

- Apply the remediation directions in blocking findings 1-3, extend focused regression coverage, regenerate evidence, freeze a new manifest, and dispatch a fresh read-only review.

## Model attribution

Inherited GPT-5-based Codex runtime; freshly instantiated independently of the implementer and prior reviewers; no model substitution recorded.

No lifecycle, release, publication, or professional-reliance act is performed by this review.
