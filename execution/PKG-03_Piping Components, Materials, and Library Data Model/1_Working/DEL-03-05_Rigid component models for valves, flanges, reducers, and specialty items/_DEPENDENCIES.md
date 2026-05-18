# Dependencies: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Generated Dependency Register
- **Status:** REFRESHED_FOR_RECONCILIATION
- **Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 9 total; 9 ACTIVE; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Last Refresh:** 2026-05-10 via `TP-DAG-004` dependency-extract refresh.
- **Consumer Context:** `RECONCILIATION`

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Refresh Notes
- Conservative UPDATE retained the existing 9 ACTIVE v3.1 rows.
- No dependency IDs, target deliverables, maturity values, satisfaction statuses, confidence values, or evidence anchors were changed.
- `LastSeen` was refreshed to 2026-05-10 for the retained active rows.
