# TASK RUN: DEV-001 Finding Resolution Stage 2, PKG-03

- Date: 2026-05-16
- Worker: TASK
- Package scope: PKG-03 only
- HumanDisposition changes: none; Review_Findings.csv rows remain `TBD` / `OPEN`

## Technical Fixes

- Aligned material, component, and section dimension vocabularies with the accepted PKG-02 canonical DimensionId set.
- Replaced retired or ambiguous dimensions:
  - `conductivity` -> `thermal_conductivity`
  - `area_moment` -> `second_moment_area`
  - section modulus output -> `section_modulus`
  - generic component `stiffness` -> `linear_stiffness` and `rotational_stiffness`
- Added PKG-02-style diagnostic `class` and `source` fields to PKG-03 material/component/section schema diagnostics.
- Updated section calculator diagnostics to carry `diagnostic_class`, `source`, `affected_object`, and provenance.
- Required section calculator input provenance and added a blocking `SECTION_PROVENANCE_MISSING` diagnostic for absent provenance.
- Added strict split fixtures:
  - `fixtures/component/invented_section_library_valid.json`
  - `fixtures/component/invented_component_library_valid.json`
- Left the legacy combined component/section fixture in place for existing import-provenance consumers, with references to the strict fixtures used for schema validation evidence.

## Changed Files

- `schemas/material.schema.yaml`
- `schemas/component.schema.yaml`
- `schemas/section.schema.yaml`
- `fixtures/material/invented_material_library_valid.json`
- `fixtures/component/invented_section_component_library_valid.json`
- `fixtures/component/invented_section_library_valid.json`
- `fixtures/component/invented_component_library_valid.json`
- `core/section_properties/calculator.py`
- `tests/test_material_schema.py`
- `tests/test_component_section_schema.py`
- `tests/test_section_properties.py`
- PKG-03 deliverable-local `Review_Findings.csv` files for DEL-03-01 through DEL-03-08

## Validation

- `pytest tests/test_material_schema.py tests/test_component_section_schema.py tests/test_section_properties.py`: PASS, 12 passed.
- `pytest tests/test_model_schema.py tests/test_units_schema.py tests/test_material_schema.py tests/test_component_section_schema.py tests/test_section_properties.py`: PASS, 18 passed.
- `pytest tests/test_library_import_provenance.py`: PASS, 6 passed.
- JSON syntax checks for modified schemas and fixtures: PASS.

## Remaining Open Items

- DEL-03-07 `ImportFinding` diagnostic envelope warning remains open because `core/library_import/provenance_checker.py` was outside this worker's allowed write scope.
- Dependency maturity rows were not lifecycle-promoted; local dependency warnings remain open for human/reconciliation disposition.
- DEL-03-06 movement-limit classes and hardware taxonomy remain TBD pending a later sealed task or human-approved taxonomy decision.
