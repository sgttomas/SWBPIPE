# Procedure: DEL-09-03 Nonlinear support regression suite

## Purpose

Define the operating procedure for producing the future nonlinear support regression suite while preserving the current setup boundary. This procedure is not an implementation runbook for solver code or test files.

## Prerequisites

- Sealed TASK context for DEL-09-03.
- Applicable invariants from `docs/CONTRACT.md`, including OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-SOLVER-2, and OPS-K-AGENT-1..4.
- DEL-09-03 register scope: SOW-026 and OBJ-008.
- Nonlinear solver maturity sufficient to define intended behavior and defensible expected observations.
- Diagnostics/result-envelope contract sufficient to record convergence, active state, gap/lift-off/friction state, iteration count, tolerance basis, and non-convergence warnings.
- Public/original/permissive source basis for each future case.

## Steps

1. Confirm the working folder is the DEL-09-03 deliverable folder and that writes are deliverable-local.
2. Maintain the four-document kit describing scope, requirements, guidance, and future execution procedure.
3. Define future case categories only at the setup level: active-set, gap, friction, lift-off, convergence, and non-convergence behavior.
4. For each future case, require a source/provenance record before implementation. Mark source gaps as `TBD`.
5. Do not use protected standards examples, code tables, proprietary commercial software outputs, or vendor data without documented redistribution rights.
6. Defer final numerical convergence tolerances until nonlinear solver evidence is available and reviewed.
7. When implementation is later authorized, create case fixtures and regression tests only in the appropriate validation/test locations named by the decomposition.
8. Validate future tests through unit/schema checks, deterministic reruns, diagnostics/result-envelope review, and protected-content/provenance review.
9. Report regression outcomes as software-quality evidence only; do not claim code compliance, certification, sealing, approval, or professional reliance.

## Verification

For this setup pass, verify:

- the four-document kit exists;
- semantic matrix and lensing artifacts exist and are internally structured;
- dependency artifacts exist and use the v3.1 schema;
- `_STATUS.md` records `SEMANTIC_READY` only after setup artifacts are generated;
- no files were written outside the assigned deliverable folder;
- no nonlinear solver code, regression tests, benchmark source files, protected examples, or final convergence tolerances were introduced.

For a later implementation pass, verification must include:

- unit-aware case schema validation;
- public/original/permissive provenance review;
- deterministic repeat-run checks;
- active-state and convergence diagnostic checks;
- protected-content scan and human review gate.

Concrete validation command names are `TBD` until the regression runner, CI entry points, and nonlinear solver diagnostics exist. Do not substitute placeholder commands as if they were accepted release gates.

## Records

Maintain these setup records in the deliverable folder:

- four-document kit;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/TASK_RUN_*.md`;
- `_STATUS.md`.
