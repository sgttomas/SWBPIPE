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
ChildInstanceID: N1
ScopePath: projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/N1
AllowedWriteTargets: ["projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs", "projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs", "projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/N1/**"]
Dependencies: baseline accepted; K1 and N1 disjoint.
Objective: DEL04-04 SOW012. R02/R04: M1-N-001 exact touching stability k100,F5,g.05 zero reaction, both signs/seeds/modes, strict open/penetrating/coupled. Established complementarity equality, no new tolerance. M1-N-004 zero max_iterations literal must error notpanic; M1-N-005 NaNtrial reject; M1-N-006 duplicate supportIDs reject. Preserve prior-iterate friction and contact-only inactive seed singularity. Do not write product adapter.
