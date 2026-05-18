# Dependencies: DEL-12-03 Telemetry off-by-default design

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority checked against `execution/_DAG/DAG-002`.
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| Class | Direction | Type | Target | Count |
|---|---|---|---|---:|
| ANCHOR | UPSTREAM | OTHER | WBS_NODE / REQUIREMENT | 2 |
| EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | 9 |
| EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | 2 |

## Active Dependency Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEL-12-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-037 | NOT_APPLICABLE | HIGH |
| DEL-12-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-010 | NOT_APPLICABLE | HIGH |
| DAG-002-E0368 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | HIGH |
| DAG-002-E0369 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | HIGH |
| DAG-002-E0370 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 / AB-00-03 | SATISFIED | HIGH |
| DAG-002-E0371 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 / AB-00-04 | SATISFIED | HIGH |
| DAG-002-E0372 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | HIGH |
| DAG-002-E0373 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 / AB-00-07 | SATISFIED | HIGH |
| DAG-002-E0374 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | HIGH |
| DAG-002-E0604 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-05 | TBD | HIGH |
| DAG-002-E0605 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | TBD | HIGH |
| DEL-12-03-E001 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-04 | TBD | HIGH |
| DEL-12-03-E002 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-04 | TBD | HIGH |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor source: `Datasheet.md`.
- Chosen execution sources: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Approved graph authority consulted: `execution/_DAG/DAG-002`; `DAG-003` was treated as preliminary and was not approved, promoted, or used as authority.
- The prior local register mirrored DAG-002 rows with project-specific values such as `ARCHITECTURE_BASIS`, `SECURITY_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`, `DELIVERABLE` as `AnchorType`, `INFERRED_DIRECT`, and `UNKNOWN`; those values are not accepted by the current enum validator.
- This refresh normalized retained DAG-002 mirror rows into accepted v3.1 enums while preserving original context in `Notes`.
- Added two anchor rows for SOW-037 and OBJ-010 because the previous mirror had no Tree anchor surface.
- Added source-supported execution dependencies for professional-claims policy and analysis-status semantics based on TEL-REQ-007.
- No uncertain candidate rows were promoted. The v3.1 `Status` enum accepted by `tools/validation/validate_enum.py` is limited to `ACTIVE` and `RETIRED`; no non-gating candidate row was necessary for this refresh.
- `tools/validation/validate_id_format.sh` still expects three-digit package/deliverable IDs and rejects current decomposition IDs such as `PKG-12` and `DEL-12-03`; canonical decomposition IDs were preserved.
- No source, status, memory, code, schema, test, DAG, coordination, lifecycle, or package-register artifact was edited.

## Lifecycle Summary

- ACTIVE rows: 13
- RETIRED rows: 0
- Anchor rows: 2
- Execution rows: 11
- Satisfaction: 7 SATISFIED; 4 TBD; 2 NOT_APPLICABLE
- Confidence: 13 HIGH; 0 MEDIUM; 0 LOW
- Origin: 13 EXTRACTED; 0 DECLARED

## Downstream Handoff Notes

- RECONCILIATION should treat the normalized DAG-002 rows as retained local evidence only; they do not alter aggregate graph authority.
- The two newly added execution rows are conservative source-supported upstream dependencies for professional-claims/status semantics; their closure status remains `TBD` because this local refresh does not change target lifecycle state.
- No DAG-003 edge was approved or promoted in this worker run.
- Architecture-basis rows remain context constraints only; they do not mark PKG-00 deliverables as `ISSUED`.

## Run History

- 2026-05-03: Local register synchronized from `execution/_DAG/DAG-002/DependencyEdges.csv`; 9 ACTIVE rows; local mirror only.
- 2026-05-10: TP-DAG-004 dependency-extract refresh; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition path `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority `execution/_DAG/DAG-002`; warnings: enum normalization required for prior DAG mirror rows; legacy ID-format helper rejects current two-digit decomposition IDs; active rows 13; retired rows 0.
