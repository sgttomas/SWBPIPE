# Procedure: DEL-08-01 Calculation report generator

## Purpose

This procedure records the setup and future implementation workflow for the calculation report generator deliverable. It does not implement renderer code.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable scope for DEL-08-01 | Present in `_CONTEXT.md` and user brief. |
| Scope item and objective mapping | Present: SOW-024 and OBJ-007. |
| Applicable invariants | Present in `docs/CONTRACT.md` and user brief. |
| Architecture basis injection | Present in `_CONTEXT.md` with AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08. |
| Renderer/template/test/schema implementation authority | Not present in this setup session; future sealed brief required. |

## Steps

### Setup sequence completed in this session

1. Run `four-documents` with `RUN_PASSES=P1_P2`.
2. Run `semantic-matrix-build`.
3. Run `lens-register`.
4. Run `four-documents` with `RUN_PASSES=P3_ONLY`.
5. Run `dependency-extract`.
6. Validate the resulting setup files and dependency schema.
7. Set `_STATUS.md` to `SEMANTIC_READY` only after the setup gates pass.

### Future implementation sequence

1. Confirm a sealed implementation brief for DEL-08-01 or a split deliverable if scope expands.
2. Read the current report/result/rule-pack schemas and application-service result envelopes.
3. Define report input contract without bypassing units, provenance, diagnostics, or private-data controls.
4. Implement report assembly with safe placeholders and metadata-only rule-pack references.
5. Add report fixtures for mechanics-only, user-rule-check, incomplete-rule-input, warning-heavy, and protected-content-risk cases.
6. Add reproducibility tests for deterministic report output and manifest/hash binding.
7. Add protected-content and prohibited-claim gates for public templates/examples.
8. Route to REVIEW with explicit evidence: fixtures, tests, lint output, dependency register, and limitations.

## Verification

Setup verification for this session:

- `tools/validation/check_four_documents.sh <deliverable>` passes.
- `tools/validation/check_min_viable_fileset.sh <deliverable>` passes.
- `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` passes.
- Dependency enum validation passes for all enum fields in ACTIVE rows.
- `_SEMANTIC.md` final result tables contain no algebra/operator leaks and no final cell over 80 characters.
- `_SEMANTIC_LENSING.md` includes complete lens coverage for matrices A, B, C, F, D, X, and E.
- `rg` checks do not identify prohibited certification or protected-standards claims in DEL-08-01 artifacts.

## Records

Required records for this setup session:

- `_run_records/TASK_RUN_2026-04-30_1200_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1205_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1210_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1215_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1220_dependency-extract.md`

Future implementation records should include source slices used, tests run, report fixture outputs, protected-content scans, reproducibility checks, and open issues.

