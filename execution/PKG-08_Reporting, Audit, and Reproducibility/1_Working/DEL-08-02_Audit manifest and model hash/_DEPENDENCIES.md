# Dependencies: DEL-08-02 Audit manifest and model hash

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_LOCAL
- **Source of Truth:** Approved ACTIVE rows remain synchronized from `execution/_DAG/DAG-006/DependencyEdges.csv`; TP-DAG-004 CANDIDATE rows are local refresh evidence for RECONCILIATION.
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 10 ACTIVE; 3 CANDIDATE.
- **Generated:** 2026-05-11

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- The package-local `DEL-02-02` `UNIT_CONTRACT` row is now recorded as an ACTIVE technical prerequisite for reproducible identity. This local metadata update does not edit or supersede aggregate DAG authority.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Active Rows Preserved
- 7 `ARCHITECTURE_BASIS` rows from the SCA-001 context injection: `DEL-00-01`, `DEL-00-02`, `DEL-00-03`, `DEL-00-04`, `DEL-00-06`, `DEL-00-07`, and `DEL-00-08`.
- 1 `UNIT_CONTRACT` row to `DEL-02-02` because model identity depends on explicit unit-system and unit-bearing value conventions rather than implicit defaults.
- 1 `PERSISTENCE_CONTRACT` row to `DEL-02-05` because audit manifests and model hashes operate on persisted project payloads.
- 1 `RULE_PACK_PREDECESSOR` row to `DEL-06-04` because audit manifests record rule-pack checksum/lifecycle metadata.

## Candidate Rows Added For Reconciliation
- `TP-DAG-004-DEL-08-02-C002`: upstream `RUNNER_CONTRACT` candidate to `DEL-10-05`, evidenced by `Procedure.md` prerequisite language for solver/version and deterministic run metadata.
- `TP-DAG-004-DEL-08-02-C003`: upstream `REPORT_INTEGRATION` candidate to `DEL-08-01`, evidenced by `Procedure.md` report renderer integration point. Confidence is `MEDIUM` because final sequencing may invert if reports consume the manifest instead of preceding it.
- `TP-DAG-004-DEL-08-02-C004`: upstream `EXPORT_CONTRACT` candidate to `DEL-08-04`, evidenced by `Procedure.md` result export contract language. Confidence is `MEDIUM` because final sequencing may invert if exports consume the manifest instead of preceding it.

## Stage 2 Resolution Notes
- DEV-001 Stage 2 accepted the PKG-02 unit contract basis for package-local resolution. The local register now treats `DEL-02-02` as an explicit package prerequisite for reproducible manifest identity.
- Aggregate DAG rows and lifecycle state remain unchanged by this package-local metadata update.

## Refresh Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Current decomposition basis checked: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Original TP-DAG-004 refresh did not edit source documents, status files, memory files, code, schemas, tests, DAG files, or coordination files. DEV-001 Stage 2 later edited package-local metadata and the audit-manifest implementation under the allowed PKG-08 scope.
