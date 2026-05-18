# Dependencies: DEL-09-01 Mechanics benchmark suite

## TP-DAG-004 Dependency-Extract Refresh

- **Status:** REFRESHED_FOR_RECONCILIATION
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Local register:** `Dependencies.csv`
- **Schema:** `v3.1`
- **Rows:** 11 total; 11 ACTIVE; 0 CANDIDATE; 0 deprecated.
- **Generated:** 2026-05-10

## Authority Boundary

- This local register is a deliverable-local evidence surface for RECONCILIATION, not an aggregate DAG authority.
- Existing DAG-sourced rows were preserved when confirmed by local context or production-document evidence.
- New rows are conservative local extraction proposals for reconciliation intake.
- Satisfaction status remains `UNKNOWN` for non-architecture predecessors because target maturity was not verified from the permitted read set.

## Active Upstream Rows

| DependencyType | Count | Targets |
|---|---:|---|
| `ARCHITECTURE_BASIS` | 4 | DEL-00-01, DEL-00-02, DEL-00-06, DEL-00-08 |
| `VALIDATION_PREDECESSOR` | 4 | DEL-04-01, DEL-04-02, DEL-04-03, DEL-05-01 |
| `GOVERNANCE_PREDECESSOR` | 1 | DEL-01-02 |
| `UNIT_CONTRACT` | 1 | DEL-02-02 |
| `DIAGNOSTICS_CONTRACT` | 1 | DEL-04-06 |

## Refresh Notes

- Confirmed the four `PKG-00` architecture-basis edges from `_CONTEXT.md` applicable basis IDs.
- Confirmed solver/load benchmark prerequisites from `Procedure.md` prerequisites.
- Confirmed protected-content/provenance predecessor from `Specification.md` requirement DEL-09-01-RQ-003.
- Added `DEL-02-02` because `Specification.md` requires unit-aware and dimensionally checked benchmark inputs and expected outputs.
- Added `DEL-04-06` because `Specification.md` requires solver diagnostics/result-envelope status, solver version, assumptions, provenance, and limitations where supported.

## Warnings

- Potential runner, export, CI, and release-gate dependencies are visible in the deliverable text, but were not added as active rows under conservative strictness because the exact fixture schema, runner command, output comparison format, numerical tolerances, and CI gates remain `TBD`.
- `DEL-00-06` and `DEL-04-06` are related but not duplicate rows: `DEL-00-06` is the architecture-basis result-envelope contract, while `DEL-04-06` is the solver diagnostic behavior predecessor.
- No target deliverable files outside the assigned deliverable were read to verify maturity; non-architecture predecessor satisfaction therefore remains `UNKNOWN`.
