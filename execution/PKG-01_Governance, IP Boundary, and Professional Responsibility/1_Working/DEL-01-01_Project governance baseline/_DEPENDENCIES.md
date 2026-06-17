# Dependencies: DEL-01-01 Project governance baseline

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
- **Semantic refresh:** 2026-06-16
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=5; EXECUTION=8.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DAG-002-E0001 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE |
| DAG-002-E0002 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE |
| DAG-002-E0003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE |
| DAG-002-E0004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE |
| DEL-01-01-A001 | ANCHOR | UPSTREAM | OTHER | DEL-01-01 | ACTIVE |
| DEL-01-01-A002 | ANCHOR | UPSTREAM | OTHER | SOW-001 | ACTIVE |
| DEL-01-01-A003 | ANCHOR | UPSTREAM | OTHER | SOW-048 | ACTIVE |
| DEL-01-01-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-001 | ACTIVE |
| DEL-01-01-A005 | ANCHOR | UPSTREAM | OTHER | OBJ-002 | ACTIVE |
| DEL-01-01-D001 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-CONTRACT | ACTIVE |
| DEL-01-01-D002 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-DIRECTIVE | ACTIVE |
| DEL-01-01-D003 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-TYPES | ACTIVE |
| DEL-01-01-D004 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-AGENTIC-WORKFLOW | ACTIVE |

## Canonical Dependency Types
- `OTHER`: 5
- `PREREQUISITE`: 8

## Run Notes
- **Mode:** UPDATE; **Strictness:** CONSERVATIVE; **Consumer context:** RECONCILIATION.
- **Architecture basis policy:** PKG00_CONSISTENCY_TRACKERS.
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for anchor/label checks.
- **Anchor doc:** `Datasheet.md`.
- **Execution docs:** `Procedure.md`, `Specification.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- **PKG-00 rows reviewed:** 4 (`DEL-00-01`, `DEL-00-02`, `DEL-00-06`, `DEL-00-08`); all retained as supported architecture-basis consistency trackers; no PKG-00 files were modified.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: none.

## Run History
- 2026-06-16 18:09 MDT - `dependency-semantic-refresh`, UPDATE, CONSERVATIVE, decomposition located, warnings: none, ACTIVE rows: 13, RETIRED rows: 0.

## Lifecycle Summary
- **Status:** ACTIVE=13; RETIRED=0.
- **SatisfactionStatus:** SATISFIED=13.
- **Closure state:** dependency register schema-valid; all ACTIVE rows cite evidence; no floating or ambiguous parent-anchor warning.

## Downstream Handoff Notes
- No candidate rows were promoted.
- Local refresh evidence remains derivative package evidence and does not replace decomposition truth or aggregate DAG authority.
