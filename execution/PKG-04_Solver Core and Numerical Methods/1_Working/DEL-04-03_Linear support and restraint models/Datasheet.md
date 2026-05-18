# Datasheet: DEL-04-03 Linear support and restraint models

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-03 |
| Package ID | PKG-04 |
| Package name | Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-011 |
| Objective | OBJ-003 |
| Context envelope | M |
| Anticipated artifacts | support model; linear restraint tests |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Covered support families | Anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `_CONTEXT.md` Description; `docs/_Registers/ScopeLedger.csv` row SOW-011 |
| Analysis basis | Support behavior is part of the primary 3D centerline/frame global model with six degrees of freedom per node. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1 |
| Support object boundary | Support records target model objects, directions, unit-bearing properties, and result hooks. | `docs/TYPES.md` section 8, object `Support` |
| Linear-only boundary | This deliverable covers linear support/restraint setup. Nonlinear one-way behavior, lift-off, gaps, and friction belong to DEL-04-04 unless explicitly deferred back by human authority. | `docs/_Decomposition/SOFTWARE_DECOMP.md` rows DEL-04-03 and DEL-04-04 |
| Imposed displacement boundary data | Imposed displacement references are node/support boundary data and must remain unit-aware. | `docs/SPEC.md` sections 3 and 5; `docs/TYPES.md` object `Node` |
| Default stiffness values | TBD. No invented support stiffness/defaults are introduced in this setup kit. | `docs/CONTRACT.md` OPS-K-AGENT-1; human hard stop |

## Conditions

- Public artifacts must not include protected standards text, protected tables, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary support/vendor data. Source: `docs/CONTRACT.md` OPS-K-IP-1.
- Missing solve-required support data must become explicit findings or diagnostics, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
- Numerical values affecting calculations must be unit-aware and dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
- Solver output remains a mechanics result; rule-pack acceptability and professional compliance remain outside this deliverable. Source: `docs/CONTRACT.md` OPS-K-MECH-2.
- Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.

## Construction

| Construction slot | Setup direction |
|---|---|
| Support model | Define a code-neutral representation for linear translational and rotational restraints, spring-like linear stiffness, rigid restraint intent, and imposed displacement boundary data. Exact API shape is TBD. |
| DOF mapping | Map support effects to the six node degrees of freedom `Ux`, `Uy`, `Uz`, `Rx`, `Ry`, and `Rz`; coordinate-frame convention is TBD. |
| Boundary-condition assembly | Preserve solver-layer ownership and inward dependency direction toward domain contracts. Exact numerical assembly implementation is out of this setup task. |
| Diagnostics | Missing/invalid support data should produce structured diagnostics/result-envelope entries with code, class, severity, source, affected object, message, remediation, and provenance. |
| Tests | Linear restraint tests should cover deterministic mechanics behavior and unit/dimensional checks using original or permissive fixture data. |

## References

- `_CONTEXT.md` for sealed deliverable identity and applicable architecture basis IDs.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-011, OBJ-003, PKG-04, DEL-04-03, and AB-00-01/02/03/06/08.
- `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` for register rows.
- `docs/CONTRACT.md` for applicable invariants.
- `docs/SPEC.md`, `docs/TYPES.md`, and `docs/INTENT.md` for public architecture and vocabulary slices.
