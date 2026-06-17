# Dependencies: DEL-06-04 Private rule-pack lifecycle and checksum handling

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
- **Semantic refreshed:** 2026-06-16
- **Rows:** 22 total; 22 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=13; EXECUTION=9.
- **PKG-00 architecture-basis rows reviewed:** 7; changed by this refresh: 0.
- **Rows added by this refresh:** 13.
- **Rows retired by this refresh:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-DEL-06-04-A001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | SOW-042 | ACTIVE |
| DAG-002-E0176 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0177 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0178 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0179 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-04 | ACTIVE |
| DAG-002-E0180 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0181 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0182 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0472 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DAG-002-E0473 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-02-05 | ACTIVE |
| DEP-DEL-06-04-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-001 | ACTIVE |
| DEP-DEL-06-04-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-002 | ACTIVE |
| DEP-DEL-06-04-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-003 | ACTIVE |
| DEP-DEL-06-04-A005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-004 | ACTIVE |
| DEP-DEL-06-04-A006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-005 | ACTIVE |
| DEP-DEL-06-04-A007 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-006 | ACTIVE |
| DEP-DEL-06-04-A008 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-007 | ACTIVE |
| DEP-DEL-06-04-A009 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-008 | ACTIVE |
| DEP-DEL-06-04-A010 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-009 | ACTIVE |
| DEP-DEL-06-04-A011 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-010 | ACTIVE |
| DEP-DEL-06-04-A012 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-011 | ACTIVE |
| DEP-DEL-06-04-A013 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | R-06-04-012 | ACTIVE |

## Canonical Dependency Types
- `INTERFACE`: 1
- `OTHER`: 13
- `PREREQUISITE`: 8

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
- 2026-06-16 23:59 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition=located; rows=22 total/22 ACTIVE/0 RETIRED; added=13; retired=0; PKG-00 reviewed=7 changed=0; warnings=none.

## Lifecycle Summary
- ACTIVE rows: 22
- RETIRED rows: 0
- SatisfactionStatus `NOT_APPLICABLE`: 13
- SatisfactionStatus `SATISFIED`: 7
- SatisfactionStatus `TBD`: 2

## Downstream Handoff Notes
- Register is refreshed for reconciliation consumption only; downstream graph authority remains governed by the active DAG/decomposition approval workflow.
