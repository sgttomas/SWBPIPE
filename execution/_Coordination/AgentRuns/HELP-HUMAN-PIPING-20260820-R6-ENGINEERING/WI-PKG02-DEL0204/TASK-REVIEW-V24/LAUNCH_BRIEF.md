RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V24
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 4/V23 FAIL remediation; Amendments 1-3 and all prior reviews/DEL requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V24.json`; verify all 13 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `175 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review after V23. Confirm raw manifest schema evaluation operates only on a detached strict exact-JSON snapshot and cannot invoke caller-overloaded equality/operations; raw evidence remains unmodified and fails exact provenance semantics. Confirm direct/composed raising-comparator regressions for both statuses and declaration/result protected privacy yield structured reject/quarantine, manifest false, protected envelope, canonical schema, runtime non-dispatch. Confirm V21-V22 closures and all prior acceptance dimensions. Review entire amended N1 diff from original basis; PASS only zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
