# Dependencies: DEL-03-07 Public/private library import provenance checker

## Generated Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** deliverable-local `Dependencies.csv` v3.1 extracted surface for reconciliation.
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary

- Aggregate DAG files remain outside this deliverable-local refresh and were not edited.
- This local register is an evidence surface for downstream RECONCILIATION, not independent approval of sequencing or blocker authority.
- Dependency rows record information flow, prerequisites, interfaces, anchors, and constraints only; they do not certify implementation readiness, legal redistribution rights, or professional acceptance.
- `PKG-00` architecture-basis rows are preserved as sealed-context evidence; `PKG-00` does not receive local dependency registers from this task.

## Extracted Dependency Register

| Class | Anchor/Type | Direction | Target | Status | Satisfaction |
|---|---|---|---|---|---|
| ANCHOR | IMPLEMENTS_NODE | UPSTREAM | PKG-03 | ACTIVE | SATISFIED |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-019 | ACTIVE | SATISFIED |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-044 | ACTIVE | SATISFIED |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-002 | ACTIVE | SATISFIED |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-004 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-01 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-02 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-04 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-06 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-07 | ACTIVE | SATISFIED |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-08 | ACTIVE | SATISFIED |
| EXECUTION | INTERFACE | UPSTREAM | DEL-03-01 | ACTIVE | TBD |
| EXECUTION | INTERFACE | UPSTREAM | DEL-03-02 | ACTIVE | TBD |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-01-02 | ACTIVE | TBD |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-01-03 | ACTIVE | TBD |
| EXECUTION | INTERFACE | UPSTREAM | DEL-02-04 | ACTIVE | TBD |
| EXECUTION | INTERFACE | UPSTREAM | DEL-02-04 | ACTIVE | SATISFIED |

## Run Notes

- **Task:** TP-DAG-004 dependency-extract refresh row for `DEL-03-07`.
- **Mode:** UPDATE.
- **Strictness:** CONSERVATIVE.
- **ConsumerContext:** RECONCILIATION.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- **Decomposition status:** available; anchors and target labels were validated against `PKG-03`, `DEL-03-07`, `SOW-019`, `SOW-044`, `OBJ-002`, and `OBJ-004` entries.
- **Source docs:** AUTO; scanned `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and existing `_DEPENDENCIES.md`.
- **Anchor doc:** AUTO -> `Datasheet.md`.
- **Execution doc order:** AUTO -> `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `Datasheet.md`.
- **Enum normalization:** previous DAG mirror values outside dependency-extract enums were normalized: `AnchorType=DELIVERABLE` -> `NOT_APPLICABLE`; custom dependency types -> `PREREQUISITE`, `INTERFACE`, or `CONSTRAINT`; `Explicitness=INFERRED_DIRECT` -> `IMPLICIT`; `Origin=CONTEXT/DECOMPOSITION` -> `EXTRACTED`; `SatisfactionStatus=UNKNOWN` -> `TBD`.
- **Retirement policy:** no existing rows were retired because the source documents and decomposition still support the preserved edges at conservative confidence.
- **Warnings:** none. Exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor is present.

## Run History

- `2026-05-03` — DAG-002 mirror synchronized into local `Dependencies.csv`; 11 ACTIVE rows.
- `2026-05-10 22:09 MDT` — TP-DAG-004 refresh; MODE=UPDATE; STRICTNESS=CONSERVATIVE; ConsumerContext=RECONCILIATION; decomposition available; 16 ACTIVE rows; 0 RETIRED rows; warnings: none.
- `2026-05-16` — DEV-001 Stage 2 added one package-local DEL-02-04 evidence row for import-finding diagnostic envelope mapping. This is technical compatibility evidence only; aggregate DAG authority and lifecycle state were not changed.

## Lifecycle Summary

| Metric | Count |
|---|---:|
| ACTIVE rows | 17 |
| RETIRED rows | 0 |
| ANCHOR rows | 5 |
| EXECUTION rows | 12 |
| SATISFIED rows | 12 |
| TBD rows | 5 |
| PENDING rows | 0 |
| IN_PROGRESS rows | 0 |
| WAIVED rows | 0 |
| NOT_APPLICABLE rows | 0 |

## Downstream Handoff Notes

- Reconciliation should treat `DEL-03-07-A001` through `DEL-03-07-A005` as local extraction anchors, not DAG scheduling edges.
- Preserved `DAG-002-*` IDs identify matchable historical rows from the synchronized mirror; their enum fields have been normalized for v3.1 dependency-extract validation.
- The five historical non-architecture execution rows remain `SatisfactionStatus=TBD` because the TP-DAG-004 refresh did not inspect target deliverable implementation status.
- The DEV-001 Stage 2 row records bounded technical evidence for DEL-02-04 compatibility; it is not a lifecycle promotion or a public import approval.
