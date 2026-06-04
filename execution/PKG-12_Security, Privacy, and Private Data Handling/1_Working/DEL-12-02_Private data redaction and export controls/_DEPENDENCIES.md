# Dependencies: DEL-12-02 Private data redaction and export controls

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Previous Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Approved Graph Authority:** `execution/_DAG/DAG-006/`
- **Preliminary Graph:** `execution/_DAG/DAG-006/` remains preliminary and is not approved/promoted by this refresh.
- **Local Register:** `Dependencies.csv`
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the approved sequencing and blocker-computation authority within its approval boundary.
- This local register is refreshed evidence for later RECONCILIATION, not independent aggregate graph authority.
- Existing DAG-002 mirror rows were preserved where usable, normalized to v3.1 enums, and annotated as local reconciliation evidence.
- No historical `DAG-003` row was approved, promoted, or used as graph authority.

## Extracted Dependency Register

| Count | Class | Type | Status | Notes |
|---:|---|---|---|---|
| 1 | ANCHOR | OTHER | ACTIVE | Parent scope anchor to SOW-040. |
| 1 | ANCHOR | OTHER | ACTIVE | Objective trace anchor to OBJ-010. |
| 7 | EXECUTION | CONSTRAINT | ACTIVE | Applicable PKG-00 architecture-basis constraints from `_CONTEXT.md`. |
| 3 | EXECUTION | CONSTRAINT | ACTIVE | Governing document constraints from `Datasheet.md` / local references. |
| 6 | EXECUTION | PREREQUISITE | ACTIVE | Preserved DAG-002 predecessor rows for reconciliation; retained as PROPOSAL/TBD rather than promoted. |

### Compact Row Index

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| TP-DAG-004-DEL-12-02-A001 | ANCHOR | UPSTREAM | OTHER | SOW-040 | ACTIVE |
| TP-DAG-004-DEL-12-02-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-010 | ACTIVE |
| DAG-002-E0361 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE |
| DAG-002-E0362 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE |
| DAG-002-E0363 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE |
| DAG-002-E0364 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 | ACTIVE |
| DAG-002-E0365 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE |
| DAG-002-E0366 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | ACTIVE |
| DAG-002-E0367 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE |
| DAG-002-E0610 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-05 | ACTIVE |
| DAG-002-E0611 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-01 | ACTIVE |
| DAG-002-E0612 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-01 | ACTIVE |
| DAG-002-E0613 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-04 | ACTIVE |
| DAG-002-E0614 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-04 | ACTIVE |
| DAG-002-E0615 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-07 | ACTIVE |
| TP-DAG-004-DEL-12-02-E001 | EXECUTION | UPSTREAM | CONSTRAINT | docs/CONTRACT.md | ACTIVE |
| TP-DAG-004-DEL-12-02-E002 | EXECUTION | UPSTREAM | CONSTRAINT | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE |
| TP-DAG-004-DEL-12-02-E003 | EXECUTION | UPSTREAM | CONSTRAINT | docs/SPEC.md | ACTIVE |

## Run Notes
- Mode: UPDATE.
- Strictness: CONSERVATIVE.
- Consumer context: RECONCILIATION.
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for SOW/OBJ label validation.
- Approved graph authority read: `execution/_DAG/DAG-006/DependencyEdges.csv`.
- Preliminary DAG-003 was not used as authority and was not edited.
- Source docs scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and existing `_DEPENDENCIES.md`.
- Anchor doc selected: `_CONTEXT.md` with corroboration from `Datasheet.md`.
- Execution doc order selected: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`, `_REFERENCES.md`.
- Existing DAG-002 mirror rows used legacy/non-v3.1 enum values (`AnchorType=DELIVERABLE`, `DependencyType=ARCHITECTURE_BASIS` / `SECURITY_PREDECESSOR`, and `Origin=CONTEXT` / `DECOMPOSITION`). These were normalized to current enum values.
- Six preserved DAG-002 predecessor rows remain ACTIVE as approved-graph reconciliation evidence, but their satisfaction status is `TBD` and notes are labeled `PROPOSAL`; this refresh does not independently promote them into new graph authority.
- [WARNING] No CANDIDATE status was emitted because the repository enum validator for `STATUS` currently allows only `ACTIVE` and `RETIRED`.
- [WARNING] ID-format validator patterns expect three-digit package/deliverable IDs (`PKG-003`, `DEL-003-07`) and do not match this repository's existing two-digit IDs (`PKG-12`, `DEL-12-02`); schema and enum validation were used as gating checks.

## Run History
- 2026-04-30: Initial dependency extract setup run created local dependency artifacts.
- 2026-05-03: Local register synchronized from approved `DAG-006` mirror with 13 ACTIVE rows.
- 2026-05-10: TP-DAG-004 dependency surface refresh in UPDATE / CONSERVATIVE mode for RECONCILIATION. Added 2 anchor rows and 3 document-constraint rows; normalized 13 DAG-002 mirror rows; 18 ACTIVE rows after refresh; warnings: CANDIDATE enum unavailable, ID-format helper incompatible with current repo ID shape.

## Lifecycle Summary
- ACTIVE rows: 18.
- RETIRED rows: 0.
- Closure state counts: 12 SATISFIED; 6 TBD.
- Parent anchor check: PASS, exactly one ACTIVE `IMPLEMENTS_NODE` anchor.
- Ambiguous anchor check: PASS, no duplicate parent anchor.
- Evidence check: PASS, every ACTIVE row contains `EvidenceFile` and `SourceRef`.

## Downstream Handoff Notes
- For RECONCILIATION, treat the 13 preserved DAG-002 rows as local evidence aligned with the approved graph authority, not as new graph approvals.
- Treat the 3 document-target rows as non-gating local constraints unless a later graph policy elects to aggregate document dependencies.
- The six DAG-002 predecessor rows remain `PROPOSAL`/`TBD` in notes and satisfaction status because the local sources support their subject matter but do not fully settle implementation readiness.
