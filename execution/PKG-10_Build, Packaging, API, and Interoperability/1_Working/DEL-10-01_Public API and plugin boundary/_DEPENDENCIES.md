# Dependencies: DEL-10-01 Public API and plugin boundary

## Refresh Summary
- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 CANDIDATE.
- **Last refreshed:** 2026-05-16

## Active Rows

| Count | Dependency type | Target package(s) | Status |
|---:|---|---|---|
| 7 | `ARCHITECTURE_BASIS` | `PKG-00` | ACTIVE |
| 1 | `SERVICE_API` | `PKG-02` | ACTIVE |
| 1 | `SCHEMA_CONTRACT` | `PKG-02` | ACTIVE |
| 1 | `UNIT_CONTRACT` | `PKG-02` | ACTIVE |
| 1 | `DOMAIN_MODEL` | `PKG-02` | ACTIVE |
| 1 | `INTERFACE` | `PKG-02` | ACTIVE |

## Authority Boundary
- This local register is dependency evidence for RECONCILIATION, not an independent graph authority.
- `PKG-00` architecture-basis rows are preserved as SCA-001 injected context evidence; they do not mark `PKG-00` as `ISSUED`.
- The refresh did not read or edit aggregate DAG, coordination, source, status, memory, code, schema, or test artifacts.
- No new dependency row was added under conservative strictness because the assigned deliverable and decomposition did not justify an additional dependency beyond the existing active rows.

## Validation Notes
- Schema version retained as `v3.1`.
- Required columns retained unchanged.
- Canonical enums were checked for dependency class, anchor type, direction, dependency type, target type, explicitness, maturity, satisfaction status, confidence, origin, and row status.
- Warnings: existing rows retain `UNKNOWN` satisfaction for inferred `PKG-02` predecessors because this bounded refresh did not read target deliverable status or maturity evidence.

## DEV-001 Stage 2 Finding Resolution

- Added package-local active row `DEV-001-STAGE2-DEL-10-01-PKG02-005` to explicitly name `DEL-02-05 Project persistence and round-trip serialization`.
- Evidence is `Specification.md` requirement `DEL-10-01-REQ-14`, which requires the accepted canonical JSON/JCS-compatible hash basis for API manifests, jobs, model snapshots, results, and reproducibility records where JSON payload hashes are used, plus manifest hashes for non-JSON/binary assets.
- This resolves the technical dependency-coverage gap identified by `PKG10-DEL1001-PKG02-W001` without changing aggregate DAG authority, lifecycle state, transport/API implementation scope, or human disposition.
