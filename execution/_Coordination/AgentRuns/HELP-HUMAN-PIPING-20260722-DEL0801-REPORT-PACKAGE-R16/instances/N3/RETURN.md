# N3 return — independent candidate refutation

RUN_STATUS: SUCCESS

ControlSurface: FILE

TaskProfile: NONE

TaskSkill: `software-code-review` + `software-test-planning`

ScopePath: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16/instances/N3`

ResolvedSkillPath: `/Users/ryan/ai-env/projects/chirality/skills/software-code-review` and `/Users/ryan/ai-env/projects/chirality/skills/software-test-planning`

ResolvedSkillVersion: `1` for both skills

ResolvedTaskProfileRequirement: `NONE` for both skills

CompanionFiles: `software-code-review/{BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md} (found)`; `software-test-planning/{BRIEF_SCHEMA.md,TOOL_POLICY.md,QA_CHECKS.md} (found)`

AllowedTools: registered deterministic review/planning tools plus brief-authorized read-only file/search/Git commands

RuntimeOverrides: none

WriteAuthorization: EXPLICIT_BRIEF_TEXT — only this `RETURN.md`

## Terminal verdict

`BLOCK`

Candidate v1 is correctly bounded to the selected DEL-08-01 residual and has a coherent no-runner/no-lifecycle posture, but it is not yet internally implementable or provable from the frozen live inputs. The blocking issues below must be resolved in an amended candidate and write matrix before owner adoption or N4 dispatch.

## Findings

| Severity | Evidence | Impact | Required disposition |
|---|---|---|---|
| BLOCKING | Candidate lines 101-117 require actual hashes/build/rule-pack data, forbid incomplete values, and require one result envelope mapped from the current DEL-08-04 packet. `WRITE_MATRIX.csv:10-12` allows only an unchanged extraction of that packet builder. The live builder hard-codes `solver_build_ref: "local_fixture_preview"`, a model hash whose algorithm/canonicalization/value are `TBD`, and a synthetic `rule-pack:user-supplied:not-loaded` record with `TBD` checksum fields (`apps/desktop/src/features/result-export/ResultExportPanel.tsx:94-184`, especially 126-170). | Exact reuse cannot satisfy the candidate's real-value/no-placeholder contract; the checked bridge must either reject every ordinary current packet or silently substitute/invent values. That defeats the objective of saving the current solved session and risks false reproducibility evidence. | Freeze one exact source-to-wire mapping for model/input hashes, solver name/version/build, rule-pack absence/presence, assets, and audit/result references. State which values come from the model, analysis-run hashes, runtime build metadata, or an honestly empty collection. Either authorize a semantics-preserving correction to the shared DEL-08-04 builder with its owner effect and equivalence tests, or explicitly make the new package builder consume raw accepted session facts without claiming exact packet reuse. No `TBD` or fixture token may be promoted to a complete hash/build/ref. |
| BLOCKING | Candidate lines 108-123 require a DEL-08-06 record derived from the current report packet's actual state, comparison, and handoff evidence. The actual packet is built by the private `reportExportPacket` inside `ReportPanel.tsx:631-756`; its inputs include comparison and numerous report/handoff state values (`ReportPanel.tsx:19-87`). `WRITE_MATRIX.csv` does not authorize `ReportPanel.tsx`, a shared report-packet extraction module, or a parity test; it authorizes only `RenderedReportPanel.tsx`, whose props omit comparison and the broader packet state (`RenderedReportPanel.tsx:24-34`). | N4 cannot consume the claimed current packet under the adopted fence. Rebuilding selected fields in `reportPackageRequest.ts` would create a second, underspecified DEL-08-06 mapping and could drift from the accepted state/comparison/handoff vocabulary. | Amend the candidate with a field-level DEL-08-06 mapping and provenance/TBD/diagnostic rules. Then either add the minimum ReportPanel/shared-builder extraction and parity-test paths to the write matrix, or explicitly define the new record from named App-held facts and retract the claim that it is derived from the current report packet. Include positive and missing-evidence fixtures. |
| BLOCKING | Candidate lines 167-181 requires both path-based symlink rejection and the absolute outcome “resolve no path outside the exact user-selected parent.” N2 already records that a string/path check is race-prone and requires the limitation to be surfaced (`instances/N2/RETURN.md:83-95`). The candidate does not freeze a directory-handle/`*at` strategy or acknowledge the check-to-rename race; its Cargo authority is limited to the four named additions (`WRITE_MATRIX.csv:18-19`; candidate lines 233-236). | A check followed by path-based temp creation/rename cannot guarantee the stated containment under concurrent parent/destination replacement. This is a private-filesystem safety claim, not a cosmetic implementation choice. | Choose and freeze one posture: (a) authorize a macOS directory-handle-relative, no-follow implementation and any minimum dependency needed, with adversarial swap tests; or (b) explicitly narrow the claim to best-effort preflight symlink refusal in a non-adversarial local-user model and record the residual TOCTOU limitation. Keep same-directory `create_new`, pre-rename preservation, and truthful post-rename durability semantics either way. Also define the entropy source or soften “unpredictable” to a testable collision-resistant property. |
| BLOCKING | Candidate lines 208-215 assigns native picker and atomic-filesystem evidence to Rust tests while both H4 lanes prove only the browser no-effect route. N2 states that browser Playwright cannot prove the packaged Tauri picker/rename and requires Rust command tests plus a packaged-macOS manual/H4-native step (`instances/N2/RETURN.md:126-131`). | The evidence plan cannot prove the user-visible native picker opens, filters/suggests correctly, cancels without effect, and reaches the real command in the packaged target. Unit/component tests can prove DTO and writer behavior but not the OS-dialog integration claim. | Add one bounded packaged-macOS native evidence step with explicit pass criteria and an AgentRuns evidence target. Keep browser source/dist H4 for honest unavailability and Rust fault-injection tests for atomic semantics; do not substitute either for native picker reachability. |

## Authority assessment

- PASS: `_STATUS.md:6-9` contains the selected desktop menu plus caller-owned atomic-save residual. DAG-008 is active and the candidate is bound to frozen SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85`.
- PASS: the candidate preserves the existing report-package producer as member/schema/hash authority and requires blocked diagnostic bytes never to persist.
- PASS: explicit exclusions prevent runner verbs/files, DEL-10-05 stub consumption, package-schema changes, lifecycle/release/issuance, and non-DEL-08-01 closeout.
- PASS: N4 is the sole serialized writer and N5/W3 fan-in gates are correctly separated.
- BLOCK: the cross-package input ownership described above is not fully represented in the write matrix, so v1's requested shared-integration adoption would not authorize its own claimed mapping.

## Route, redaction, and producer assessment

- `ROUTE_MATRIX.csv` gives a coherent five-stage route from shared menu sink through controlled request, existing producer, backend picker/writer, and browser no-effect behavior.
- `DREP-PACKAGE-SAVE-009` is fixed to `public_report`; source non-mutation, decisions/findings/summary preservation, and the two independent pre-picker blocks are explicit. No local-private override or false public reclassification is authorized.
- Keeping redaction evidence in caller/UI receipt evidence rather than adding a package member is internally consistent with the no-schema-change fence.
- Producer member order, hashes, exact HTML/PDF/container bytes, diagnostics, status/TBD meaning, and professional boundary are protected adequately by v1, subject to resolving the live input-mapping blockers.

## Write-matrix assessment

- Current pre-effect scope validation: PASS. `validate_change_scope.py` found only the candidate and R16 coordination/run artifacts relative to the frozen basis.
- Product fence minimality is otherwise plausible: menu, report UI, fixed redaction route, service, native bridge/writer, Cargo files, focused tests, H4 records, run evidence, and later DEL-08-01-only closeout are separated.
- Completeness: FAIL for the live DEL-08-06 packet ownership and potentially for any dependency needed by the selected race-safe atomic strategy. No new path or dependency may be inferred by N4; the matrix must be amended first.
- Explicit negative check remains required: no changed path under `core/runner/**`, no new/changed runner command ID, and no DEL-10-05 stub consumption.

## Risk-to-test and registered-check assessment

Deterministic selection over the candidate's expected desktop plus execution paths returned `desktop-test`, `desktop-build`, `harness-pytest`, and always-on `harness-self-check`. Candidate v1 includes those and additionally requires piping pytest and the single terminal acceptance-eligible DEC-025 evidence sweep, which is conservative rather than deficient.

| Risk | Required proof after amendment |
|---|---|
| Real request mapping and producer integrity | Positive current-session fixture with non-placeholder hashes/build metadata; exact DTO conversion; canonical package member/order/hash regression; negative incomplete/unknown/non-finite/unit/reference cases before assembly. |
| DEL-08-06 truth preservation | Field-level state/comparison/handoff parity or named raw-fact mapping; unresolved/TBD, diagnostics, provenance, and professional-boundary fixtures; missing evidence blocks rather than synthesizes. |
| Redaction/no bypass | Fixed route/context assertion; private/unknown leaves block or sanitize per accepted contract; decisions/findings/summary retained; source deep equality; no picker/invoke/file on either redaction or producer block. |
| Atomic persistence | Same-parent create-new temp, exact bytes, replacement preservation across injected pre-rename failures, cleanup-primary dual diagnostics, post-rename uncertain durability, symlink/path posture matching the amended threat model, collision and extension cases. |
| Menu/native route | DOM/native ID parity, sink readiness/reentrancy/retry/error tests, browser source/dist no-download H4, plus packaged-macOS native picker reachability/cancel/save evidence. |
| Runner exclusion | Changed-path and command-registration assertions showing no runner file, verb, or DEL-10-05 stub change. |

Acceptance remains: all focused checks plus the selected registered union green; exactly one acceptance-eligible DEC-025 sweep for the terminal attempt; fresh N5 diff/evidence review returns `COMMIT-SAFE` before W3.

## Residual risk

After the four amendments, expected residual risk is bounded to macOS filesystem semantics, native dialog automation/manual evidence quality, and the fidelity of the new cross-language DTO conversion. Those risks are testable and do not require a producer-schema or runner change. In v1 they remain unresolved, so the return is not valid for adoption or implementation fan-in.

## Tools Used

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `rg`, `sed`, `nl`, `find`, and `wc`

## Tool Policy Compliance

PASS — scope validation was performed before implementation judgment; affected registered checks were selected from the accepted profile; no product tests, builds, installers, network commands, or unauthorized writes ran.

## Outputs Produced

- This terminal independent-refutation return with four blocking findings, authority and coverage assessments, and minimum amendment directions.

## Missing

- none for the bounded refutation

## Needs Human Ruling

- none at N3; owner adoption must remain held while WORKING_ITEMS prepares and freshly refutes an amended candidate

## Dependency Notes

- No dependency cycle was identified. Implementation remains ordered after the accepted producer, DEL-08-04 result contract, DEL-08-06 record vocabulary, and DEL-12-02 redaction contract.

## Applied Changes

- Added only this `RETURN.md`; candidate, product, deliverable state, and all other run artifacts remained read-only.
