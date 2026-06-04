---
run-id: TASK_RUN_2026-06-04_DEL-17-04_source-basis-guardrails
run-status: SUCCESS
deliverable-id: DEL-17-04
package-id: PKG-17
agent: WORKING_ITEMS
tranche: TP-DEL-17-04-SOURCE-BASIS-GUARDRAILS
date: 2026-06-04
lifecycle-changes: not_authorized
coordination-dag-edits: not_authorized
release-claims: not_made
professional-claims: not_made
---
# TASK Run Record - TP-DEL-17-04-SOURCE-BASIS-GUARDRAILS

## Objective

Tighten the DEL-17-04 CAEPIPE MBF export foundation so caller-supplied
`export_profile.source_basis_refs` cannot omit required CAEPIPE source-basis
authority or promote implementation-pattern evidence into target/source
authority.

## Authority Basis

- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`.
- Approved graph authority: `execution/_DAG/DAG-006/`.
- Deliverable-local lifecycle state read from `_STATUS.md`: `IN_PROGRESS`.
- Upstream context: DEL-17-01 source basis and DEL-17-02 export package/profile
  contract as recorded in local four-document kit and memory.

## Files Updated

- `core/handoff/caepipe_mbf/package.py`
- `schemas/caepipe_mbf_export.schema.json`
- `tests/test_caepipe_mbf_export_package.py`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `MEMORY.md`
- This run record

## Changes

- Added required source-basis authority refs for CAEPIPE MBF profiles:
  `DEL-17-01`, `DEL-17-02`, `CAEPIPE-IMPORT-MBF`, and `CAEPIPE-EXPORT-MBF`.
- Added blocking diagnostic `MBF-SOURCE-BASIS-REFS-MISSING` when those refs are
  omitted by a caller-supplied profile.
- Added blocking diagnostic `MBF-SOURCE-BASIS-REFS-UNSAFE` when `DEL-17-03` is
  supplied as CAEPIPE target/source authority.
- Tightened schema validation for export-profile and manifest source-basis refs
  with the same required/forbidden authority set.
- Added focused regression tests for missing source-basis refs and unsafe
  `DEL-17-03` promotion.
- Recorded the guardrail as `DEL-17-04-REQ-011` in the four-document kit.

## Validation

- `python3 -m py_compile core/handoff/caepipe_mbf/package.py tests/test_caepipe_mbf_export_package.py`: PASS.
- `python3 -m json.tool schemas/caepipe_mbf_export.schema.json`: PASS.
- `python3 -m json.tool fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json`: PASS.
- `pytest tests/test_caepipe_mbf_export_package.py`: PASS, 17 passed.
- `pytest tests/test_caepipe_mbf_export_package.py tests/test_native_json_export_package.py tests/test_handoff_export_workflow.py`: PASS, 29 passed.
- `python3 tools/validation/validate_dependencies_schema.py .../Dependencies.csv`: PASS.
- `/Users/ryan/ai-env/projects/chirality/tools/validation/check_four_documents.sh .../DEL-17-04...`: PASS.
- `/Users/ryan/ai-env/projects/chirality/tools/validation/check_min_viable_fileset.sh .../DEL-17-04...`: PASS.
- `python3 /Users/ryan/ai-env/projects/chirality/tools/validation/validate_semantic_matrix.py .../DEL-17-04...`: PASS.
- `python3 /Users/ryan/ai-env/projects/chirality/tools/validation/validate_lens_register.py .../DEL-17-04...`: PASS.
- `git diff --check -- <tranche paths>`: PASS.
- Boundary-language scan: reviewed hits as negative guardrails, schema/property
  text, historical run records, or explicit no-claim language.

## Boundaries Preserved

- No lifecycle status edit.
- No coordination, DAG, candidate-edge, blocker-queue, release, or dependency
  authority edit.
- No fixture regeneration was required.
- No CAEPIPE compatibility claim, release claim, solver-validation claim,
  code-compliance claim, professional-acceptance claim, commercial solver
  behavior, proprietary example, protected standards content, or
  reverse-engineering was introduced.

## Remaining TBDs

- CAEPIPE target version/profile.
- Definitive MBF record-family and required-field subset.
- Direct MBF stable-ID carrier.
- External execution and CSV parsing.
- Runtime/API/GUI integration.
- Lifecycle/acceptance decisions and target-specific compatibility claims.
