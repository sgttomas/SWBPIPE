# Specification: DEL-03-01 Material library schema with provenance

## Scope

This deliverable defines setup evidence for a future material-library schema covering private material records, temperature-dependent properties, allowables, provenance fields, redistribution status, and completeness flags.

This setup pass does not implement `schemas/material.schema.yaml`, create material editor fixtures, define engineering allowable values, reproduce standards content, claim code compliance, or certify engineering suitability.

## Requirements

| ID | Requirement | Evidence basis | Verification approach |
|---|---|---|---|
| REQ-03-01-001 | The future schema shall support material records with temperature-dependent property slots and explicit units. | SOW-017; OPS-K-UNIT-1 | Schema review and unit-aware fixture validation. |
| REQ-03-01-002 | The future schema shall support allowable-value slots without bundling protected or code-specific allowable tables in public artifacts. | SOW-017 note; OPS-K-IP-1; OPS-K-DATA-1 | Protected-content review and fixture review. |
| REQ-03-01-003 | The future schema shall require provenance metadata for material properties and allowables. | OPS-K-DATA-3; OPS-K-IP-2 | Schema required/conditional-field validation. |
| REQ-03-01-004 | The future schema shall record public/private classification and redistribution status for material library data. | OPS-K-IP-2; OPS-K-PRIV-1; OPS-K-GOV-4 | Import/review fixture validation. |
| REQ-03-01-005 | The future schema shall express missing solve-required or rule-check-required material values as explicit diagnostics or completeness findings, not defaults. | OPS-K-DATA-2; AB-00-06 | Negative fixture validation. |
| REQ-03-01-006 | The future schema shall support quarantine/escalation status for suspected protected material content. | OPS-K-IP-3 | Protected-content gate fixture. |
| REQ-03-01-007 | The future schema shall preserve deterministic, versioned, provenance-preserving, schema-governed persistence behavior where material data is serialized. | AB-00-04 | Round-trip schema/persistence tests in a later implementation pass. |
| REQ-03-01-008 | The future schema shall not treat agent-generated setup text as engineering authority or source data. | OPS-K-AGENT-4 | Review checklist. |

## Standards

| Standard or policy source | Use in this setup evidence |
|---|---|
| OpenPipeStress CONTRACT | Governs protected-content, provenance, privacy, data, unit, governance, and agent-output constraints. |
| SOFTWARE_DECOMP revision 0.4 | Provides package/deliverable scope and architecture basis IDs. |
| External engineering standards | Referenced only as possible source categories; exact text, tables, values, and clauses are not locally available and remain `TBD`. |

## Verification

Future implementation verification should include:

- Schema validation for required identity, unit, provenance, redistribution, privacy, and completeness fields.
- Negative tests for missing units, missing provenance, missing redistribution rights, suspected protected content, and missing required values.
- Fixture review confirming no protected standards tables, real material allowable tables, or proprietary data were bundled.
- Round-trip serialization tests once persistence integration exists.
- Human review for public contribution acceptance and protected-content decisions.

## Documentation

Expected future product artifacts remain:

- `schemas/material.schema.yaml`
- material editor fixtures
- provenance/completeness fixture notes
- protected-content and redistribution review evidence

