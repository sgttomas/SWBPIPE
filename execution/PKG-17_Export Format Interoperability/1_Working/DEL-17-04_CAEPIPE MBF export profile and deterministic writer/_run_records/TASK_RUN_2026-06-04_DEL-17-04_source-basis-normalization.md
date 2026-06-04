---
run_id: TASK_RUN_2026-06-04_DEL-17-04_source-basis-normalization
deliverable_id: DEL-17-04
package_id: PKG-17
tranche: TP-DEL17-04-SOURCE-BASIS-NORMALIZATION
run_status: SUCCESS
created: 2026-06-04 00:48 MDT
agent_role: WORKING_ITEMS
scope_path: execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer
---

# TASK RUN: DEL-17-04 source-basis normalization

## Objective

Normalize the CAEPIPE MBF export foundation so emitted `source_basis_refs` match the DEL-17-04 authority surface: DEL-17-01 source facts, DEL-17-02 export contract, and admitted CAEPIPE MBF source IDs. DEL-17-03 remains historical implementation-pattern evidence only and is not emitted as CAEPIPE target/source authority.

## Authority Intake

- `execution/_Coordination/_COORDINATION.md`
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- `execution/_DAG/_LATEST.md`
- `execution/_DAG/DAG-006/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-006/DependencyEdges.csv`
- DEL-17-04 `_CONTEXT.md`, `_STATUS.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and four-document kit
- DEL-17-01 and DEL-17-02 specifications for source-basis and export-contract authority

## Working Tree State

- Initial `git status --short`: clean.

## Changes

- Removed `DEL-17-03` from default CAEPIPE MBF `export_profile.source_basis_refs` in `core/handoff/caepipe_mbf/package.py`.
- Regenerated the invented CAEPIPE MBF fixture so profile and manifest source-basis references and manifest hashes match the corrected builder output.
- Added focused test assertions that default source-basis refs include DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, and `CAEPIPE-EXPORT-MBF`, exclude DEL-17-03, and keep manifest refs aligned with profile refs.

## Validation

- `pytest tests/test_caepipe_mbf_export_package.py` — PASS, 15 passed.
- `pytest tests/test_native_json_export_package.py tests/test_handoff_export_workflow.py` — PASS, 12 passed.
- `python3 -m json.tool schemas/caepipe_mbf_export.schema.json` — PASS.
- `python3 -m json.tool fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json` — PASS.
- `python3 tools/validation/validate_dependencies_schema.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Dependencies.csv"` — PASS.
- `python3 -m py_compile core/handoff/caepipe_mbf/package.py tests/test_caepipe_mbf_export_package.py` — PASS.
- `/Users/ryan/ai-env/projects/chirality/tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer"` — PASS.
- `/Users/ryan/ai-env/projects/chirality/tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer"` — PASS.
- `python3 /Users/ryan/ai-env/projects/chirality/tools/validation/validate_semantic_matrix.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer"` — PASS.
- `python3 /Users/ryan/ai-env/projects/chirality/tools/validation/validate_lens_register.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer"` — PASS.
- `git diff --check -- core/handoff/caepipe_mbf/package.py tests/test_caepipe_mbf_export_package.py fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json` — PASS.

## Boundaries Preserved

- No `_STATUS.md` lifecycle edit.
- No coordination, DAG, candidate-edge, or dependency-register authority edit.
- No CAEPIPE compatibility claim, release claim, solver-validation claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering introduced.

## Remaining TBDs

- CAEPIPE target version/profile.
- Definitive MBF record-family and required-field subset.
- Direct MBF stable-ID carrier.
- External execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
