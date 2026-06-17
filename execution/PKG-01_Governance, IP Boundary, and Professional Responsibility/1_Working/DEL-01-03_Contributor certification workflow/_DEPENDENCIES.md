# Dependencies: DEL-01-03 Contributor certification workflow

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
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=11.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DAG-002-E0009 | EXECUTION | UPSTREAM | OTHER | DEL-00-01 | ACTIVE |
| DAG-002-E0010 | EXECUTION | UPSTREAM | OTHER | DEL-00-02 | ACTIVE |
| DAG-002-E0011 | EXECUTION | UPSTREAM | OTHER | DEL-00-06 | ACTIVE |
| DAG-002-E0012 | EXECUTION | UPSTREAM | OTHER | DEL-00-08 | ACTIVE |
| DAG-002-E0390 | EXECUTION | UPSTREAM | OTHER | DEL-01-01 | ACTIVE |
| DAG-002-E0391 | EXECUTION | UPSTREAM | OTHER | DEL-01-02 | ACTIVE |
| DEL-01-03-A001 | ANCHOR | UPSTREAM | OTHER | PKG-01 | ACTIVE |
| DEL-01-03-A002 | ANCHOR | UPSTREAM | OTHER | SOW-028 | ACTIVE |
| DEL-01-03-A003 | ANCHOR | UPSTREAM | OTHER | SOW-048 | ACTIVE |
| DEL-01-03-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-002 | ACTIVE |
| DEL-01-03-E001 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-CONTRACT | ACTIVE |
| DEL-01-03-E002 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-IP-DATA-BOUNDARY | ACTIVE |
| DEL-01-03-E003 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-SOFTWARE-DECOMP | ACTIVE |
| DEL-01-03-E004 | EXECUTION | DOWNSTREAM | HANDOVER | CONTRIBUTING.md | ACTIVE |
| DEL-01-03-E005 | EXECUTION | DOWNSTREAM | HANDOVER | OPS-CONTRIBUTOR-CERTIFICATION-TEMPLATE | ACTIVE |

## Canonical Dependency Types
- `HANDOVER`: 2
- `OTHER`: 10
- `PREREQUISITE`: 3

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
- Warnings: final DCO/CLA/other legal mechanism and human acceptance remain TBD in row notes; no register warning.

## Run History
- 2026-06-16 18:09 MDT - `dependency-semantic-refresh`, UPDATE, CONSERVATIVE, decomposition located, warnings: final certification/legal mechanism remains TBD in evidence notes, ACTIVE rows: 15, RETIRED rows: 0.

## Lifecycle Summary
- **Status:** ACTIVE=15; RETIRED=0.
- **SatisfactionStatus:** SATISFIED=15.
- **Closure state:** dependency register schema-valid; all ACTIVE rows cite evidence; no floating or ambiguous parent-anchor warning.

## Downstream Handoff Notes
- Draft governance artifacts are represented only as handoff evidence; final mechanism and acceptance remain human-gated.
- Local refresh evidence remains derivative package evidence and does not replace decomposition truth or aggregate DAG authority.
