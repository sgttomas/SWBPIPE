# Dependencies: DEL-14-04 Analysis-run comparison engine

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded outside the local canonical register.

## Declared Downstream Dependencies
- None recorded outside the local canonical register.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=13.
- **PKG-00 architecture-basis trackers:** 7 reviewed; 0 changed; all retained ACTIVE as supported consistency trackers.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | TargetPackage | Target | Status |
|---|---|---:|---|---|---|---|
| `DEL-14-04-A001` | ANCHOR | UPSTREAM | OTHER | PKG-14 | PKG-14 | ACTIVE |
| `DEL-14-04-A002` | ANCHOR | UPSTREAM | OTHER | - | SOW-073 | ACTIVE |
| `DEL-14-04-A003` | ANCHOR | UPSTREAM | OTHER | - | SOW-072 | ACTIVE |
| `DEL-14-04-A004` | ANCHOR | UPSTREAM | OTHER | - | OBJ-016 | ACTIVE |
| `DAG-002-E0688` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-01 | ACTIVE |
| `DAG-002-E0689` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-02 | ACTIVE |
| `DAG-002-E0690` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-03 | ACTIVE |
| `DAG-002-E0691` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-04 | ACTIVE |
| `DAG-002-E0692` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-06 | ACTIVE |
| `DAG-002-E0693` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-07 | ACTIVE |
| `DAG-002-E0694` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-00 | DEL-00-08 | ACTIVE |
| `DAG-002-E0795` | EXECUTION | UPSTREAM | INTERFACE | PKG-14 | DEL-14-02 | ACTIVE |
| `DAG-002-E0796` | EXECUTION | UPSTREAM | INTERFACE | PKG-14 | DEL-14-05 | ACTIVE |
| `DAG-002-E0797` | EXECUTION | UPSTREAM | INTERFACE | PKG-08 | DEL-08-04 | ACTIVE |
| `DAG-002-E0798` | EXECUTION | UPSTREAM | CONSTRAINT | PKG-02 | DEL-02-02 | ACTIVE |
| `DEL-14-04-D001` | EXECUTION | DOWNSTREAM | ENABLES | PKG-07 | DEL-07-08 | ACTIVE |
| `DEL-14-04-D002` | EXECUTION | DOWNSTREAM | ENABLES | PKG-08 | DEL-08-06 | ACTIVE |

## Canonical Dependency Types
- `CONSTRAINT`: 8
- `ENABLES`: 2
- `INTERFACE`: 3
- `OTHER`: 4

## Run Notes
- Defaults applied: `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `SOURCE_DOCS=AUTO`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor validation and target labels were checked against revision 0.7 excerpts for PKG-14, SOW-071 through SOW-073, and OBJ-016.
- Source docs reviewed: `Datasheet.md`, `_CONTEXT.md`, `Guidance.md`, `Procedure.md`, `Specification.md`, `_REFERENCES.md` plus the existing `Dependencies.csv` and `_DEPENDENCIES.md`.
- PKG-00 rows were checked against local `_CONTEXT.md` Applicable Basis IDs and PKG-00 downstream-use notes; no PKG-00 files were written.
- Core enum fields conform to canonical v3.1 write-form enums. Legacy project-specific labels remain preserved in `Notes` as `legacy_*` fields where present.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Warnings
None.

## Run History
- 2026-06-16: `dependency-extract` semantic refresh for reconciliation; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings: none; ACTIVE rows: 17; RETIRED rows: 0.

## Lifecycle Summary
- Extraction lifecycle: 17 ACTIVE; 0 RETIRED.
- Closure status by `SatisfactionStatus`: SATISFIED=17.
- Closure note: register is schema-valid and evidence-reviewed for dependency semantics only; downstream graph authority remains with approved/accepted graph workflows.
