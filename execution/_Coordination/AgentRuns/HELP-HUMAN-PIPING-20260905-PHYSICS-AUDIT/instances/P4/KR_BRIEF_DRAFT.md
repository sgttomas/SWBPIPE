RequestedBy: WORKING_ITEMS /root/repair_pkg04
RunID: HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT
ParentInstanceID: P4
ChildInstanceID: KR
Role: Agent2 TASK; no delegation or sibling messaging.
TaskSkill: software-code-review
ApplyEdits: false
PackageID: PKG-04; DEL04-01/02/03/04/05/06, SOW005/006/011/012/035/053; OPS-K-UNIT-1, SOLVER-1/2, DATA-2, AGENT-1/2, MECH-2.
RuntimeOverrides.INSTRUCTION_ROOT: resolve REPO_ROOT git rev-parse --show-toplevel.
Status: DRAFT pending frozen K1 return.
Objective: independent read-only review over 100% frozen K1/N1 production and test diffs for R01/R02/R03/R04; source invariants and tests, no self-authoring.
DeclaredReads: root/project AGENTS, TASK, skill, software profile; parent P4 launch v2 and child K1/N1 briefs; accepted S1 REPAIR_PLAN/REQUIREMENT_BINDINGS; live PKG04 SOW/context/status/memory; exact source and dependency callers; K1/N1 evidence including independent expectations/original baseline failures.
AllowedTools: bounded reads and read-only git diff/show; optional targeted no-build arithmetic if useful. No compile needed by default, no network/Git mutations/userapp.
ScopePath: projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/KR
AllowedWriteTargets: none for project source. Own ScopePath/** only for evidence/run record.
AcceptanceCriteria: every changed source/test line reviewed against established contracts. Check finite guards through actual callers; condition observer has explicit convergence with honest singular/unavailable result and finite residual/delta publication; no new production threshold; literal bypass guards preserve valid zero stiffness; exact-touch complementarity convention, signs/seeds/modes/coupled tests. Error reuse compatibility and no new DTO changes. Diff/scope/source hashes and tests bind freeze. Only actionable findings with locations, effect, evidence and remediation; PASS only no actionable findings.
ExpectedOutputs: full reviewed-file/line coverage, source hash binding, findings or PASS, test limitations, TASK run record, final manifest. Review output is derivative, no lifecycle/engineering acceptance. Exact model unknown if unexposed; native role/non-delegation instruction/config asserted.
Escalation: return bounded actionable issues to P4, never edit implementation.
