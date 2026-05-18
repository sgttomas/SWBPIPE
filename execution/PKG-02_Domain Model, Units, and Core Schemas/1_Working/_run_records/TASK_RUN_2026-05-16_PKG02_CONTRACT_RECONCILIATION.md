---
doc_id: TASK-RUN-2026-05-16-PKG02-CONTRACT-RECONCILIATION
doc_kind: execution.run_record
status: draft
created: 2026-05-16
package_id: PKG-02
stage: DEV-001 finding resolution Stage 1
---

# TASK Run Record - PKG-02 Contract Reconciliation

## Scope

Implemented the PKG-02 contract gate for DEV-001 Stage 1 finding resolution.
PKG-02 is treated as the source of truth after this gate.

Applied human rulings:

- Expanded the PKG-02 semantic dimension vocabulary.
- Added `elbow` and `specialty` as canonical component types.
- Added `slope` as a canonical dimension.
- Added explicit JSON Schema validator tooling as a dev/test dependency.
- Preserved lifecycle state and made no lifecycle promotion.

Out of scope and still `TBD`: conversion constants, final unit catalog,
tolerances, code-specific values, release claims, and professional claims.

## Files Changed

- `schemas/units.schema.yaml`
- `schemas/model.schema.yaml`
- `tests/schema_validation.py`
- `tests/test_units_schema.py`
- `tests/test_model_schema.py`
- `requirements-dev.txt`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `core/units/README.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_CONTRACT_RECONCILIATION.md`

No `_STATUS.md`, DAG, blocker queue, candidate edge, dependency register,
release, professional, or lifecycle promotion record was edited.

## Validation Commands

- `python3 tests/test_units_schema.py`
  - Passed with explicit JSON Schema dependency skip:
    `jsonschema>=4,<5 is required for full PKG-02 JSON Schema validation; install with: python3 -m pip install -r requirements-dev.txt`
- `python3 tests/test_model_schema.py`
  - Passed with explicit JSON Schema dependency skip:
    `jsonschema>=4,<5 is required for full PKG-02 JSON Schema validation; install with: python3 -m pip install -r requirements-dev.txt`
- `pytest tests/test_units_schema.py tests/test_model_schema.py`
  - Passed: `4 passed, 2 skipped`.
  - The two skipped tests are the full Draft 2020-12 JSON Schema validation helper checks blocked by missing `jsonschema`.
- `python3 -m pip install --target /tmp/chirality-jsonschema-20260516 -r requirements-dev.txt`
  - Passed in a temporary directory outside the repository.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 python3 tests/test_units_schema.py`
  - Passed with full Draft 2020-12 schema and fixture validation.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 python3 tests/test_model_schema.py`
  - Passed with full Draft 2020-12 schema and fixture validation.
- `PYTHONPATH=/tmp/chirality-jsonschema-20260516 pytest tests/test_units_schema.py tests/test_model_schema.py`
  - Passed: `6 passed`.
- `python3 -m json.tool schemas/units.schema.yaml >/dev/null`
  - Passed.
- `python3 -m json.tool schemas/model.schema.yaml >/dev/null`
  - Passed.

## Blockers

No Stage 1 blocker remains. The base Python environment still does not include
`jsonschema`, so unqualified pytest keeps the dependency-sensitive checks
skipped. Installing the declared dev dependency in a temporary directory and
running with `PYTHONPATH` executed the full Draft 2020-12 validation path.

## Schema/Fixture Mismatch Corrections

- `DimensionCheckRecord.diagnostics` now accepts either inline
  `UnitDiagnostic` records or `Reference` records, matching the existing
  fixture pattern where a dimension check references a top-level diagnostic.
- Existing fixture-compatible schema corrections in `OpenDecision` and
  diagnostic source fields remain structural only; they do not add conversion
  constants, unit catalogs, tolerances, professional claims, or release claims.
