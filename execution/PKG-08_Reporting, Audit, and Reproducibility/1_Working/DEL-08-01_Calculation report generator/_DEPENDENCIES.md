# Dependencies: DEL-08-01 Calculation report generator

## Declared Upstream Dependencies
- None separately declared by a human-owned section in this local artifact.

## Declared Downstream Dependencies
- None separately declared by a human-owned section in this local artifact.

## Extracted Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Schema:** Dependencies.csv v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **Source of Truth:** local dependency-extract refresh from DEL-08-01 source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 18 total; 17 ACTIVE; 0 RETIRED; 1 CANDIDATE.
- **ACTIVE anchors:** 3
- **ACTIVE execution edges:** 14
- **Generated:** 2026-05-11

| DependencyID | Class | Direction | Type | Target | TargetName | Status |
|---|---|---|---|---|---|---|
| DEL-08-01-A001 | ANCHOR | UPSTREAM | OTHER | DEL-08-01 | Calculation report generator | ACTIVE |
| DEL-08-01-A002 | ANCHOR | UPSTREAM | OTHER | SOW-024 | Generate auditable calculation reports | ACTIVE |
| DEL-08-01-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | Generate reproducible reports and result exports suitable for professional review | ACTIVE |
| DAG-002-E0239 | EXECUTION | UPSTREAM | OTHER | DEL-00-01 | Architecture decision record baseline | ACTIVE |
| DAG-002-E0240 | EXECUTION | UPSTREAM | OTHER | DEL-00-02 | Repository and module boundary architecture | ACTIVE |
| DAG-002-E0241 | EXECUTION | UPSTREAM | OTHER | DEL-00-03 | Application service command-query-job model | ACTIVE |
| DAG-002-E0242 | EXECUTION | UPSTREAM | OTHER | DEL-00-04 | Persistence and schema versioning architecture | ACTIVE |
| DAG-002-E0243 | EXECUTION | UPSTREAM | OTHER | DEL-00-06 | Diagnostics, warning, and result-envelope contract | ACTIVE |
| DAG-002-E0244 | EXECUTION | UPSTREAM | OTHER | DEL-00-07 | API boundary and adapter contract map | ACTIVE |
| DAG-002-E0245 | EXECUTION | UPSTREAM | OTHER | DEL-00-08 | Layered software test and acceptance strategy | ACTIVE |
| DAG-002-E0522 | EXECUTION | UPSTREAM | OTHER | DEL-02-05 | Project persistence and round-trip serialization | ACTIVE |
| DAG-002-E0523 | EXECUTION | UPSTREAM | OTHER | DEL-05-03 | Fundamental stress recovery module | ACTIVE |
| DAG-002-E0524 | EXECUTION | UPSTREAM | OTHER | DEL-05-04 | Analysis status semantics | ACTIVE |
| DAG-002-E0525 | EXECUTION | UPSTREAM | OTHER | DEL-06-04 | Private rule-pack lifecycle and checksum handling | ACTIVE |
| DAG-002-E0526 | EXECUTION | UPSTREAM | OTHER | DEL-08-02 | Audit manifest and model hash | ACTIVE |
| DAG-002-E0527 | EXECUTION | UPSTREAM | OTHER | DEL-08-03 | Warnings, assumptions, and provenance report section | ACTIVE |
| DAG-002-E0528 | EXECUTION | UPSTREAM | OTHER | DEL-01-04 | Professional responsibility and product-claims policy | ACTIVE |
| DEL-08-01-E001 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-08-05 | Report protected-content linter | CANDIDATE |

## Run Notes
- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Selected anchor evidence: `_CONTEXT.md`, `Datasheet.md`, and `Specification.md` because they carry explicit deliverable, scope, objective, and acceptance/verification statements.
- Selected execution evidence: `Specification.md`, `Guidance.md`, `Procedure.md`, and existing `Dependencies.csv` rows, with decomposition lookup through `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Existing DAG-002 mirror rows were preserved non-destructively and normalized where their legacy/custom enum values did not conform to current v3.1 write-form enums.
- Added one parent `IMPLEMENTS_NODE` anchor for DEL-08-01 and two trace anchors for SOW-024 and OBJ-007.
- Added one conservative execution constraint candidate to DEL-08-05 because the local specification explicitly requires a protected-content report gate for public templates/examples; parent fan-in kept it non-gating because active promotion creates a DEL-08-01 / DEL-08-05 cycle.
- Parent anchor check passed: exactly one ACTIVE IMPLEMENTS_NODE row.

## Run History
- 2026-04-30: Initial dependency-extract register created.
- 2026-05-03: DAG-002 synchronized mirror recorded 14 ACTIVE execution rows.
- 2026-05-11: TP-DAG-004 refresh, mode UPDATE, strictness CONSERVATIVE, consumer context RECONCILIATION; rows=18, active=17, candidate=1, retired=0, decomposition=`execution/_Decomposition/SOFTWARE_DECOMP.md`.
- 2026-05-11: Parent fan-in converted `DEL-08-01-E001` to non-gating `CANDIDATE` and normalized CSV line endings after validation found a DEL-08-01 / DEL-08-05 bidirectional active pair.

## Lifecycle Summary
| Status | Count |
|---|---:|
| ACTIVE | 17 |
| RETIRED | 0 |
| CANDIDATE | 1 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 3 |
| SATISFIED | 7 |
| TBD | 8 |

## Downstream Handoff Notes
- Consumer context is `RECONCILIATION`; use this register as a local evidence surface for cross-deliverable consistency checks, not as independent graph authority.
- Review custom dependency semantics preserved in `Notes` from older DAG rows after enum normalization.
- Reconciliation should verify whether DEL-08-05 is intended as a gating predecessor for DEL-08-01 public-template/report acceptance; current local evidence preserves the relationship as non-gating candidate evidence only.
