---
run-id: TASK_RUN_2026-06-04_DEL-17-04_profile-basis-guardrails
run-status: SUCCESS
deliverable-id: DEL-17-04
package-id: PKG-17
agent: TASK
parent-agent: WORKING_ITEMS
tranche: TP-DEL17-04-PROFILE-BASIS-GUARDRAILS
date: 2026-06-04
lifecycle-changes: not_authorized
aggregate-dag-edits: not_authorized
review-disposition-edits: not_authorized
---
# TASK Run Record - DEL-17-04 Profile-Basis Guardrails

## Objective

Harden the bounded CAEPIPE MBF export foundation so unresolved target-version,
record-subset, and direct stable-ID uncertainty cannot be silently weakened.

## Pre-Run State

- Local lifecycle state read from `_STATUS.md`: `IN_PROGRESS`.
- Initial `git status --short`: clean.
- Current authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md`
  revision `0.7` and approved `execution/_DAG/DAG-006/`.

## Files Updated

- `core/handoff/caepipe_mbf/package.py`
- `schemas/caepipe_mbf_export.schema.json`
- `tests/test_caepipe_mbf_export_package.py`
- DEL-17-04 `Datasheet.md`, `Specification.md`, `Guidance.md`,
  `Procedure.md`, `MEMORY.md`, and this run record.

## Results

- Added blocking diagnostics for unsafe profile-basis values:
  `MBF-TARGET-VERSION-BASIS-UNSAFE`,
  `MBF-RECORD-SUBSET-BASIS-UNSAFE`, and
  `MBF-CARRIED-TBD-REFS-MISSING`.
- Tightened schema profile evidence so `target_version_basis` and
  `record_subset_basis` are non-empty and `carried_tbd_refs` must include
  `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`.
- Added focused tests for target-version, record-subset, and carried-TBD
  guardrails.
- Verified the invented CAEPIPE MBF fixture still matches deterministic builder
  output; no fixture checksum churn was introduced.

## Validation

- `python3 -m py_compile core/handoff/caepipe_mbf/*.py tests/test_caepipe_mbf_export_package.py`: PASS.
- `pytest tests/test_caepipe_mbf_export_package.py`: PASS, 15 passed.
- `pytest tests/test_native_json_export_package.py tests/test_handoff_export_workflow.py`: PASS, 12 passed.
- `python3 -m json.tool schemas/caepipe_mbf_export.schema.json` and fixture parse checks: PASS.
- `python3 tools/validation/validate_dependencies_schema.py .../Dependencies.csv`: PASS.
- Parent Chirality validators for four-document kit, minimum viable fileset,
  semantic matrix, and lens register: PASS.
- `git diff --check`: PASS.

## Boundaries Preserved

- No `_STATUS.md` lifecycle edit.
- No coordination, DAG, candidate-edge, aggregate, or review-disposition edit.
- No release, CAEPIPE compatibility, solver-validation, code-compliance,
  professional-acceptance, certification, sealing, authentication, or reliance
  claim.
- No commercial solver behavior, proprietary example, protected standards
  content, or reverse-engineering was introduced.

## Remaining TBDs

- CAEPIPE target version/profile.
- Definitive MBF record-family and required-field subset.
- Direct MBF stable-ID carrier.
- External execution, CSV parsing, runtime/API/GUI integration, lifecycle and
  acceptance decisions, and any target-specific compatibility claim.
