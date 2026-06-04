# Dependencies: DEL-15-04 External prover boundary metadata

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FOR_RECONCILIATION
- **Approved graph authority used:** `execution/_DAG/DAG-006/`
- **Source of Truth Boundary:** DAG-006 remains the approved graph authority; DAG-003 was not approved, promoted, or used as authority.
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
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
| DEL-15-04-A001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | OTHER | SOW-075 | ACTIVE | EXTRACTED |
| DEL-15-04-A002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-017 | ACTIVE | EXTRACTED |
| DEL-15-04-A003 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | OBJ-018 | ACTIVE | EXTRACTED |
| DAG-002-E0723 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | DECLARED |
| DAG-002-E0724 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | DECLARED |
| DAG-002-E0725 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | DECLARED |
| DAG-002-E0726 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-04 | ACTIVE | DECLARED |
| DAG-002-E0727 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | DECLARED |
| DAG-002-E0728 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | DECLARED |
| DAG-002-E0729 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | DECLARED |
| DAG-002-E0818 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-01-04 | ACTIVE | DECLARED |
| DAG-002-E0819 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-15-01 | ACTIVE | DECLARED |
| DAG-002-E0820 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-15-02 | ACTIVE | DECLARED |
| DAG-002-E0821 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-15-03 | ACTIVE | DECLARED |
| DAG-002-E0822 | EXECUTION | NOT_APPLICABLE | UPSTREAM | PREREQUISITE | DEL-14-01 | ACTIVE | DECLARED |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Approved graph authority: `execution/_DAG/DAG-006/DependencyEdges.csv` and `execution/_DAG/DAG-006/APPROVAL_RECORD.md`.
- Anchor source documents: `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution source documents: existing `Dependencies.csv` DAG-002 mirror, `Procedure.md`, `Specification.md`, `Guidance.md`, and `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md`.
- Preserved 12 DAG-002 mirror rows by `DependencyID`; updated `LastSeen` to 2026-05-11.
- Added 3 explicit ANCHOR rows: one parent anchor to `SOW-075` and two objective traces to `OBJ-017` and `OBJ-018`.
- Normalized legacy DAG-002 execution row fields for v3.1 validation: `AnchorType=DELIVERABLE` became `NOT_APPLICABLE`; custom `DependencyType` values became `PREREQUISITE`; `Origin=CONTEXT/GRAPH_REVIEW` became `DECLARED`; `Explicitness=INFERRED_DIRECT` became `IMPLICIT`; `SatisfactionStatus=UNKNOWN` became `TBD`.
- No downstream consumer row was added for `DEL-08-06` because the approved DAG-006 edge is already represented as an upstream dependency in the consumer deliverable; adding an inverse active row here would be a reconciliation risk.
- [WARNING] ENUM_NORMALIZATION: Custom DAG-002 edge semantics were preserved in `Notes`, but local enum fields were normalized to the current validator vocabulary.
- [WARNING] ID_FORMAT_HELPER_DRIFT: `validate_id_format.sh` rejects project-local IDs such as `DEL-15-04`, `PKG-15`, and `SOW-075` because the helper expects older three-digit package/deliverable and four-digit SOW formats. IDs were validated against the decomposition/register evidence instead.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.

## Run History
- 2026-05-03: Local register synchronized from approved DAG-006 mirror; 12 ACTIVE rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh for `CONSUMER_CONTEXT=RECONCILIATION`; mode `UPDATE`; strictness `CONSERVATIVE`; DAG-002 authority preserved; 15 ACTIVE rows; 0 RETIRED; warnings: `ENUM_NORMALIZATION`, `ID_FORMAT_HELPER_DRIFT`.

## Lifecycle Summary

| Breakdown | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |
| ANCHOR | 3 |
| EXECUTION | 12 |
| OTHER | 3 |
| PREREQUISITE | 12 |
| SATISFIED | 7 |
| TBD | 5 |
| NOT_APPLICABLE | 3 |

## Downstream Handoff Notes
- For RECONCILIATION: treat `DAG-002-*` rows as preserved approved-graph evidence with canonicalized local enum fields, not as new edge proposals.
- For RECONCILIATION: `DEL-15-04-A001`, `DEL-15-04-A002`, and `DEL-15-04-A003` are Tree/trace anchors only and should not be used as execution sequencing edges.
- Later graph reconciliation should compare these refreshed rows against DAG-002 and any preliminary DAG-003 candidate material without silently promoting candidates or inverse downstream edges.
