# Dependencies: DEL-07-04 Missing-data warning and blocking UX

## Generated Dependency Register
- **Status:** REFRESHED_BY_DEPENDENCY_EXTRACT_TP_DAG_004
- **Source of Truth:** Deliverable-local source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 16 total; 15 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10 22:43 MDT

## Authority Boundary
- This local register is a deliverable-local extraction surface for downstream `RECONCILIATION`.
- It is not a project-level dependency graph, blocker-computation authority, or Type 2 implementation approval.
- Architecture-basis rows reflect sealed-context constraints; they do not mark `PKG-00` as `ISSUED`.
- Retired rows remain visible for reconciliation and are not deleted.

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---:|---|---|---|---|
| DEL-07-04-A001 | ANCHOR | UPSTREAM | OTHER | PKG-07 | ACTIVE | NOT_APPLICABLE |
| DEL-07-04-A002 | ANCHOR | UPSTREAM | OTHER | SOW-022 | ACTIVE | NOT_APPLICABLE |
| DEL-07-04-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-006 | ACTIVE | NOT_APPLICABLE |
| DEL-07-04-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE | NOT_APPLICABLE |
| DAG-002-E0211 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0212 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0213 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0214 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-05 | ACTIVE | SATISFIED |
| DAG-002-E0215 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0216 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0217 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0496 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-04 | ACTIVE | PENDING |
| DAG-002-E0497 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-03 | ACTIVE | PENDING |
| DAG-002-E0498 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-06 | ACTIVE | PENDING |
| DAG-002-E0495 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 | RETIRED | TBD |
| DEV-001-STAGE2-DEL-07-04-PKG02-001 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 | ACTIVE | SATISFIED |

## DEV-001 Stage 2 Addendum

- Added active package-local PKG-02 compatibility dependency `DEV-001-STAGE2-DEL-07-04-PKG02-001` to `DEL-02-03`.
- Preserved the retired legacy mirror row `DAG-002-E0495` for history; the new active row is based on Stage 2 code/test evidence rather than aggregate DAG authority.
- Evidence: `core/gui/warnings/engine.py` now emits `analysis_boundary_contract` and maps local warning classes to canonical software statuses plus external hash-bound human-record policy; `tests/test_missing_data_warning_ux.py` covers the mapping.

## Run Notes
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **SCOPE:** DEL-07-04
- **RUN_ROOT:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **DECOMPOSITION_PATH:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** Found; revision 0.7 read-only validation basis.
- **Anchor doc:** `Datasheet.md`
- **Execution docs scanned:** `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`
- **Reference doc scanned:** `_REFERENCES.md`
- **Defaults:** `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`
- **Normalization:** Existing DAG mirror rows were retained where matchable and normalized to v3.1 enum values (`AnchorType=NOT_APPLICABLE`, generic execution `DependencyType`, `Explicitness=EXPLICIT|IMPLICIT`, `Origin=EXTRACTED`).
- **Retirement:** `DAG-002-E0495` was marked `RETIRED` because the current assigned source documents do not directly state `DEL-02-03` as a required input.
- **Warnings:** `ID_FORMAT_TOOL_MISMATCH` - `tools/validation/validate_id_format.sh` expects three-digit package/deliverable IDs (`PKG-007`, `DEL-007-04`), while the current decomposition and assigned deliverable use canonical IDs such as `PKG-07` and `DEL-07-04`. IDs were not rewritten.
- **Tree x DAG check:** Parent anchor count is exactly one.

## Lifecycle Summary
- **ACTIVE rows:** 15
- **RETIRED rows:** 1
- **ANCHOR rows:** 4 ACTIVE
- **EXECUTION rows:** 11 ACTIVE, 1 RETIRED
- **SatisfactionStatus counts:** `SATISFIED=8`, `PENDING=3`, `NOT_APPLICABLE=4`, `TBD=1`

## Downstream Handoff Notes
- For `RECONCILIATION`, the active dependency surface now separates decomposition anchors from execution edges.
- The three active non-architecture prerequisites are `DEL-04-06`, `DEL-05-04`, and `DEL-06-03`.
- The legacy inferred `DEL-02-03` edge is intentionally retained as `RETIRED` for conflict/closure review rather than removed.
- The Stage 2 `DEL-02-03` row is package-local technical evidence only; it does not promote or rewrite aggregate dependency authority.
- Architecture-basis edges are constraints/interfaces from sealed context, not independent evidence that PKG-00 deliverables are issued.

## Run History
- 2026-05-10 22:43 MDT — TP-DAG-004 dependency-extract refresh; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition path found at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings: `ID_FORMAT_TOOL_MISMATCH`; ACTIVE rows: 14; RETIRED rows: 1.
- 2026-05-16 — DEV-001 Stage 2 technical resolution; added 1 ACTIVE package-local `DEL-02-03` compatibility row with code/test evidence; no aggregate DAG or human disposition changed.
