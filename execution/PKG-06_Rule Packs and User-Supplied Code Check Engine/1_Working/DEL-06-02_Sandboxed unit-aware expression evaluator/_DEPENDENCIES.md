# Dependencies: DEL-06-02 Sandboxed unit-aware expression evaluator

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
- **Rows:** 22 total; 21 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=12; EXECUTION=10.
- **PKG-00 architecture-basis rows reviewed:** 7; changed by this refresh: 0.
- **Rows added by this refresh:** 6.
- **Rows retired by this refresh:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEL-06-02-A001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | SOW-045 | ACTIVE |
| DEL-06-02-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-001 | ACTIVE |
| DEL-06-02-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-003 | ACTIVE |
| DEL-06-02-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-004 | ACTIVE |
| DEL-06-02-A005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-007 | ACTIVE |
| DEL-06-02-A006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-008 | ACTIVE |
| DAG-002-E0162 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0163 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0164 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0165 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-04 | ACTIVE |
| DAG-002-E0166 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0167 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0168 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0467 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DAG-002-E0468 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-02-02 | ACTIVE |
| DAG-002-E0623 | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | DEL-12-05 | RETIRED |
| DEP-DEL-06-02-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-002 | ACTIVE |
| DEP-DEL-06-02-A006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-005 | ACTIVE |
| DEP-DEL-06-02-A007 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-006 | ACTIVE |
| DEP-DEL-06-02-A010 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-009 | ACTIVE |
| DEP-DEL-06-02-A011 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-010 | ACTIVE |
| DEP-DEL-06-02-A012 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | REQ-06-02-011 | ACTIVE |

## Canonical Dependency Types
- `CONSTRAINT`: 8
- `INTERFACE`: 2
- `OTHER`: 12

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
- 2026-06-16 23:59 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition=located; rows=22 total/21 ACTIVE/1 RETIRED; added=6; retired=0; PKG-00 reviewed=7 changed=0; warnings=none.

## Lifecycle Summary
- ACTIVE rows: 21
- RETIRED rows: 1
- SatisfactionStatus `NOT_APPLICABLE`: 12
- SatisfactionStatus `SATISFIED`: 7
- SatisfactionStatus `TBD`: 3

## Downstream Handoff Notes
- Register is refreshed for reconciliation consumption only; downstream graph authority remains governed by the active DAG/decomposition approval workflow.
