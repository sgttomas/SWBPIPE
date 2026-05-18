# Procedure: DEL-04-06 Solver diagnostics and singularity detection

## Purpose

Define the setup procedure for preparing the solver diagnostic deliverable for a future implementation task.

## Prerequisites

- Sealed context for DEL-04-06 in `_CONTEXT.md`.
- Decomposition and register rows for SOW-053, SOW-035, OBJ-003, OBJ-008, and OBJ-012.
- Applicable architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Contract invariants OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-UNIT-1, OPS-K-MECH-2, OPS-K-DATA-2, OPS-K-REPORT-1, OPS-K-AUTH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.

## Steps

1. Confirm the deliverable identity and write scope from `_CONTEXT.md`.
2. Read governing decomposition/register rows and local references.
3. Preserve the no-implementation boundary for this setup run.
4. Draft the four-document kit with all unknown thresholds and implementation settings marked `TBD`.
5. Generate the semantic matrix lens as a question-shaping artifact, not engineering authority.
6. Generate the semantic lensing register and record only warranted enrichment items.
7. Extract conservative local dependency rows for scope/objective anchors and governing constraints.
8. Validate `Dependencies.csv` against the v3.1 schema.
9. Leave `_STATUS.md` in a safe non-issued state.

## Future Implementation Fixture Workflow (P3)

1. Select original or public/permissive singular-model fixtures after solver-library and threshold policy decisions are sealed.
2. Include invalid-restraint fixtures that exercise model-definition findings separately from numerical singularity findings.
3. Include conditioning-warning fixtures only after the conditioning metric and threshold are decided.
4. Include nonconvergence fixtures for nonlinear support behavior when nonlinear diagnostics enter scope.
5. Verify that each fixture emits machine-readable diagnostics and does not claim professional approval or code compliance.

## Verification

- Confirm all four production documents exist.
- Confirm `_SEMANTIC.md` contains matrices A, B, C, F, D, K, G, X, T, and E with audit PASS.
- Confirm `_SEMANTIC_LENSING.md` contains coverage for matrices A, B, C, F, D, X, and E.
- Confirm `Dependencies.csv` validates with the local dependency schema tool.
- Confirm no files outside the assigned DEL-04-06 folder were edited.

## Records

- Four production documents in this folder.
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
