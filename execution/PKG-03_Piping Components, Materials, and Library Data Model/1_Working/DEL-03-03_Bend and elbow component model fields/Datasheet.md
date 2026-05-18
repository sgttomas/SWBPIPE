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
| Decomposition Basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 |
| Anticipated Artifacts | bend component model; validation tests |

## Attributes

This setup deliverable concerns data-model fields for bends and elbows. The scoped model must support:

| Attribute Group | Field Intent | Source |
|---|---|---|
| Component identity | Distinguish bend/elbow records from other component families. | DEL-03-03 context; SOW-007 |
| Bend geometry | Store user-entered bend geometry needed by later solver and rule workflows. Exact field names and units are implementation TBD. | SOW-007 |
| SIF inputs | Store user-entered stress intensification factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
| Flexibility inputs | Store user-entered flexibility factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
| Provenance/source metadata | Record source, provenance, redistribution status where applicable, and evidence references for user-supplied or imported data. | OPS-K-DATA-3; AB-00-04; AB-00-07 |
| Validation state | Represent missing or unresolved required inputs explicitly rather than silently defaulting values. | OPS-K-DATA-2; AB-00-06 |

## Conditions

- The public repository must not contain protected SIF or flexibility tables, protected standards text, copied examples, or code-derived formulas.
- Code-specific values for SIFs, flexibility factors, bend geometry, and related rule-pack inputs are user-supplied or lawfully imported private data.
- Units must be explicit and dimensionally checked for geometry and any numeric factors that participate in calculation or export.
- Diagnostics must identify missing solve-required or rule-check-required bend/elbow values as findings.
- This setup pass is evidence planning only; it does not implement the model or tests.

## Construction

Minimum draft field categories for later implementation:

| Category | Draft Field Concept | Status |
|---|---|---|
| Identity | component family/type, component identifier, optional library identifier | TBD |
| Geometry | bend radius basis, bend angle, end connection/orientation references, dimensional unit metadata | TBD |
| Factors | user SIF values, user flexibility factors, direction/basis metadata where needed | TBD |
| Source metadata | source type, source note, provenance reference, redistribution/license marker, contributor/reviewer note | TBD |
| Validation | missing-value flags, unit-check diagnostics, protected-content gate diagnostics | TBD |
| Persistence | schema version, deterministic serialization participation, hash/provenance hooks | TBD |

## References

- `_CONTEXT.md` for deliverable identity, objective, scope, and architecture-basis injection.
- `docs/_Registers/Deliverables.csv` row `DEL-03-03`.
- `docs/_Registers/ScopeLedger.csv` row `SOW-007`.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 rows for `DEL-03-03`, `SOW-007`, `OBJ-004`, `PKG-03`, and applicable architecture basis IDs.
- `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.
