# Dependencies: DEL-17-09 Export adapter SDK and additional targets

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/`.
- **Authority Boundary:** `DAG-006` is the approved active graph authority.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-02-04`
- `DEL-10-01`
- `DEL-10-02`

## Declared Downstream Dependencies
- To be materialized by DAG-005 and later dependency-extract.

## Extracted Dependency Register
- `Dependencies.csv` was created by TASK + `dependency-extract` after production documents existed.
- Register schema version: `v3.1`.
- ACTIVE rows: 11 total.
- ACTIVE ANCHOR rows: 7 total, including 1 `IMPLEMENTS_NODE` parent anchor and 6 `TRACES_TO_REQUIREMENT` anchors.
- ACTIVE EXECUTION rows: 4 total, all preserving declared upstream edges.

| DependencyID | Class | Direction | Type | Target | Origin | Satisfaction |
|---|---|---|---|---|---|---|
| `DEL-17-09-A001` | ANCHOR | UPSTREAM | OTHER | `DEL-17-09` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A002` | ANCHOR | UPSTREAM | OTHER | `SOW-030` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A003` | ANCHOR | UPSTREAM | OTHER | `SOW-074` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A004` | ANCHOR | UPSTREAM | OTHER | `SOW-075` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A005` | ANCHOR | UPSTREAM | OTHER | `OBJ-009` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A006` | ANCHOR | UPSTREAM | OTHER | `OBJ-017` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-A007` | ANCHOR | UPSTREAM | OTHER | `OBJ-018` | EXTRACTED | NOT_APPLICABLE |
| `DEL-17-09-E001` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-17-02` | DECLARED | TBD |
| `DEL-17-09-E002` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-02-04` | DECLARED | TBD |
| `DEL-17-09-E003` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-10-01` | DECLARED | TBD |
| `DEL-17-09-E004` | EXECUTION | UPSTREAM | INTERFACE | `DEL-10-02` | DECLARED | TBD |

## Run Notes
- Runtime overrides: `SCOPE=DEL-17-09`, `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`, `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=NONE`.
- Source document defaults: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md`, with `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md` used to validate deliverable, scope-item, and objective anchors.
- Chosen execution document order: `Procedure.md`, `Guidance.md`, `Specification.md`, `Datasheet.md`, with `_DEPENDENCIES.md` used to preserve declared edges.
- Decomposition validation status: available; explicit targets `DEL-17-09`, `SOW-030`, `SOW-074`, `SOW-075`, `OBJ-009`, `OBJ-017`, `OBJ-018`, `DEL-17-02`, `DEL-02-04`, `DEL-10-01`, and `DEL-10-02` were matched against `SOFTWARE_DECOMP.md`.
- Declared edge preservation: all four declared upstream dependencies were retained as `Origin=DECLARED` rows.
- Conservative unresolved-target policy: no downstream rows were created because the local declared downstream section does not name targets and `Procedure.md` labels future work items as candidate dispatch notes rather than dependency declarations.
- Target locations for declared upstream deliverables are recorded as `location TBD` rather than inferred from sibling folders.
- Default dependency maturity threshold recorded as `INITIALIZED`; closure status for declared upstream execution edges remains `TBD` because this run did not validate sibling lifecycle state.
- Warnings: none.

## Run History
- 2026-05-18T12:26:41-06:00 - TASK + `dependency-extract`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, decomposition path available, warnings: none, ACTIVE counts: 7 ANCHOR / 4 EXECUTION / 11 total.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 11 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 7 |
| TBD | 4 |

## Notes
- Candidate rows remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.
