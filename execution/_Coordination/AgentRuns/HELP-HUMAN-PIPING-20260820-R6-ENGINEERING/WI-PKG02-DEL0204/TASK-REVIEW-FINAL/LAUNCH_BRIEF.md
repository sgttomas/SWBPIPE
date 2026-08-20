RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-FINAL
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false

ImplementationBrief: `../TASK-REMEDIATE-2/LAUNCH_BRIEF.md`
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; DEL-02-04 REQ-02/03/05/06/07/14, AC-001, VER-001; prior review returns.
DiffBasis: `../FROZEN_NODE_DIFF_V4.json`; verify all seven hashes/line counts and review 100% of every final integrated file.
AllowedWriteTargets: none.
VerificationEvidence: all three immutable TASK run records; remediation returns; child and manager `62 passed`; scope/diff evidence.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`

Objective: fresh final read-only review. Confirm every prior finding is closed: canonical schema execution, fail-closed unit catalog/quantity compatibility, full and accurate per-finding diagnostic source/provenance propagation, malformed input containment, complete frozen diff, quarantine, and runtime non-dispatch. Audit API compatibility, public exports, schema evaluator construct completeness, unit catalog trust boundary, error handling, result semantics, and all focused tests. Do not repair.

ExpectedReturn: `PASS` only with zero actionable findings and explicit 100% V4 coverage; otherwise `FAIL` with exact location/impact/evidence/remediation. State residual risk and fan-in validity.

EXCLUSIONS: no edits/install/network/release/Git mutation/lifecycle acceptance/policy invention.
