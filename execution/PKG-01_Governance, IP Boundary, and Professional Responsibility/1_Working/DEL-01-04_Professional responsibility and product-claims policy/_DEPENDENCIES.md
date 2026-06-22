# Dependencies: DEL-01-04 Professional responsibility and product-claims policy

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 17 total; 16 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=5; EXECUTION=12.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-01-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-034 | ACTIVE |
| DEP-DEL-01-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE |
| DEP-DEL-01-04-A003 | ANCHOR | UPSTREAM | OTHER | PKG-01 | ACTIVE |
| DEP-DEL-01-04-A004 | ANCHOR | UPSTREAM | OTHER | SOW-064 | ACTIVE |
| DEP-DEL-01-04-A005 | ANCHOR | UPSTREAM | OTHER | OBJ-018 | ACTIVE |
| DAG-002-E0013 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE |
| DAG-002-E0014 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE |
| DAG-002-E0015 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE |
| DAG-002-E0016 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE |
| DAG-002-E0392 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-01 | RETIRED |
| DEP-DEL-01-04-E001 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-CONTRACT | ACTIVE |
| DEP-DEL-01-04-E002 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-IP-DATA-BOUNDARY | ACTIVE |
| DEP-DEL-01-04-E005 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-SOFTWARE-DECOMP | ACTIVE |
| DEP-DEL-01-04-E003 | EXECUTION | DOWNSTREAM | HANDOVER | docs/PROFESSIONAL_BOUNDARY.md | ACTIVE |
| DEP-DEL-01-04-E004 | EXECUTION | DOWNSTREAM | HANDOVER | OPS-REPORT-NOTICE-TEMPLATE | ACTIVE |
| DEP-DEL-01-04-E006 | EXECUTION | UPSTREAM | PREREQUISITE | HUMAN_LEGAL_PROFESSIONAL_REVIEW | ACTIVE |
| DEP-DEL-01-04-E007 | EXECUTION | UPSTREAM | PREREQUISITE | HUMAN_PROJECT_AUTHORITY | ACTIVE |

## Canonical Dependency Types
- `CONSTRAINT`: 1
- `HANDOVER`: 2
- `OTHER`: 5
- `PREREQUISITE`: 9

## Run Notes
- **Mode:** UPDATE; **Strictness:** CONSERVATIVE; **Consumer context:** RECONCILIATION.
- **Architecture basis policy:** PKG00_CONSISTENCY_TRACKERS.
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for anchor/label checks.
- **Anchor doc:** `Datasheet.md`.
- **Execution docs:** `Procedure.md`, `Specification.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- **PKG-00 rows reviewed:** 4 (`DEL-00-01`, `DEL-00-02`, `DEL-00-06`, `DEL-00-08`); all retained as supported architecture-basis consistency trackers; no PKG-00 files were modified.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- `DAG-002-E0392` remains RETIRED because current conservative local extraction did not reconfirm the prior aggregate-inferred DEL-01-01 execution dependency.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: two external prerequisites remain PENDING (`HUMAN_LEGAL_PROFESSIONAL_REVIEW`, `HUMAN_PROJECT_AUTHORITY`); one retired historical aggregate-inferred row remains in the register.

## Run History
- 2026-06-16 18:09 MDT - `dependency-semantic-refresh`, UPDATE, CONSERVATIVE, decomposition located, warnings: pending external review/project-authority prerequisites and one retired historical row, ACTIVE rows: 16, RETIRED rows: 1.

## Lifecycle Summary
- **Status:** ACTIVE=16; RETIRED=1.
- **SatisfactionStatus:** SATISFIED=14; PENDING=2; TBD=1.
- **Closure state:** dependency register schema-valid; all ACTIVE rows cite evidence; no floating or ambiguous parent-anchor warning.

## Downstream Handoff Notes
- External legal/professional review and project-authority acceptance remain blockers for final acceptance.
- Local refresh evidence remains derivative package evidence and does not replace decomposition truth or aggregate DAG authority.
