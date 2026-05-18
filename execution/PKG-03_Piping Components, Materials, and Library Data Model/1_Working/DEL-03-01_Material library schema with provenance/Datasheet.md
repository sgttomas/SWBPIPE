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
| Current evidence status | Setup evidence only; no product implementation |

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

- The schema may describe slots for temperature-dependent properties and allowables, but this setup pass records no actual engineering values.
- Any future public fixture must use invented non-engineering data or permissively licensed data with documented rights; exact fixture contents are `TBD`.
- Code-specific values and allowables remain user-supplied or lawfully imported private data, not bundled public defaults.
- Architecture basis requires JSON Schema 2020-12, deterministic/versioned/provenance-preserving persistence, unit awareness, diagnostics, adapter validation, and protected-content/provenance gates where relevant.

## Construction

The future material schema should be evaluated for these descriptive record groups:

| Record group | Fields or slots to resolve later |
|---|---|
| Identity | Material record ID, display name, library namespace, version, lifecycle status. Exact naming rules are `TBD`. |
| Property series | Temperature-indexed property records with explicit units and dimensional category. Specific property set is `TBD`. |
| Allowable series | Temperature-indexed allowable records with explicit units, source note, public/private marker, and completeness status. Actual values are not part of this deliverable evidence. |
| Provenance | Source type, source citation pointer, contributor identity or role, review disposition, import path, timestamp/hash basis where approved. Exact fields are `TBD`. |
| Redistribution | Public/private classification, redistribution rights status, license/reference pointer, quarantine status for suspected protected content. |
| Completeness flags | Missing required value, missing provenance, missing units, unresolved redistribution status, and private-data warning flags. |
| Diagnostics | Structured warning/error records compatible with AB-00-06 diagnostics envelope expectations. |

## References

- docs/_Registers/Deliverables.csv row DEL-03-01
- docs/_Registers/ScopeLedger.csv row SOW-017
- docs/_Registers/ContextBudgetQA.csv row DEL-03-01
- docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
- docs/CONTRACT.md invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1..4

