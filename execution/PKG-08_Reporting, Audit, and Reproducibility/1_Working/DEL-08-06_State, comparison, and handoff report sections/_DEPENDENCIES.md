# Dependencies: DEL-08-06 State, comparison, and handoff report sections

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FOR_RECONCILIATION
- **Source of Truth:** approved DAG-002 mirror plus conservative decomposition refresh evidence
- **Local Register:** `Dependencies.csv`
- **Rows:** 23 total; 22 ACTIVE; 1 CANDIDATE.
- **Generated:** 2026-05-03
- **Last Refreshed:** 2026-05-11 by TP-DAG-004 dependency-extract refresh.

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## TP-DAG-004 Refresh Note

- Added candidate row `TP-DAG-004-DEL-08-06-C0001` for `DEL-15-02 Target mapping and unsupported-behavior contract`.
- Rationale: `DEL-08-06` handoff report sections preserve target mapping metadata and unsupported-target flags; decomposition revision 0.5 assigns that contract to `DEL-15-02`.
- Strictness posture: `CONSERVATIVE`; the new row is `CANDIDATE`, `UNKNOWN` satisfaction, and non-gating for `RECONCILIATION` review.
