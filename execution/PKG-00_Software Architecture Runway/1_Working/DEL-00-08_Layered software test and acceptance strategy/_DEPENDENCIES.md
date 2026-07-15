# Dependencies: DEL-00-08 Layered software test and acceptance strategy

## Coordination Authority
- **Mode:** DAG-007_ACTIVE_GRAPH
- **DAG Status:** APPROVED_ACTIVE_GRAPH_AUTHORITY
- **Authority:** `execution/_DAG/DAG-007/` is approved relationship context. Deliverable-local `_STATUS.md` remains lifecycle authority.
- **Architecture Basis:** AB-00-08 from `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9 is accepted architecture-basis context for downstream sealed briefs without making PKG-00 `ISSUED`.

## Upstream
- No package-local upstream deliverable is required before using this PKG-00 architecture-basis deliverable as context.
- Governing upstream authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, the SCA-001 architecture basis as amended through SCA-006, and approved `DAG-007`.

## Downstream
- Downstream consumers use AB-00-08 only through sealed briefs, accepted context injection, or governed review/dispatch surfaces.
- Missing downstream implementation artifacts are not blockers for this PKG-00 architecture-basis record unless a later human gate changes the closure criteria.

## Current Dependency Resolution
- **Status:** SATISFIED_BY_DAG_007_AUTHORITY
- **Dependencies.csv:** Not deliverable-local for PKG-00; use `execution/_DAG/DAG-007/DependencyEdges.csv` for approved relationship context.
- **Lifecycle:** `_STATUS.md` is authoritative and currently records `IN_PROGRESS`; dependency satisfaction does not change lifecycle or create implementation authority.
- **History:** The former DAG-006/`SEMANTIC_READY` extraction remains recoverable from pre-D-43 Git history and is not current state.

## Consumer Handoff Notes
- Inject the applicable AB-00-08 row and current resolved/TBD architecture baseline, not the full PKG-00 prose.
- Human approval is required for lifecycle changes, release claims, acceptance records, or professional/code-compliance claims.
