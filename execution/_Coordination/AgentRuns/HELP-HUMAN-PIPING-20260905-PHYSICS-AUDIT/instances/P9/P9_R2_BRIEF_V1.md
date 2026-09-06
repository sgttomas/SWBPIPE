# P9-R2 full authored-test code review v1
RequestedBy: WORKING_ITEMS P9 /root/repair_pkg09
ParentInstanceID: P9
ChildInstanceID: P9-R2
RunID: HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT
Role: Agent2 TASK, read-only review of production/test files; no delegation
PackageID: PKG-09
DeliverableID: DEL-09-01/02/03 R10 SOW-026 OBJ-008
TaskSkill: software-code-review
RuntimeOverrides.INSTRUCTION_ROOT: /Users/ryan/.codex/worktrees/8728/chirality
WorkingRoot: {REPO_ROOT}/projects/chirality-piping
ScopePath: {WORKING_ROOT}/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P9/code_review
ApplyEdits: true, evidence only
AllowedWriteTargets: ScopePath/** only
AllowedTools: read files, own evidence writes; skill-registered scope/check-selection helpers. No product execution, build, network, Git mutation or test/source writes.
Objective: Independently inspect100% of four new validation/benchmarks/physics_audit_regression files frozen in author/AUTHORED_SOURCE_FINAL.json. Author manifest SHA1693dbbb74f534c82ede08ebe7c59e319b42c5a48906953c30c2a52dea26c1c2; source lib SHA91ea3ad88f76a617075ebc24d2d5a931f4b0cf30faf45f55b8da4f613a3081fc. Verify precise quantity selectors/units and actual product reachability, independently expected equations including EXPECTED_SUPPLEMENT_V1.json, source refs nonvacuity and baseline corrected W3 mutation direction. Existing reference PASS c30fa7d0fa082332fafd439a712aae0169f951497d371915b1bbc5c4974995bb is reference-only. Current checkpoint compiles but all10 tests red pending P4/P5 product repairs, so do not require current product PASS or declare source acceptance; judge validity of tests against actual accepted scope. Two schema fixture corrections preserved, expected math unchanged. Code review checks tests meaningful not mirroring repaired implementation and no fabricated criteria/new engineering policy.
AcceptedBasis: root whole baseline S1 V3 and R1 PASS, root P9 activation + exact benchmark-path amendment; source baseline2be412ccea62bdc4bd96deb082c46d7a792076ea. Current kernel files may differ; cite actual reads only, do not claim exact runtime checkpoint if dependency hashes drifted.
ExpectedOutputs: REVIEW.md actionable findings or PASS, full reviewed file/hash map, QA, manifest and TASK run record. No author modifications. Parent handles remediation and scheduling. Ordinary additive evidence preserves old review/author bytes.
