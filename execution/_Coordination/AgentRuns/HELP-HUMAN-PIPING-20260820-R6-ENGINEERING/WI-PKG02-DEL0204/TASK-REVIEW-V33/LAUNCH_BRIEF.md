RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V33
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 6 V32 remediation; all prior requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V33.json`; verify all 22 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `318 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review. Confirm normalized schema-valid manifest plugin IDs always pass through canonical 256-byte safe-reference handling before every diagnostic/source ref; huge/adversarial IDs cannot enter paths/messages/refs; Amendment 6 mismatch path/text bounds, protected precedence, canonical envelopes, runtime false, and all prior acceptance remain closed. Review entire original-basis amended N1 diff; PASS only zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
