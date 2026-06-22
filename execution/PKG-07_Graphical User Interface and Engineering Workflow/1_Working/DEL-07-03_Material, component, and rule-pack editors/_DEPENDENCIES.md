# Dependencies: DEL-07-03 Material, component, and rule-pack editors

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
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=14.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE` row(s).
- **Rows added this run:** 2
- **Rows retired this run:** 0
- **Rows changed this run:** 1
- **PKG-00 rows reviewed/changed:** 7/0

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEP-DEL-07-03-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | PKG-07 | ACTIVE |
| DEP-DEL-07-03-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-021 | ACTIVE |
| DEP-DEL-07-03-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-006 | ACTIVE |
| DAG-002-E0204 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0205 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0206 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0207 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-05 | ACTIVE |
| DAG-002-E0208 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0209 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0210 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0489 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-01 | ACTIVE |
| DAG-002-E0490 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-02 | ACTIVE |
| DAG-002-E0491 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-03-07 | ACTIVE |
| DAG-002-E0492 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-01 | ACTIVE |
| DAG-002-E0493 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-06-04 | ACTIVE |
| DAG-002-E0494 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-12-01 | ACTIVE |
| DEV-001-STAGE2-DEL-07-03-PKG02-001 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-02 | ACTIVE |

## Canonical Dependency Types
- `OTHER`: 3
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
- 2026-06-16: dependency semantic refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; rows 17 total, 17 ACTIVE, 0 RETIRED; warnings 0.

## Lifecycle Summary
- ACTIVE rows: 17
- RETIRED rows: 0
- Satisfaction status counts: {'NOT_APPLICABLE': 3, 'PENDING': 6, 'SATISFIED': 8}

## Downstream Handoff Notes
- Consume this register as a deliverable-local semantic refresh shard only. It is not graph authority and does not update `_DAG/_LATEST.md`.
