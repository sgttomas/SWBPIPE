# Dependencies: DEL-04-04 Nonlinear support active-set solver

## Dependency Register

- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Register schema:** v3.1 with extension columns `EstimateImpactClass` and `ConsumerHint`
- **Local register:** `Dependencies.csv`
- **Rows:** 9 total; 9 ACTIVE; 0 RETIRED
- **Last refreshed:** 2026-05-11 by TP-DAG-004 dependency-extract refresh

## Refresh Summary

This refresh replaces prior DAG-002-synchronized local rows with deliverable-local v3.1 enum-normalized dependency rows. It preserves the five explicit PKG-00 architecture-basis constraints from `_CONTEXT.md`, preserves the conservative solver-core upstream dependencies on DEL-04-01, DEL-04-03, and DEL-04-06, and adds one explicit domain constraint on DEL-02-02 for unit-aware and dimensionally checked nonlinear support quantities.

## Active Upstream Dependencies

| DependencyID | Type | Target | Basis | Satisfaction |
|---|---|---|---|---|
| TP-DAG-004-DEL-04-04-E001 | CONSTRAINT | DEL-00-01 Architecture decision record baseline | AB-00-01 context injection | SATISFIED |
| TP-DAG-004-DEL-04-04-E002 | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | AB-00-02 context injection | SATISFIED |
| TP-DAG-004-DEL-04-04-E003 | INTERFACE | DEL-00-03 Application service command-query-job model | AB-00-03 result-envelope baseline | SATISFIED |
| TP-DAG-004-DEL-04-04-E004 | INTERFACE | DEL-00-06 Diagnostics, warning, and result-envelope contract | convergence and active-set reporting | SATISFIED |
| TP-DAG-004-DEL-04-04-E005 | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | deterministic verification requirement | SATISFIED |
| TP-DAG-004-DEL-04-04-E006 | PREREQUISITE | DEL-04-03 Linear support and restraint models | nonlinear supports extend support/restraint behavior | PENDING |
| TP-DAG-004-DEL-04-04-E007 | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | active-set iteration operates on frame mechanics | PENDING |
| TP-DAG-004-DEL-04-04-E008 | INTERFACE | DEL-04-06 Solver diagnostics and singularity detection | nonconvergence and active-set diagnostics | PENDING |
| TP-DAG-004-DEL-04-04-E009 | CONSTRAINT | DEL-02-02 Unit system and dimensional-analysis core contract | unit-bearing nonlinear support quantities | PENDING |

## Authority Boundary

- This local register is a deliverable-local dependency evidence surface for RECONCILIATION.
- It does not edit aggregate DAG files, coordination files, status files, source documents, schemas, tests, or code.
- `PKG-00` architecture-basis rows remain context constraints only; they do not mark PKG-00 deliverables as `ISSUED`.
- `PENDING` rows identify required upstream surfaces whose satisfaction was not adjudicated during this bounded refresh.
