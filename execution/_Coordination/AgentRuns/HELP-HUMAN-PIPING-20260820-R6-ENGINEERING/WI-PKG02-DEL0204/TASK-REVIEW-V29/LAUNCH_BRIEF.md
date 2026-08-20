RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V29
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 5 V28 remediation; Amendments 1-4 and all prior reviews/DEL requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V29.json`; verify all 18 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `274 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review after V28 fallback remediation. Confirm raw marker recovery cannot invoke hostile keys/equality; exact-key work is deterministic and bounded. Confirm unit fallback is capped at 2,048 entries and 64 keys/object, retains markers through index 2,047, excludes index 2,048 consistently, and never does unbounded/per-entry deep snapshot work. Confirm malformed-manifest marker and identifier fallback uses plugin-specific limits only; plugin IDs are exact canonical strings no more than 256 UTF-8 bytes or become TBD; no raw over-limit or hostile values enter diagnostics. Confirm all inputs fail closed with schema-valid reject/quarantine envelopes and runtime false, all Amendment 1-5 and prior acceptance remain closed. Review entire amended N1 diff from original basis; PASS only with zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
