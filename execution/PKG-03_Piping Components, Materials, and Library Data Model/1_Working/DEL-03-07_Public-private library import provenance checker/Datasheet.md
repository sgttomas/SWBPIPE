# Datasheet: DEL-03-07 Public/private library import provenance checker

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-07 |
| Package ID | PKG-03 |
| Package Name | Piping Components, Materials, and Library Data Model |
| Deliverable Type | BACKEND_FEATURE_SLICE |
| Scope Items | SOW-019; SOW-044 |
| Objectives | OBJ-002; OBJ-004 |
| Anticipated Artifacts | library import validator; provenance tests |
| Decomposition Basis | docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 |
| Status | Draft setup evidence |

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Primary function | Validate library import records so source, provenance, license, and redistribution metadata are recorded before component/material data is accepted. |
| Data boundary | Public component data requires documented provenance and redistribution rights; private library data must not become a bundled public default. |
| Input format list | TBD. The decomposition states that no external import formats are selected yet. |
| Rights determination authority | TBD. The checker can record and flag metadata, but legal acceptance requires human/project review. |
| Protected-content response | Suspected protected standards or vendor content is quarantined and escalated rather than transformed into public data. |
| Unit handling | Imported component/material values remain unit-aware where numeric values are present; no unit defaults are introduced here. |

## Conditions

- Public data contributions are acceptable only when provenance and redistribution status are documented.
- Import mechanisms must record source, provenance, and license metadata for component and material data.
- Missing solve-required or rule-check-required values are findings, not silent defaults.
- Public setup evidence must not include protected standards text, protected tables, vendor data, or invented rights/provenance examples.

## Construction

The future implementation is expected to contain:

- a library import validator that checks metadata presence and disposition fields;
- validation paths for public and private library records;
- quarantine or rejection signaling for missing provenance, missing redistribution status, or suspected protected content;
- provenance tests that use invented or placeholder data only;
- result diagnostics that preserve the public/private boundary and do not make legal conclusions.

Implementation locations, concrete schemas, external formats, and dependency versions are TBD.

## References

- `_CONTEXT.md` for deliverable identity, scope envelope, objectives, and architecture-basis injection.
- `_REFERENCES.md` for governing source pointers.
- `docs/_Registers/Deliverables.csv` row DEL-03-07.
- `docs/_Registers/ScopeLedger.csv` rows SOW-019 and SOW-044.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-07.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- `docs/CONTRACT.md` invariants OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, and OPS-K-AGENT-1..4.
