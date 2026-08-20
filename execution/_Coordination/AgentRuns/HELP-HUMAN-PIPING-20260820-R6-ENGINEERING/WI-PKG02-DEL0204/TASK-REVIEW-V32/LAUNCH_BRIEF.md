RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V32
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 6 implementation; Amendments 1-5/V31 and all requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V32.json`; verify all 21 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `314 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review after integrated review v6. Confirm every post-snapshot schema mismatch path segment is canonical and byte bounded, all caller-derived instance text in mismatch messages is sanitized/bounded, finite adversarial and near-1-MiB keys cannot enter diagnostic refs/messages, protected marker precedence remains, envelopes are canonical schema-valid, and runtime remains false. Confirm all prior Amendment 1-5 acceptance remains closed. Review entire original-basis amended N1 diff; PASS only zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
