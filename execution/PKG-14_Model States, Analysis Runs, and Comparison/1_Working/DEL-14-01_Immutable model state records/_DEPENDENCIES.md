# Dependencies: DEL-14-01 Immutable model state records

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FOR_RECONCILIATION
- **Approved graph authority used:** `execution/_DAG/DAG-006/`
- **Source of Truth Boundary:** DAG-006 remains the approved graph authority; DAG-003 was not approved, promoted, or used as authority.
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Last refreshed:** 2026-05-11

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed evidence surface for reconciliation, not an independent graph approval.
- Existing DAG-002 mirror rows were preserved by `DependencyID` and evidence, with legacy/custom enum values normalized to canonical v3.1 values for local validation.
- No `DAG-003` row was approved or promoted in this refresh.
- `PKG-00` architecture-basis rows remain context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| DependencyID | Class | AnchorType | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|---|
| DEL-14-01-A001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | OTHER | SOW-071 | ACTIVE | EXTRACTED |
| DEL-14-01-A002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-016 | ACTIVE | EXTRACTED |
| DAG-002-E0667 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | DECLARED |
| DAG-002-E0668 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | DECLARED |
| DAG-002-E0669 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | DECLARED |
| DAG-002-E0670 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-04 | ACTIVE | DECLARED |
| DAG-002-E0671 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | DECLARED |
| DAG-002-E0672 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | DECLARED |
| DAG-002-E0673 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | DECLARED |
| DAG-002-E0779 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-02-01 | ACTIVE | DECLARED |
| DAG-002-E0780 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-02-05 | ACTIVE | DECLARED |
| DAG-002-E0781 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-08-02 | ACTIVE | DECLARED |
| DAG-002-E0782 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-05-04 | ACTIVE | DECLARED |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Approved graph authority: `execution/_DAG/DAG-006/DependencyEdges.csv` and `execution/_DAG/DAG-006/APPROVAL_RECORD.md`.
- Anchor source documents: `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution source documents: existing `Dependencies.csv` DAG-002 mirror, `Procedure.md`, `Specification.md`, `Guidance.md`, and `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md`.
- Preserved 11 DAG-002 mirror rows by `DependencyID`; updated `LastSeen` to 2026-05-11.
- Added 2 explicit ANCHOR rows: one parent anchor to `SOW-071` and one objective trace to `OBJ-016`.
- Normalized legacy DAG-002 execution row fields for v3.1 validation: `AnchorType=DELIVERABLE` became `NOT_APPLICABLE`; custom `DependencyType` values became `PREREQUISITE`; `Origin=CONTEXT/GRAPH_REVIEW` became `DECLARED`; `Explicitness=INFERRED_DIRECT` became `IMPLICIT`; `SatisfactionStatus=UNKNOWN` became `TBD`.
- No downstream consumer rows were added for DEL-14-02, DEL-14-03, DEL-14-05, DEL-07-08, DEL-08-06, DEL-15-01, DEL-15-04, or DEL-16-03 because those are already represented as approved DAG-006 upstream dependencies in those consumer deliverables; adding inverse active rows here would be a reconciliation risk.
- [WARNING] ENUM_NORMALIZATION: Custom DAG-002 edge semantics were preserved in `Notes`, but local enum fields were normalized to the current validator vocabulary.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.

## Run History
- 2026-05-03: Local register synchronized from approved DAG-006 mirror; 11 ACTIVE rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh for `CONSUMER_CONTEXT=RECONCILIATION`; mode `UPDATE`; strictness `CONSERVATIVE`; DAG-002 authority preserved; 13 ACTIVE rows; 0 RETIRED; warnings: `ENUM_NORMALIZATION`.

## Lifecycle Summary

| Breakdown | Count |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |
| CANDIDATE | 0 |
| ANCHOR | 2 |
| EXECUTION | 11 |
| OTHER | 2 |
| PREREQUISITE | 11 |
| SATISFIED | 7 |
| TBD | 4 |
| NOT_APPLICABLE | 2 |

## Downstream Handoff Notes
- For RECONCILIATION: treat `DAG-002-*` rows as preserved approved-graph evidence with canonicalized local enum fields, not as new edge proposals.
- For RECONCILIATION: `DEL-14-01-A001` and `DEL-14-01-A002` are Tree/trace anchors only and should not be used as execution sequencing edges.
- Later graph reconciliation should compare these refreshed rows against DAG-002 and any preliminary DAG-003 candidate material without silently promoting candidates or inverse downstream edges.
