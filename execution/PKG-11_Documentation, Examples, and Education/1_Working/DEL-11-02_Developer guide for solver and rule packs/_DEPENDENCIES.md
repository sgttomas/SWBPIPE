# Dependencies: DEL-11-02 Developer guide for solver and rule packs

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Last semantic refresh:** 2026-06-16
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=11.
- **Parent anchors:** 1 ACTIVE `IMPLEMENTS_NODE`.
- **PKG-00 architecture-basis rows:** 5 reviewed; 5 retained ACTIVE; 0 changed.
- **Candidate rows moved to worklist:** 0.

## Canonical Dependency Types
- `OTHER`: 4
- `PREREQUISITE`: 11

## Run Notes
- Semantic refresh controls: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Anchor doc used: `Datasheet.md`.
- Execution docs reviewed: `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, and current `Dependencies.csv`.
- PKG-00 rows were reviewed against `_CONTEXT.md` applicable basis IDs AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08 and corresponding PKG-00 datasheet/status files. They remain architecture-consistency trackers, not lifecycle issuance or Type 2 dispatch authority.
- No unsupported extracted rows were found. No rows were added, retired, or changed in `Dependencies.csv`.
- Core enum fields conform to the canonical Chirality dependency model. Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Validation: `python3 tools/validation/validate_dependencies_schema.py "execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs/Dependencies.csv"` returned VALID with 29 required columns and 15 data rows.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16 - TASK + dependency-extract semantic refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`, warnings: none, ACTIVE rows: 15.

## Lifecycle Summary
- ACTIVE rows: 15.
- RETIRED rows: 0.
- Satisfaction status: SATISFIED=15.
- Closure note: local dependency register validates; no graph-authority, lifecycle, implementation, or issuance claim is made.
