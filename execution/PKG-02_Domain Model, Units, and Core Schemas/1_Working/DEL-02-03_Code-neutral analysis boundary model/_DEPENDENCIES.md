# Dependencies: DEL-02-03 Code-neutral analysis boundary model

## Generated Dependency Register
- **Status:** REFRESHED_FROM_DEPENDENCY_EXTRACT
- **Prior source mirror:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain the sequencing and blocker-computation authority within their approval boundary.
- This local register is a deliverable-local evidence surface for downstream aggregation and reconciliation, not an independent project graph authority.
- Rows in this register do not authorize Type 2 execution by themselves.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

### Counts

| Metric | Count |
|---|---:|
| Total rows | 11 |
| ACTIVE rows | 11 |
| RETIRED rows | 0 |
| ANCHOR rows | 3 |
| EXECUTION rows | 8 |

### Compact Table

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-002-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-002 | ACTIVE | SATISFIED |
| DEP-002-03-002 | ANCHOR | UPSTREAM | OTHER | OBJ-001 | ACTIVE | SATISFIED |
| DEP-002-03-003 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE | SATISFIED |
| DAG-002-E0031 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0032 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0033 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0034 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 | ACTIVE | SATISFIED |
| DAG-002-E0035 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0036 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0037 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0394 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 | ACTIVE | TBD |

## Run Notes

- **TaskSkill:** `dependency-extract`
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer context:** `RECONCILIATION`
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** located and used for anchor validation and canonical labels.
- **Source docs:** `AUTO`.
- **Anchor doc:** `Datasheet.md` selected by default heuristic.
- **Execution doc order:** `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, then existing `Dependencies.csv` evidence surface for legacy DAG mirror rows.
- **Normalization:** legacy DAG mirror values were normalized to v3.1 enums: `AnchorType=NOT_APPLICABLE` for execution rows, `DependencyType=CONSTRAINT` for architecture-basis rows, `DependencyType=PREREQUISITE` for the DEL-02-01 predecessor row, `Origin=EXTRACTED`, and `SatisfactionStatus=TBD` where the prior value was `UNKNOWN`.
- **Warnings:** none. Exactly one ACTIVE parent anchor (`IMPLEMENTS_NODE`) is present.
- **Conflicts:** none detected in the refreshed local dependency surface.
- **Failed input notes:** none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE counts |
|---|---|---|---|---|---|
| 2026-05-10 21:48 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` located | none | 11 total; 3 ANCHOR; 8 EXECUTION |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 11 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 10 |
| TBD | 1 |

## Downstream Handoff Notes

- Reconciliation should treat this file as deliverable-local evidence, not aggregate DAG authority.
- The DEL-02-01 predecessor row remains `SatisfactionStatus=TBD` because the prior DAG mirror asserted the predecessor relationship but did not provide a local satisfaction finding.
- The architecture-basis rows remain ACTIVE as SCA-001 context constraints; their notes preserve the prior DAG mirror provenance and the warning that those rows are not Type 2 dispatch authority.
