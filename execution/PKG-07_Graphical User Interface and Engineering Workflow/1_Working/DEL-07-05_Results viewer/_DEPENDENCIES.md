# Dependencies: DEL-07-05 Results viewer

## Generated Dependency Register
- **Status:** TP_DAG_004_REFRESH
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 13 ACTIVE; 2 CANDIDATE.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG files remain outside this Type 2 write scope.
- This local register is a reconciliation evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.

## Refresh Notes
- Preserved seven ACTIVE SCA-001 architecture-basis edges from `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- Preserved three ACTIVE direct predecessor edges: stress recovery (`DEL-05-03`), analysis status semantics (`DEL-05-04`), and solver diagnostics (`DEL-04-06`).
- Preserved one non-gating candidate for result export format (`DEL-08-04`).
- Added one non-gating SCA-002/v0.5 reconciliation candidate for analysis run records (`DEL-14-02`), because persisted result review may consume analysis-run records but the deliverable can still consume validated result envelopes directly.

## DEV-001 Stage 2 Addendum

- Added active package-local PKG-02 compatibility dependencies to `DEL-02-02`, `DEL-02-03`, and `DEL-02-05`.
- Evidence: `core/gui/results_viewer/engine.py` now emits `unit_contract`, `analysis_boundary_contract`, `persistence_hash_contract`, canonical `analysis_status` lists, result `unit_metadata`, `hash_boundary`, and `provenance_refs`.
- Tests: `tests/test_results_viewer_contract.py` covers canonical status, dimension visibility, and hash evidence.
- These rows are local technical evidence for PKG-07 review closure only. They do not promote candidate dependencies or edit aggregate DAG files.

## DEV-001 Stage 2 Added Rows

| DependencyID | Target | Status | Satisfaction | Evidence |
|---|---|---|---|---|
| DEV-001-STAGE2-DEL-07-05-PKG02-001 | DEL-02-02 | ACTIVE | SATISFIED | Result units and dimensions |
| DEV-001-STAGE2-DEL-07-05-PKG02-002 | DEL-02-03 | ACTIVE | SATISFIED | Canonical analysis status boundary |
| DEV-001-STAGE2-DEL-07-05-PKG02-003 | DEL-02-05 | ACTIVE | SATISFIED | Hash/provenance handoff |
