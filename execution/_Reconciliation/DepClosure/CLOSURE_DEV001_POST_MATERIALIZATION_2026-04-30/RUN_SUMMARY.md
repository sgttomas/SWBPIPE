---
doc_id: DEP-CLOSURE-DEV001-POST-MATERIALIZATION
doc_kind: audit.run_summary
status: complete
created: 2026-04-30
run_label: DEV001_POST_MATERIALIZATION
scope: non-PKG-00 deliverable-local dependency mirrors
---

# Dependency Closure Run Summary - DEV001_POST_MATERIALIZATION

## Purpose

Verify the deliverable-local `Dependencies.csv` mirrors after materializing them from `execution/_DAG/DAG-001/DependencyEdges.csv`.

## Inputs

- Execution root: `execution/`
- Aggregate authority: `execution/_DAG/DAG-001/DependencyEdges.csv`
- Materialization summary: `execution/_DAG/DAG-001/evidence/dev001_local_materialization_summary.json`
- Audit tool: `tools/coordination/analyze_dep_closure.py`

## Results

- Local `Dependencies.csv` files found: 65.
- Local dependency rows loaded: 624.
- Schema-valid local registers: 65.
- Schema-invalid local registers: 0.
- Evidence populated: 624 / 624.
- Active execution graph SCCs: 0.
- Bidirectional active pairs: 0.
- ID normalizations: 0.

## Notes

- `PKG-00` remains excluded from local dependency-register requirements.
- Architecture-basis rows targeting `PKG-00` are preserved in non-`PKG-00` local mirrors as injected context evidence.
- `IMPLEMENTS_NODE` anchor rows are intentionally absent from the materialized mirrors because DAG-001 is the aggregate execution-dependency authority for DEV-001 control-plane sequencing.
- Candidate rows remain `CANDIDATE` and non-gating.
