# Dependencies: DEL-07-06 Accessibility and usability baseline

## Declared Dependency Lists

No deliverable-local declared dependency list was present before this refresh. Existing DAG-synchronized rows were preserved non-destructively in `Dependencies.csv` and normalized to v3.1 enums.

## Extracted Dependency Register

- **Register schema:** Dependencies.csv v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 15 total; 9 ACTIVE; 6 RETIRED
- **ACTIVE classes:** 2 ANCHOR; 7 EXECUTION
- **RETIRED classes:** 0 ANCHOR; 6 EXECUTION

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---:|---|---|---|---|
| DEP-07-06-A001 | ANCHOR | UPSTREAM | OTHER | SOW-036 | ACTIVE | NOT_APPLICABLE |
| DEP-07-06-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-006 | ACTIVE | NOT_APPLICABLE |
| DAG-002-E0225 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0226 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0227 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0228 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-05 | ACTIVE | SATISFIED |
| DAG-002-E0229 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0230 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0231 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0506 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-01 | RETIRED | TBD |
| DAG-002-E0507 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-02 | RETIRED | TBD |
| DAG-002-E0508 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-03 | RETIRED | TBD |
| DAG-002-E0509 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-04 | RETIRED | TBD |
| DAG-002-E0510 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-05 | RETIRED | TBD |
| DAG-002-E0511 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-07 | RETIRED | TBD |

## Run Notes

- **Run timestamp:** 2026-05-10 22:44 MDT
- **Deliverable:** DEL-07-06 / PKG-07
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** located and used for SOW-036, OBJ-006, PKG-07, DEL-07-06, and architecture-basis validation.
- **Source docs:** AUTO.
- **Anchor doc:** `Datasheet.md`.
- **Execution doc order:** `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`.
- **Defaults applied:** `DOC_ROLE_MAP=DEFAULT`; `MODE=UPDATE`; `STRICTNESS=CONSERVATIVE`; `CONSUMER_CONTEXT=RECONCILIATION`.
- **Normalization:** Legacy/non-v3.1 local DAG mirror values were normalized: `AnchorType=DELIVERABLE` to `NOT_APPLICABLE`, `DependencyType=ARCHITECTURE_BASIS` and `GUI_PREDECESSOR` to `PREREQUISITE`, `Explicitness=INFERRED_DIRECT` to `IMPLICIT`, and `Origin=CONTEXT`/`DECOMPOSITION` to `EXTRACTED`.
- **Conservative retirement:** GUI predecessor mirror rows `DAG-002-E0506` through `DAG-002-E0511` were retained but marked `RETIRED` because the scoped DEL-07-06 source pass mentions covered GUI surfaces but does not explicitly state those deliverables are local execution prerequisites.
- **Warnings:** none. One ACTIVE parent anchor (`IMPLEMENTS_NODE`) is present; decomposition was available.
- **ID-format validator note:** `tools/validation/validate_id_format.sh` rejected two-digit project IDs such as `DEL-07-06` and `PKG-07` because the helper expects three-digit forms (`DEL-000-00`, `PKG-000`). This appears to be helper/schema drift; values match the active decomposition.

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 9 |
| RETIRED | 6 |

| SatisfactionStatus | ACTIVE rows | RETIRED rows |
|---|---:|---:|
| SATISFIED | 7 | 0 |
| NOT_APPLICABLE | 2 | 0 |
| TBD | 0 | 6 |

## Downstream Handoff Notes

For RECONCILIATION, treat the ACTIVE local surface as:

- one parent anchor to `SOW-036`;
- one objective trace to `OBJ-006`;
- seven explicit upstream architecture-basis prerequisites from `_CONTEXT.md`.

Do not treat the six retired GUI predecessor mirror rows as active blockers from this deliverable-local refresh. They remain in the register for non-destructive history and can be reconciled against aggregate DAG authority by a Type 1 workflow.

## Run History

- 2026-05-10 22:44 MDT - TP-DAG-004 dependency-extract refresh for DEL-07-06. Mode UPDATE; Strictness CONSERVATIVE; ConsumerContext RECONCILIATION; decomposition located; 15 rows total, 9 ACTIVE, 6 RETIRED; warnings: none; validation: schema/enums pass, ID helper drift noted.
- 2026-05-03 - Prior local register synchronized from `execution/_DAG/DAG-006/DependencyEdges.csv`; 13 ACTIVE rows; local register treated as mirror/evidence surface rather than graph authority.
