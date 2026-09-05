# R_EXISTING — independent M_EXISTING review

RUN_STATUS: SUCCESS (review executed); review verdict: CHANGES_REQUIRED.
ControlSurface: FILE; TaskProfile: NONE; TaskSkill: software-code-review version 1.
Role: fresh ephemeral Agent 2 under N1_WI_PKG07, no delegation, no implementation edits. Native descendant non-delegation is instruction+config asserted; inherited exact model identifier unavailable.
ScopePath: projects/chirality-piping/apps/desktop/src, bounded M_EXISTING integration.
WriteAuthorization: own RETURN.md and STATUS.json only. ToolPolicyCompliance: PASS for sealed brief read/search/shell and evidence writes; no tests executed, install, network or Git mutation.
CompanionFiles: BRIEF_SCHEMA.md, TOOL_POLICY.md, QA_CHECKS.md found/read.

## Accepted basis and coverage

Reviewed 100% of the frozen cumulative changes and new files: all 19 source/test files in M_EXISTING_FREEZE.json (14 B0 plus 5 B2), against baseline 740569598f9d00440636b8ea25264127f418e4ec. All 19 live SHA256 values match that freeze. Integrated patch SHA256 verified as 004f983fb3b2e9efa9ff0253af0f8720b8a189a98df1d19f24b8ccf261598e4a. Exact reviewed source hashes are persisted in STATUS.json.

Read root/project AGENTS, TASK base, software-code-review method and companions, applicable DEL-07-01/02/03 SOW requirements, docs/CONTRACT.md, PHASE_A_ACCEPTED_001 organization/routing contract, B0/B2 briefs and returns, and N2 B1/B2/B3 accepted snapshot evidence. Traced rich support/material/wind consumers in rich_authoring.rs and lib.rs, Section assignment/cache contract in section_bindings.rs, structured operation routes, App queue/application/history/model replacement and review ledger callers.

Coverage includes catalog availability/navigation/focus; guarded removals; Section type/provenance and shared reference editing; explicit pipe continuation and reset; all rich controls/quantity parsing/configuration omission; exact canonical before and nested-unit wire; synchronous and asynchronous basis guards; changed types/styles and all new/changed tests. No additional actionable semantic defect was established beyond the two below.

## Findings

### F1 [P1] Clearing pending operations does not invalidate an in-flight request

Location: apps/desktop/src/App.tsx:862 (new stillCurrent apply guard), related validation guard at 841 and queue-clear handler at 829; caller apps/desktop/src/features/operations/OperationLedgerPanel.tsx:59.

Trigger: start a delayed validate/apply for a queued intent, then choose the always-enabled Clear pending operations action in the review ledger before the asynchronous response finishes. handleClearReviewQueue removes intents and outcomes but changes neither operationRequest.sequence nor modelRevision. Both stillCurrent predicates consequently continue to pass. A delayed validation republishes an outcome for the withdrawn intent; a delayed successful apply commits its model and emits a receipt/checkpoint even though its pending operation was removed. Model replacement is correctly guarded, but queue withdrawal is not.

Remediation: invalidate the request generation synchronously when clearing the queue, release only its busy ownership, and ensure every async stage respects that invalidation. Add deferred validation and apply tests proving no outcome, receipt, checkpoint or model mutation after queue withdrawal; old completion must not reset a subsequently started request.

### F2 [P2] Pristine pipe cancellation is disabled without an accessible reason

Location: apps/desktop/src/features/viewport/PipeViewport.tsx:1075.

The new Cancel pipe draft button is disabled for a pristine draft but has no title, aria-label or aria-describedby explaining why. This violates the existing dead-control audit and leaves the registered desktop suite failing. Exact evidence: checks-v3-full.json reports App.deadControls.test.tsx:211 failure, with `[initial] testid:cancel-pipe-draft: disabled control has no accessible reason`.

Remediation: supply an accurate accessible explanation for the pristine disabled state and rerun the dead-control audit and applicable desktop checks.

## Verification evidence and limitations

checks-v3-full.json: desktop-build PASS; desktop-test FAIL, 603 passed and 1 failed across 33 files. The one failure is F2. Actual B3 Wasm integration evidence includes ExistingToolkitEngine tests for assignment/propagation/detachment, referenced material removal, rich form application, and explicit missing-versus-zero mass inputs. Focused freeze reports 19 tests/4 files PASS; broad desktop tests include these tests. Source read confirms native and Wasm call the common operation seam. This reviewer did not independently rerun tests or claim host/browser visual verification.

No engineering defaults, hidden direct model writes, provider binding, cascade deletion, new lifecycle authority or copied protected data was found in this reviewed source. Net-new Tier3 entries and live agent route remain correctly unavailable/gated at this milestone. No broad toolkit, release, solver or lifecycle closure is implied.

## Handoff

Derivative package: review evidence only, consuming PHASE_A_ACCEPTED_001 and N2 accepted B1/B2/B3 implementation snapshots; not decomposition truth. Closure verdict: NOT_READY_FOR_M_EXISTING_ACCEPTANCE. Manager must repair both findings, freeze the replacement source, rerun focused regressions/full applicable checks, and obtain fresh review before releasing the dependent milestone. Remaining blockers F1/F2; required final integrated capability derivative stays manager-owned.
Outputs: RETURN.md and STATUS.json. MISSING: passing repaired full checks and re-review. NEEDS_HUMAN_RULING: none. DEPENDENCY_NOTES: Tier3 awaits manager milestone; no delegation performed.
