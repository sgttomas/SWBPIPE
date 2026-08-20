RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V31
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 5 V30 remediation; all prior reviews/requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V31.json`; verify all 20 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `306 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review. Confirm every preflight invalid path is canonical/bounded before diagnostics across adapter/manifest/catalog/evidence; caller keys never appear raw. Confirm caller plugin schema uses fully exception-contained bounded exact-JSON snapshotting before hashing/evaluation and hostile/deep/cyclic/nonfinite input cannot escape or mask quarantine. Confirm all V28-V30 and earlier acceptance remain closed, envelopes canonical, runtime false. Review entire original-basis amended diff; PASS only zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
