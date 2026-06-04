# Dependencies: DEL-02-01 Canonical domain model schema

## Generated Dependency Register
- **Status:** REFRESHED_BY_TP_DAG_004
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv` remains the prior aggregate mirror source for preserved DAG rows.
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain the sequencing and blocker-computation authority within their approval boundary.
- This local register is a refreshed deliverable-local evidence surface for RECONCILIATION, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as declared context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Class | Status | Count |
|---|---:|---:|
| ANCHOR | ACTIVE | 6 |
| EXECUTION | ACTIVE | 7 |

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| DEL-02-01-DEP-A001 | ANCHOR | UPSTREAM | OTHER | PKG-02 Domain Model, Units, and Core Schemas | ACTIVE | EXTRACTED |
| DEL-02-01-DEP-A002 | ANCHOR | UPSTREAM | OTHER | SOW-041 Machine-readable project, model, material, component, load, result, and report schemas | ACTIVE | EXTRACTED |
| DEL-02-01-DEP-A003 | ANCHOR | UPSTREAM | OTHER | SOW-065 Schema-backed physical model source of truth | ACTIVE | EXTRACTED |
| DEL-02-01-DEP-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-001 Open, auditable piping stress analysis platform | ACTIVE | EXTRACTED |
| DEL-02-01-DEP-A005 | ANCHOR | UPSTREAM | OTHER | OBJ-012 Unit-safe deterministic reproducible model data flow | ACTIVE | EXTRACTED |
| DEL-02-01-DEP-A006 | ANCHOR | UPSTREAM | OTHER | OBJ-014 Schema-backed piping design model with physical design context | ACTIVE | EXTRACTED |
| DAG-002-E0017 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | DECLARED |
| DAG-002-E0018 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | DECLARED |
| DAG-002-E0019 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | DECLARED |
| DAG-002-E0020 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | DECLARED |
| DAG-002-E0021 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | DECLARED |
| DAG-002-E0022 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | ACTIVE | DECLARED |
| DAG-002-E0023 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | DECLARED |

## Run Notes
- **Mode:** UPDATE.
- **Strictness:** CONSERVATIVE.
- **Consumer context:** RECONCILIATION.
- **Scope:** DEL-02-01 only.
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- **Decomposition status:** located and used to validate PKG-02, SOW-041, SOW-065, OBJ-001, OBJ-012, OBJ-014, DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-06, DEL-00-07, and DEL-00-08 labels.
- **Source docs:** AUTO; scanned `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, `Dependencies.csv`, and prior `_DEPENDENCIES.md`.
- **Anchor doc:** `Datasheet.md` plus `_CONTEXT.md` for explicit Scope Coverage and Objective Support fields.
- **Execution doc order:** `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`.
- **Normalization:** preserved prior DAG-002 row IDs and evidence while converting legacy/non-v3.1 enum values to canonical v3.1 values: `AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE`, and `Origin=DECLARED`.
- **Warnings:** none.
- **Conflicts:** `Guidance.md` records unresolved objective/revision conflicts for human ruling; this refresh did not resolve or modify those source conflicts.
- **Failed-input notes:** none.

## Run History
- 2026-05-10 21:48 - MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition located; warnings=none; ACTIVE counts: ANCHOR=6, EXECUTION=7.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 13 |

| Origin | Count |
|---|---:|
| DECLARED | 7 |
| EXTRACTED | 6 |

## Downstream Handoff Notes
- For RECONCILIATION: this register now separates Tree anchors (`ANCHOR`) from execution edges (`EXECUTION`) and uses canonical v3.1 enum values.
- The preserved DAG-002 architecture-basis edges remain local mirror evidence, not aggregate DAG update authority.
- Source conflict notes in `Guidance.md` remain open human-ruling items and should not be silently reconciled by dependency aggregation.
