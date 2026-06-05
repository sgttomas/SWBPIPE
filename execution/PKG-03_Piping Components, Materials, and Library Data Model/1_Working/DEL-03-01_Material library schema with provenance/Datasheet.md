# Datasheet: DEL-03-01 Material library schema with provenance

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-01 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Type | DATA_MODEL_CHANGE |
| Scope item | SOW-017 |
| Objective | OBJ-004 |
| Anticipated artifacts | `schemas/material.schema.yaml`; material editor fixtures |
| Current evidence status | Schema, fixture, and test implementation evidence present; deliverable remains `IN_PROGRESS` pending review and dependency disposition. |

## Attributes

| Attribute | Required setup meaning | Source |
|---|---|---|
| Material library scope | Private material libraries with temperature-dependent properties, allowables, and provenance fields. | ScopeLedger row SOW-017 |
| Data boundary | Public repository must not bundle protected standards tables, material allowables, copied examples, proprietary data, or paraphrased protected tables. | CONTRACT OPS-K-IP-1, OPS-K-IP-3; SOW-017 notes |
| Provenance fields | Material, component, allowable, and rule-pack values carry provenance fields. | CONTRACT OPS-K-DATA-3 |
| Redistribution status | Public data contributions require source, provenance, license or redistribution status, contributor certification, and review disposition. | CONTRACT OPS-K-IP-2 |
| Unit handling | Imported values, exported values, and calculations using material values must be unit-aware and dimensionally checked. | CONTRACT OPS-K-UNIT-1 |
| Missing required values | Missing solve-required or rule-check-required material values must become explicit findings, not silent defaults. | CONTRACT OPS-K-DATA-2 |
| Private data handling | Private material data must not be transmitted or committed publicly by default. | CONTRACT OPS-K-PRIV-1 |

## Conditions

- `schemas/material.schema.yaml` defines structure for temperature-dependent properties and allowables, but it does not provide actual engineering values.
- `fixtures/material/invented_material_library_valid.json` is a public schema fixture with omitted engineering values, `TBD` source/license disposition, and explicit blocking diagnostics for missing solve-required data.
- Code-specific values and allowables remain user-supplied or lawfully imported private data, not bundled public defaults.
- Architecture basis requires JSON Schema 2020-12, deterministic/versioned/provenance-preserving persistence, unit awareness, diagnostics, adapter validation, and protected-content/provenance gates where relevant.
- Public material source catalog, public fixture value policy, temperature interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions remain `TBD`.

## Construction

The implemented material schema and fixture evidence cover these record groups:

| Record group | Implemented evidence and remaining disposition |
|---|---|
| Library identity | `MaterialLibrary` requires `library_id`, `name`, `library_scope`, `privacy_class`, `provenance`, and `review_status`. |
| Material identity | `MaterialRecord` requires `material_id`, `name`, `material_family`, privacy/redistribution fields, properties, allowables, completeness, provenance, and review status. |
| Property definitions | `MaterialPropertyDefinition` declares property kind, dimension ID, quantity kind, unit requirement, required-for context, public fixture policy, and review status. |
| Property values | `MaterialPropertyValue` carries property kind, value status, required-for context, provenance, review status, and optional unit-bearing quantity value. |
| Allowable slots | `MaterialAllowableSlot` records slot metadata, value status, public repository value policy, required-for context, provenance, and review status without supplying protected/code-specific values. |
| Provenance | `Provenance` requires source name/location/license, contributor, contributor certification, redistribution status, and review status. Fixture source/license disposition remains `TBD`. |
| Redistribution and privacy | Schema enums record public, private, unknown, rejected, suspected-protected, and `TBD` dispositions. Human acceptance of public material data remains separate. |
| Completeness flags | `CompletenessRule` and `MaterialCompletenessFinding` record required property kinds, missing behavior, status, and diagnostic code. |
| Diagnostics | `MaterialDiagnostic` records code, class, severity, source, affected reference, message, remediation, and provenance. The fixture emits `MATERIAL_PROPERTY_MISSING` as a blocking solve diagnostic. |
| Open decisions | `OpenDecision` preserves unresolved public fixture, source catalog, allowable storage, and interpolation policy topics. |

## References

- docs/_Registers/Deliverables.csv row DEL-03-01
- docs/_Registers/ScopeLedger.csv row SOW-017
- docs/_Registers/ContextBudgetQA.csv row DEL-03-01
- execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- docs/CONTRACT.md invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1..4
