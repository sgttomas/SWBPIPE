# Dependencies: DEL-04-01 3D frame stiffness kernel
## Declared Upstream Dependencies
- None recorded.
## Declared Downstream Dependencies
- None recorded.
## Extracted Dependency Register
- **Status:** SEMANTIC_REFRESHED_2026-06-16
- **Source of Truth:** Deliverable-local evidence plus `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED.
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION

| Class | Status | Count |
|---|---|---:|
| ANCHOR | ACTIVE | 3 |
| EXECUTION | ACTIVE | 8 |

## Active Dependency Summary

| DependencyID | Class | Direction | Type | Target | Satisfaction |
|---|---|---|---|---|---|
| DEL-04-01-A001 | ANCHOR | UPSTREAM | OTHER | DEL-04-01 | NOT_APPLICABLE |
| DEL-04-01-A002 | ANCHOR | UPSTREAM | OTHER | SOW-005 | NOT_APPLICABLE |
| DEL-04-01-A003 | ANCHOR | UPSTREAM | OTHER | SOW-035 | NOT_APPLICABLE |
| DAG-002-E0100 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | SATISFIED |
| DAG-002-E0101 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | SATISFIED |
| DAG-002-E0102 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | SATISFIED |
| DAG-002-E0103 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | SATISFIED |
| DAG-002-E0104 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | SATISFIED |
| DAG-002-E0429 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 | SATISFIED |
| DAG-002-E0430 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | SATISFIED |
| DAG-002-E0431 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 | SATISFIED |

## Run Notes

- Applied `TaskSkill=dependency-extract` in `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Brief: `execution/_Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/WorkerBriefs/PKG-04_dependency_semantic_refresh.md`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for package, deliverable, SOW, OBJ, and AB basis confirmation.
- Anchor document selection: `_CONTEXT.md` supplied explicit deliverable identity, package identity, and scope coverage.
- Execution document selection: existing register rows were reviewed against local `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and cited PKG-00 architecture basis where applicable.
- PKG-00 architecture-basis rows reviewed: 5 (DAG-002-E0100, DAG-002-E0101, DAG-002-E0102, DAG-002-E0103, DAG-002-E0104).
- PKG-00 architecture-basis rows changed: 0 (none).
- Rows added in this refresh run: 3 (DEL-04-01-A001, DEL-04-01-A002, DEL-04-01-A003).
- Rows changed in this refresh run: 0 (none).
- Rows retired in this refresh run: 0 (none).
- Warning: None

## Downstream Handoff Notes

- This deliverable-local register is reconciliation evidence, not aggregate DAG authority.
- `DAG-007` remains a canonical type-system rectification successor pending human approval; this refresh does not update `_DAG/_LATEST.md`.
- Candidate/non-gating relationships remain outside `Status=CANDIDATE`; promotion requires explicit graph approval and revalidation.
- Validation: PASS (`python3 tools/validation/validate_dependencies_schema.py Dependencies.csv`).
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Lifecycle Summary

| SatisfactionStatus | ACTIVE Rows |
|---|---:|
| NOT_APPLICABLE | 3 |
| SATISFIED | 8 |

## Run History

- 2026-06-16T0000-0600: dependency semantic refresh for `DEL-04-01`; mode UPDATE; strictness CONSERVATIVE; consumer RECONCILIATION; 11 total rows, 11 ACTIVE, 0 RETIRED; warnings: None.
