# Dependencies: DEL-03-06 Expansion joint component model

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 9 total; 9 ACTIVE; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-10
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION

## Refresh Summary
- Conservative dependency-extract review retained the existing nine active rows.
- No new candidate rows were added for solver behavior, GUI editor behavior, import formats, persistence details, manufacturer catalogs, or hardware taxonomy because the deliverable evidence keeps those items downstream or `TBD`.
- `Dependencies.csv` remains a canonicalized v3.1 local mirror/evidence surface for reconciliation, with row `LastSeen` values refreshed to 2026-05-10.
- Legacy dependency-specific enum labels were normalized to the canonical dependency-extract enum set; original labels are preserved in row notes for reconciliation traceability.

## Run Notes
- [WARNING] FLOATING_NODE: No ACTIVE parent anchor row with `DependencyClass=ANCHOR` and `AnchorType=IMPLEMENTS_NODE` is present in this DAG-synchronized local register.

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
