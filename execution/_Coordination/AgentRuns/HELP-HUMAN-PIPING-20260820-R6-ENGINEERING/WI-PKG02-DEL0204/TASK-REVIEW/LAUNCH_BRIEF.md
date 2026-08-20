RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false

ImplementationBrief: `../TASK-IMPLEMENT/LAUNCH_BRIEF.md`
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; DAG-009; R5; DEL-02-04 REQ-02/03/05/06/07/14, AC-001, VER-001.
DiffBasis: `../FROZEN_NODE_DIFF.json`; review 100% of every frozen file at its exact SHA-256. Compare modified tracked content to the accepted basis and read added files in full.
AllowedWriteTargets: none (managed read-only review; runtime-owned launch/status/return records are written by the parent).
VerificationEvidence: implementation return `../TASK-IMPLEMENT/RETURN.md`, deliverable TASK run record, and manager rerun `29 passed in 0.88s`.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`

Objective: independently review the complete frozen node diff for correctness, fail-closed behavior, contract fidelity, regressions, security/privacy posture, API/result semantics, maintainability, tests, and evidence. Do not repair.

Required review coverage:

- confirm all four frozen file hashes before judging the diff;
- trace each of the five residual dimensions through implementation and tests: unit safety, provenance, diagnostics, protected content, adapter/plugin runtime regression;
- confirm malformed/missing/disabled controls reject, suspected protected content quarantines, and verified pairs cannot dispatch runtime;
- inspect mapping/type edge cases, outcome aggregation, structured findings, public exports, and existing adapter-gate composition;
- assess whether the 8 focused tests plus 21 existing adapter/plugin tests substantiate the change;
- validate changed paths against the sealed implementation fence;
- identify only actionable findings with exact path/line, impact, evidence, and remediation direction.

ExpectedReturn: `PASS` only with zero actionable findings; otherwise `FAIL` with blocking/non-blocking findings. State residual risk and whether the implementation is valid for manager fan-in. No lifecycle acceptance.

EXCLUSIONS: no edits; no install/network/release/Git mutation; no policy invention; no scope beyond the frozen node diff and directly needed callers/contracts/tests.
