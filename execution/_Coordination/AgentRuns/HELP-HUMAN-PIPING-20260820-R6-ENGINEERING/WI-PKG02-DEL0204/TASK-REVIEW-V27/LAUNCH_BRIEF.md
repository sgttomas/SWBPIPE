RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V27
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 5 implementation return; Amendments 1-4 and all prior reviews/DEL requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V27.json`; verify all 16 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `219 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review after integrated review v5. Confirm every caller adapter, unit_catalog, and unit_evidence input is independently bounded exact-JSON snapshotted before validation/traversal/diagnostics/catalog lookup/boundary derivation; hostile/custom accessors, subclasses, cycles, depth/node/byte overflow, nonfinite values, and serialization failures cannot escape or mutate evidence. Confirm safely observable exact protected/quarantined markers from manifest/adapter/evidence retain quarantine and envelope precedence despite unrelated malformed structure without invoking hostile accessors. Confirm direct/composed regressions across every surface return structured reject/quarantine, verification false where malformed, canonical envelopes, runtime false. Confirm Amendment 1-4 and all prior acceptance dimensions. Review entire amended N1 diff from original basis; PASS only zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
