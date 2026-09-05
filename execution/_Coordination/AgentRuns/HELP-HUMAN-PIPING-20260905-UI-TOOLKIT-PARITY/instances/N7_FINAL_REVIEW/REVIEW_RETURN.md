# N7 independent integrated code review

**Verdict: CHANGES_REQUIRED.** Review execution completed over 100% of the declared frozen diff: 108 members (99 product/code/test/schema/documentation/configuration members and 9 supporting product contracts), including all new untracked product files. Three actionable findings prevent acceptance. Valid for parent/manager remediation fan-in; not valid as a no-findings acceptance or lifecycle transition.

Reviewer: `/root/final_review`, fresh TASK Agent 2 + `software-code-review` version 1; parent `/root` HELP_HUMAN. No delegation, source edits, network, installs, Git mutations, or lifecycle changes. Exact model unavailable/inherited. Native role/non-delegation is instruction+config asserted, not mechanism-proven. Actual start is in `START.json`; actual completion and elapsed time are in `FINISH.json`.

## Frozen basis and coverage

Repository root was resolved using `git rev-parse --show-toplevel`; instruction root is `{REPO_ROOT}`, working root `{REPO_ROOT}/projects/chirality-piping`. Preserved prepared `LAUNCH_BRIEF_V1.md` is activated by the actual parent dispatch, its four hash bindings, and explicit all-writers freeze. Base commit: `740569598f9d00440636b8ea25264127f418e4ec`.

| Binding | SHA-256 |
|---|---|
| LAUNCH_BRIEF_V1.md | 827f9c0b9a5e1456fcb2f12a2e85a77e890c2ff36ce488e233acf79fca3d3235 |
| GLOBAL_PRODUCT_FREEZE_V1.json | 5acdc07bfc4259dcd229f5dd4573a4a0c6c867002c03d3c82116a0194ba4d540 |
| GLOBAL_PRODUCT_DIFF_V1.patch | 5db0f364ad8d3cb805a8850175e5b7415c04de046a09b1f9ec85ea49234cc762 |
| GLOBAL_EVIDENCE_INVENTORY_V1.json | 4742c15085553aacb6bbdf9646fd1bf27a8382e4984d22fde03099c6b7572eda |

Read actual root/project instructions, TASK shell, review skill and companions; accepted SCA-009 acceptance and vocabulary, decomposition revision 0.12, DAG-010 approval/handoff, project invariants, applicable deliverable requirement/acceptance sections, and frozen cross-package contracts. Instruction/basis hashes are in `INSTRUCTION_BASIS_HASHES.json`. Current run-local implementation snapshots are derivative evidence, not replacement decomposition authority.

`COVERAGE.json` records every manifest member. The full 27,235-line patch was reviewed independently by file/hunk, alongside relevant unchanged callers. The repetitive 11,033-line hanger case fixture was reviewed through its full valid base and exact typed recursive deltas for all 86 cases, preserved in `FIXTURE_SEMANTIC_REDUCTION.json`; all changed values and expectations were inspected. Cargo lock contents were inspected as complete parsed dependency/version/checksum records. CSV coverage/comparison rows were inspected with shared constant columns deduplicated; all 27 rows in each ledger were covered. No truncated inspection output was treated as coverage of omitted content.

`MEMBERSHIP_VERIFICATION.json` proves all 108 patch sections reconstruct the current files from their recorded base preimages, with no missing or extra declared member. Current Git tracked and untracked changes were compared with the source manifest and separate evidence inventory; additional postfreeze files are review/runtime records or proposed remediation plans, not changed frozen product source. Before/after file hashes remain exact. Supporting runtime-path relocations were explicitly relayed by the parent and do not alter any of the 108 source/contract members. N7 preserved original START and interim run-record bytes before portable evidence finalization.

## Actionable findings

### N7-F1 — P1: reject contradictory top-level and hanger stiffness

**Location:** `core/model_operations/operation_applier/src/rich_authoring.rs:371–374`, also independent validation at lines 282–283 and 331–332. Consuming precedence: `core/product_physics/src/lib.rs:7668–7674`.

A whole support configuration can carry `stiffness={dof:UY,value:{value:100,unit:N/m}}` and `hanger.stiffness={dof:UY,value:{value:20,unit:N/m}}`. Both pass individual quantity validation. The variable-hanger requirement selects the top-level entry with `or_else`, and product physics later consumes that entry while retaining both conflicting authored values. This defeats the accepted conflict-rejection contract and silently solves a different spring stiffness than the nested hanger record declares.

**Independent reproduction:** `F1_REPRODUCTION.json` preserves the exact original model, complete proposed user operation, batch, current canonical hash claim, actual invocation timestamp, and actual generated-Wasm outcome. With valid installed/cold/hot loads and movement limit, `apply_operation_batch_json` returns `applied_to_session_model`, passed validation, no diagnostics, and retains both 100 and 20 N/m. Source remains unchanged. This was a bounded existing-artifact Node invocation authorized by the parent; no build was performed. The solver precedence conclusion comes from the unchanged consumer, not from claiming this reproduction ran a mechanics solve.

**Remediation:** enforce one unambiguous effective stiffness in the shared create/configuration validator. Reject contradictory duplicate DOFs/quantities under the accepted equality policy, preserving entered source values and supported unit aliases. Add public single/batch create and replacement regressions proving failure publishes no model/receipt, plus a compatible imported-hanger case. Native/Wasm must use the same rule. The complete compatibility matrix for other family/payload combinations is not established by this finding.

**Contract:** N4 `CONTRACT_PROPOSAL_v2.md` and N2 `A2_RETURN.md` explicitly require rejecting conflicting duplicate stiffness; OPS-K-DATA-2, UNIT-1 and AGENT-2 apply. Owning remediation: N2 PKG-16, with N1/N4 consumer backchecks.

### N7-F3 — P1: emit canonical named support families and prevent silent fallback

**Location:** `apps/desktop/src/features/support-configuration/SupportConfigurationForm.tsx:4`. Consumer: `core/product_physics/src/lib.rs:3209–3214`; family-specific constraint: `core/solver/linear_supports/src/lib.rs:634–639`.

The new family picker writes `LineStop` and `VerticalSupport`, while the actual physics adapter recognizes `line_stop` and `vertical_support`. Unknown explicit tokens fall through to Guide (or Anchor for six restraints). Thus selecting VerticalSupport with UX queues/applies a model labeled VerticalSupport, then produces a Guide at the mechanics boundary, where UX is allowed. The canonical VerticalSupport would restrict the DOF to UZ and reject that input. This is an incorrect engineering-family mapping, not only a cosmetic spelling difference. Existing new tests assert the incorrect UI tokens and therefore miss the consumer regression.

**Evidence:** direct full caller/validator/consumer trace; `richForms.test.tsx:256–268` and `ExistingToolkitEngine.test.tsx` codify the noncanonical token. The accepted SCA-009 row 16 explicitly requires named families not collapse to Guide; current coverage row 16 claims preserved backend mapping that this path does not provide.

**Remediation:** use canonical wire values with readable option labels; validate explicit family tokens at the shared operation and relevant physics read boundaries, preserving the separately established legacy missing-family behavior. Explicit canonical Guide must remain Guide rather than being inferred as Anchor from its DOF count. Test UI emission through actual operation and physics consumers with valid named-family examples and invalid DOFs/unknown explicit tokens. Update the affected coverage evidence after repair. This finding does not authorize broad alias migration or assert that a new family/nonlinear/hanger payload-exclusivity matrix is solved; such contradictions require their own established compatibility rules.

**Calibration:** the unknown-token fallback existed before this tranche. The regression is the new UI emitting tokens that enter that fallback and the newly accepted rich configuration permitting it. Owning remediation: N1 UI, N2 operation boundary, N6 physics owner; managers retain source fan-in.

### N7-F2 — P2: preserve rotational stiffness dimension in inspector readouts

**Location:** `apps/desktop/src/features/model-workspace/modelView.ts:104–106`.

The new typed readout takes top-level or hanger stiffness regardless of DOF, then always labels it `Linear stiffness` and assigns `linear_stiffness`. An accepted boundary spring with RX/RZ and `{value:125,unit:N*m/rad}` is therefore presented with the wrong quantity kind. Switching to SI requests N/m and reports conversion unavailable, although the shared catalog and SI targets support rotational stiffness in N*m/rad. The source model stays intact, but the inspector fails to represent the authored mechanical quantity correctly.

**Evidence:** the actual boundary panel/engine regression creates an RX spring in `BoundaryAuthoringPanel.test.tsx`; `targets.ts` supplies the SI rotational target. Static dataflow proves the incorrect dimension at the readout. No additional UI test execution is claimed by this reviewer.

**Remediation:** derive dimension and label from the effective stiffness DOF, including accepted case aliases; keep legacy `properties.linear_stiffness` explicitly linear. Test translational and rotational sources through the real inspector and shared conversion service, preserving source/model/hash/drafts across display preferences. Owning remediation: N1 PKG-07.

## Verification and limits

- Reviewer-executed scope validator: PASS, no scope violations. Affected-check selector chose desktop-build, desktop-test, evidence-sweep, harness-pytest, harness-self-check, piping-pytest. Exact results are preserved in `SCOPE_VALIDATION.json` and `AFFECTED_CHECKS.json`.
- Reviewer independently executed the bounded F1 Wasm reproduction, exact hash comparisons and in-memory patch reconstruction. Source hashes are recorded before and after. No expensive or host suite was duplicated.
- Inspected hash-bound N1 terminal evidence: desktop build PASS; 42 files / 665 tests PASS; actual built Chromium dist regression 1 PASS, fetching both Wasm artifacts and exercising density edit → self-weight plan → batch preview/apply → one undo. Parent-supplied N1 V2 evidence supersedes its older pending-test state. `SUPPORTING_EVIDENCE_HASHES.json` verifies 28 original/supplementary bindings with no mismatches.
- Inspected N2 native final evidence: 80 tests PASS, 0 failures; inherited rule-notice assertion repaired without altering production wording. Inspected actual RC/RN returns and N5 combined source review, N3 section review and their accepted snapshots. These reviews are supporting evidence and did not substitute for N7 source review. Broader registered check completion/disposition remains parent-owned.
- Independently traced batch preflight/temporary replay/rollback/publication, final asynchronous revision+hash guards, stale withdrawal, one checkpoint, retained raw untrusted operation metadata and explicit unknown acceptance on reload, shared-section propagation/cache checks, geometry attachments and finite/unit/frame guards, boundary context isolation, original-document self-weight hashing, native/Wasm facades, native library validation/private persistence, and display/model invariance.
- Parent-requested US imported hanger stiffness concern is **not a finding**: `core/units/src/lib.rs:1291` admits ForcePerLength aliases for LinearStiffness. Rich validation and physics normalization at `product_physics/src/lib.rs:4005–4046,4653` use this shared path for lbf/ft and lbf/in, before mechanics consumption. Preserve that support when repairing F1.

No further actionable finding was established. This is not a claim that exhaustive testing proves absence of every defect. Proposed-context retention remains bounded provenance, not complete applied-batch SQLite audit history. DEL-16-03/SOW-070 durable decisions, grouping, receipts and timestamps remain open; D58 live-provider integration remains held, and R1–R3 remain deferred. Existing exclusions for richer routing, finite-component insertion, attached physical-run transformations, selected-pipe-only self-weight, native-only library import and unavailable display targets remain explicit residuals, not newly discovered regressions.

## Handoff

Return to HELP_HUMAN for bounded owner remediation of F1/F2/F3, fresh source freeze, affected tests and independent re-review. Regenerate/reconcile affected derivative coverage claims against the repaired source. Do not accept the present frozen product snapshot as final. No additional human ruling is required for this review return; remaining release, lifecycle, DEC025 and CHANGE gates belong to the parent workflow.
