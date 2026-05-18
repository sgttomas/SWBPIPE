# Datasheet: DEL-04-04 Nonlinear support active-set solver

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-04 |
| Name | Nonlinear support active-set solver |
| Package | PKG-04 Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Decomposition basis | docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 |
| Scope item | SOW-012 |
| Objective | OBJ-003 |
| Context envelope | L; WATCH |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Solver role | Iterative activation for nonlinear support behavior in the global 3D centerline/frame solver. | _CONTEXT.md Description; ScopeLedger.csv SOW-012 |
| Covered behavior categories | One-way supports, gaps, lift-off, and friction. | Deliverables.csv DEL-04-04; ScopeLedger.csv SOW-012 |
| Reporting obligation | Convergence reporting, active-set state, and unresolved non-convergence reporting are in scope. | CONTRACT.md OPS-K-SOLVER-2; _CONTEXT.md Description |
| Anticipated artifacts | Nonlinear solver loop; convergence tests. | _CONTEXT.md Anticipated Artifacts |
| Explicit deferrals | Exact solver numerical library, detailed convergence criteria, and implementation-level defaults are TBD. | _CONTEXT.md Architecture Basis Injection; human brief hard stops |

## Conditions

| Condition | Status |
|---|---|
| The primary mechanics model remains a 3D centerline/frame model. | Required by OPS-K-MECH-1. |
| The solver computes mechanics and does not decide professional compliance. | Required by OPS-K-MECH-2 and package exclusions. |
| Missing solve-required values are findings, not silent defaults. | Required by OPS-K-DATA-2. |
| Unit-bearing quantities must be unit-aware and dimensionally checked. | Required by OPS-K-UNIT-1. |
| Nonlinear convergence tolerances, friction defaults, and case-specific values are not invented in this setup kit. | Required by OPS-K-AGENT-1 and human hard stops. |

## Construction

This setup kit defines a documentation boundary for a future nonlinear active-set solver deliverable. It does not implement solver code.

| Construction element | Local setup position |
|---|---|
| Active-set iteration loop | In scope for future implementation; algorithm details TBD. |
| Support activation state model | In scope for future implementation; exact data contract TBD. |
| Gap/lift-off/one-way state transitions | In scope for future implementation; rules must be evidence-backed and tested. |
| Friction behavior | In scope as a named behavior category; numerical model, defaults, and limits are TBD. |
| Convergence reporting | Required output surface; result-envelope integration follows AB-00-03 and diagnostics follow AB-00-06. |
| Convergence tests | Required anticipated artifact; deterministic verification follows OPS-K-SOLVER-1 and AB-00-08. |

## References

- _CONTEXT.md for sealed deliverable identity, scope, objective, and architecture basis.
- docs/_Registers/Deliverables.csv row DEL-04-04.
- docs/_Registers/ScopeLedger.csv row SOW-012.
- docs/_Registers/ContextBudgetQA.csv row DEL-04-04.
- docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 rows for PKG-04, DEL-04-04, SOW-012, OBJ-003, and AB-00-01/02/03/06/08.
- docs/CONTRACT.md invariants OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-DATA-2, OPS-K-REPORT-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
