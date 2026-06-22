# Dependencies: DEL-07-08 Design-authoring state and comparison workspace

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
- **Rows:** 29 total; 27 ACTIVE; 2 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=25.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE` row(s).
- **Rows added this run:** 4
- **Rows retired this run:** 0
- **Rows changed this run:** 0
- **PKG-00 rows reviewed/changed:** 7/0

| DependencyID | Class | Direction | Type | TargetType | Target | Status |
|---|---|---|---|---|---|---|
| DEL-07-08-A001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | PKG-07 | ACTIVE |
| DEL-07-08-A002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-076 | ACTIVE |
| DEL-07-08-A003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-015 | ACTIVE |
| DEL-07-08-A004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-016 | ACTIVE |
| DAG-002-E0625 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 | ACTIVE |
| DAG-002-E0626 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 | ACTIVE |
| DAG-002-E0627 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 | ACTIVE |
| DAG-002-E0628 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-05 | ACTIVE |
| DAG-002-E0629 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 | ACTIVE |
| DAG-002-E0630 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 | ACTIVE |
| DAG-002-E0631 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 | ACTIVE |
| DAG-002-E0840 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-07-01 | ACTIVE |
| DAG-002-E0841 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-07-02 | ACTIVE |
| DAG-002-E0842 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-07-04 | ACTIVE |
| DAG-002-E0843 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-07-05 | ACTIVE |
| DAG-002-E0844 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-13-01 | ACTIVE |
| DAG-002-E0845 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-13-03 | ACTIVE |
| DAG-002-E0846 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-13-04 | ACTIVE |
| DAG-002-E0847 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-14-01 | ACTIVE |
| DAG-002-E0848 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-14-03 | ACTIVE |
| DAG-002-E0849 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-14-04 | ACTIVE |
| DAG-002-E0850 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-14-05 | ACTIVE |
| DAG-002-E0851 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-16-01 | ACTIVE |
| DAG-002-E0852 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-16-02 | ACTIVE |
| DAG-002-E0853 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-16-03 | ACTIVE |
| DEV-001-STAGE2-DEL-07-08-PKG02-001 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-03 | ACTIVE |
| DEV-001-STAGE2-DEL-07-08-PKG02-002 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-05 | ACTIVE |
| TP-DAG-004-DEL-07-08-C001 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-13-02 | RETIRED |
| TP-DAG-004-DEL-07-08-C002 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-14-02 | RETIRED |

## Canonical Dependency Types
- `INTERFACE`: 7
- `OTHER`: 4
- `PREREQUISITE`: 18

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
- 2026-06-16: dependency semantic refresh; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; rows 29 total, 27 ACTIVE, 2 RETIRED; warnings 0.

## Lifecycle Summary
- ACTIVE rows: 27
- RETIRED rows: 2
- Satisfaction status counts: {'NOT_APPLICABLE': 4, 'SATISFIED': 9, 'TBD': 16}

## Downstream Handoff Notes
- Consume this register as a deliverable-local semantic refresh shard only. It is not graph authority and does not update `_DAG/_LATEST.md`.
