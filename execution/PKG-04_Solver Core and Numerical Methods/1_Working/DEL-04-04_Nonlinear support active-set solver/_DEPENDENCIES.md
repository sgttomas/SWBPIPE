# Dependencies: DEL-04-04 Nonlinear support active-set solver
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
| ANCHOR | ACTIVE | 2 |
| EXECUTION | ACTIVE | 9 |

## Active Dependency Summary

| DependencyID | Class | Direction | Type | Target | Satisfaction |
|---|---|---|---|---|---|
| DEL-04-04-A001 | ANCHOR | UPSTREAM | OTHER | DEL-04-04 | NOT_APPLICABLE |
| DEL-04-04-A002 | ANCHOR | UPSTREAM | OTHER | SOW-012 | NOT_APPLICABLE |
| TP-DAG-004-DEL-04-04-E001 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | SATISFIED |
| TP-DAG-004-DEL-04-04-E002 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | SATISFIED |
| TP-DAG-004-DEL-04-04-E003 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | SATISFIED |
| TP-DAG-004-DEL-04-04-E004 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | SATISFIED |
| TP-DAG-004-DEL-04-04-E005 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | SATISFIED |
| TP-DAG-004-DEL-04-04-E006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 | PENDING |
| TP-DAG-004-DEL-04-04-E007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | PENDING |
| TP-DAG-004-DEL-04-04-E008 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-06 | PENDING |
| TP-DAG-004-DEL-04-04-E009 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-02-02 | PENDING |

## Run Notes

- Applied `TaskSkill=dependency-extract` in `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Brief: `execution/_Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/WorkerBriefs/PKG-04_dependency_semantic_refresh.md`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for package, deliverable, SOW, OBJ, and AB basis confirmation.
- Anchor document selection: `_CONTEXT.md` supplied explicit deliverable identity, package identity, and scope coverage.
- Execution document selection: existing register rows were reviewed against local `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and cited PKG-00 architecture basis where applicable.
- PKG-00 architecture-basis rows reviewed: 5 (TP-DAG-004-DEL-04-04-E001, TP-DAG-004-DEL-04-04-E002, TP-DAG-004-DEL-04-04-E003, TP-DAG-004-DEL-04-04-E004, TP-DAG-004-DEL-04-04-E005).
- PKG-00 architecture-basis rows changed: 2 (TP-DAG-004-DEL-04-04-E003, TP-DAG-004-DEL-04-04-E004).
- Rows added in this refresh run: 2 (DEL-04-04-A001, DEL-04-04-A002).
- Rows changed in this refresh run: 2 (TP-DAG-004-DEL-04-04-E003, TP-DAG-004-DEL-04-04-E004).
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
| NOT_APPLICABLE | 2 |
| PENDING | 4 |
| SATISFIED | 5 |

## Run History

- 2026-06-16T0000-0600: dependency semantic refresh for `DEL-04-04`; mode UPDATE; strictness CONSERVATIVE; consumer RECONCILIATION; 11 total rows, 11 ACTIVE, 0 RETIRED; warnings: None.
