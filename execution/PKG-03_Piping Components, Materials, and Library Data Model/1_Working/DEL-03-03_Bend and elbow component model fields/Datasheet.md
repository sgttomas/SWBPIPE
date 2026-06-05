# Datasheet: DEL-03-03 Bend and elbow component model fields

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-03 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Type | BACKEND_FEATURE_SLICE |
| Scope Item | SOW-007 |
| Objective | OBJ-004 |
| Decomposition Basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
| Evidence Artifacts | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py` |

## Attributes

This deliverable concerns data-model fields for bends and elbows. The current repository evidence supports:

| Attribute Group | Field Intent | Source |
|---|---|---|
| Component identity | Distinguish bend/elbow records from other component families. | DEL-03-03 context; SOW-007 |
| Bend geometry | Store user-entered centerline bend geometry through schema field kinds including `bend_centerline_radius`, `bend_included_angle`, and `bend_plane_orientation`. | SOW-007 |
| SIF inputs | Store user-entered stress intensification factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
| Flexibility inputs | Store user-entered flexibility factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
| Provenance/source metadata | Record source, provenance, redistribution status where applicable, and evidence references for user-supplied or imported data. | OPS-K-DATA-3; AB-00-04; AB-00-07 |
| Validation state | Represent missing or unresolved required inputs explicitly rather than silently defaulting values. | OPS-K-DATA-2; AB-00-06 |

## Conditions

- The public repository must not contain protected SIF or flexibility tables, protected standards text, copied examples, or code-derived formulas.
- Code-specific values for SIFs, flexibility factors, bend geometry, and related rule-pack inputs are user-supplied or lawfully imported private data.
- Units must be explicit and dimensionally checked for geometry and any numeric factors that participate in calculation or export.
- Diagnostics must identify missing solve-required or rule-check-required bend/elbow values as findings.
- The active evidence is schema/fixture/test evidence only; it does not close dependency satisfaction or human review dispositions.

## Construction

Current evidence categories:

| Category | Draft Field Concept | Status |
|---|---|---|
| Identity | `bend` and `elbow` are explicit `ComponentType` values, aligned with `schemas/model.schema.yaml`. | Implemented evidence |
| Geometry | Bend geometry slots include centerline radius, included angle, tangent/orientation fields, and geometry source references. | Implemented evidence |
| Factors | User SIF and flexibility factor slots are present as user/private inputs with no public code-specific values. | Implemented evidence |
| Source metadata | Component fields require provenance, redistribution status, contributor certification, and review status. | Implemented evidence |
| Validation | Missing bend geometry/rule inputs are represented through `BEND_GEOMETRY_INCOMPLETE` and `BEND_RULE_INPUT_MISSING`. | Implemented evidence |
| Persistence | Strict component fixture validates as a standalone `component.schema.yaml` instance; combined fixture points to the strict component fixture. | Implemented evidence |

## References

- `_CONTEXT.md` for deliverable identity, objective, scope, and architecture-basis injection.
- `docs/_Registers/Deliverables.csv` row `DEL-03-03`.
- `docs/_Registers/ScopeLedger.csv` row `SOW-007`.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows for `DEL-03-03`, `SOW-007`, `OBJ-004`, `PKG-03`, and applicable architecture basis IDs.
- `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.
