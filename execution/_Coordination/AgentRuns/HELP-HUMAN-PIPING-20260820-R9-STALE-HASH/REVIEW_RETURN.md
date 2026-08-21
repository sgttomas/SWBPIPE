# TASK software-code-review return — N1 stale claimed-model-hash gate

RUN_STATUS: `SUCCESS`

ControlSurface: `INLINE`

TaskProfile: `NONE`

TaskSkill: `software-code-review`

ScopePath: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`

ResolvedSkillPath: `skills/software-code-review`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

AllowedTools: `python3 tools/software_workflow/validate_change_scope.py:*`, `python3 tools/software_workflow/select_affected_checks.py:*`, `python3 tools/software_workflow/compare_structured.py:*`, `python3 tools/software_workflow/verify_generated_manifest.py:*`, plus brief-authorized read-only file/Git inspection

RuntimeOverrides: none

WriteAuthorization: `ALLOWED_WRITE_TARGETS` — only this managed `REVIEW_RETURN.md` and `REVIEW_STATUS.json`; no product, deliverable, shared, or Git writes.

ReviewVerdict: `PASS`

FanInValidity: `VALID` for the mandatory independent code-review gate. The separately ordered DEC-025 sweep and closeout checks remain required before publication.

## Blocking findings

None.

## Non-blocking findings

None.

## Scope and frozen-basis audit

- Base and current activation `HEAD`: `cd823be3badd034c86390f2707dcf01952c782f0` on `codex/piping-product-20260820b`; the candidate is an uncommitted worktree diff against that base.
- All 19 SHA-256 values listed in `FROZEN_DIFF_MANIFEST.md` matched independently before substantive review and again at terminalization. No frozen input was missing or changed during review.
- Coverage was 100%: every tracked patch and every listed untracked file was read, including the complete Rust gate and tests, native and browser/Wasm corpus runners, cases 15 and 79–81, schema/test coupling, deliverable records, and all activation/work-graph/brief records. Directly relevant callers and contracts were also traced through the Tauri command boundary, browser adapter, Wasm JSON boundary, hash service, and TypeScript outcome/hash types.
- `python3 tools/software_workflow/validate_change_scope.py` returned `PASS` against the activation fence plus the frozen v2 schema/test amendment. No changed candidate path is outside effective authorization.
- `git diff --check` returned `PASS`; original corpus cases 01–03 have no diff from `HEAD`.

## Correctness and contract assessment

- Matching path: only a non-empty `algorithm=sha256`, `canonicalization=rfc8785_jcs`, `payload_scope=model_payload`, and well-formed lowercase `sha256:` digest can reach equality comparison. Equality with the backend hash yields `claimed_model_hash_matches_current_backend_model` and leaves ordinary validation/application behavior available.
- Stale/mismatch path: a well-formed unequal digest emits stable blocking diagnostic `OP-CLAIMED-MODEL-HASH-MISMATCH`, yields `claimed_model_hash_mismatch_current_backend_model`, suppresses diff/application, returns `application_status=blocked` for apply mode, and cannot return an applied model, applied-model hash, or user-application acceptance.
- Malformed path: non-object, missing/non-string/empty required comparison fields, malformed prefix, wrong length, uppercase, or non-hex digest fail closed with `OP-CLAIMED-MODEL-HASH-METADATA-INVALID`, `schema_validation=blocked`, and truthful invalid binding status.
- Unsupported path: an incomparable algorithm, canonicalization, or payload scope fails closed with `OP-CLAIMED-MODEL-HASH-METADATA-UNSUPPORTED`, `schema_validation=blocked`, and truthful unsupported binding status.
- Absent/no-claim path: `None`/JSON `null` retains `no_claimed_hash_before_state_check_is_the_staleness_guard`; existing before-state validation remains the guard and no new block is introduced.
- No applied-model or acceptance reuse is possible on any blocking path because application is guarded by the accumulated blocking state and acceptance is derived only from the presence of a newly applied model.
- The operation-outcome JSON Schema enum and its focused Python expectation contain exactly the matched, mismatched, invalid, unsupported, and no-claim terminal vocabulary. The TypeScript transport type remains intentionally open-string and requires no migration.
- Professional-boundary output is unchanged: human review remains required and all software compliance, certification, sealing, approval, and authentication claims remain false.

## Native/Wasm corpus and verification assessment

- The corpus contains exactly 81 JSON cases. Both the native Rust runner and the browser adapter/direct-Wasm runner assert that exact completeness floor and execute every case discovered from the same directory.
- Case 15 now proves a genuinely matching current RFC8785/JCS model hash. Cases 79–81 prove stale mismatch, malformed evidence, and unsupported comparison metadata, respectively, with null applied model/hash and no application acceptance. The direct-Wasm and adapter lanes compare the same full semantic projection as native, including binding status and backend model hash.
- Affected-check selection returned `desktop-build`, `desktop-test`, `evidence-sweep`, `harness-pytest`, `harness-self-check`, and `piping-pytest`.
- Frozen verification evidence reports PASS for the focused Rust gate, full operation-applier crate, focused outcome-schema tests, Wasm build plus 171 focused adapter/direct-Wasm cases, 539 desktop tests, desktop production build, 902 piping pytest tests using pinned dev requirements, harness self-check, and 350 harness pytest tests. The initial dependency-incomplete Python attempt is truthfully recorded and superseded by the pinned-requirements PASS.
- The DEC-025/evidence sweep was not rerun by this reviewer and remains intentionally ordered after independent review. No supplied evidence claims otherwise.

## Residual risk

- This was a static, independent, read-only review. It relied on the frozen run record for dynamic test results and did not rerun builds or tests under the review skill's tool policy.
- The registered DEC-025 sweep, including host/CI surface 4, remains outstanding by design and must bind to the eventual committed candidate before closeout.
- Case 15's regenerated expectation and new cases 79–81 remain explicitly marked pending their own human corpus-review entry. That is a lifecycle/evidence disposition, not a code-review finding and is not inherited from earlier corpus acceptance.

## Tools Used

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `shasum`, `rg`, `sed`, `cat`, `wc`, `sort`, `head`, `tail`, and shell inspection

`python3 tools/software_workflow/compare_structured.py` and `python3 tools/software_workflow/verify_generated_manifest.py` were not used because no structured before/after artifact or generated-artifact manifest was declared.

## Tool Policy Compliance

`PASS` — scope validation was the first substantive registered workflow tool, followed by affected-check selection. Only effective-allowlist tools and brief-authorized read-only file/Git inspection were used. No product edit, test/build rerun, install, network access, host execution, delegation, staging, commit, branch operation, push, PR, lifecycle, release, publication, or professional-reliance act occurred.

## Outputs Produced

- This `REVIEW_RETURN.md`.
- `REVIEW_STATUS.json`.

## Missing

- Deferred DEC-025 sweep and closeout evidence, owned by the governing fan-in stages.

## Needs Human Ruling

None for the code-review verdict.

## Dependency Notes

None.

## Proposed Changes

None.

## Model and capability attribution

- Model: inherited GPT-5-based Codex runtime; exact model variant is not exposed to this agent. Fresh reviewer context; no substitution recorded and no implementer result was reused as a review verdict.
- Capabilities used: repository file reading, read-only shell/Git inspection, SHA-256 verification, registered scope validation, registered affected-check selection, and the two explicitly authorized managed-return writes.

Terminal verdict: `PASS`.

No lifecycle, release, publication, issuance, professional acceptance, certification, sealing, authentication, or code-compliance decision is performed by this review.
