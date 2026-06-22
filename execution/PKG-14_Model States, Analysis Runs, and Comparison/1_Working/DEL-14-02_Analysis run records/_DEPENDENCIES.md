# Dependencies: DEL-14-02 Analysis run records

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded outside the local canonical register.

## Declared Downstream Dependencies
- None recorded outside the local canonical register.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=12.
- **PKG-00 architecture-basis trackers:** 7 reviewed; 0 changed; all retained ACTIVE as supported consistency trackers.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | TargetPackage | Target | Status |
|---|---|---:|---|---|---|---|
| `DAG-002-E0674` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-01 | ACTIVE |
| `DAG-002-E0675` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-02 | ACTIVE |
| `DAG-002-E0676` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-03 | ACTIVE |
| `DAG-002-E0677` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-04 | ACTIVE |
| `DAG-002-E0678` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-06 | ACTIVE |
| `DAG-002-E0679` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-07 | ACTIVE |
| `DAG-002-E0680` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-00 | DEL-00-08 | ACTIVE |
| `DAG-002-E0783` | EXECUTION | UPSTREAM | INTERFACE | PKG-14 | DEL-14-01 | ACTIVE |
| `DAG-002-E0784` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-05 | DEL-05-04 | ACTIVE |
| `DAG-002-E0785` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-08 | DEL-08-02 | ACTIVE |
| `DAG-002-E0786` | EXECUTION | UPSTREAM | PREREQUISITE | PKG-08 | DEL-08-04 | ACTIVE |
| `DAG-002-E0787` | EXECUTION | UPSTREAM | INTERFACE | PKG-02 | DEL-02-05 | ACTIVE |
| `DEL-14-02-A001` | ANCHOR | UPSTREAM | OTHER | - | SOW-072 | ACTIVE |
| `DEL-14-02-A002` | ANCHOR | UPSTREAM | OTHER | - | OBJ-016 | ACTIVE |

## Canonical Dependency Types
- `INTERFACE`: 2
- `OTHER`: 2
- `PREREQUISITE`: 10

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
- 2026-06-16: `dependency-extract` semantic refresh for reconciliation; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings: none; ACTIVE rows: 14; RETIRED rows: 0.

## Lifecycle Summary
- Extraction lifecycle: 14 ACTIVE; 0 RETIRED.
- Closure status by `SatisfactionStatus`: PENDING=2; SATISFIED=7; TBD=5.
- Closure note: register is schema-valid and evidence-reviewed for dependency semantics only; downstream graph authority remains with approved/accepted graph workflows.
