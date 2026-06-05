# Specification: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Scope

This deliverable covers the reconciled evidence specification for the implemented rigid and semi-rigid component schema contract for valves, flanges, reducers, rigid placeholders, and specialty items. It is limited to behavior grounded in the sealed brief, decomposition/register rows, contract invariants, `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, `tests/test_component_section_schema.py`, and deliverable-local memory/review evidence.

Out of scope for this reconciliation:

- Editing repository-level schemas, code, fixtures, tests, package metadata, lifecycle/status files, dependency registers, review CSVs, DAG files, coordination files, or DEL-03-01.
- Introducing protected dimensional tables, manufacturer data, catalog weights, COGs, stiffness values, or default component properties.
- Claiming compliance, certification, or professional acceptance.

## Requirements

| ID | Requirement | Source / basis | Verification hook |
|---|---|---|---|
| R01 | Support rigid and semi-rigid component data structures for valves, flanges, reducers, rigid placeholders, and specialty items. | SOW-009; Deliverables.csv row DEL-03-05; `ComponentType`; fixture rigid/semi-rigid contract | Tests verify the component enum against the PKG-02 canonical model enum and verify fixture contract coverage for `valve`, `flange`, `reducer`, `rigid`, and `specialty`. |
| R02 | Treat dimensions, weights, and centers of gravity as user-supplied or lawfully imported private/library data. | SOW-009; OPS-K-DATA-1; fixture public-value policies | Strict fixture records omit actual public component values and mark required values as missing with schema-shape or private/public-permissive policy gates. |
| R03 | Preserve unit awareness and dimensional checking for dimensional, weight, COG, and split stiffness fields. | OPS-K-UNIT-1; AB-00-04; `ComponentQuantityDimension` | Tests verify `linear_stiffness` and `rotational_stiffness` are accepted component dimensions and that retired/generic stiffness dimensions are not present. |
| R04 | Carry provenance fields for reusable component data, including source and redistribution status where public contribution or import is possible. | OPS-K-IP-2; OPS-K-DATA-3; AB-00-07; schema required fields | Schema validation requires provenance/review metadata on library, record, field, and diagnostic surfaces. |
| R05 | Represent unknown required values explicitly as `TBD`, validation findings, or diagnostics, not silent defaults. | OPS-K-DATA-2; OPS-K-AGENT-1; AB-00-06; fixture completeness rules | Strict fixture records carry missing value states, incomplete completeness findings, and blocking diagnostics such as `RIGID_COMPONENT_GEOMETRY_INCOMPLETE`. |
| R06 | Preserve the 3D centerline/frame model boundary; shell/solid FEA remains a local-analysis handoff path. | OPS-K-MECH-1 | Model interfaces do not require shell/solid elements for global analysis. |
| R07 | Keep dependencies inward toward domain contracts and through validation/diagnostics boundaries. | AB-00-02; AB-00-07 | Architecture review confirms adapters/plugins cannot bypass validation or public/private data boundaries. |
| R08 | Use deterministic, versioned, schema-governed persistence for component records. | AB-00-04 | Round-trip and canonical JSON/hash checks where JSON payloads are hashed. |
| R09 | Include strict fixture evidence for the rigid/semi-rigid contract using synthetic schema-shape records only. | Deliverables.csv artifact expectation; OPS-K-IP-1; invented component fixture | Fixture tests confirm the family contract and rigid record slots without proprietary/vendor/protected values. Concrete public fixture values for reducer, flange, valve, and specialty items remain `TBD`. |
| R10 | Define COG coordinate convention and reference frame before project or library COG values are accepted for mechanics use. | SOW-009; OPS-K-UNIT-1 | Human ruling remains `TBD`; implemented schema slots do not settle the convention. |
| R11 | Preserve minimum provenance fields for user-supplied and library-imported component data before reusable public fixtures are accepted. | OPS-K-IP-2; OPS-K-DATA-3; AB-00-07 | Provenance validator checks source, rights/redistribution status where applicable, and review disposition. Source catalog and public fixture-value policy remain `TBD`. |

## Review Finding Reconciliation

| FindingID | Existing local CSV state | Reconciled technical evidence | Human disposition |
|---|---|---|---|
| PKG03-DEL-03-05-PKG02-001 | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `ComponentType` includes `specialty`, and `tests/test_component_section_schema.py` verifies the PKG-03 component enum equals the PKG-02 canonical model enum. | `TBD`; CSV not edited. |
| PKG03-DEL-03-05-PKG02-002 | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | Generic `stiffness` is absent from `ComponentQuantityDimension`; implemented stiffness slots use `linear_stiffness` and `rotational_stiffness` with schema and strict fixture coverage. | `TBD`; CSV not edited. |

## Standards

No standards text, protected tables, or code-specific component defaults are locally available or authorized for this reconciliation. References to PRD sections in the registers identify scope provenance only; they do not authorize copying standard, vendor, or catalog data.

## Verification

Current verification signals:

- Schema validation for required component-family tags, unit dimensions, provenance fields, and explicit missing-value states.
- Protected-content review for fixtures and public examples.
- Unit tests for field validation, component enum alignment, split stiffness dimensions, strict fixture shape, and protected-content guardrails.
- Architecture tests or reviews for no-bypass validation, diagnostics, and public/private data boundaries.
- Fixture tests for the rigid/semi-rigid family contract and a rigid schema-shape record with missing required values.
- Protected-content fixture review for every public reducer, flange, valve, rigid, and specialty-item value example before any such value is accepted.
- Acceptance checks that required dimensions, weights, COGs, split stiffness entries, units, provenance, and missing-value diagnostics cannot be bypassed by adapters or public APIs.

## Documentation

Expected documentation artifacts retained for implementation handoff include:

- Rigid/semi-rigid component model notes.
- Reducer/flange/valve fixture provenance notes.
- Explicit `TBD` list for coordinate conventions, exact stiffness solver treatment, fixture values, source catalogs, import formats, review disposition, dependency satisfaction, lifecycle closure, and reusable public-data acceptance criteria.
