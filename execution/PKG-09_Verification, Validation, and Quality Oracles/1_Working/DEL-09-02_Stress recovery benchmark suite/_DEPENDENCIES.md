# Dependencies: DEL-09-02 Stress recovery benchmark suite

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
- **Rows:** 11 total; 10 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=8.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEL-09-02-A001` | ANCHOR | UPSTREAM | OTHER | DEL-09-02 | ACTIVE | Datasheet.md / Identification |
| `DEL-09-02-A002` | ANCHOR | UPSTREAM | OTHER | SOW-026 | ACTIVE | Datasheet.md / Identification |
| `DEL-09-02-A003` | ANCHOR | UPSTREAM | OTHER | OBJ-008 | ACTIVE | Datasheet.md / Identification |
| `DAG-002-E0278` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection |
| `DAG-002-E0279` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection |
| `DAG-002-E0280` | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection |
| `DAG-002-E0281` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection |
| `DAG-002-E0537` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-03 | ACTIVE | Procedure.md / Prerequisites |
| `DAG-002-E0538` | EXECUTION | UPSTREAM | INTERFACE | DEL-03-08 | ACTIVE | Specification.md / Scope / Requirements |
| `DAG-002-E0539` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-02 | RETIRED | Deliverables.csv / location TBD |
| `DAG-002-E0540` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | ACTIVE | Procedure.md / Prerequisites |

## Canonical Dependency Types
- `CONSTRAINT`: 4
- `INTERFACE`: 2
- `OTHER`: 3
- `PREREQUISITE`: 2

## PKG-00 Architecture-Basis Review
- **Rows reviewed:** 4
- **Rows changed:** 0
- Reviewed `DAG-002-E0278` -> `DEL-00-01` (Architecture decision record baseline): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0279` -> `DEL-00-02` (Repository and module boundary architecture): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0280` -> `DEL-00-06` (Diagnostics, warning, and result-envelope contract): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.
- Reviewed `DAG-002-E0281` -> `DEL-00-08` (Layered software test and acceptance strategy): retained as supported by `_CONTEXT.md` architecture-basis injection and SOFTWARE_DECOMP AB rows; no PKG-00 files changed.

## Retired Row Disposition
- `DAG-002-E0539` remains RETIRED: Prior inferred edge claimed DEL-09-02 consumes straight-pipe element force recovery, but current sealed source docs do not state this dependency directly.

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
- 2026-06-16 0000 America/Edmonton: semantic refresh from sealed brief `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/WorkerBriefs/PKG-09_dependency_semantic_refresh.md`; ACTIVE rows=10; RETIRED rows=1; PKG-00 rows reviewed=4; warnings=0.

## Lifecycle Summary
- ACTIVE rows: 10
- RETIRED rows: 1
- Closure-state breakdown:
- `PENDING`: 3
- `SATISFIED`: 7
- `TBD`: 1
