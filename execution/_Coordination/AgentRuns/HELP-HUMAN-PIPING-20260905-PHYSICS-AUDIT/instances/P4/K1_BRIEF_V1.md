RequestedBy: WORKING_ITEMS /root/repair_pkg04
RunID: HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT
ParentInstanceID: P4
Role: Agent2 TASK; do not delegate or message siblings.
TaskSkill: software-bounded-implementation
ApplyEdits: true
PackageID: PKG-04
RuntimeOverrides.INSTRUCTION_ROOT: resolve REPO_ROOT with git rev-parse --show-toplevel.
AcceptedBasis: source 2be412ccea62bdc4bd96deb082c46d7a792076ea; decomp0.12/SCA009/DAG010; S1 V3 e0fc4c26455067eca2a22864d82d75cfed884d60f16d7d10543682fbe05fe75f and R1 PASS45de32ea6e0e52a9afa903887a2c62c9ad9fdedc93e49bac2a960b8c64958e08. Parent launch P4_REPAIR_LAUNCH_BRIEF_V2.md.
DeclaredReads: root/project AGENTS, agents/AGENT_TASK.md, skill, software-workflow.json, docs/CONTRACT.md; S1 REPAIR_PLAN.md REQUIREMENT_BINDINGS.csv; relevant M1/I1 evidence; PKG04 selected deliverable ScopeOfWork/_CONTEXT/_STATUS/MEMORY and source dependencies. Read live contracts before editing.
AllowedTools: read/write/exec for scoped edits and isolated cargo tests, one lightweight compile allocation with unique /tmp CARGO_TARGET_DIR; no network/Git/user app. Global registered checks deferred to root.
ExpectedOutputs: changed hashes, frozen patch, independent expected quantities before comparisons, original and post-repair reproducer observations, targeted tests, scope validation, TASK record and terminal return in own evidence. Preserve all baseline bytes.
AcceptanceCriteria: findings repaired within existing contract, neighboring ordinary/negative controls pass. No public schema, engineering magnitude limits, pivot/convergence threshold, friction/history or pressure/connector alteration. Fresh review is separate.
Escalation: route extra source needs and D01-D06 to P4, continue independent work. Model unknown if unexposed; instruction/config asserted native role.
ChildInstanceID: K1
ScopePath: projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/K1
AllowedWriteTargets: ["projects/chirality-piping/core/solver/frame_kernel/src/lib.rs", "projects/chirality-piping/core/solver/sparse_direct/src/lib.rs", "projects/chirality-piping/core/solver/straight_pipe/src/lib.rs", "projects/chirality-piping/core/solver/linear_supports/src/lib.rs", "projects/chirality-piping/core/solver/performance_harness/src/lib.rs", "projects/chirality-piping/core/solver/diagnostics/src/lib.rs", "projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/K1/**"]
Dependencies: baseline accepted; K1 and N1 disjoint.
Objective: DEL04-01/02/03/05/06 SOW005/006/011/035/053. R01/R02/R03: stiffness overflow M1-E-003, solve nonfinite M1-L-002, scalar spring DOF/negative stiffness literal bypass M1-E-002, point fraction silentignore M1-E-004, condition helper M1-L-001, residual/delta NaN hidden M1-L-003. Baseline witnesses diag[1e-10,1], singular[0,1], tridiag32 analytic440.68856, chain8 independent2158698.8565. Honest unavailable for singular/unconverged conditions. .5*x=1e308 reject; finite2x2 exact[0,1] either correct or structured rejection. Preserve zero stiffness supported contract. DEC053 observation refresh deferred root/P9. Ordinary SPD/prescribed, valid springs/point jumps controls.
