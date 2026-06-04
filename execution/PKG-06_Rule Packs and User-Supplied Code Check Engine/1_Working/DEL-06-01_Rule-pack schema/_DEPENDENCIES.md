# Dependencies: DEL-06-01 Rule-pack schema

## Refreshed Dependency Register

- **Status:** TP_DAG_004_REFRESHED_CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Current Basis Read:** `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 plus assigned DEL-06-01 folder context/evidence
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 CANDIDATE.
- **Refreshed:** 2026-05-10

## Refresh Result

The prior 12-row DAG-002 dependency surface was retained. Revision 0.5 still supports the DEL-06-01 upstream architecture-basis, schema/unit/domain-model, protected-data, and professional-boundary dependencies. No conservative basis was found in the permitted read scope to add, remove, or downgrade a DEL-06-01 dependency row.

## Authority Boundary

- This local register is a refreshed evidence surface for RECONCILIATION, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
- Satisfaction remains `UNKNOWN` for inferred non-architecture predecessors unless an approved coordination graph or owning lifecycle evidence establishes maturity.
