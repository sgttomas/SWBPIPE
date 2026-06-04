# Dependencies: DEL-16-02 Operation validation and diff preview

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FOR_RECONCILIATION
- **Approved graph authority used:** `execution/_DAG/DAG-006/`
- **Source of Truth Boundary:** DAG-006 remains the approved graph authority; DAG-003 was not approved, promoted, or used as authority.
- **Local Register:** `Dependencies.csv`
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED; 0 CANDIDATE.
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
| DEL-16-02-A001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | OTHER | SOW-069 | ACTIVE | EXTRACTED |
| DEL-16-02-A002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-015 | ACTIVE | EXTRACTED |
| DAG-002-E0737 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | DECLARED |
| DAG-002-E0738 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | DECLARED |
| DAG-002-E0739 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | DECLARED |
| DAG-002-E0740 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-04 | ACTIVE | DECLARED |
| DAG-002-E0741 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | DECLARED |
| DAG-002-E0742 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | DECLARED |
| DAG-002-E0743 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | DECLARED |
| DAG-002-E0827 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-16-01 | ACTIVE | DECLARED |
| DAG-002-E0828 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-13-03 | ACTIVE | DECLARED |
| DAG-002-E0829 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-14-03 | ACTIVE | DECLARED |
| DAG-002-E0830 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-14-05 | ACTIVE | DECLARED |
| DAG-002-E0831 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-04-06 | ACTIVE | DECLARED |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Approved graph authority: `execution/_DAG/DAG-006/DependencyEdges.csv` and `execution/_DAG/DAG-006/APPROVAL_RECORD.md`.
- Anchor source documents: `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution source documents: existing `Dependencies.csv` DAG-002 mirror, `Procedure.md`, `Specification.md`, `Guidance.md`, and `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md`.
- Preserved 12 DAG-002 mirror rows by `DependencyID`; updated `LastSeen` to 2026-05-11.
- Added 2 explicit ANCHOR rows: one parent anchor to `SOW-069` and one objective trace to `OBJ-015`.
- Normalized legacy DAG-002 execution row fields for v3.1 validation: `AnchorType=DELIVERABLE` became `NOT_APPLICABLE`; custom `DependencyType` values became `PREREQUISITE`; `Origin=CONTEXT/GRAPH_REVIEW` became `DECLARED`; `Explicitness=INFERRED_DIRECT` became `IMPLICIT`; `SatisfactionStatus=UNKNOWN` became `TBD`.
- No downstream consumer row was added for `DEL-16-03` or `DEL-07-08` because the approved DAG-006 edges are already represented as upstream dependencies in the consumer deliverables; adding inverse active rows here would be a reconciliation risk.
- [WARNING] ENUM_NORMALIZATION: Custom DAG-002 edge semantics were preserved in `Notes`, but local enum fields were normalized to the current validator vocabulary.
- [WARNING] ID_FORMAT_HELPER_DRIFT: `validate_id_format.sh` rejects project-local IDs such as `DEL-16-02`, `PKG-16`, and `SOW-069` because the helper expects older three-digit package/deliverable and four-digit SOW formats. IDs were validated against the decomposition/register evidence instead.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.

## Run History
- 2026-05-03: Local register synchronized from approved DAG-006 mirror; 12 ACTIVE rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh for `CONSUMER_CONTEXT=RECONCILIATION`; mode `UPDATE`; strictness `CONSERVATIVE`; DAG-002 authority preserved; 14 ACTIVE rows; 0 RETIRED; warnings: `ENUM_NORMALIZATION`, `ID_FORMAT_HELPER_DRIFT`.

## Lifecycle Summary

| Breakdown | Count |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |
| CANDIDATE | 0 |
| ANCHOR | 2 |
| EXECUTION | 12 |
| OTHER | 2 |
| PREREQUISITE | 12 |
| SATISFIED | 7 |
| TBD | 5 |
| NOT_APPLICABLE | 2 |

## Downstream Handoff Notes
- For RECONCILIATION: treat `DAG-002-*` rows as preserved approved-graph evidence with canonicalized local enum fields, not as new edge proposals.
- For RECONCILIATION: `DEL-16-02-A001` and `DEL-16-02-A002` are Tree/trace anchors only and should not be used as execution sequencing edges.
- Later graph reconciliation should compare these refreshed rows against DAG-002 and any preliminary DAG-003 candidate material without silently promoting candidates or inverse downstream edges.
