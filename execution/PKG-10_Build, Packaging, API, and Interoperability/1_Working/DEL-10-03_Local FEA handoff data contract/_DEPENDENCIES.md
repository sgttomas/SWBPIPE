# Dependencies: DEL-10-03 Local FEA handoff data contract

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FOR_RECONCILIATION
- **Source of Truth:** approved DAG-006 mirror plus conservative decomposition refresh evidence
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Candidate disposition rows:** 2 active rows are non-gating candidates for RECONCILIATION.
- **Generated:** 2026-05-03
- **Last Refreshed:** 2026-05-10 by TP-DAG-004 dependency-extract refresh.

## Authority Boundary
- Aggregate DAG authority remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- Candidate-disposition rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Count | Value |
|---:|---|
| Total rows | 17 |
| ACTIVE rows | 17 |
| RETIRED rows | 0 |
| UPSTREAM active rows | 16 |
| DOWNSTREAM active rows | 1 |
| Non-gating candidate-disposition rows | 2 |

| DependencyID | Direction | Type | Target | Status | Satisfaction | Notes |
|---|---|---|---|---|---|---|
| `DAG-002-E0308` | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | SATISFIED | active |
| `DAG-002-E0309` | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | SATISFIED | active |
| `DAG-002-E0310` | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | SATISFIED | active |
| `DAG-002-E0311` | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | SATISFIED | active |
| `DAG-002-E0312` | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED | active |
| `DAG-002-E0313` | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | ACTIVE | SATISFIED | active |
| `DAG-002-E0314` | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | SATISFIED | active |
| `DAG-002-E0561` | UPSTREAM | INTERFACE | DEL-10-01 Public API and plugin boundary | ACTIVE | TBD | active |
| `DAG-002-E0562` | UPSTREAM | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | ACTIVE | TBD | active |
| `DAG-002-E0563` | UPSTREAM | PREREQUISITE | DEL-05-03 Fundamental stress recovery module | ACTIVE | TBD | active |
| `DAG-002-E0564` | UPSTREAM | PREREQUISITE | DEL-01-04 Professional responsibility and product-claims policy | ACTIVE | TBD | active |
| `DAG-002-E0618` | UPSTREAM | INTERFACE | DEL-08-04 Result export format | ACTIVE | TBD | candidate |
| `TP-DAG-004-DEL-10-03-C0001` | DOWNSTREAM | ENABLES | DEL-15-01 Canonical handoff package schema and manifest | ACTIVE | TBD | candidate |
| `DEV-001-STAGE2-DEL-10-03-PKG02-001` | UPSTREAM | SCHEMA_CONTRACT | DEL-02-01 Canonical domain model schema | ACTIVE | SATISFIED | active |
| `DEV-001-STAGE2-DEL-10-03-PKG02-002` | UPSTREAM | UNIT_CONTRACT | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | SATISFIED | active |
| `DEV-001-STAGE2-DEL-10-03-PKG02-003` | UPSTREAM | DOMAIN_MODEL | DEL-02-03 Code-neutral analysis boundary model | ACTIVE | SATISFIED | active |
| `DEV-001-STAGE2-DEL-10-03-PKG02-005` | UPSTREAM | INTERFACE | DEL-02-05 Project persistence and round-trip serialization | ACTIVE | SATISFIED | active |

## TP-DAG-004 Refresh Note

- Added exactly one TP-DAG-004 refresh row: `TP-DAG-004-DEL-10-03-C0001` targeting `DEL-15-01 Canonical handoff package schema and manifest`.
- Rationale: decomposition revision 0.5 adds PKG-15 handoff package scope; DEL-10-03's local FEA handoff data-contract concepts likely enable the canonical handoff schema surface.
- Strictness posture: `CONSERVATIVE`; the new row is `IMPLICIT`, `MEDIUM` confidence, `TBD` satisfaction, and non-gating for `RECONCILIATION` review.
- Existing DAG-synchronized rows were enum-normalized to `dependency-extract` v3.1 write-form values; candidate disposition is preserved in `Notes` because `Status=CANDIDATE` is not a valid v3.1 enum.

## Run Notes

- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- ConsumerContext: `RECONCILIATION`.
- Scope: `DEL-10-03` only.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Source docs read: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`.
- Warning: parent ANCHOR extraction was not expanded because this bounded refresh was requested as exactly one TP-DAG-004 dependency-extract refresh row.
- Warning: `tools/validation/validate_id_format.sh` expects legacy `PKG-000` / `DEL-000-00` identifiers and rejects current decomposition IDs such as `PKG-10` and `DEL-10-03`; schema and enum validation still passed.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE | RETIRED |
|---|---|---|---|---|---:|---:|
| 2026-05-10 23:12 MDT | UPDATE | CONSERVATIVE | execution/_Decomposition/SOFTWARE_DECOMP.md | Existing noncanonical enum values normalized; exactly-one TP-DAG-004 row added; anchor expansion skipped by brief constraint; ID-format helper is legacy/stale for current IDs. | 13 | 0 |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 17 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 11 |
| TBD | 6 |

## Downstream Handoff Notes

- For RECONCILIATION: review the new downstream `DEL-15-01` candidate against the split between legacy PKG-10 local FEA handoff guidance and new PKG-15 canonical handoff package scope.
- Treat candidate-disposition rows as non-gating until reconciled and approved by CHANGE/DAG authority.

## DEV-001 Stage 2 Finding Resolution

- Added package-local active rows `DEV-001-STAGE2-DEL-10-03-PKG02-001`, `DEV-001-STAGE2-DEL-10-03-PKG02-002`, `DEV-001-STAGE2-DEL-10-03-PKG02-003`, and `DEV-001-STAGE2-DEL-10-03-PKG02-005`.
- These rows explicitly name accepted PKG-02 foundations used by the local FEA handoff contract: `DEL-02-01` for source model/result references and selected-entity identity, `DEL-02-02` for explicit units manifests, `DEL-02-03` for advisory/human-review authority separation, and `DEL-02-05` for source snapshot and package hash metadata.
- This resolves the technical dependency-coverage gap identified by `PKG10-DEL1003-PKG02-W001` without changing aggregate DAG authority, lifecycle state, external FEA implementation scope, or human disposition.
