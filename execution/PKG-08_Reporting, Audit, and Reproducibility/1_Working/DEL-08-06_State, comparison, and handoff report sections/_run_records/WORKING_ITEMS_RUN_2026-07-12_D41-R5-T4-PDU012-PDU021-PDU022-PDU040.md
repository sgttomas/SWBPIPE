# D-41 R5 T4 — persisted run-history to report sections

**Date:** 2026-07-12
**Authority:** DEC-074; bounded T4 evidence under O11
**Rows:** PDU-012, PDU-021, PDU-022, PDU-040
**Lifecycle:** IN_PROGRESS (unchanged)

## Implemented seam

- Added a read-side binding from canonical DEL-02-05
  `project.run_history` model-state/analysis-run records to the existing
  DEL-08-06 backend report-section assembler.
- Preserved warnings, assumptions, limitations, solver context, governed
  analysis-status distinctions, and provenance.
- Recorded project mutation, solver execution, human-approval inference, and
  code-compliance inference as false.

## Evidence

`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider tests/test_state_comparison_handoff_report_sections.py tests/test_project_persistence_service.py -q`

Result: `26 passed`.

Adjacent analysis-run/comparison contract backcheck:

`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider tests/test_state_comparison_handoff_report_sections.py tests/test_project_persistence_service.py tests/test_analysis_run_records.py tests/test_analysis_run_comparison.py tests/test_comparison_contracts.py -q`

Result: `48 passed`.

## Holds and fences

Exact external/non-JSON payload partitioning, rendered-report layout, concrete
solver/rule/adapter producer bindings, broader interop/runtime closure, GLB,
and broader geometry remain outside this seam. No review, dependency, DAG,
register, decomposition, ISSUED, validation, compatibility, release,
professional-reliance, human-approval, or code-compliance outcome is created.
Cargo copy-out was not applicable because no Rust source changed.
