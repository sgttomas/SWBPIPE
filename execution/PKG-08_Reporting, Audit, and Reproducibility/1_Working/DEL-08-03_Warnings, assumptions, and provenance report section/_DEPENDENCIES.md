# Dependencies: DEL-08-03 Warnings, assumptions, and provenance report section

## Extracted Dependency Register

| Field | Value |
|---|---|
| Status | TP_DAG_004_REFRESHED_FROM_DAG_002_MIRROR |
| Source of prior local surface | `execution/_DAG/DAG-002/DependencyEdges.csv` mirror rows already present in `Dependencies.csv` |
| Local register | `Dependencies.csv` |
| Register schema | v3.1, 29 required columns |
| Rows | 11 total; 11 ACTIVE; 0 RETIRED; 0 CANDIDATE |
| Anchor rows | 0 |
| Execution rows | 11 |
| Generated | 2026-05-03 |
| Refreshed | 2026-05-10 |

## Active Edge Summary

| DependencyID | Direction | DependencyType | TargetDeliverableID | TargetName | Status |
|---|---|---|---|---|---|
| DAG-002-E0253 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-01 | Architecture decision record baseline | ACTIVE |
| DAG-002-E0254 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-02 | Repository and module boundary architecture | ACTIVE |
| DAG-002-E0255 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-03 | Application service command-query-job model | ACTIVE |
| DAG-002-E0256 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-04 | Persistence and schema versioning architecture | ACTIVE |
| DAG-002-E0257 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-06 | Diagnostics, warning, and result-envelope contract | ACTIVE |
| DAG-002-E0258 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-07 | API boundary and adapter contract map | ACTIVE |
| DAG-002-E0259 | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-08 | Layered software test and acceptance strategy | ACTIVE |
| DAG-002-E0518 | UPSTREAM | DIAGNOSTICS_CONTRACT | DEL-04-06 | Solver diagnostics and singularity detection | ACTIVE |
| DAG-002-E0519 | UPSTREAM | DOMAIN_MODEL | DEL-05-04 | Analysis status semantics | ACTIVE |
| DAG-002-E0520 | UPSTREAM | GOVERNANCE_PREDECESSOR | DEL-03-07 | Public/private library import provenance checker | ACTIVE |
| DAG-002-E0521 | UPSTREAM | GOVERNANCE_PREDECESSOR | DEL-01-04 | Professional responsibility and product-claims policy | ACTIVE |

## Run Notes

| Field | Value |
|---|---|
| TaskSkill | dependency-extract |
| Mode | UPDATE |
| Strictness | CONSERVATIVE |
| ConsumerContext | RECONCILIATION |
| Scope | DEL-08-03 only |
| ScopePath | `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section` |
| Decomposition path | `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Dispatch row | `plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv` row for DEL-08-03 |
| Source documents read | Assigned deliverable folder, governing docs, dependency-extract skill docs, and `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Update policy | Existing local rows were treated as prior DAG-002 mirror evidence and preserved non-destructively. |
| Graph authority | Aggregate `DAG-002` remains the approved graph authority; this local refresh does not approve `DAG-003`. |
| Candidate treatment | No candidate rows exist in this local register; no candidate status was promoted. |
| Warnings | `FLOATING_NODE`: no ACTIVE local ANCHOR rows are present because this register is a DAG-002 mirror surface. Preserved for reconciliation rather than inventing anchors in this bounded refresh. |
| Open reconciliation note | The dependency-extract skill's setup-era enum helper does not contain current DAG mirror enum values such as `ARCHITECTURE_BASIS`, `INFERRED_DIRECT`, `CONTEXT`, or `DECOMPOSITION`; current CSV values were preserved and checked against observed v3.1 local-surface conventions. |

## Lifecycle Summary

| Metric | Count |
|---|---:|
| Total rows | 11 |
| ACTIVE | 11 |
| RETIRED | 0 |
| CANDIDATE | 0 |
| SATISFIED | 7 |
| UNKNOWN | 4 |
| HIGH confidence | 11 |
| CONTEXT origin | 7 |
| DECOMPOSITION origin | 4 |

## Downstream Handoff Notes

- Consumer context is `RECONCILIATION`.
- This surface remains a local evidence mirror for DEL-08-03, not an independent graph authority.
- Reconciliation should compare the preserved DAG-002 mirror rows against refreshed graph proposals without treating this TASK closeout as approval to change lifecycle state or aggregate DAG authority.
- The four non-architecture execution predecessors still have `SatisfactionStatus=UNKNOWN` and should remain visible to reconciliation.

## Run History

| Timestamp | Mode | Strictness | ConsumerContext | Rows | Active | Candidate | Warnings | Validation |
|---|---|---|---|---:|---:|---:|---|---|
| 2026-05-03 | SYNCHRONIZED_FROM_DAG_002 | N/A | AGGREGATE_MIRROR | 11 | 11 | 0 | None recorded | Schema-valid v3.1 |
| 2026-05-10_2256_MDT | UPDATE | CONSERVATIVE | RECONCILIATION | 11 | 11 | 0 | FLOATING_NODE; stale enum helper noted | PASS: schema and current enum inventory checked |

## Authority Boundary

- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers in TP-DAG-004.
- This refresh does not edit source documents, lifecycle state, memory, code, schemas, tests, aggregate DAG artifacts, or coordination files.
