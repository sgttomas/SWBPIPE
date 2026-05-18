# Datasheet: DEL-03-06 Expansion joint component model

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-06 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Type | BACKEND_FEATURE_SLICE |
| Scope Item | SOW-010 |
| Objective | OBJ-004 |
| Setup Status | Draft setup evidence; not product implementation |

## Attributes

The component model is scoped to represent expansion joint data supplied by a manufacturer, user, or lawfully imported private library. Required data categories are:

| Attribute Category | Required Treatment | Source |
|---|---|---|
| Stiffness values | Unit-aware user/manufacturer/private-library supplied fields; no bundled defaults. Specific degrees of freedom and required tensor/vector shape remain TBD for implementation. | docs/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-1 |
| Effective area | Unit-aware supplied value with provenance. Required pressure/thrust usage semantics remain TBD for solver integration. | docs/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
| Movement limits | Supplied limits with explicit units and missing-value diagnostics. Limit taxonomy remains TBD. | docs/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-2 |
| Hardware data | Flags/fields for hardware characteristics supplied by user/manufacturer/private data. Enumerations remain TBD. | docs/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
| Provenance | Source, license/redistribution status, contributor certification/review disposition where public contribution is involved. | docs/CONTRACT.md#OPS-K-IP-2; docs/CONTRACT.md#OPS-K-DATA-3 |

### Field Taxonomy TBDs from semantic lensing

- `TBD`: exact stiffness field shape and degree-of-freedom mapping.
- `TBD`: required vs optional split for expansion joint fields.
- `TBD`: movement-limit classes and dimensional validation categories.
- `TBD`: hardware flag/enumeration taxonomy.

## Conditions

- Public repository artifacts must not include manufacturer proprietary values, protected standards text, protected examples, or copied data tables.
- Missing solve-required or rule-check-required expansion joint values must remain explicit findings, never silent defaults.
- All numeric quantities must be unit-aware and dimensionally checked when implemented.
- Outputs may support review but must not claim certification, authentication, or code compliance.

## Construction

This setup pass defines the expected local model shape only. Product implementation is anticipated to create an expansion joint model and validation tests under future authorized write scope.

Recommended model partitions for later implementation:

| Partition | Content |
|---|---|
| Identity | Component ID, component type, library/source reference, provenance metadata |
| Mechanical inputs | Stiffness fields, effective area, movement limits, hardware flags/data |
| Unit metadata | Quantity units, dimensional checks, conversion policy |
| Validation state | Missing data diagnostics, provenance warnings, assumption warnings |
| Persistence hooks | Schema version, migration behavior, canonical serialization/hash participation where applicable |

## References

- `_CONTEXT.md` for deliverable identity, architecture basis, and package scope.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-010, OBJ-004, PKG-03, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md` for OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-AGENT-1..4.
