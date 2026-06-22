# Dependencies: DEL-04-06 Solver diagnostics and singularity detection
## Declared Upstream Dependencies
- None recorded.
## Declared Downstream Dependencies
- None recorded.
## Extracted Dependency Register
- **Status:** SEMANTIC_REFRESHED_2026-06-16
- **Source of Truth:** Deliverable-local evidence plus `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 11 ACTIVE; 1 RETIRED.
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION

| Class | Status | Count |
|---|---|---:|
| ANCHOR | ACTIVE | 3 |
| EXECUTION | ACTIVE | 8 |
| EXECUTION | RETIRED | 1 |

## Active Dependency Summary

| DependencyID | Class | Direction | Type | Target | Satisfaction |
|---|---|---|---|---|---|
| TP-DAG-004-DEL-04-06-A001 | ANCHOR | UPSTREAM | OTHER | PKG-04 | NOT_APPLICABLE |
| TP-DAG-004-DEL-04-06-A002 | ANCHOR | UPSTREAM | OTHER | SOW-053 | NOT_APPLICABLE |
| TP-DAG-004-DEL-04-06-A003 | ANCHOR | UPSTREAM | OTHER | SOW-035 | NOT_APPLICABLE |
| DAG-002-E0125 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | SATISFIED |
| DAG-002-E0126 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | SATISFIED |
| DAG-002-E0127 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | SATISFIED |
| DAG-002-E0128 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | SATISFIED |
| DAG-002-E0129 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | SATISFIED |
| DAG-002-E0438 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | TBD |
| DAG-002-E0439 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 | TBD |
| DAG-002-E0440 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | TBD |

## Retired Rows

| DependencyID | Prior Target | Reason |
|---|---|---|
| DAG-002-E0622 | DEL-04-04 | Retained non-destructively from prior local register but retired during TP-DAG-004 conservative refresh because the accessible DEL-04-06 source documents keep nonlinear diagnostics conditional and separate from the nonlinear support solver implementation; not a gating dependency unless later RECONCILIATION plus CHANGE approval restores it.; legacy_anchortype=NOT_APPLICABLE; legacy_direction=UPSTREAM; legacy_dependencytype=OTHER; legacy_targettype=DELIVERABLE; legacy_explicitness=IMPLICIT; legacy_satisfactionstatus=TBD; legacy_origin=EXTRACTED; legacy_status=RETIRED |

## Run Notes

- Applied `TaskSkill=dependency-extract` in `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Brief: `execution/_Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/WorkerBriefs/PKG-04_dependency_semantic_refresh.md`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for package, deliverable, SOW, OBJ, and AB basis confirmation.
- Anchor document selection: `_CONTEXT.md` supplied explicit deliverable identity, package identity, and scope coverage.
- Execution document selection: existing register rows were reviewed against local `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and cited PKG-00 architecture basis where applicable.
- PKG-00 architecture-basis rows reviewed: 5 (DAG-002-E0125, DAG-002-E0126, DAG-002-E0127, DAG-002-E0128, DAG-002-E0129).
- PKG-00 architecture-basis rows changed: 0 (none).
- Rows added in this refresh run: 0 (none).
- Rows changed in this refresh run: 0 (none).
- Rows retired in this refresh run: 0 (none).
- Warning: None

## Downstream Handoff Notes

- This deliverable-local register is reconciliation evidence, not aggregate DAG authority.
- `DAG-007` is the current approved canonical graph authority; this deliverable-local register remains reconciliation evidence, not aggregate DAG authority.
- Candidate/non-gating relationships remain outside `Status=CANDIDATE`; promotion requires explicit graph approval and revalidation.
- Validation: PASS (`python3 tools/validation/validate_dependencies_schema.py Dependencies.csv`).
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Lifecycle Summary

| SatisfactionStatus | ACTIVE Rows |
|---|---:|
| NOT_APPLICABLE | 3 |
| SATISFIED | 5 |
| TBD | 3 |

## Run History

- 2026-06-16T0000-0600: dependency semantic refresh for `DEL-04-06`; mode UPDATE; strictness CONSERVATIVE; consumer RECONCILIATION; 12 total rows, 11 ACTIVE, 1 RETIRED; warnings: None.
