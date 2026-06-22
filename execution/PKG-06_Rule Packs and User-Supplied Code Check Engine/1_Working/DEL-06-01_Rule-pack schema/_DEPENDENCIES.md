# Dependencies: DEL-06-01 Rule-pack schema

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
- **Rows:** 25 total; 25 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=13; EXECUTION=12.
- **PKG-00 architecture-basis rows reviewed:** 7; changed by this refresh: 0.
- **Rows added by this refresh:** 13.
- **Rows retired by this refresh:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-DEL-06-01-A001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | SOW-016 | ACTIVE |
| DAG-002-E0155 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0156 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0157 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0158 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-04 | ACTIVE |
| DAG-002-E0159 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0160 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0161 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0462 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-02-01 | ACTIVE |
| DAG-002-E0463 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-02-02 | ACTIVE |
| DAG-002-E0464 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-03 | ACTIVE |
| DAG-002-E0465 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-01-02 | ACTIVE |
| DAG-002-E0466 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-01-04 | ACTIVE |
| DEP-DEL-06-01-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-001 | ACTIVE |
| DEP-DEL-06-01-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-002 | ACTIVE |
| DEP-DEL-06-01-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-003 | ACTIVE |
| DEP-DEL-06-01-A005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-004 | ACTIVE |
| DEP-DEL-06-01-A006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-005 | ACTIVE |
| DEP-DEL-06-01-A007 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-006 | ACTIVE |
| DEP-DEL-06-01-A008 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-007 | ACTIVE |
| DEP-DEL-06-01-A009 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-008 | ACTIVE |
| DEP-DEL-06-01-A010 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-009 | ACTIVE |
| DEP-DEL-06-01-A011 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-010 | ACTIVE |
| DEP-DEL-06-01-A012 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-011 | ACTIVE |
| DEP-DEL-06-01-A013 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-01-012 | ACTIVE |

## Canonical Dependency Types
- `INTERFACE`: 2
- `OTHER`: 13
- `PREREQUISITE`: 10

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
- 2026-06-16 23:59 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition=located; rows=25 total/25 ACTIVE/0 RETIRED; added=13; retired=0; PKG-00 reviewed=7 changed=0; warnings=none.

## Lifecycle Summary
- ACTIVE rows: 25
- RETIRED rows: 0
- SatisfactionStatus `NOT_APPLICABLE`: 13
- SatisfactionStatus `SATISFIED`: 7
- SatisfactionStatus `TBD`: 5

## Downstream Handoff Notes
- Register is refreshed for reconciliation consumption only; downstream graph authority remains governed by the active DAG/decomposition approval workflow.
