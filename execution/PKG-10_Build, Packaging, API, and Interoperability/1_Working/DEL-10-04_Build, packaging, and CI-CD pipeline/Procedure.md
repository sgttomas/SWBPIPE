# Procedure: DEL-10-04 Build, packaging, and CI/CD pipeline

## Purpose

Run and verify the deliverable-local setup workflow for DEL-10-04 without creating product CI, packaging, release, manifest, or source-code artifacts.

## Prerequisites

- Root bootstrap, agent index, contract, decomposition, registers, and project-local skill instructions have been read.
- The working folder is `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/`.
- Current write scope is limited to this deliverable folder.
- Protected-data, private-data, no-certification, and human-authority boundaries are active.
- CI provider, release matrix, and thresholds remain `TBD` unless human authority resolves them.

## Steps

1. Execute `four-documents` with `RUN_PASSES=P1_P2` by drafting `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from the sealed context and accessible governing references.
2. Confirm the four documents preserve the setup-only boundary and do not create CI workflows, packaging scripts, manifests, release files, or source code.
3. Execute `semantic-matrix-build` by replacing `_SEMANTIC.md` with a deliverable-local semantic lens and setting or verifying `_STATUS.md` as `SEMANTIC_READY` only after semantic QA passes.
4. Execute `lens-register` by generating `_SEMANTIC_LENSING.md` from `_SEMANTIC.md` and the four production documents.
5. Execute `four-documents` with `RUN_PASSES=P3_ONLY` by checking the lensing register for warranted enrichment items, applying only source-supported setup-document changes, and recording unresolved items as `TBD`.
6. Execute `dependency-extract` by writing `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md`.
7. Run local validation checks for document presence, dependency schema, semantic/lensing structure, status, and write-scope compliance.

## Verification

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist.
- `_SEMANTIC.md` contains canonical matrices A and B plus derived matrices C, F, D, K, G, X, T, and E.
- `_SEMANTIC_LENSING.md` contains complete coverage for matrices A, B, C, F, D, X, and E.
- `Dependencies.csv` validates with `python3 tools/validation/validate_dependencies_schema.py`.
- `_DEPENDENCIES.md` summarizes the same ACTIVE dependency rows present in `Dependencies.csv`.
- `_STATUS.md` current state is `SEMANTIC_READY`.
- No files outside the deliverable folder were edited.
- No CI workflow, packaging script, manifest, release file, source code, or repo-level artifact was created or modified.

## Records

Preserve these setup records:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_*.md`

## Completion Condition

The setup workflow is complete when the document kit, semantic artifacts, dependency register, run records, and status file exist; local checks pass; and unresolved CI/release decisions remain explicitly marked `TBD` for human authority.
