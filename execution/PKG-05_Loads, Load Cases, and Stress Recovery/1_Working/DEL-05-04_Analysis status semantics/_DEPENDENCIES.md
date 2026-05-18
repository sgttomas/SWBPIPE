# Dependencies: DEL-05-04 Analysis status semantics

## Declared Upstream Dependencies

- None declared outside the extracted register in this refresh.

## Declared Downstream Dependencies

- None declared outside the extracted register in this refresh.

## Extracted Dependency Register

- **Status:** REFRESHED_BY_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Schema:** v3.1
- **Rows:** 10 total; 10 ACTIVE; 0 RETIRED.
- **ANCHOR rows:** 4 ACTIVE.
- **EXECUTION rows:** 6 ACTIVE.
- **Candidate rows:** 0.
- **Refreshed:** 2026-05-10

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---|---|---|---|---|
| DEL-05-04-DEP-001 | ANCHOR | UPSTREAM | OTHER | DEL-05-04 Analysis status semantics | ACTIVE | HIGH |
| DEL-05-04-DEP-002 | ANCHOR | UPSTREAM | OTHER | SOW-047 | ACTIVE | HIGH |
| DEL-05-04-DEP-003 | ANCHOR | UPSTREAM | OTHER | OBJ-005 | ACTIVE | HIGH |
| DEL-05-04-DEP-004 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE | HIGH |
| DAG-002-E0145 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | ACTIVE | HIGH |
| DAG-002-E0146 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | ACTIVE | HIGH |
| DAG-002-E0147 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 Application service command-query-job model | ACTIVE | HIGH |
| DAG-002-E0148 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | HIGH |
| DAG-002-E0149 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | HIGH |
| DAG-002-E0450 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 Code-neutral analysis boundary model | ACTIVE | HIGH |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md`, with `_CONTEXT.md` used for scope/objective trace anchors.
- Chosen execution documents: `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC_LENSING.md`, `_REVIEW.md`, `_REFERENCES.md`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for DEL-05-04, PKG-05, SOW-047, OBJ-005, and OBJ-011 validation.
- Prior local register state was a synchronized DAG-002 mirror. This refresh preserves matchable DAG-002 dependency IDs for existing execution edges and normalizes write-form enum fields.
- Architecture-basis rows from SCA-001 are retained as `DependencyType=CONSTRAINT` because `ARCHITECTURE_BASIS` is not a v3.1 write-form enum.
- Prior `Origin` values `CONTEXT` and `DECOMPOSITION` are normalized to `EXTRACTED`; prior provenance is retained in row notes where relevant.
- 2026-05-16 DEV-001 Stage 2 local metadata alignment updated DAG-002-E0450 to `EXPLICIT`, `SATISFIED`, and `HIGH` based on DEL-05-04 schema/status-separation evidence. This is package-local technical evidence only and does not edit aggregate DAG files, lifecycle state, or human approval records.
- No protected standards text, private rule values, proprietary tables, engineering defaults, or professional approval claims were added.
- No `_STATUS.md`, schema files, aggregate DAG artifacts, candidate rows, or human approval records were edited by this local metadata alignment.
- Integrity warnings: none. Exactly one ACTIVE parent `IMPLEMENTS_NODE` anchor is present.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 10 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 4 |
| SATISFIED | 6 |
| TBD | 0 |

## Downstream Handoff Notes

- Consumer context is `RECONCILIATION`; use this local register as refreshed evidence, not aggregate graph authority.
- Preserve the DEL-02-03 prerequisite as package-local technical evidence. Aggregate DAG promotion, candidate disposition, lifecycle state, and human approval remain outside this file.
- Treat PKG-00 architecture-basis constraints as injected context evidence; this refresh does not approve DAG-003, change lifecycle state, or create Type 2 implementation authority.

## Run History

- 2026-04-30 1530: Initial dependency-extract setup run created local dependency artifacts.
- 2026-05-03: Local register synchronized from `execution/_DAG/DAG-002/DependencyEdges.csv`; 6 ACTIVE rows.
- 2026-05-10 2227: TP-DAG-004 dependency-extract refresh for `DEL-05-04`; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition found; 10 ACTIVE rows, 0 RETIRED rows, warnings none.
- 2026-05-16: DEV-001 Stage 2 package-local alignment for PKG-05 finding resolution updated DAG-002-E0450 evidence, explicitness, satisfaction, confidence, and last-seen metadata while preserving result-envelope/non-JSON hash integration as explicit TBD scope.
