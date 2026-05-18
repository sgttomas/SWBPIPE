# Specification: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Scope

This setup deliverable covers the local evidence specification for future rigid and semi-rigid component model work for valves, flanges, reducers, and specialty items. It is limited to model expectations that can be grounded in the sealed brief, decomposition/register rows, and contract invariants.

Out of scope for this setup pass:

- Editing repository-level schemas, code, fixtures, tests, or package metadata.
- Introducing protected dimensional tables, manufacturer data, catalog weights, COGs, stiffness values, or default component properties.
- Claiming compliance, certification, or professional acceptance.

## Requirements

| ID | Requirement | Source / basis | Verification hook |
|---|---|---|---|
| R01 | Support rigid and semi-rigid component data structures for valves, flanges, reducers, and specialty items. | SOW-009; Deliverables.csv row DEL-03-05 | Check future model discriminates these component families without bundling protected data. |
| R02 | Treat dimensions, weights, and centers of gravity as user-supplied or lawfully imported private/library data. | SOW-009; OPS-K-DATA-1 | Confirm no public defaults or protected tables are introduced. |
| R03 | Preserve unit awareness and dimensional checking for all dimensional, weight, COG, and stiffness fields. | OPS-K-UNIT-1; AB-00-04 | Unit tests and schema validation for quantity dimensions. |
| R04 | Carry provenance fields for reusable component data, including source and redistribution status where public contribution or import is possible. | OPS-K-IP-2; OPS-K-DATA-3; AB-00-07 | Provenance validation rejects or flags missing source/rights metadata. |
| R05 | Represent unknown required values explicitly as TBD, validation findings, or diagnostics, not silent defaults. | OPS-K-DATA-2; OPS-K-AGENT-1; AB-00-06 | Missing required values produce diagnostics or completeness findings. |
| R06 | Preserve the 3D centerline/frame model boundary; shell/solid FEA remains a local-analysis handoff path. | OPS-K-MECH-1 | Model interfaces do not require shell/solid elements for global analysis. |
| R07 | Keep dependencies inward toward domain contracts and through validation/diagnostics boundaries. | AB-00-02; AB-00-07 | Architecture review confirms adapters/plugins cannot bypass validation or public/private data boundaries. |
| R08 | Use deterministic, versioned, schema-governed persistence for component records. | AB-00-04 | Round-trip and canonical JSON/hash checks where JSON payloads are hashed. |
| R09 | Include fixture coverage for reducer, flange, and valve cases using synthetic or user-supplied values only. | Deliverables.csv anticipated artifacts; OPS-K-IP-1 | Fixture review confirms no proprietary/vendor/protected data appears. |
| R10 | Define COG coordinate convention and reference frame before implementation accepts project or library COG values. | SOW-009; OPS-K-UNIT-1 | Human ruling or future sealed brief records convention; until then this remains `TBD`. |
| R11 | Define minimum provenance fields for user-supplied and library-imported component data before reusable public fixtures are accepted. | OPS-K-IP-2; OPS-K-DATA-3; AB-00-07 | Provenance validator checks source, rights/redistribution status where applicable, and review disposition. |

## Standards

No standards text, protected tables, or code-specific component defaults are locally available or authorized for this setup pass. References to PRD sections in the registers identify scope provenance only; they do not authorize copying standard, vendor, or catalog data.

## Verification

Minimum future verification signals:

- Schema validation for required component-family tags, unit dimensions, provenance fields, and explicit missing-value states.
- Protected-content review for fixtures and public examples.
- Unit tests for field validation and deterministic serialization.
- Architecture tests or reviews for no-bypass validation, diagnostics, and public/private data boundaries.
- Fixture tests for reducer, flange, and valve records with synthetic values clearly marked as non-authoritative examples.
- Protected-content fixture review for every public reducer, flange, valve, and specialty-item example.
- Acceptance checks that required dimensions, weights, COGs, stiffness entries, units, provenance, and missing-value diagnostics cannot be bypassed by adapters or public APIs.

## Documentation

Expected documentation artifacts for future implementation include:

- Rigid/semi-rigid component model notes.
- Reducer/flange/valve fixture provenance notes.
- Explicit TBD list for coordinate conventions, stiffness representation, fixture values, and any reusable public-data acceptance criteria.
