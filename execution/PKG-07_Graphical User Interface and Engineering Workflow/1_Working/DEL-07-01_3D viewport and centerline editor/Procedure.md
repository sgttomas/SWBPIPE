# Procedure: DEL-07-01 3D viewport and centerline editor

## Purpose

This procedure records how to produce and verify the setup artifacts for `DEL-07-01`, and it gives future implementation work a bounded execution path for the 3D viewport and centerline editor.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for `DEL-07-01` | Present in `_CONTEXT.md` and the user brief |
| Write scope limited to this deliverable folder | Required for this setup session |
| Governing references available locally | Present through `_REFERENCES.md` and repo docs |
| SCA-001 architecture basis injected | Present in `_CONTEXT.md` |
| Protected standards or proprietary data needed | Not needed for setup; must be excluded |
| GUI source/package/test write authorization | Not present in this setup session |

## Steps

1. Confirm the active deliverable path is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor`.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, governing docs, decomposition, registers, and the four requested skill files.
3. Produce the four-document setup kit:
   - `Datasheet.md` for identity, attributes, conditions, construction, and references;
   - `Specification.md` for scope, requirements, standards, verification, and documentation;
   - `Guidance.md` for rationale, principles, considerations, trade-offs, and examples;
   - `Procedure.md` for prerequisites, steps, verification, and records.
4. Mark unknown or unresolved implementation choices as `TBD`; do not finalize exact GUI dependency versions or component/state libraries.
5. Run `semantic-matrix-build` for this deliverable and write `_SEMANTIC.md` with the semantic lens, audit result, and status update if the audit passes.
6. Run `lens-register` for this deliverable and write `_SEMANTIC_LENSING.md` without modifying production documents.
7. Run `four-documents` with `RUN_PASSES=P3_ONLY` by treating `_SEMANTIC_LENSING.md` as a candidate worklist only; incorporate only source-supported changes.
8. Run `dependency-extract` for this deliverable and write `Dependencies.csv` plus `_DEPENDENCIES.md` with anchor and execution dependencies.
9. Validate local artifacts:
   - four-document kit exists;
   - dependency schema is valid;
   - dependency enum values are canonical;
   - semantic artifacts and run records exist;
   - changed paths remain inside the assigned deliverable folder.
10. Leave the deliverable at `SEMANTIC_READY` only if setup gates pass. Do not move anything to `ISSUED`.

## Future Implementation Procedure

When a later implementation brief authorizes GUI source and tests, the future worker should:

1. Re-read the sealed brief, SCA-001 architecture basis, and these setup artifacts.
2. Keep implementation within the future authorized source/test paths, not this setup folder unless instructed.
3. Define viewport responsibilities for rendering, camera behavior, selection, snapping/drag handles, entity creation, and command dispatch.
4. Keep durable model edits behind application-service commands and validation envelopes.
5. Keep transient viewport/session state separate from model persistence.
6. Use unit-aware domain data for coordinates and editable quantities.
7. Display missing-data and provenance diagnostics without supplying code-specific or proprietary defaults.
8. Add GUI tests for rendering, selection, edit commands, undo/redo eligibility, and diagnostic visibility.
9. Run the accepted GUI validation gates before review.

## Verification

| Check | Command or method | Expected result |
|---|---|---|
| Four-document kit | `tools/validation/check_four_documents.sh <DELIVERABLE_PATH>` | PASS |
| Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <DELIVERABLE_PATH>/Dependencies.csv` | VALID |
| Enum values | `python3 tools/validation/validate_enum.py <ENUM_NAME> <value>` | VALID for emitted enum values |
| Semantic/lensing artifacts | File existence check | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` present |
| Status | Review `_STATUS.md` | `Current State: SEMANTIC_READY` only after all setup gates pass |
| Scope | `git status --short -- <DELIVERABLE_PATH>` | Changed paths are deliverable-local only |

## Records

The setup run should leave:

- four production setup documents;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- refreshed `_DEPENDENCIES.md`;
- `_run_records/*` entries for each setup step;
- `_STATUS.md` history showing initialization and semantic readiness.
