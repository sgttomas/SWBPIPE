# Dependencies: DEL-09-01 Mechanics benchmark suite

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Refresh run:** dependency semantic refresh, 2026-06-16
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=11.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEL-09-01-A001` | ANCHOR | UPSTREAM | OTHER | DEL-09-01 | ACTIVE | Datasheet.md / Identification |
| `DEL-09-01-A002` | ANCHOR | UPSTREAM | OTHER | SOW-026 | ACTIVE | Datasheet.md / Identification |
| `DEL-09-01-A003` | ANCHOR | UPSTREAM | OTHER | OBJ-008 | ACTIVE | Datasheet.md / Identification |
| `DAG-002-E0274` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0275` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0276` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0277` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0532` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | ACTIVE | Procedure.md / Prerequisites |
| `DAG-002-E0533` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-02 | ACTIVE | Procedure.md / Prerequisites |
| `DAG-002-E0534` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 | ACTIVE | Procedure.md / Prerequisites |
| `DAG-002-E0535` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 | ACTIVE | Procedure.md / Prerequisites |
| `DAG-002-E0536` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-02 | ACTIVE | Specification.md / Requirements / DEL-09-01-RQ-003 |
| `TP-DAG-004-DEL-09-01-E001` | EXECUTION | UPSTREAM | INTERFACE | DEL-02-02 | ACTIVE | Specification.md / Requirements / DEL-09-01-RQ-004 |
| `TP-DAG-004-DEL-09-01-E002` | EXECUTION | UPSTREAM | INTERFACE | DEL-04-06 | ACTIVE | Specification.md / Requirements / DEL-09-01-RQ-005 |

## Canonical Dependency Types
- `INTERFACE`: 2
- `OTHER`: 3
- `PREREQUISITE`: 9

## PKG-00 Architecture-Basis Review
- **Rows reviewed:** 4
- **Rows changed:** 0
- Reviewed `DAG-002-E0274` -> `DEL-00-01` (Architecture decision record baseline): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0275` -> `DEL-00-02` (Repository and module boundary architecture): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0276` -> `DEL-00-06` (Diagnostics, warning, and result-envelope contract): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0277` -> `DEL-00-08` (Layered software test and acceptance strategy): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.

## Retired Row Disposition
- None.

## Run Notes
- TaskSkill: `dependency-extract`; MODE: `UPDATE`; STRICTNESS: `CONSERVATIVE`; CONSUMER_CONTEXT: `RECONCILIATION`; ARCHITECTURE_BASIS_POLICY: `PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for anchor/target validation.
- Source document selection: `AUTO`; anchor document: `Datasheet.md`; execution documents reviewed from local `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, and `_REFERENCES.md` as needed.
- Core enum fields conform to the canonical Chirality dependency model. No legacy core enum values or `Status=CANDIDATE` values are emitted.
- PKG-00 / DEL-00-* rows were treated as valid architecture-consistency dependency trackers when supported; no PKG-00 files were edited.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Warnings
- None.

## Downstream Handoff Notes
- Consumer context is `RECONCILIATION`; downstream aggregation may consume ACTIVE rows as canonical local register evidence.
- Candidate/non-gating ideas require explicit human approval plus graph revalidation before promotion.
- RETIRED rows are preserved for provenance and must not be treated as active gating dependencies.

## Run History
- 2026-06-16 0000 America/Edmonton: semantic refresh from sealed brief `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/WorkerBriefs/PKG-09_dependency_semantic_refresh.md`; ACTIVE rows=14; RETIRED rows=0; PKG-00 rows reviewed=4; warnings=0.

## Lifecycle Summary
- ACTIVE rows: 14
- RETIRED rows: 0
- Closure-state breakdown:
- `NOT_APPLICABLE`: 3
- `SATISFIED`: 4
- `TBD`: 7
