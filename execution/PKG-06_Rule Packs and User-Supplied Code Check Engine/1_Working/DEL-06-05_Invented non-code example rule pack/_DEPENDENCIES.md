# Dependencies: DEL-06-05 Invented non-code example rule pack

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
- **Semantic refreshed:** 2026-06-16
- **Rows:** 20 total; 18 ACTIVE; 2 RETIRED.
- **Classes:** ANCHOR=9; EXECUTION=11.
- **PKG-00 architecture-basis rows reviewed:** 7; changed by this refresh: 0.
- **Rows added by this refresh:** 6.
- **Rows retired by this refresh:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-DEL-06-05-A001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | SOW-016 | ACTIVE |
| DEP-DEL-06-05-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-01 | ACTIVE |
| DEP-DEL-06-05-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-03 | ACTIVE |
| DAG-002-E0183 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0184 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0185 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0186 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-04 | ACTIVE |
| DAG-002-E0187 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0188 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0189 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0474 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DAG-002-E0475 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-02 | ACTIVE |
| DAG-002-E0476 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-01-02 | RETIRED |
| DAG-002-E0477 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-01-04 | RETIRED |
| DEP-DEL-06-05-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-02 | ACTIVE |
| DEP-DEL-06-05-A005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-04 | ACTIVE |
| DEP-DEL-06-05-A006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-05 | ACTIVE |
| DEP-DEL-06-05-A007 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-06 | ACTIVE |
| DEP-DEL-06-05-A008 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-07 | ACTIVE |
| DEP-DEL-06-05-A009 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | DEL-06-05-REQ-08 | ACTIVE |

## Canonical Dependency Types
- `CONSTRAINT`: 9
- `OTHER`: 9
- `PREREQUISITE`: 2

## Run Notes
- Run controls: TaskSkill `dependency-extract`; MODE `UPDATE`; STRICTNESS `CONSERVATIVE`; CONSUMER_CONTEXT `RECONCILIATION`; ARCHITECTURE_BASIS_POLICY `PKG00_CONSISTENCY_TRACKERS`; ApplyEdits `true`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for scope/package/architecture-basis validation.
- Anchor document selection: `Datasheet.md` and `_CONTEXT.md` for scope identity; `Specification.md` for local requirement trace anchors.
- Execution document order: `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, and relevant PKG-00 source excerpts for architecture-basis consistency.
- PKG-00 / DEL-00-* rows are retained as architecture-consistency dependency trackers because `_CONTEXT.md` explicitly lists AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 for this sealed context.
- Existing RETIRED rows were preserved; no rows were deleted.
- Candidate/non-gating ideas were not emitted as `Status=CANDIDATE`.
- Warnings: None.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16 23:59 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition=located; rows=20 total/18 ACTIVE/2 RETIRED; added=6; retired=0; PKG-00 reviewed=7 changed=0; warnings=none.

## Lifecycle Summary
- ACTIVE rows: 18
- RETIRED rows: 2
- SatisfactionStatus `NOT_APPLICABLE`: 9
- SatisfactionStatus `PENDING`: 2
- SatisfactionStatus `SATISFIED`: 7
- SatisfactionStatus `TBD`: 2

## Downstream Handoff Notes
- Register is refreshed for reconciliation consumption only; downstream graph authority remains governed by the active DAG/decomposition approval workflow.
