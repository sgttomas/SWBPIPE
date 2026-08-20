RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REVIEW-V28
Role: TASK
TaskSkill: software-code-review
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: false
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; Amendment 5 V27 remediation return; Amendments 1-4 and all prior reviews/DEL requirements.
DiffBasis: `../FROZEN_NODE_DIFF_V28.json`; verify all 17 hashes/lines and review 100% full amended N1 diff.
AllowedWriteTargets: none.
VerificationEvidence: `253 passed`, composed schema assertions, diff/scope PASS.
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
Objective: fresh final review after V27 remediation. Confirm malformed-manifest fallback preserves safely observable exact provenance/metadata protected or quarantine markers without invoking hostile accessors; deterministic manifest bounds cover 10,000 nodes, 1 MiB UTF-8 text, depth 512, and 1 MiB serialization; unit fallback inspects the complete already-bounded outer evidence list and retains protected/quarantine markers beyond index 1,024. Confirm every caller manifest, adapter, unit_catalog, and unit_evidence input is independently bounded exact-JSON snapshotted before validation/traversal/diagnostics/catalog lookup/boundary derivation; all malformed inputs fail closed with schema-valid reject/quarantine results and runtime false. Confirm direct/composed regressions and Amendments 1-4/all prior acceptance dimensions. Review entire amended N1 diff from original basis; PASS only with zero findings.
ExpectedReturn: PASS with valid fan-in or exact actionable FAIL. No edits.
EXCLUSIONS: no writes/install/network/release/Git/lifecycle/policy/schema/N2/N3/shared-fan-in change.
