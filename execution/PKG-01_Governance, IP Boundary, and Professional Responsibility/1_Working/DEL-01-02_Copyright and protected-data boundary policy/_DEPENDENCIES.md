# Dependencies: DEL-01-02 Copyright and protected-data boundary policy

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
- **Classes:** ANCHOR=4; EXECUTION=9.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DAG-002-E0005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE |
| DAG-002-E0006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE |
| DAG-002-E0007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE |
| DAG-002-E0008 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE |
| DAG-002-E0389 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-01 | ACTIVE |
| DEL-01-02-A001 | ANCHOR | UPSTREAM | OTHER | PKG-01 | ACTIVE |
| DEL-01-02-A002 | ANCHOR | UPSTREAM | OTHER | SOW-003 | ACTIVE |
| DEL-01-02-A003 | ANCHOR | UPSTREAM | OTHER | SOW-028 | ACTIVE |
| DEL-01-02-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-002 | ACTIVE |
| DEL-01-02-E001 | EXECUTION | UPSTREAM | PREREQUISITE | docs/CONTRACT.md | ACTIVE |
| DEL-01-02-E002 | EXECUTION | UPSTREAM | PREREQUISITE | HUMAN_LEGAL_REVIEW | ACTIVE |
| DEL-01-02-E003 | EXECUTION | UPSTREAM | PREREQUISITE | HUMAN_PROJECT_AUTHORITY | ACTIVE |
| DEL-01-02-E004 | EXECUTION | DOWNSTREAM | HANDOVER | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE |

## Canonical Dependency Types
- `HANDOVER`: 1
- `OTHER`: 4
- `PREREQUISITE`: 8

## Run Notes
- **Mode:** UPDATE; **Strictness:** CONSERVATIVE; **Consumer context:** RECONCILIATION.
- **Architecture basis policy:** PKG00_CONSISTENCY_TRACKERS.
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for anchor/label checks.
- **Anchor doc:** `Datasheet.md`.
- **Execution docs:** `Procedure.md`, `Specification.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- **PKG-00 rows reviewed:** 4 (`DEL-00-01`, `DEL-00-02`, `DEL-00-06`, `DEL-00-08`); all retained as supported architecture-basis consistency trackers; no PKG-00 files were modified.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: two external prerequisites remain PENDING (`HUMAN_LEGAL_REVIEW`, `HUMAN_PROJECT_AUTHORITY`).

## Run History
- 2026-06-16 18:09 MDT - `dependency-semantic-refresh`, UPDATE, CONSERVATIVE, decomposition located, warnings: pending external human/legal and project-authority prerequisites, ACTIVE rows: 13, RETIRED rows: 0.

## Lifecycle Summary
- **Status:** ACTIVE=13; RETIRED=0.
- **SatisfactionStatus:** SATISFIED=11; PENDING=2.
- **Closure state:** dependency register schema-valid; all ACTIVE rows cite evidence; no floating or ambiguous parent-anchor warning.

## Downstream Handoff Notes
- External legal/project-authority prerequisites remain blockers for final acceptance; they are not resolved by this refresh.
- Local refresh evidence remains derivative package evidence and does not replace decomposition truth or aggregate DAG authority.
