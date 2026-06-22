# Dependencies: DEL-07-02 Model tree and property inspector

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
- **Rows:** 20 total; 20 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=16.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE` row(s).
- **Rows added this run:** 1
- **Rows retired this run:** 0
- **Rows changed this run:** 0
- **PKG-00 rows reviewed/changed:** 7/0

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-007-02-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | PKG-07 | ACTIVE |
| DEP-007-02-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-020 | ACTIVE |
| DEP-007-02-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-021 | ACTIVE |
| DEP-007-02-010 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-006 | ACTIVE |
| DAG-002-E0197 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0198 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0199 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0200 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-05 | ACTIVE |
| DAG-002-E0201 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0202 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0203 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0486 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-01 | ACTIVE |
| DAG-002-E0487 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-05 | ACTIVE |
| DAG-002-E0488 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-02 | ACTIVE |
| DEP-007-02-004 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-02 | ACTIVE |
| DEP-007-02-005 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-01 | ACTIVE |
| DEP-007-02-006 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DEP-007-02-007 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-04 | ACTIVE |
| DEP-007-02-008 | EXECUTION | DOWNSTREAM | INTERFACE | DELIVERABLE | DEL-07-03 | ACTIVE |
| DEP-007-02-009 | EXECUTION | DOWNSTREAM | INTERFACE | DELIVERABLE | DEL-07-04 | ACTIVE |

## Canonical Dependency Types
- `INTERFACE`: 2
- `OTHER`: 4
- `PREREQUISITE`: 14

## Run Notes
- Mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; architecture-basis policy `PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Anchor document: `_CONTEXT.md`; execution documents reviewed as needed: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, existing `Dependencies.csv`, and `_DEPENDENCIES.md`.
- PKG-00 architecture-basis rows were reviewed against `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-01`, `DEL-00-02`, `DEL-00-03`, `DEL-00-05`, `DEL-00-06`, `DEL-00-07`, and `DEL-00-08` source documents; supported rows were retained.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where previously present.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Warnings
- None.

## Run History
- 2026-06-16: dependency semantic refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; rows 20 total, 20 ACTIVE, 0 RETIRED; warnings 0.

## Lifecycle Summary
- ACTIVE rows: 20
- RETIRED rows: 0
- Satisfaction status counts: {'NOT_APPLICABLE': 4, 'SATISFIED': 7, 'TBD': 9}

## Downstream Handoff Notes
- Consume this register as a deliverable-local semantic refresh shard only. It is not graph authority and does not update `_DAG/_LATEST.md`.
