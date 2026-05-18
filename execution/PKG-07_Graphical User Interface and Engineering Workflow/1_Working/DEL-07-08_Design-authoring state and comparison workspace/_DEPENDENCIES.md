# Dependencies: DEL-07-08 Design-authoring state and comparison workspace

## Generated Dependency Register
- **Status:** TP-DAG-004_CONSERVATIVE_REFRESH
- **Source of Truth:** `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 plus preserved approved DAG-002 mirror rows
- **Local Register:** `Dependencies.csv`
- **Rows:** 25 total; 23 ACTIVE; 2 CANDIDATE.
- **Generated:** 2026-05-10
- **Consumer Context:** RECONCILIATION
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## TP-DAG-004 Refresh Notes
- Preserved the 21 existing ACTIVE rows from the approved DAG-002 mirror.
- Added `TP-DAG-004-DEL-07-08-C001` as a CANDIDATE dependency on `DEL-13-02 Constraint entity and provenance model` because SOW-076 includes constraint/warning panels and SOW-068/DEL-13-02 define constraint entity/provenance semantics.
- Added `TP-DAG-004-DEL-07-08-C002` as a CANDIDATE dependency on `DEL-14-02 Analysis run records` because SOW-076 includes state/run browsers and DEL-14-02 defines analysis-run records consumed by run browsing and run comparison.
- No source documents, status files, memory files, code, schemas, tests, aggregate DAG files, or coordination files were edited.

## DEV-001 Stage 2 Addendum

- Added active package-local PKG-02 compatibility dependencies to `DEL-02-03` and `DEL-02-05`.
- Evidence: `core/gui/design_workspace/engine.py` now emits `analysis_boundary_contract`, `persistence_hash_contract`, canonical model-state/run `analysis_status`, per-row `hash_boundary`, and `state_run_browser.persistence_hash_boundary`.
- Tests: `tests/test_design_authoring_comparison_workspace.py` covers state/run hash presence and workspace hash scoping.
- The workspace hash remains a GUI composition/review-state hash, not a project payload hash. Persistence and human-record invalidation obligations remain bound to the PKG-02/PKG-14/PKG-16 record hashes surfaced by the workspace.
- These rows are local technical evidence for PKG-07 review closure only. They do not promote candidate dependencies or edit aggregate DAG files.

## DEV-001 Stage 2 Added Rows

| DependencyID | Target | Status | Satisfaction | Evidence |
|---|---|---|---|---|
| DEV-001-STAGE2-DEL-07-08-PKG02-001 | DEL-02-03 | ACTIVE | SATISFIED | Canonical state/run analysis status |
| DEV-001-STAGE2-DEL-07-08-PKG02-002 | DEL-02-05 | ACTIVE | SATISFIED | State/run hash boundary and workspace hash scope |
