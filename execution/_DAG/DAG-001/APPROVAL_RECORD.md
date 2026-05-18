---
doc_id: DAG-001-APPROVAL-RECORD
doc_kind: coordination.approval_record
status: approved_blocker_computation_enabled
created: 2026-04-30
approved: 2026-04-30
approved_by: human_project_authority
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
dag_path: execution/_DAG/DAG-001/
approval_scope: active_edge_set_only
blocker_computation: enabled_active_edges_only
pilot_dispatch_candidate: DEL-01-01
---

# DAG-001 Approval Record

## Decision

The human project authority approved `DAG-001` on 2026-04-30 as the development coordination basis for the OpenPipeStress SOFTWARE workflow.

This approval accepts the active acyclic DAG edge set for governed product-development coordination. It does not mark `PKG-00` or any deliverable as `ISSUED`, and it does not certify any engineering result.

## Approved Artifacts

| Artifact | Path |
|---|---|
| Node register | `execution/_DAG/DAG-001/DeliverableNodes.csv` |
| Edge register | `execution/_DAG/DAG-001/DependencyEdges.csv` |
| Machine graph | `execution/_DAG/DAG-001/dag.json` |
| Topological waves | `execution/_DAG/DAG-001/TopologicalWaves.md` |
| Cycle report | `execution/_DAG/DAG-001/Cycle_Report.md` |
| DAG audit | `execution/_DAG/DAG-001/DAG_Audit.md` |

## Approved Graph Facts

| Fact | State |
|---|---:|
| Packages represented | 13 |
| Deliverable nodes represented | 73 |
| Active edges | 615 |
| Candidate edges | 9 |
| Active-edge cycle status | ACYCLIC |
| Candidate warning SCCs | 4 |
| Topological waves | 12 |

## Approval Conditions

- Approval applies to `ACTIVE` edges only.
- `CANDIDATE` edges remain non-gating and must not affect wave placement, dispatch readiness, or blocker computation unless later promoted by human-approved reconciliation.
- Topological waves are dependency order only. They are not schedule, priority, staffing, readiness, or professional approval.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- No deliverable lifecycle state is changed by this approval.
- No product code, protected standards data, proprietary engineering values, or deliverable production documents are introduced by this approval record.
- Blocker queue computation is explicitly enabled by the human project authority on 2026-04-30, using the approved `ACTIVE` DAG edges only and the current filesystem lifecycle states. Candidate edges remain excluded from blocker computation.

## Post-Approval Human Rulings

| Date | Ruling |
|---|---|
| 2026-04-30 | `ORCHESTRATOR` owns the DEV-001 control plane and write-scope gate; `WORKING_ITEMS` is the persona for the actual deliverable work. |
| 2026-04-30 | Repo-level write targets for the `DEL-01-01` pilot are authorized: `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `governance/MAINTAINERS.md`. |
| 2026-04-30 | Blocker queue computation is enabled from the approved active DAG only. |

## Candidate Edge Dispositions

All candidate edges are retained as `CANDIDATE` and non-gating pending later `RECONCILIATION`.

| EdgeID | Edge | Disposition |
|---|---|---|
| DAG-001-E0616 | `DEL-05-02` -> `DEL-06-02` | Retain candidate; expression-engine reuse requires later decision. |
| DAG-001-E0617 | `DEL-07-05` -> `DEL-08-04` | Retain candidate; GUI results and export schema coupling remains unresolved. |
| DAG-001-E0618 | `DEL-10-03` -> `DEL-08-04` | Retain candidate; local FEA handoff may use separate package/export contract. |
| DAG-001-E0619 | `DEL-12-05` -> `DEL-10-02` | Retain candidate; adapter/threat-model ordering requires reconciliation before promotion. |
| DAG-001-E0620 | `DEL-09-05` -> `DEL-10-04` | Retain candidate; CI/release-gate feedback requires reconciliation before promotion. |
| DAG-001-E0621 | `DEL-08-05` -> `DEL-11-04` | Retain candidate; linter/example fixture ordering requires reconciliation before promotion. |
| DAG-001-E0622 | `DEL-04-06` -> `DEL-04-04` | Retain candidate; nonlinear diagnostics ordering requires reconciliation before promotion. |
| DAG-001-E0623 | `DEL-06-02` -> `DEL-12-05` | Retain candidate; evaluator threat-model ordering requires reconciliation before promotion. |
| DAG-001-E0624 | `DEL-07-07` -> `DEL-10-05` | Retain candidate; solve UX/headless runner job orchestration coupling remains unresolved. |

## Dependency Closure Audit Route

Selected route: adapt or wrap dependency-closure audit tooling to consume the aggregate `execution/_DAG/DAG-001/DependencyEdges.csv`.

Rationale: deliverable-local `Dependencies.csv` files are absent, and this approval does not authorize materializing aggregate DAG edges into deliverable-local dependency registers.

## First Pilot Dispatch

Selected pilot candidate: `DEL-01-01 - Project governance baseline`.

Rationale: it is Wave 2, foundational, medium context envelope, and has only satisfied `PKG-00` architecture-basis predecessors in the approved active DAG.

The pilot is prepared as a control-loop proof, not broad fan-out. The `DEL-01-01` repo-level write scope is now authorized for `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `governance/MAINTAINERS.md`; the next ORCHESTRATOR should hand actual work to one `WORKING_ITEMS` session and at most one bounded `TASK` from that session.
