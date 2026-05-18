# Datasheet: DEL-04-01 3D frame stiffness kernel

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-01 |
| Deliverable name | 3D frame stiffness kernel |
| Package ID | PKG-04 |
| Package name | Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Decomposition basis | docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 |
| Register basis | docs/_Registers/Deliverables.csv row DEL-04-01 |
| Scope items | SOW-005, SOW-035 |
| Objective support | OBJ-003 |
| Context envelope | L |
| Context QA risk | WATCH |

## Attributes

| Attribute | Current value |
|---|---|
| Primary model class | 3D centerline/frame model |
| Nodal degrees of freedom | Six degrees of freedom per node |
| Intended kernel responsibility | Global frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface |
| Anticipated implementation location | core/solver/frame_kernel |
| Anticipated verification artifacts | Unit tests |
| Sparse performance scope | Required design concern; concrete performance targets are TBD |
| Reproducibility scope | Required for practical piping models; exact reproducibility envelope is TBD |
| Solver numerical library | TBD |
| Tolerance policy | TBD; no numerical tolerances are defined by this setup kit |
| Physical formulation details | TBD; future implementation must use lawful, source-grounded mechanics references and avoid protected formulas/data |

## Conditions

- This deliverable is bounded to the global stiffness kernel and sparse solve interface setup for a 3D centerline/frame system.
- This deliverable does not implement straight pipe element mechanics, support families, nonlinear support active-set logic, load cases, stress recovery, diagnostics, rule-pack evaluation, GUI behavior, reports, or packaging except as interface constraints named by the sealed context.
- The kernel must remain unit-aware and dimensionally checked through the project unit contracts.
- Missing solve-required values must be surfaced as explicit findings rather than silently defaulted.
- Solver mechanics and rule-pack acceptability decisions remain separated; this deliverable must not claim code compliance or professional certification.
- Protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data are out of bounds.

## Construction

The setup kit identifies these future implementation surfaces without creating code:

| Surface | Setup expectation |
|---|---|
| Model topology input | Node and element connectivity for a 3D frame/centerline model, details TBD |
| Degree-of-freedom mapping | Stable six-DOF-per-node indexing contract, exact ordering TBD |
| Coordinate transform handling | Transform interface for local-to-global assembly, convention TBD |
| Boundary-condition handling | Interface for applying restraints/imposed conditions supplied by other deliverables, details TBD |
| Sparse assembly | Sparse global matrix assembly interface, storage format TBD |
| Sparse solve interface | Solver adapter boundary for selected numerical library, library TBD |
| Result envelope | Must fit architecture-basis command/query/job result and diagnostic envelope constraints |
| Verification | Deterministic unit tests are required before release use |

## References

- `_CONTEXT.md` - local sealed context for DEL-04-01.
- `_REFERENCES.md` - governing references and register pointers.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 - PKG-04, DEL-04-01, SOW-005, SOW-035, OBJ-003, AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08.
- `docs/_Registers/Deliverables.csv` - row DEL-04-01.
- `docs/_Registers/ScopeLedger.csv` - rows SOW-005 and SOW-035.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-04-01.
- `docs/CONTRACT.md` - applicable invariants named in the task brief.
