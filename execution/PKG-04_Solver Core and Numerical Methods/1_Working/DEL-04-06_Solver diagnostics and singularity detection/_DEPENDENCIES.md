# Dependencies: DEL-04-06 Solver diagnostics and singularity detection

## Extracted Dependency Register

- **Status:** TP-DAG-004_REFRESHED
- **Source of Truth:** Deliverable-local evidence plus `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 11 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION

| Class | Status | Count |
|---|---:|---:|
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
| DAG-002-E0622 | DEL-04-04 | Retained non-destructively but retired because the DEL-04-06 sources make nonlinear active-set diagnostics conditional, not a conservative prerequisite for this diagnostic deliverable. |

## Run Notes

- Applied `TaskSkill=dependency-extract` in `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Used `execution/_Decomposition/SOFTWARE_DECOMP.md` as the decomposition path supplied by the brief.
- Extraction evidence was restricted to the assigned DEL-04-06 folder and the supplied decomposition path.
- Required governance/skill instructions were read. A prior completed TP-DAG-004 dependency refresh was consulted for output-format alignment only; no dependency evidence was extracted from it.
- Anchor document selection: `_CONTEXT.md`, `Datasheet.md`, and `Specification.md` supplied explicit package/scope anchors.
- Execution document selection: `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and `_SEMANTIC_LENSING.md` supplied conservative execution-edge evidence.
- SCA-002 / decomposition revision 0.5 was reviewed; no new conservative execution dependency was added for DEL-04-06.
- The five SCA-001 architecture-basis rows were retained and enum-normalized as `DependencyType=CONSTRAINT`.
- The three active predecessor rows were retained and enum-normalized as `DependencyType=PREREQUISITE`.
- The previous low-confidence candidate edge to `DEL-04-04` was not promoted; it is retained as `RETIRED` for reconciliation visibility.

## Lifecycle Summary

| SatisfactionStatus | ACTIVE Rows |
|---|---:|
| NOT_APPLICABLE | 3 |
| SATISFIED | 5 |
| TBD | 3 |

## Downstream Handoff Notes

- This deliverable-local register is reconciliation evidence, not aggregate DAG authority.
- Aggregate DAG and coordination surfaces may still be stale relative to `SOFTWARE_DECOMP.md` revision 0.5.
- The retained predecessor rows have `SatisfactionStatus=TBD` because this local refresh did not inspect target deliverable maturity outside the allowed read boundary.
- The retired `DEL-04-04` row should be reviewed by RECONCILIATION if nonlinear diagnostic sequencing is later formalized.

## Run History

- 2026-05-10T22:16:16-0600: TP-DAG-004 refresh for `DEL-04-06`; mode UPDATE; strictness CONSERVATIVE; consumer RECONCILIATION; 12 total rows, 11 ACTIVE, 1 RETIRED.
