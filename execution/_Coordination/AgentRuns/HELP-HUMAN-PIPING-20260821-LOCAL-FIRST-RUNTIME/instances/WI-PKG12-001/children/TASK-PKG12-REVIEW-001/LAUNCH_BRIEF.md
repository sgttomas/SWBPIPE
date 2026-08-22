# TASK-PKG12-REVIEW-001 Launch Brief v1

- RequestedBy: `WI-PKG12-001`
- RunID: `HELP-HUMAN-PIPING-20260821-LOCAL-FIRST-RUNTIME`
- ParentInstanceID: `WI-PKG12-001`
- ChildInstanceID: `TASK-PKG12-REVIEW-001`
- AgentType: `2` fresh read-only generalist applying `software-code-review`; do not delegate.
- PackageID: `PKG-12`
- DeliverableID: `DEL-12-01`
- Objective: review 100% of the frozen product/test diff for correctness, security fail-closure, compatibility, test sufficiency, and scope compliance.
- AcceptedBasis: parent brief, activation/work graph, DEL-12-01 SOW/status/context, exact diff and SHA-256 inventory supplied at dispatch.
- Dependencies: accepted `TASK-PKG12-IMPL-001` return and manager-focused validation.
- DeclaredReads: all changed product/test paths plus directly relevant governing/runtime code and tests.
- AllowedTools: read, search, read-only shell and test inspection.
- AllowedWriteTargets: none.
- ExpectedOutputs: `PASS` with no actionable findings or a prioritized finding list; reviewed path list; exact SHA-256 inventory; statement that 100% of frozen diff was reviewed.
- AcceptanceCriteria: local-first guard is actually invoked at applicable runtime choke points; no payload inspection/network/cloud/telemetry behavior; local-private intent is wrapper-owned; invalid/missing evidence fails closed; compatibility and tests are adequate.
- EXCLUSIONS: no repair, formatting, file writes, Git mutation, policy selection, or scope expansion.
- Escalation: finding requiring owner policy/value or crossing a hard fence.
