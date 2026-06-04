# Dependencies: DEL-11-01 User guide skeleton

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority checked against `execution/_DAG/DAG-006`.
- **Local Register:** `Dependencies.csv`
- **Rows:** 20 total; 20 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| Class | Direction | Type | Target | Count |
|---|---|---|---|---:|
| ANCHOR | UPSTREAM | OTHER | WBS_NODE / REQUIREMENT | 3 |
| EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | 7 |
| EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | 10 |

## Active Dependency Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEL-11-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-033 | NOT_APPLICABLE | HIGH |
| DEL-11-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-001 | NOT_APPLICABLE | HIGH |
| DEL-11-01-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | NOT_APPLICABLE | HIGH |
| DAG-002-E0329 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | HIGH |
| DAG-002-E0330 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | HIGH |
| DAG-002-E0331 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | HIGH |
| DAG-002-E0332 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 / AB-00-07 | SATISFIED | HIGH |
| DAG-002-E0333 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | HIGH |
| DAG-002-E0575 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-01 | TBD | MEDIUM |
| DAG-002-E0576 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-03 | TBD | MEDIUM |
| DAG-002-E0577 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-05 | TBD | MEDIUM |
| DAG-002-E0578 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-01 | TBD | MEDIUM |
| DAG-002-E0579 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-04 | TBD | HIGH |
| DEL-11-01-E001 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | TBD | HIGH |
| DEL-11-01-E002 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-04 | TBD | HIGH |
| DEL-11-01-E003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-03 | TBD | MEDIUM |
| DEL-11-01-E004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-04 | TBD | MEDIUM |
| DEL-11-01-E005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-07-07 | TBD | MEDIUM |
| DEL-11-01-E006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-02 | TBD | MEDIUM |
| DEL-11-01-E007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-03 | TBD | MEDIUM |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor source: `Datasheet.md`.
- Chosen execution sources: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Approved graph authority consulted: `execution/_DAG/DAG-006`; `DAG-003` was treated as preliminary and was not approved, promoted, or used as authority.
- The prior local register mirrored DAG-002 rows with project-specific values such as `ARCHITECTURE_BASIS`, `DOCS_PREDECESSOR`, `GOVERNANCE_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`, and `UNKNOWN`; those values are not accepted by the current enum validator.
- This refresh normalized retained DAG-002 mirror rows into accepted v3.1 enums while preserving original context in `Notes`.
- Added three anchor rows for SOW-033, OBJ-001, and OBJ-011 because the previous mirror had no Tree anchor surface.
- Added source-supported execution dependencies for protected-data boundary, status semantics, rule-check input completeness, warning UX, solve diagnostics UX, audit manifest/model hash, and warnings/assumptions/provenance report content.
- No uncertain candidate rows were promoted. The v3.1 `Status` enum accepted by `tools/validation/validate_enum.py` is limited to `ACTIVE` and `RETIRED`; no non-gating candidate row was necessary for this refresh.
- `tools/validation/validate_id_format.sh` still expects three-digit package/deliverable IDs and rejects current decomposition IDs such as `PKG-11` and `DEL-11-01`; canonical decomposition IDs were preserved.
- No source, status, memory, code, schema, test, DAG, coordination, lifecycle, or package-register artifact was edited.

## Lifecycle Summary

- ACTIVE rows: 20
- RETIRED rows: 0
- Anchor rows: 3
- Execution rows: 17
- Satisfaction: 5 SATISFIED; 12 TBD; 3 NOT_APPLICABLE
- Confidence: 11 HIGH; 9 MEDIUM; 0 LOW
- Origin: 20 EXTRACTED; 0 DECLARED

## Downstream Handoff Notes

- RECONCILIATION should treat the normalized DAG-002 rows as retained local evidence only; they do not alter aggregate graph authority.
- The seven newly added execution rows are conservative source-supported upstream dependencies for user-guide content areas, but their closure status remains `TBD` because this local refresh does not change target lifecycle state.
- No DAG-003 edge was approved or promoted in this worker run.
- Architecture-basis rows remain context constraints only; they do not mark PKG-00 deliverables as `ISSUED`.

## Run History

- 2026-05-03: Local register synchronized from `execution/_DAG/DAG-006/DependencyEdges.csv`; 10 ACTIVE rows; local mirror only.
- 2026-05-10: TP-DAG-004 dependency-extract refresh; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition path `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority `execution/_DAG/DAG-006`; warnings: enum normalization required for prior DAG mirror rows; legacy ID-format helper rejects current two-digit decomposition IDs; active rows 20; retired rows 0.
