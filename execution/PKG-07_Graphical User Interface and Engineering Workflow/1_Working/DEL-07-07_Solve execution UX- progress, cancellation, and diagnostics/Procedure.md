# Procedure: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

## Purpose

This procedure records how to produce and later use the setup basis for the solve-execution UX deliverable. It is not a product implementation procedure.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-07-07 | Present in `_CONTEXT.md` |
| Governing invariants and stop rules | Present in `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `INIT.md` |
| Command/job/cancellation/progress architecture basis | Present in DEL-00-03 specification and architecture-basis injection |
| GUI transient job-progress state basis | Present in DEL-00-05 specification and architecture-basis injection |
| Diagnostics/result-envelope basis | Present in DEL-00-06 specification and architecture-basis injection |
| Protected-data and professional-boundary constraints | Present in `docs/CONTRACT.md`, `docs/TYPES.md`, and `docs/SPEC.md` |

## Steps

1. Confirm the deliverable scope.
   - Verify `Deliverable ID = DEL-07-07`, `Package ID = PKG-07`, and `Scope Coverage = SOW-055` in `_CONTEXT.md`.
   - Verify objectives `OBJ-006` and `OBJ-007`.

2. Draft the four-document setup kit.
   - Populate `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
   - Keep content at setup/specification level.
   - Do not create GUI source code, tests, schemas, job/solver implementation, manifests, repo-level docs, or issued artifacts.

3. Apply architecture-basis constraints.
   - Route solve launch through command/job boundaries.
   - Treat progress and cancellation as job-contract behavior.
   - Preserve diagnostic/result-envelope fields and warning classes.
   - Separate mechanics solved, user-rule checked, and human-review-required states.

4. Preserve data and authority boundaries.
   - Do not introduce protected standards text, code tables, proprietary values, or private project data.
   - Mark unknown implementation details as `TBD`.
   - Avoid certification, approval, sealing, or code-compliance claims.

5. Build semantic setup artifacts.
   - Generate `_SEMANTIC.md` as a lens, not an authority.
   - Generate `_SEMANTIC_LENSING.md` with complete matrix coverage.
   - Use lensing output only as a candidate worklist for consistency review.

6. Refresh dependency artifacts.
   - Create or update `Dependencies.csv` using v3.1 columns.
   - Keep anchors and execution/interface edges evidence-linked.
   - Update `_DEPENDENCIES.md` with counts, run notes, lifecycle summary, and handoff notes.

7. Validate local setup gates.
   - Run dependency schema validation.
   - Confirm required setup files exist.
   - Confirm no writes occurred outside the deliverable folder.
   - Confirm `_STATUS.md` remains at `SEMANTIC_READY` only if all setup gates pass.

## Verification

| Check | Expected result |
|---|---|
| Four documents exist | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist |
| Semantic artifacts exist | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist |
| Dependency artifacts exist | `Dependencies.csv` and `_DEPENDENCIES.md` exist |
| Dependency schema validates | `validate_dependencies_schema.py` reports valid v3.1 schema |
| Protected data absent | No protected standards/code text, copied tables, proprietary values, or certification claims appear |
| Scope respected | Only DEL-07-07 deliverable-local files are written |

## Records

The setup run records are stored in `_run_records/`:

- `TASK_RUN_2026-04-30_1049_four-documents_P1_P2.md`
- `TASK_RUN_2026-04-30_1050_semantic-matrix-build.md`
- `TASK_RUN_2026-04-30_1051_lens-register.md`
- `TASK_RUN_2026-04-30_1052_four-documents_P3_ONLY.md`
- `TASK_RUN_2026-04-30_1053_dependency-extract.md`

## Completion Criteria

The deliverable may be left at `SEMANTIC_READY` when:

- all setup artifacts listed in `Specification.md` exist,
- dependency validation passes,
- semantic and lensing artifacts are present,
- no protected-data or certification boundary issue is found,
- no out-of-scope files were edited,
- open implementation choices remain explicitly marked as `TBD` rather than silently chosen.
