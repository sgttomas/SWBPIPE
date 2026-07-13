# Procedure: DEL-04-06 Solver diagnostics and singularity detection

## Purpose

Define the review and maintenance procedure for the implemented solver diagnostic deliverable.

## Prerequisites

- Sealed context for DEL-04-06 in `_CONTEXT.md`.
- Decomposition and register rows for SOW-053, SOW-035, OBJ-003, OBJ-008, and OBJ-012.
- Applicable architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Contract invariants OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-UNIT-1, OPS-K-MECH-2, OPS-K-DATA-2, OPS-K-REPORT-1, OPS-K-AUTH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
- Current implementation evidence in `core/solver/diagnostics/README.md` and `core/solver/diagnostics/src/lib.rs`.
- Recent validation evidence in `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`.

## Steps

1. Confirm the deliverable identity and write scope from `_CONTEXT.md`.
2. Read the deliverable-local truth set: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
3. Read the implementation evidence in `core/solver/diagnostics/README.md` and `core/solver/diagnostics/src/lib.rs`.
4. Confirm the diagnostic envelope fields: `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`.
5. Confirm provenance/remediation/unit metadata surfaces through `DiagnosticProvenance`, default remediation text, `with_canonical_ref`, and `with_quantity_unit`.
6. Confirm analysis-boundary status mapping through `SolverStatus::analysis_boundary_mapping`; preserve `solver_result_only` authority, human review required, no rule-status emission, and no human-acceptance emission.
7. Confirm frame-kernel mappings for singular systems, invalid restraints, invalid topology/orientation, invalid numeric input, and matrix/vector shape failures.
8. Confirm linear-support and support-application mappings, including support IDs and `dof:*` affected references where implemented.
9. Confirm primitive-load mappings, including axial-effect finding codes and stable `load:<missing-id>` affected references for missing or blank load IDs.
10. Confirm conditioning and nonconvergence diagnostics with caller-supplied thresholds/tolerances and input validation.
11. Confirm sparse-solver and tolerance-policy TBD diagnostics remain warning-level unresolved-policy evidence.
12. Check recent run records for validation evidence; the June 5 Worker B record reports 19 passing diagnostics tests.
13. Preserve deferrals for sparse solver selection, accepted tolerance thresholds, nonlinear support warning finalization, final result-envelope integration, release claims, professional approval, and code-compliance claims.
14. For document-only alignment work, do not edit code, `_STATUS.md`, review finding dispositions, dependency registers, DAG files, or coordination prompts.

## Verification And Release-Hardening Workflow

1. Use existing diagnostics unit tests as implementation evidence when the code has not changed.
2. Rerun `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` when diagnostics source or tests change.
3. Add original/public/permissive singular, restraint, topology, load, conditioning, and nonconvergence fixtures only within an authorized implementation scope.
4. Add nonlinear support active-set regression coverage only after nonlinear diagnostic policy is accepted.
5. Add release gate coverage for accepted sparse solver and tolerance policy only after those decisions are governed.
6. Verify that each fixture emits machine-readable diagnostics and does not claim professional approval or code compliance.

## Verification

- Confirm all four production documents exist.
- Confirm the four production documents align with current implementation evidence.
- Confirm current documents include diagnostic envelope fields, provenance/remediation/unit metadata, analysis-boundary status mapping, frame/support/primitive-load mappings, conditioning/nonconvergence diagnostics, sparse/tolerance TBD diagnostics, and 19-test evidence.
- Confirm obsolete non-implementation wording has been removed from the four production documents except where a preserved deferral is explicitly named.
- Confirm no files outside the assigned write scope were edited.

## Records

- Four production documents in this folder.
- `MEMORY.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
- `core/solver/diagnostics/README.md`
- `core/solver/diagnostics/src/lib.rs`

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The solver-diagnostics and singularity-detection suite is live with current tests and evidence. Setup-era exact counts and checklists are historical rather than current authority; unresolved solver-policy matters remain only where explicitly recorded.
