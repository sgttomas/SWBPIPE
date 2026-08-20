---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent: HELP_HUMAN
instance_id: WI-PKG02-DEL0204
role: WORKING_ITEMS
package_id: PKG-02
deliverable_id: DEL-02-04
status: active
---

# Package Activation — PKG-02 / DEL-02-04

Objective: close the exact `_STATUS.md ## Remaining` engineering residual for executable adapter/plugin verification layers covering unit safety, provenance, diagnostics, protected content, and adapter/plugin runtime regression, using the smallest coherent product-and-test change.

Accepted basis: HEAD `357a58b56726feba49507534159c3fbc4656b818`; `DAG-009`; target `R5`; root/project instructions; selected deliverable kit; active DAG-discovered dependencies. Dependency posture: DAG-unblocked, with no edge to sibling run nodes.

Declared reads: repository instructions; software-workflow profile; DEL-02-04 kit and recent run records; relevant adapter/plugin runtime code, tests, and schemas.

Allowed tools: read, `rg`, registered tests/build commands, `apply_patch`, and governed TASK/subagent delegation.

Allowed writes:

- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/**` as shared coordination integration owner, while sibling instances remain confined to their own instance folders;
- `core/adapters/framework/**`;
- `apps/desktop/src/features/adapter-framework/**`;
- adapter/plugin-specific schemas and focused tests whose filenames clearly bind DEL-02-04;
- DEL-02-04 `_STATUS.md`, `MEMORY.md`, and `_run_records/**`.

Exclusions: no lifecycle transition; no register, decision, DAG, decomposition, or PRD changes; no cloud/network/telemetry; no protected/private data; no writes outside Piping; no receipt, Git, push, or PR.

Acceptance: all five named verification dimensions are executable and fail closed where applicable; focused registered tests pass; a fresh read-only review over 100% of the frozen node diff returns PASS when product source changes; write containment is clean; `_STATUS.md ## Remaining` is closed or narrowed only to genuine residue.

Expected return: exact changed paths, check results, review evidence, blockers/reruns, derivative disposition, and package closure state. Do not commit.
