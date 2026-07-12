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
| Lifecycle state read for reconciliation | IN_PROGRESS (`_STATUS.md` read only; no lifecycle edit) |

## Attributes

This deliverable describes implemented evidence for rigid and semi-rigid component library schema slots, fixture shape, and validation coverage for valves, flanges, reducers, rigid placeholders, and specialty items. The implementation evidence is `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, `tests/test_component_section_schema.py`, and the deliverable-local memory/review records.

Implemented descriptive slots and retained gaps are:

| Slot | Source / basis | Reconciled value |
|---|---|---|
| Component families | SOW-009; `ComponentType`; fixture `contract.component.rigid-semirigid`; test enum checks | `valve`, `flange`, `reducer`, `rigid`, and `specialty` are accepted component types and are covered by the rigid/semi-rigid family contract. |
| Geometry slots | `ComponentFieldKind`; fixture field definitions; strict fixture tests | `rigid_body_length`, `end_a_size`, `end_b_size`, `connection_end_a_reference`, and `connection_end_b_reference` are schema/fixture-recognized slots. Public fixture values remain schema-shape-only. |
| Weight input | SOW-009; fixture rigid record; protected-value policy | `weight` is implemented as a required mechanics-solve field kind. Public catalog/vendor weights remain omitted; value policy remains private or public-permissive only after review. |
| Center of gravity input | SOW-009; fixture rigid record; protected-value policy | `center_of_gravity` is implemented as a required mechanics-solve field kind. Coordinate convention/reference-frame policy remains `TBD`. |
| Semi-rigid behavior | `ComponentQuantityDimension`; fixture field definitions; review finding resolution evidence | Stiffness is split into `linear_stiffness` and `rotational_stiffness`; generic `stiffness` is not used as a component quantity dimension. Exact solver treatment remains `TBD`. |
| Provenance and review metadata | OPS-K-DATA-3; OPS-K-IP-2; schema required fields | Component library metadata, records, fields, diagnostics, and fixture slots carry provenance/review status requirements. Human review disposition remains `TBD` where recorded in `Review_Findings.csv`. |
| Unit handling | OPS-K-UNIT-1; AB-00-04; schema/test evidence | Unit-bearing dimensions include length, force, linear stiffness, and rotational stiffness. Connection/reference slots keep `dimension_id: TBD` where no unit-bearing value is defined. |
| Public-data boundary | OPS-K-IP-1..3; OPS-K-DATA-1; fixture policies | Public artifacts carry schema shape and invented diagnostics only. Protected tables, vendor data, catalog dimensions, actual weights, COGs, and stiffness values remain excluded. |

The base rigid/semi-rigid field taxonomy is implemented for schema validation and strict fixture coverage. Policy-level source catalogs, public fixture-value acceptance, concrete import formats, exact solver consumption, per-family engineering profiles, dependency satisfaction, human disposition, and lifecycle closure remain `TBD`.

## Conditions

- Public artifacts must not include proprietary component/vendor data, protected dimensional tables, copied standards content, or invented weights/COGs.
- User-entered or lawfully imported private data must carry provenance and redistribution status where it can enter a library or reusable fixture.
- Missing solve-required or model-required values must remain explicit findings or validation diagnostics, never silent defaults.
- Outputs are documentation reconciliation evidence and do not claim certification, compliance, or fitness for professional reliance.

## Construction

Repository-level implementation evidence already exists in the component schema, invented component fixture, and component schema tests. This reconciliation run does not edit those repository-level schema, fixture, source, test, DAG, dependency, status, review CSV, or coordination files.

The implemented model evidence preserves the architecture basis stated in the sealed brief: inward dependency direction toward domain contracts, deterministic unit-aware persistence, schema-governed data, diagnostics/result envelopes where validation fails, internal/public API boundary preservation, and layered tests.

## References

- `_CONTEXT.md` for deliverable identity, architecture-basis injection, and scope envelope.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows for PKG-03, DEL-03-05, SOW-009, OBJ-004, and AB-00-01/02/04/06/07/08.
- `docs/_Registers/Deliverables.csv` row DEL-03-05.
- `docs/_Registers/ScopeLedger.csv` row SOW-009.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-05.
- `docs/CONTRACT.md` invariants OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-AGENT-1..4.

## D-41 R5 T2B PDU-013 Evidence State

| Surface | Current evidence | Disposition |
|---|---|---|
| COG mechanics convention | Value slot exists; coordinate convention and reference frame are unset. | Held pending authorized selection; no mechanics value accepted. |
| Component geometry scalar trace | Paired field paths on valid copied quantities. | Bounded transform evidence; runtime result-envelope continuation held. |
