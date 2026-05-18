# Procedure: DEL-10-05 Headless CLI and structured I/O analysis runner

## Purpose

Define the setup and future execution procedure for a governed headless runner deliverable without implementing the runner in this session.

## Prerequisites

- Sealed context for DEL-10-05 with write scope limited to this deliverable folder.
- Governing references listed in `_REFERENCES.md`.
- Architecture-basis constraints injected in `_CONTEXT.md`.
- No protected standards data, proprietary values, private project data, or certification/compliance claims in the artifacts.
- Future implementation must receive a separate sealed brief before changing CLI/source files, fixtures, package manifests, CI files, or repo-level automation.

## Steps

1. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, decomposition and register rows for DEL-10-05, SOW-054, and SOW-032.
2. Draft the four-document kit from accessible governance, decomposition, and register sources.
3. Keep exact CLI commands, schema fields, public API transport, CI provider, release matrix, and package/container details as `TBD` unless a cited source or human ruling resolves them.
4. Generate `_SEMANTIC.md` from the deliverable-local perspective and production documents.
5. Generate `_SEMANTIC_LENSING.md` from `_SEMANTIC.md` and the four production documents.
6. Apply warranted semantic-lensing items conservatively, with source reread evidence and no new implementation particulars.
7. Generate `Dependencies.csv` and `_DEPENDENCIES.md` from local source documents and the decomposition basis.
8. Validate the four-document kit, dependency schema, semantic coverage, lens coverage, and status state.
9. Record run evidence under `_run_records/`.

## Verification

Setup is acceptable for `SEMANTIC_READY` only when:

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist.
- `_SEMANTIC.md` is populated and has no failed final-cell semantic gate.
- `_SEMANTIC_LENSING.md` includes all required lens coverage rows.
- `Dependencies.csv` validates against v3.1 schema.
- `_DEPENDENCIES.md` counts align with `Dependencies.csv`.
- `_STATUS.md` records `SEMANTIC_READY` only, not `ISSUED`.
- `git status` shows changes only inside this deliverable folder for this run.

## Records

Required records:

- Four production documents.
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_2026-04-30_1105_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1105_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1105_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1105_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1105_dependency-extract.md`

