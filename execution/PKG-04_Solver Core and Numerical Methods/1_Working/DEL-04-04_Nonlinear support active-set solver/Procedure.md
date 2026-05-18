# Procedure: DEL-04-04 Nonlinear support active-set solver

## Purpose

Define the documentation setup procedure for DEL-04-04 without implementing nonlinear solver code.

## Prerequisites

- Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_STATUS.md`.
- Apply docs/CONTRACT.md invariants OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-DATA-2, OPS-K-REPORT-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
- Apply architecture basis AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Keep all writes inside the DEL-04-04 folder.

## Steps

1. Confirm the deliverable identity is DEL-04-04 under PKG-04.
2. Confirm the setup scope is documentation only: four documents, semantic files, dependency artifacts, and run records.
3. Capture nonlinear support behavior categories from SOW-012 and the deliverables register: one-way supports, gaps, lift-off, and friction.
4. Record future implementation needs as `TBD` when exact numerical library choices, convergence thresholds, friction defaults, or data contracts are not provided.
5. Preserve mechanics/reporting boundaries: mechanics solved by solver, acceptability handled by rule packs or humans, and no compliance/certification claims.
6. Generate dependency rows only from explicit scope, architecture, invariant, or local-document evidence.
7. Run local schema validation for `Dependencies.csv`.

## Verification

- Four production documents exist and retain Datasheet, Specification, Guidance, and Procedure roles.
- `_SEMANTIC.md` exists and separates semantic lensing from engineering authority.
- `_SEMANTIC_LENSING.md` exists and records warranted enrichment items without rewriting production documents.
- `Dependencies.csv` validates against v3.1 schema.
- `_STATUS.md` is not advanced to `ISSUED`.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_2026-04-30_1015_*.md`
