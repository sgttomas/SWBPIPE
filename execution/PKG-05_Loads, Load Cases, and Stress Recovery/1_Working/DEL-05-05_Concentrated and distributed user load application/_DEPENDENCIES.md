# Dependencies: DEL-05-05 Concentrated and distributed user load application

## Extracted Dependency Register

- **Status:** REFRESHED_FOR_RECONCILIATION
- **Register schema:** v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Classes:** 4 ANCHOR; 9 EXECUTION.
- **Generated:** 2026-05-10

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---:|---|---|---|---|
| DEL-05-05-A001 | ANCHOR | UPSTREAM | OTHER | SOW-052 | ACTIVE | SATISFIED |
| DEL-05-05-A002 | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE | SATISFIED |
| DEL-05-05-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE | SATISFIED |
| DEL-05-05-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-012 | ACTIVE | SATISFIED |
| DAG-002-E0150 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0151 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0152 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0153 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0154 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0459 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 | ACTIVE | TBD |
| DAG-002-E0460 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | ACTIVE | TBD |
| DAG-002-E0461 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | SATISFIED |
| DEL-05-05-E001 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-03 | ACTIVE | TBD |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Anchor document chosen: `Datasheet.md`; supplemental anchor evidence from `Guidance.md`.
- Execution documents used: `_CONTEXT.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- Existing `Dependencies.csv` rows were treated as prior DAG-002 mirror evidence, then refreshed against local source evidence.
- Prior DAG-002 dependency IDs were preserved where the refreshed local evidence matched the same target and dependency intent.
- v3.1 enum normalization changed prior mirror-only `DependencyType` values such as `ARCHITECTURE_BASIS`, `LOAD_STRESS_PREDECESSOR`, `SOLVER_PREDECESSOR`, and `UNIT_CONTRACT` into canonical `CONSTRAINT` or `PREREQUISITE` values.
- v3.1 enum normalization changed prior noncanonical `Origin` values to `EXTRACTED` and prior unresolved `SatisfactionStatus=UNKNOWN` values to `TBD`.
- Parent anchor check: PASS. Exactly one ACTIVE `ANCHOR` row uses `AnchorType=IMPLEMENTS_NODE`, with target `SOW-052`.
- `[WARNING] NONE`: No floating node, ambiguous anchor, missing decomposition, protected-content, or professional-claim issue was detected.
- No candidate rows were promoted. The downstream `DEL-05-03` interface is included as local evidence for later reconciliation because `Procedure.md` explicitly names downstream stress recovery.
- Reporting is mentioned in local evidence but was not converted into a separate downstream reporting edge under conservative strictness because the target deliverable is not named locally.
- 2026-05-16 DEV-001 Stage 2 local metadata alignment updated DAG-002-E0461 to `SATISFIED` based on user-load boundary records requiring explicit unit metadata, accepted canonical dimensions, provenance references, and payload/hash refs.

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Rows |
|---|---:|
| SATISFIED | 10 |
| TBD | 3 |

## Downstream Handoff Notes

- Consumer context is `RECONCILIATION`; this register is a local evidence surface, not aggregate graph authority.
- Architecture-basis edges remain constraints inherited from sealed context and do not mark `PKG-00` deliverables as issued.
- Upstream product-development prerequisites requiring later graph reconciliation: `DEL-04-01` and `DEL-05-01`. `DEL-02-02` unit metadata is technically satisfied for the PKG-05 user-load boundary only.
- Downstream interface requiring later graph reconciliation: `DEL-05-03`.
- Conservative unresolved item: local evidence refers to downstream reporting, but no reporting deliverable target was resolved from assigned-folder evidence alone.

## Authority Boundary

- Aggregate `DAG-002` remains the approved sequencing and blocker-computation authority until a later refreshed graph is approved.
- This local register is refreshed evidence for reconciliation and does not approve `DAG-003`, promote candidates, change lifecycle state, or authorize product implementation.
- Status files, schemas, aggregate DAG files, and coordination artifacts were outside write scope. PKG-05 user-load source code was edited separately within the allowed finding-resolution scope.

## Run History

- 2026-04-30: Initial `dependency-extract` run produced v3.1 dependency artifacts.
- 2026-05-03: Local register was synchronized from approved `DAG-002` mirror; 8 ACTIVE rows.
- 2026-05-10: TP-DAG-004 refresh for `RECONCILIATION`; mode `UPDATE`, strictness `CONSERVATIVE`, decomposition path `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; 13 ACTIVE rows, 4 ANCHOR, 9 EXECUTION; warnings: none.
- 2026-05-16: DEV-001 Stage 2 package-local alignment updated DEL-02-02 unit predecessor evidence and satisfaction for user-load boundary metadata only; no aggregate DAG or lifecycle action.
