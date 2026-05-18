# Datasheet: DEL-03-08 Pipe section property and mass-property calculator

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-08 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Decomposition basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 |
| Scope items | SOW-051, SOW-018 |
| Objectives | OBJ-004, OBJ-012 |
| Context envelope | M |
| Lifecycle state during setup | Draft setup evidence only; not implementation and not ISSUED |

## Attributes

| Attribute | Setup value |
|---|---|
| Calculator purpose | Calculate pipe section and mass-property outputs from user-entered dimensions and material data with unit checks. |
| Permitted data source posture | User-entered or lawfully imported private/project data only. |
| Public bundled data posture | No protected pipe dimensional tables, material allowables, contents defaults, insulation defaults, corrosion allowances, or proprietary component data. |
| Unit posture | All inputs, intermediate quantities, outputs, schema hooks, and tests must be unit-aware and dimensionally checked. |
| Provenance posture | Inputs that originate from libraries or imports must preserve source/provenance and redistribution status. |
| Solver boundary | This deliverable prepares section and mass properties; it does not implement the global solver, code compliance, or rule-pack evaluation. |

## Conditions

The calculator is scoped to user-entered pipe outside diameter, wall, density, contents, insulation, and corrosion basis where applicable. Exact allowed unit catalog, dimensional validation API, data types, precision policy, error codes, and schema field names remain `TBD` until the relevant unit and schema contracts are accepted.

Mass-property tests are anticipated, but test fixtures must use invented synthetic values or clearly licensed/user-provided examples. No protected published pipe tables, material tables, or vendor proprietary data may be encoded as test data.

## Construction

| Construction item | Status |
|---|---|
| Section property calculator artifact | Anticipated; implementation not created in this setup pass. |
| Mass property tests | Anticipated; test data values are `TBD` and must avoid protected/reference-table content. |
| Schema hooks | Anticipated; concrete schema names and field locations are `TBD` pending PKG-02/PKG-03 schema contracts. |
| Diagnostic envelope integration | Required conceptually by architecture basis; exact codes/classes are `TBD`. |
| Private library linkage | Required conceptually for SOW-018; exact library record linkage is `TBD`. |

## References

- `_CONTEXT.md` for deliverable identity, scope, objectives, anticipated artifacts, and architecture basis injection.
- `_REFERENCES.md` for governing local references.
- `docs/_Registers/Deliverables.csv` row DEL-03-08.
- `docs/_Registers/ScopeLedger.csv` rows SOW-051 and SOW-018.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-08.
- `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-SOLVER-1, and OPS-K-AGENT-1..4.

## Open Setup Questions

| Question | Needed from |
|---|---|
| Which unit-system API and dimensional dimensions must this calculator call? | DEL-02-02 / human architecture ruling |
| Which schema record owns calculator inputs and outputs? | DEL-03-02 / schema owner |
| Which diagnostic codes/classes represent invalid geometry, missing provenance, or unit mismatch? | DEL-00-06 / diagnostics owner |
| What synthetic fixture policy is acceptable for mass-property tests? | Validation/QA owner |
