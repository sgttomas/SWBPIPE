# Specification: DEL-04-04 Nonlinear support active-set solver

## Scope

DEL-04-04 covers the future implementation surface for iterative activation of nonlinear support behavior: one-way supports, gaps, lift-off, friction, and convergence reporting. This setup kit is documentation-only and does not implement solver logic, set numerical tolerances, choose friction defaults, or make certification/compliance claims.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-04-04-REQ-01 | The deliverable shall preserve the 3D centerline/frame mechanics boundary for global piping analysis. | OPS-K-MECH-1; PKG-04 package scope | Architecture and solver-boundary tests confirm no local FEA substitution is introduced. |
| DEL-04-04-REQ-02 | The nonlinear support solver shall address one-way restraints, lift-off, gaps, and friction through a controlled iterative method. | SOW-012; Deliverables.csv DEL-04-04 | Convergence tests cover invented representative cases for each behavior category. |
| DEL-04-04-REQ-03 | The future implementation shall report convergence state, active-set state, and unresolved non-convergence. | OPS-K-SOLVER-2; AB-00-06 | Result-envelope and diagnostic tests assert reported state fields and warning class behavior. |
| DEL-04-04-REQ-04 | Solver changes shall have deterministic verification tests before release use. | OPS-K-SOLVER-1; AB-00-08 | Test inventory includes nonlinear support loop and convergence/non-convergence regression cases. |
| DEL-04-04-REQ-05 | Missing solve-required nonlinear support inputs shall be explicit findings, never silent defaults. | OPS-K-DATA-2; AB-00-06 | Negative tests assert missing data produces diagnostics rather than assumed values. |
| DEL-04-04-REQ-06 | Unit-bearing support, displacement, force, and friction-related quantities shall be unit-aware and dimensionally checked where applicable. | OPS-K-UNIT-1 | Unit tests cover accepted, rejected, and missing unit metadata. |
| DEL-04-04-REQ-07 | The solver shall compute mechanics only; rule-pack acceptability and professional compliance remain outside this deliverable. | OPS-K-MECH-2; package exclusions | Reports and result labels avoid compliance/certification language. |
| DEL-04-04-REQ-08 | Result and diagnostic outputs shall support report disclosure of solver version, warnings, assumptions, limitations, and provenance notes. | OPS-K-REPORT-1; AB-00-06 | Report-facing fixture tests confirm required metadata is present or explicitly TBD. |
| DEL-04-04-REQ-09 | Implementation choices for solver numerical library, convergence tolerances, friction defaults, and exact data contracts remain TBD until later authorized work. | _CONTEXT.md Still TBD; OPS-K-AGENT-1; human hard stops | Review confirms no invented values or defaults appear in setup artifacts. |

## Standards

No protected standards text, formulas, tables, examples, or code-specific values are included in this setup kit. Public/protected data boundaries follow docs/CONTRACT.md. Any later code-specific or project-specific engineering values must be user-supplied or lawfully imported private data where applicable.

## Verification

Future verification shall include deterministic invented-example tests for active-set activation/deactivation, gap closure/opening, lift-off, one-way support behavior, friction state reporting, convergence, and non-convergence diagnostics. Exact pass/fail thresholds, test matrices, and numerical tolerances are TBD and must not be invented by this setup pass.

## Documentation

Required documentation artifacts for this setup are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, and `_run_records/`. Future implementation artifacts remain outside this setup task.
