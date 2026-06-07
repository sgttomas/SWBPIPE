---
run-id: TASK_RUN_DEL-14-05_2026-06-06_comparison-contract-evidence-hardening
timestamp: 2026-06-06T23:45:58Z
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts
task-profile: NONE
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
resolved-task-profile-requirement: NONE
companion-files: []
allowed-tools:
  - unrestricted
write-authorization: ALLOWED_WRITE_TARGETS
runtime-overrides: {}
---

## Requested Tasks

- Act as bounded Type 2 TASK-style worker for OpenPipeStress DEL-14-05.
- Verify mapping/tolerance/export contracts preserve stable IDs, manual/unmatched classifications, unit metadata, no default numeric tolerances, hashes, provenance, and professional-boundary notices.
- Apply allowed non-run-record writes only to the two comparison schemas, comparison contract test file, deliverable MEMORY.md, and this run record.
- Run `python3 -m pytest tests/test_comparison_contracts.py -q` if files are changed.

## Expected Outputs

- Hardened schema and/or test evidence where needed.
- Required run record with status, inputs, tools/tests, outputs, missing items, dependency notes, applied changes, and no lifecycle claim.
- Concise TASK-style run report and changed file list.

## Tools Used

- zsh `sed`
- zsh `rg`
- zsh `date`
- Codex `apply_patch`
- python3 `-m pytest tests/test_comparison_contracts.py -q`
- zsh `git`

## Tool Policy Compliance

N/A

## Write Authorization

ALLOWED_WRITE_TARGETS. Non-run-record writes are limited to:

- `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/schemas/comparison_mapping.schema.json`
- `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/schemas/comparison_tolerance.schema.json`
- `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/tests/test_comparison_contracts.py`
- `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/MEMORY.md`
- this run record

## Outputs Produced

- Hardened `schemas/comparison_mapping.schema.json` for explicit mapping evidence, required CSV preservation columns, export-level hash inclusion, report-section preservation flags, and unmatched-record hash references.
- Hardened `schemas/comparison_tolerance.schema.json` for per-rule unit metadata policy, per-rule hash references, and numeric tolerance status guards.
- Hardened `tests/test_comparison_contracts.py` to assert the new preservation fields and no-default tolerance evidence.
- Updated `MEMORY.md` with a bounded evidence-hardening note and no lifecycle or professional/code-compliance claim.
- Ran `python3 -m pytest tests/test_comparison_contracts.py -q`: 3 passed in 0.11s.

## Missing

- none

## Needs Human Ruling

- none for this bounded evidence-hardening run.
- Governed tolerance values, final mapping workflow, final CSV/report layout, report rendering, comparison engines, and external validation decisions remain outside this run and stay downstream or `TBD`.

## Dependency Notes

- Read the TASK shell instructions, project `AGENTS.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/SPEC.md`, and DEL-14-05 deliverable context files.
- Preserved the accepted authority basis from SOFTWARE_DECOMP revision 0.7 and approved DAG-006 mirror as context only; no dependency rows, aggregate DAG files, review dispositions, lifecycle status files, release records, commits, or coordination files were changed.
- Worktree boundary check showed unrelated existing changes in other workers' files (`tests/test_analysis_run_records.py`, `tests/test_model_state_schema.py`, `tests/test_project_persistence_service.py`, DEL-14-01 run record, and DEL-14-02 run record); this run did not modify or revert them.

## Applied Changes

- `schemas/comparison_mapping.schema.json`: added `MappingEvidence`; required `mapping_evidence` on mapping records; added required CSV column containment for stable IDs, unit metadata, tolerance profile refs, hash refs, provenance refs, and professional-boundary notice; added export-level `hash_refs_included`; added report-section preservation booleans; added unmatched-record `hash_refs`.
- `schemas/comparison_tolerance.schema.json`: added `UnitMetadataPolicy`; required per-rule `unit_metadata_policy` and `hash_refs`; added numeric tolerance guard requiring numeric values to use `externally_governed` or `project_specific_review_required` status.
- `tests/test_comparison_contracts.py`: added structural assertions for mapping evidence, unmatched hashes, CSV/report/export hash and notice preservation, tolerance unit metadata policy, per-rule hashes, and numeric tolerance status guards.
- `MEMORY.md`: appended the 2026-06-06 comparison contract evidence-hardening note without lifecycle, release, professional approval, certification, sealing, authentication, or code-compliance claims.
