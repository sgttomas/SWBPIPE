# Datasheet: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-05 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-009 |
| Objective | OBJ-004 |
| Lifecycle state at Pass 1/2 read | OPEN |

## Attributes

This deliverable describes setup evidence for rigid and semi-rigid component model work. The intended component families are valves, flanges, reducers, and specialty items. Per SOW-009, the model must support user-supplied dimensions, weights, and centers of gravity. The sealed brief also adds stiffness behavior as an explicit setup consideration.

Required descriptive slots for later implementation are:

| Slot | Source / basis | Setup value |
|---|---|---|
| Component family | SOW-009; Deliverables.csv row DEL-03-05 | Valve, flange, reducer, specialty item |
| Dimensional inputs | SOW-009 | User-supplied; exact field list TBD |
| Weight input | SOW-009 | User-supplied; no default weights |
| Center of gravity input | SOW-009 | User-supplied; coordinate convention TBD |
| Semi-rigid behavior | _CONTEXT.md description; sealed brief | Stiffness behavior supported; tensor/vector representation TBD |
| Provenance | OPS-K-DATA-3; OPS-K-IP-2 | Required for component data |
| Unit handling | OPS-K-UNIT-1; AB-00-04 | Unit-aware and dimensionally checked |
| Public-data boundary | OPS-K-IP-1..3; OPS-K-DATA-1 | No protected tables, vendor data, or default dimensional data |

Family-specific field taxonomy remains `TBD`. Future implementation must decide which slots are common to all rigid/semi-rigid components and which slots are specific to valves, flanges, reducers, or specialty items.

## Conditions

- Public artifacts must not include proprietary component/vendor data, protected dimensional tables, copied standards content, or invented weights/COGs.
- User-entered or lawfully imported private data must carry provenance and redistribution status where it can enter a library or reusable fixture.
- Missing solve-required or model-required values must remain explicit findings or validation diagnostics, never silent defaults.
- Outputs are draft setup evidence and do not claim certification, compliance, or fitness for professional reliance.

## Construction

Anticipated artifacts are rigid component models and reducer/flange/valve fixtures. In this setup pass, these are discussed only as deliverable-local evidence targets. No repository-level schema, source, fixture, or implementation file is created.

The future model should preserve the architecture basis stated in the sealed brief: inward dependency direction toward domain contracts, deterministic unit-aware persistence, schema-governed data, diagnostics/result envelopes where validation fails, internal/public API boundary preservation, and layered tests.

## References

- `_CONTEXT.md` for deliverable identity, architecture-basis injection, and scope envelope.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 rows for PKG-03, DEL-03-05, SOW-009, OBJ-004, and AB-00-01/02/04/06/07/08.
- `docs/_Registers/Deliverables.csv` row DEL-03-05.
- `docs/_Registers/ScopeLedger.csv` row SOW-009.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-05.
- `docs/CONTRACT.md` invariants OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-AGENT-1..4.
