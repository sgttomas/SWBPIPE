# P9-R independent reference review v1
RequestedBy: WORKING_ITEMS P9 /root/repair_pkg09
ParentInstanceID: P9
ChildInstanceID: P9-R
RunID: HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT
Role: Agent2 TASK, no delegation
PackageID: PKG-09
DeliverableID: DEL-09-01/02/03, R10 SOW-026 OBJ-008
TaskSkill: NONE (bounded original analytical audit)
RuntimeOverrides.INSTRUCTION_ROOT: /Users/ryan/.codex/worktrees/8728/chirality
WorkingRoot: {REPO_ROOT}/projects/chirality-piping
ScopePath: {WORKING_ROOT}/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P9/review
ApplyEdits: true, evidence only
AllowedWriteTargets: ScopePath/**
AllowedTools: filesystem read/write own evidence, shell and Python analytical calculations; no compilation or product execution/network/Git.
Objective: Fresh independent read-only review of frozen P9-A EXPECTED_BEFORE_RUN.json/EXPECTED_MANIFEST.json, mathematical values, dimensional consistency, statics/EB applicability, original-vetted provenance, exact output expectations and tolerance rationale. Refer accepted I1/V1 source packets and project contracts. Do not edit author files or tests. Report actionable errors with proposed directions, no speculative claim inflation. These are software fixture checks, not new engineering acceptance policy.
AcceptedBasis: root P9 release, whole accepted S1 V3 and final R1 PASS; expected manifest SHA51465611bebef370b1542308bcb93e89be09b0ea3b3fa3dbc963015415d92d1a; source2be412ccea62bdc4bd96deb082c46d7a792076ea. No repaired output-derived oracle.
ExpectedOutputs: REFERENCE_REVIEW.md, independent arithmetic evidence, manifest and TASK run record. PASS or CHANGES_REQUIRED with exact findings. Later full test review is separately activated after author freeze. Native role instruction+config asserted; model unknown if unexposed.
