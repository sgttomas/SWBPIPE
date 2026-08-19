# TASK-REVIEW-001 Return

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

FanInValidity: `INVALID` until all blocking findings are remediated, a new diff manifest is frozen, and a fresh independent review returns PASS.

## Blocking findings

1. **The frozen tranche contains writes outside the sealed manager authorization.** The manager launch brief limits writes to `projects/chirality-piping/tools/release/**`, conditional `projects/chirality-piping/validation/**`, the DEL-10-04 folder, and the run root (`instances/WI-PKG10-DEL1004-DEC093/LAUNCH_BRIEF.md:23-26`). The frozen manifest nevertheless includes modified `projects/chirality-piping/tests/test_evidence_sweep.py` and `projects/chirality-piping/tests/test_release_gate_records_script.py`; deterministic scope validation reports both as violations. The later intra-package work graph names `tests/**` at `WORK_GRAPH.md:13-17`, but a derivative child graph cannot widen the sealed parent brief. **Impact:** the implementation fails its explicit write-containment acceptance criterion and is not valid for manager fan-in. **Remediation:** obtain an authority-valid replacement/amendment of the sealed manager brief that explicitly authorizes the two test paths, or revert/move the unauthorized writes; then freeze a new manifest and re-review.

2. **Historical schema-v2 compatibility is fail-open rather than structurally validated.** `tools/release/run_evidence_sweep.py:687-694` returns after checking only that no surface says `execution_capability: ci`. A schema-v2 artifact with missing/duplicate/wrong-order surfaces, malformed surface entries, wrong decision basis, or incoherent statuses therefore returns `[]` and is counted as valid by `run_release_gate_records.py:682-688`. **Impact:** malformed or truncated v2 JSON can be selected and reported as clean commit-bound DEC-025 evidence; compatibility with valid historical summaries has become acceptance of arbitrary v2-shaped summaries. **Remediation:** validate the historical v2 contract explicitly (artifact/basis, Git binding shape, exact five surface identities/order, permitted host/sandboxed capabilities, commands/status/overall-status coherence) while preserving compatibility with the actual committed v2 corpus; add positive historical-fixture and malformed-v2 rejection tests.

3. **Partial sweep summaries remain eligible as full downstream sweep evidence.** `run_evidence_sweep.py:745-753` intentionally skips the five-surface requirement whenever `surface_selection` is an object, but `run_release_gate_records.py:682-688` filters only on `validate_summary(body) == []` and cleanliness. Thus a clean `--only-capability ci` (or other partial) artifact can be selected as the commit's DEC-025 sweep evidence even though its own metadata says `complete_dec025_sweep: false`. **Impact:** downstream criteria can consume a partial run under the full-sweep evidence label, and latest-artifact selection can displace a complete summary. **Remediation:** keep partial summaries valid as diagnostic artifacts if desired, but require absence of `surface_selection` (or an explicit true completeness invariant plus exact five surfaces) in every consumer that requires DEC-025 full-sweep evidence; add a regression proving a newer partial artifact cannot displace a complete one.

4. **The release-packaging authenticity chain does not invoke the new summary validator.** `tools/release/package_release_artifact.py:321-360` parses a caller-supplied sweep and checks only dirty state, commit/evidence-only ancestry, and `overall_status == "pass"`; it never calls `sweep_module.validate_summary`. A schema-v3 summary whose CI binding has the wrong `head_sha`, wrong workflow/project proof, or other invalid binding can therefore still reach `CHAIN_VERIFIED`. It can also accept a partial overall-pass artifact. **Impact:** D-65's exact-SHA rejection can be bypassed at a release-artifact downstream consumer even though the release-gate-record consumer rejects the same summary. **Remediation:** validate the parsed sweep before reading it as chain evidence, return a distinct invalid-sweep status, require a complete five-surface summary, and add packaging regressions for mismatched CI head and partial-sweep rejection.

## Non-blocking findings

None.

## Scope and evidence audit

- Frozen basis: base `219f695d348f1d83ba904ef4dd38781636b423a6`; all 12 SHA-256 values in `N1_DIFF_MANIFEST.md` matched exactly.
- Coverage: 100% of the six tracked file diffs (573 insertions, 36 deletions) and all six listed untracked files were inspected. Post-freeze child runtime files were excluded exactly as the manifest states.
- Authority/contracts read: root and project `AGENTS.md`; complete `agents/AGENT_TASK.md`; all four software-code-review contracts; D-65 ruling; DEC-093 row; manager and child briefs; live workflow, Playwright viewport configuration, and software workflow profile.
- Exact D-65 emitter behavior is otherwise present: fixed workflow path, positive run ID/attempt, full lowercase head SHA, success-only conclusion, affirmative registered-spec field, exact two viewport projects, pre-surface and pre-write SHA matching, same-SHA higher attempts, unchanged default host capability/commands, and schema-v2 prohibition on CI claims.
- Evidence supplied by the brief/run record: 48/48 focused evidence-sweep tests; 20/20 focused release-gate tests with the jsonschema-only case excluded; combined 67 passes with one `jsonschema` environment failure; `py_compile` PASS; `git diff --check` PASS. No raw command logs were frozen, and this read-only reviewer did not rerun tests.
- Affected-check selection for project-relative paths requires `evidence-sweep`, `piping-pytest`, `harness-pytest`, and `harness-self-check`. The current evidence is focused only; the orchestration plan explicitly defers the full sweep/repository/harness gates to later closeout, so they remain required after remediation and re-review.

## Residual risk

- The review was static/read-only and did not use network or host capabilities, so it did not independently authenticate an Actions run or execute Playwright. The CI binding remains caller-supplied evidence whose external provenance must be checked by the governing closeout process.
- The jsonschema-dependent release-gate case remains unexecuted in the reported environment.
- Full registered checks and committed-HEAD DEC-025 evidence remain outstanding by design.

## Tools Used

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `shasum`, `rg`, `sed`, `nl`, `wc`, `ls`, `find`, `head`, and shell path-existence inspection

`python3 tools/software_workflow/compare_structured.py` and `python3 tools/software_workflow/verify_generated_manifest.py` were not used because no structured before/after artifact or generated manifest was declared.

## Tool Policy Compliance

`PASS` — scope validation ran before implementation judgment; only effective-allowlist tools and brief-authorized read-only shell/Git inspection were used; no unregistered test/build command, install, edit to implementation/deliverable state, network, host execution, delegation, commit, or push occurred.

## Outputs Produced

- This `RETURN.md`.
- Final child `STATUS.json`.

## Missing

- Remediated implementation and a newly frozen manifest.
- Fresh independent PASS review after remediation.
- Deferred full registered closeout checks.

## Needs Human Ruling

- An authority-valid scope amendment is needed if the two `tests/**` modifications are to be retained; the intra-package work graph alone cannot widen the sealed manager launch brief.

## Dependency Notes

None.

## Proposed Changes

- Apply the remediation directions in blocking findings 1-4, regenerate focused evidence, freeze a new manifest, and dispatch a fresh read-only review.

## Model attribution

Inherited GPT-5-based Codex runtime; fresh reviewer context from the implementation manager; no model substitution recorded.

No lifecycle, release, publication, or professional-reliance act is performed by this review.
