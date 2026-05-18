# Dependencies: DEL-09-02 Stress recovery benchmark suite

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 10 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG files remain outside this bounded TASK write scope.
- This local register is evidence for downstream RECONCILIATION, not a project-level sequencing authority.
- Retired rows are preserved for reconciliation visibility and are not deleted.
- No source, status, memory, code, schema, test, DAG, or coordination files were edited.

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---:|---|---|---|---|
| DEL-09-02-A001 | ANCHOR | UPSTREAM | OTHER | DEL-09-02 Stress recovery benchmark suite | ACTIVE | HIGH |
| DEL-09-02-A002 | ANCHOR | UPSTREAM | OTHER | SOW-026 Verification benchmarks regression tests and numerical quality checks | ACTIVE | HIGH |
| DEL-09-02-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-008 Verification validation regression testing and release gates | ACTIVE | HIGH |
| DAG-002-E0278 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | ACTIVE | HIGH |
| DAG-002-E0279 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | ACTIVE | HIGH |
| DAG-002-E0280 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | HIGH |
| DAG-002-E0281 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | HIGH |
| DAG-002-E0537 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-03 Fundamental stress recovery module | ACTIVE | MEDIUM |
| DAG-002-E0538 | EXECUTION | UPSTREAM | INTERFACE | DEL-03-08 Pipe section property and mass-property calculator | ACTIVE | MEDIUM |
| DAG-002-E0539 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-02 Straight pipe element | RETIRED | LOW |
| DAG-002-E0540 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 Copyright and protected-data boundary policy | ACTIVE | HIGH |

## Run Notes

- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Anchor document selected: `Datasheet.md`.
- Execution documents read: `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Decomposition status: found; revision `0.5`; target IDs validated by text lookup.
- Existing DAG mirror IDs were preserved where the refreshed row matched the same dependency.
- Canonical enum normalization applied: execution `AnchorType=NOT_APPLICABLE`; execution `DependencyType` limited to `PREREQUISITE`, `INTERFACE`, `CONSTRAINT`, or `OTHER`; `Origin=EXTRACTED`.
- Warning `CONSERVATIVE_RETIREMENT`: prior inferred row `DAG-002-E0539` was marked `RETIRED` because current assigned source documents do not directly state a straight-pipe-element prerequisite.
- Warning `ID_FORMAT_VALIDATOR_MISMATCH`: local ID validator expects three-digit package/deliverable/scope/objective formats, while this project uses two-digit IDs such as `DEL-09-02`, `PKG-09`, `SOW-026`, and `OBJ-008`; schema and enum validation still passed.
- Warning `OBJECTIVE_TRACE_TARGETTYPE`: `OBJ-008` is represented as `TargetType=REQUIREMENT` because v3.1 trace anchors do not define a separate objective target type.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 10 |
| RETIRED | 1 |
| Total | 11 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 7 |
| PENDING | 3 |
| TBD | 1 |

## Downstream Handoff Notes

- RECONCILIATION should review the conservative retirement of `DAG-002-E0539` against aggregate DAG expectations before treating the straight-pipe-element predecessor as removed from sequencing logic.
- RECONCILIATION should decide whether objective anchors need a future `TARGET_TYPE=OBJECTIVE` enum or should remain encoded as requirement trace anchors.
- Architecture-basis rows remain local context evidence only; they do not change PKG-00 lifecycle status.

## Run History

- 2026-04-30 / 2026-05-03: prior local register synchronized from `DAG-002` aggregate dependency edges; 8 ACTIVE rows.
- 2026-05-10 23:05 MDT: `TP-DAG-004 dependency-extract refresh`; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` found at revision `0.5`; warnings: `CONSERVATIVE_RETIREMENT`, `ID_FORMAT_VALIDATOR_MISMATCH`, `OBJECTIVE_TRACE_TARGETTYPE`; counts: 10 ACTIVE, 1 RETIRED.
