# Procedure: DEL-07-05 Results viewer

## Purpose

This procedure records how the DEL-07-05 setup artifacts are produced and what a future implementation brief must verify before building a results viewer.

## Prerequisites

- Sealed deliverable context for DEL-07-05.
- Write scope limited to this deliverable folder.
- Governing references in `_REFERENCES.md` are available.
- Applicable invariants from `docs/CONTRACT.md` are preserved.
- Architecture-basis constraints in `_CONTEXT.md` are treated as dispatch constraints, not as issued implementation content.

## Steps

1. Confirm the deliverable identity, package, scope item, objectives, context envelope, and architecture-basis IDs from `_CONTEXT.md`.
2. Read the governing references named in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/DIRECTIVE.md`, and the decomposition/register rows for DEL-07-05.
3. Run `four-documents` with `RUN_PASSES=P1_P2` by drafting `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from accessible sources.
4. Run `semantic-matrix-build` by replacing `_SEMANTIC.md` with a deliverable-local semantic lens and marking semantic audit status.
5. Run `lens-register` by producing `_SEMANTIC_LENSING.md` with complete matrix-cell coverage and any warranted enrichment items.
6. Run `four-documents` with `RUN_PASSES=P3_ONLY` by checking the lensing register against the four-document kit and applying only warranted, source-supported enrichments.
7. Run `dependency-extract` by producing `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md` with conservative anchor and execution edges.
8. Run local validation gates for four-document presence, dependency schema, status enum, and protected-boundary spot checks.
9. Set `_STATUS.md` Current State to `SEMANTIC_READY` only if all setup gates pass.

## Verification

| Check | Command or method | Expected result |
|---|---|---|
| Four documents exist | `tools/validation/check_four_documents.sh <deliverable path>` | PASS |
| Dependency schema valid | `python3 tools/validation/validate_dependencies_schema.py <deliverable path>/Dependencies.csv` | VALID with 29 required columns |
| Lifecycle enum valid | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` | VALID |
| Protected-data boundary | Text scan for protected standards values, code thresholds, copied formulas, or certification claims | No prohibited content found |
| Scope boundary | `git status --short -- <deliverable path>` | Only deliverable-local files changed |

## Records

The setup run must leave these records in the deliverable folder:

- four-document kit;
- semantic matrix and lensing artifacts;
- dependency register and dependency summary;
- run records for all five required setup steps;
- status history showing `OPEN -> INITIALIZED -> SEMANTIC_READY`.

## Future Implementation Procedure Notes

A future implementation brief must not begin from these setup documents alone. It must receive source-code write scope, schema/result-envelope contracts, test requirements, and any human decisions needed for result category layout, unit handling, ratio terminology, equipment-load semantics, and report/export integration.
