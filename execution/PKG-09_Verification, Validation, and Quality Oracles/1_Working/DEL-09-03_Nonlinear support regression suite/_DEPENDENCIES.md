# Dependencies: DEL-09-03 Nonlinear support regression suite

## Authority Boundary

- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed deliverable-local evidence surface for `TP-DAG-004`; it is not independent graph authority.
- Candidate or uncertain dependency questions remain non-gating until later `RECONCILIATION` plus `CHANGE` approval.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers in this tranche.

## Extracted Dependency Register

- **Schema:** `Dependencies.csv` v3.1
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer Context:** `RECONCILIATION`
- **Rows:** 8 total; 8 ACTIVE; 0 RETIRED.
- **Classes:** 2 ANCHOR; 6 EXECUTION.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE`.
- **Trace anchors:** 1 ACTIVE `TRACES_TO_REQUIREMENT`.
- **Execution directions:** 6 UPSTREAM; 0 DOWNSTREAM.

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---:|---|---|---|---|
| DEL-09-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-026 | ACTIVE | SATISFIED |
| DEL-09-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-008 | ACTIVE | SATISFIED |
| DAG-002-E0282 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0283 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0284 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0285 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0541 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-04 | ACTIVE | PENDING |
| DAG-002-E0542 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-06 | ACTIVE | PENDING |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md`.
- Chosen execution document order: `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Decomposition status: loaded; `SOW-026`, `OBJ-008`, `PKG-09`, `DEL-09-03`, `DEL-04-04`, `DEL-04-06`, and applicable `PKG-00` architecture-basis targets were resolved from the decomposition and assigned context.
- Prior local state: 6-row `DAG-002` mirror. The refresh preserved matchable prior `DependencyID` values for the four architecture-basis rows and two nonlinear-support predecessor/interface rows.
- Enum normalization: prior aggregate graph values were normalized to dependency-extract enums: `AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE|INTERFACE|OTHER`, `Origin=EXTRACTED`, `Status=ACTIVE`, and `SatisfactionStatus=SATISFIED|PENDING`.
- No protected standards content, proprietary benchmark values, final convergence tolerances, code-compliance claims, or professional acceptance claims were introduced.
- Unresolved but not emitted as dependency rows under `CONSERVATIVE`: future regression runner, CI entry points, exact diagnostic field names, final tolerances, and public/original/permissive case source list remain `TBD` in the deliverable documents.
- Warnings: none for parent anchor integrity; exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 8 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 6 |
| PENDING | 2 |

| Origin | Count |
|---|---:|
| EXTRACTED | 8 |

## Downstream Handoff Notes

- `RECONCILIATION` should treat this file as refreshed local evidence, not graph authority.
- The new ANCHOR rows establish the Tree connection to `SOW-026` and `OBJ-008`; they should not be interpreted as execution blockers.
- The retained `DEL-04-04` and `DEL-04-06` rows are active local evidence for nonlinear solver maturity and diagnostics/result-envelope dependencies; both remain `PENDING`.
- No downstream rows were emitted for release-gate, CI, or runner targets because the current DEL-09-03 documents mention those surfaces as future/TBD implementation dependencies without enough conservative target certainty to promote them as active local edges.

## Run History

- 2026-05-03: Local register synchronized from `execution/_DAG/DAG-002/DependencyEdges.csv`; 6 total rows, 6 ACTIVE, 0 CANDIDATE.
- 2026-05-10 2305 MDT: `TP-DAG-004` dependency-extract refresh for `DEL-09-03`; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition loaded from `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; 8 total rows, 8 ACTIVE, 0 RETIRED; warnings: none.
