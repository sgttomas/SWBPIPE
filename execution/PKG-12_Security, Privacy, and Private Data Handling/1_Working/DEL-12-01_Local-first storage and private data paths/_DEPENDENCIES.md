# Dependencies: DEL-12-01 Local-first storage and private data paths

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority checked against `execution/_DAG/DAG-002`.
- **Local Register:** `Dependencies.csv`
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| Class | Direction | Type | Target | Count |
|---|---|---|---|---:|
| ANCHOR | UPSTREAM | OTHER | WBS_NODE / REQUIREMENT | 2 |
| EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | 8 |
| EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | 2 |
| EXECUTION | DOWNSTREAM | HANDOVER | DELIVERABLE | 1 |
| EXECUTION | DOWNSTREAM | INTERFACE | DELIVERABLE | 1 |

## Active Dependency Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DEL-12-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-029 | NOT_APPLICABLE | HIGH |
| DEL-12-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-010 | NOT_APPLICABLE | HIGH |
| DAG-002-E0354 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | HIGH |
| DAG-002-E0355 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | HIGH |
| DAG-002-E0356 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 / AB-00-03 | SATISFIED | HIGH |
| DAG-002-E0357 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 / AB-00-04 | SATISFIED | HIGH |
| DAG-002-E0358 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | HIGH |
| DAG-002-E0359 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 / AB-00-07 | SATISFIED | HIGH |
| DAG-002-E0360 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | HIGH |
| DAG-002-E0601 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 | TBD | HIGH |
| DAG-002-E0602 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-05 | TBD | MEDIUM |
| DAG-002-E0603 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | TBD | HIGH |
| DEL-12-01-E001 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-12-02 | TBD | HIGH |
| DEL-12-01-E002 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-12-04 | TBD | HIGH |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor source: `Datasheet.md`.
- Chosen execution sources: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, and approved local-mirror rows from `execution/_DAG/DAG-002`.
- Approved graph authority consulted: `execution/_DAG/DAG-002`; `DAG-003` was treated as preliminary and was not approved, promoted, or used as authority.
- The prior local register mirrored DAG-002 rows with project-specific graph-authoring values such as `ARCHITECTURE_BASIS`, `SECURITY_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`, and `UNKNOWN`; those values are not accepted by the current enum validator for local v3.1 dependency-extract rows.
- This refresh normalized retained DAG-002 mirror rows into accepted v3.1 enums while preserving original graph authority context in `Notes`.
- Added two anchor rows for SOW-029 and OBJ-010 because the previous mirror had no Tree anchor surface.
- Added downstream handoff/interface rows for DEL-12-02 and DEL-12-04 because DEL-12-01 explicitly states those deliverables consume or own adjacent report/export and secret/private-library boundaries.
- No uncertain candidate row was necessary for this refresh. The local enum validator accepts only `ACTIVE` and `RETIRED` for `Status`; non-gating uncertainty was preserved in confidence, satisfaction, and notes rather than promoted into graph authority.
- `tools/validation/validate_id_format.sh` still expects three-digit package/deliverable IDs and rejects current decomposition IDs such as `PKG-12` and `DEL-12-01`; canonical decomposition IDs were preserved.
- No source, status, memory, code, schema, test, DAG, coordination, lifecycle, or package-register artifact was edited.

## Lifecycle Summary

- ACTIVE rows: 14
- RETIRED rows: 0
- Anchor rows: 2
- Execution rows: 12
- Satisfaction: 7 SATISFIED; 5 TBD; 2 NOT_APPLICABLE
- Confidence: 13 HIGH; 1 MEDIUM; 0 LOW
- Origin: 14 EXTRACTED; 0 DECLARED

## Downstream Handoff Notes

- RECONCILIATION should treat normalized DAG-002 rows as retained local evidence only; they do not alter aggregate graph authority.
- The DEL-12-02 and DEL-12-04 downstream rows represent local source-supported information flow from DEL-12-01, matching existing approved DAG-002 inverse dependencies without approving DAG-003.
- Architecture-basis rows remain context constraints only; they do not mark PKG-00 deliverables as `ISSUED`.
- DEL-12-05 remains active from approved DAG-002 mirror evidence with medium confidence because local DEL-12-01 docs support threat-model relevance at package level but do not close the target lifecycle state.

## Run History

- 2026-05-03: Local register synchronized from `execution/_DAG/DAG-002/DependencyEdges.csv`; 10 ACTIVE rows; local mirror only.
- 2026-05-10: TP-DAG-004 dependency-extract refresh; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition path `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved graph authority `execution/_DAG/DAG-002`; warnings: enum normalization required for prior DAG mirror rows; legacy ID-format helper rejects current two-digit decomposition IDs; active rows 14; retired rows 0.
