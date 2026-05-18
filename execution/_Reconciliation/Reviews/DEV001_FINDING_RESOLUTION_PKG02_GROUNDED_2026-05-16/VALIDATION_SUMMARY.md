# Validation Summary: DEV-001 PKG-02-Grounded Finding Resolution

## Python / Schema / Fixture Checks

- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 pytest tests/test_model_schema.py tests/test_units_schema.py tests/test_material_schema.py tests/test_component_section_schema.py tests/test_design_knowledge_schema.py tests/test_constraint_schema.py tests/test_rule_pack_schema.py tests/test_model_operation_schema.py tests/test_persistence_schema.py tests/test_project_persistence_service.py` -> 21 passed.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 pytest tests/test_section_properties.py tests/test_constraint_validation.py tests/test_physical_to_analytical_transform.py tests/test_nonlinear_support_regression.py tests/test_external_prover_boundary_metadata.py tests/test_operation_validation_preview.py tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py` -> 48 passed.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 pytest tests/test_analysis_status_schema.py tests/test_report_sections_contract.py tests/test_report_protected_content_linter.py tests/test_report_generator_contract.py tests/test_state_comparison_handoff_report_sections.py tests/test_api_boundary_contract.py tests/test_adapter_framework_contract.py tests/test_local_fea_handoff_contract.py tests/test_headless_runner_contract.py tests/test_handoff_export_workflow.py tests/test_target_mapping_contract.py` -> 29 passed.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 pytest tests/test_gui_editors_contract.py tests/test_missing_data_warning_ux.py tests/test_results_viewer_contract.py tests/test_solve_execution_ux.py tests/test_design_authoring_comparison_workspace.py tests/test_accessibility_usability_baseline.py tests/test_invented_example_models.py tests/test_user_guide_status_wording.py tests/test_library_import_provenance.py` -> 19 passed.
- Direct script checks passed for design knowledge, constraint, analysis-status, report, API, handoff, headless, and PKG-16 operation test files.

`jsonschema` validation used the declared dev dependency from the temporary local install path `/tmp/chirality-jsonschema-20260516`; the repository records the dependency in `requirements-dev.txt` and does not vendor it.

## Rust Checks

- Solver crates: frame kernel, straight pipe, linear supports, nonlinear supports, performance harness, diagnostics -> 82 tests passed.
- Load crates: primitive loads, load-case algebra, user loads, stress recovery -> 63 tests passed.
- Rule/report/validation crates: expression evaluator, completeness checker, rule-pack lifecycle, audit manifest, mechanics benchmarks, stress benchmarks, nonlinear benchmarks -> 71 tests passed.

## Mechanical Checks

- `git diff --check` -> passed.
- Forbidden-file scan for `_STATUS.md`, DAG, blocker queue, candidate, and global register mutations -> no matches.
