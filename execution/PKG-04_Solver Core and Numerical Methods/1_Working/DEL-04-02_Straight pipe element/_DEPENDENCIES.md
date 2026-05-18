# Dependencies: DEL-04-02 Straight pipe element

## Generated Dependency Register

- **Status:** TP_DAG_004_REFRESHED_CONSERVATIVE
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Schema Version:** v3.1
- **Local Register:** `Dependencies.csv`
- **Rows:** 8 total; 8 ACTIVE; 0 CANDIDATE.
- **Generated:** 2026-05-10

## Refresh Basis

- This refresh used the assigned deliverable folder plus `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- `Dependencies.csv` retains the prior row identifiers for reconciliation traceability.
- No target deliverable folders, aggregate DAG files, source code, schemas, tests, status surfaces, or coordination artifacts were edited.

## Active Upstream Edges

| DependencyID | DependencyType | Target | Status | Rationale |
|---|---|---|---|---|
| DAG-002-E0105 | ARCHITECTURE_BASIS | DEL-00-01 / AB-00-01 | ACTIVE | Sealed context lists AB-00-01 as applicable architecture basis. |
| DAG-002-E0106 | ARCHITECTURE_BASIS | DEL-00-02 / AB-00-02 | ACTIVE | Sealed context lists AB-00-02 as applicable architecture basis. |
| DAG-002-E0107 | ARCHITECTURE_BASIS | DEL-00-03 / AB-00-03 | ACTIVE | Sealed context lists AB-00-03 as applicable architecture basis. |
| DAG-002-E0108 | ARCHITECTURE_BASIS | DEL-00-06 / AB-00-06 | ACTIVE | Sealed context lists AB-00-06 as applicable architecture basis. |
| DAG-002-E0109 | ARCHITECTURE_BASIS | DEL-00-08 / AB-00-08 | ACTIVE | Sealed context lists AB-00-08 as applicable architecture basis. |
| DAG-002-E0432 | SOLVER_PREDECESSOR | DEL-04-01 | ACTIVE | Straight pipe element must fit the 3D frame stiffness kernel boundary. |
| DAG-002-E0433 | DOMAIN_MODEL | DEL-03-08 | ACTIVE | Section-property integration and weight hooks require governed section and mass-property inputs. |
| DAG-002-E0434 | UNIT_CONTRACT | DEL-02-02 | ACTIVE | Stiffness, section-property integration, weight hooks, and force recovery require unit-aware checks. |

## Conservative Rulings

- No new candidate or active rows were added for load-case application or downstream stress recovery. Local docs identify those as future interface concerns, but the assigned scope can expose hooks and recovered forces without requiring those downstream deliverables as gating predecessors.
- No `AB-00-04` row was added. `Datasheet.md` and `Procedure.md` mention `AB-00-04`, but `_CONTEXT.md` lists only `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-06`, and `AB-00-08` for this sealed context, and `execution/_Decomposition/SOFTWARE_DECOMP.md` gives `AB-00-04` downstream reach that does not include `PKG-04`. This is surfaced for RECONCILIATION rather than silently promoted.
- Satisfaction for inferred predecessor rows remains `UNKNOWN` because this bounded refresh did not read target deliverable evidence.

## Authority Boundary

- This local register is a dependency evidence surface for `DEL-04-02`; it is not an independent aggregate graph authority.
- Architecture-basis rows remain sealed-context constraints and do not mark `PKG-00` or its deliverables as `ISSUED`.
- Any future promotion of candidate edges, blocker computation, or cross-package sequencing change requires RECONCILIATION/CHANGE approval outside this bounded TASK run.
