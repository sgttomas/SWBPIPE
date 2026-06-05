# Specification: DEL-03-01 Material library schema with provenance

## Scope

This deliverable records implementation evidence for a material-library schema covering private material records, temperature-dependent properties, allowable slots, provenance fields, redistribution status, and completeness flags.

The implemented evidence includes `schemas/material.schema.yaml`, `fixtures/material/invented_material_library_valid.json`, and `tests/test_material_schema.py`. It does not define engineering allowable values, reproduce standards content, claim code compliance, certify engineering suitability, resolve human review dispositions, or close dependency satisfaction.

## Requirements

| ID | Requirement | Evidence basis | Verification approach |
|---|---|---|---|
| REQ-03-01-001 | The schema supports material records with property slots, explicit dimensions, quantity kind, unit-required flags, and unit-bearing quantity values. | SOW-017; OPS-K-UNIT-1 | `tests/test_material_schema.py` validates schema and fixture structure. |
| REQ-03-01-002 | The schema supports allowable-value slots without bundling protected or code-specific allowable tables in public artifacts. | SOW-017 note; OPS-K-IP-1; OPS-K-DATA-1 | Test coverage checks public repository value policy and forbidden public-data text. |
| REQ-03-01-003 | The schema requires provenance metadata for material libraries, material records, property values, allowable slots, and diagnostics. | OPS-K-DATA-3; OPS-K-IP-2 | Test coverage asserts required provenance fields. |
| REQ-03-01-004 | The schema records public/private classification and redistribution status for material library data. | OPS-K-IP-2; OPS-K-PRIV-1; OPS-K-GOV-4 | Schema enums and fixture validation cover privacy and redistribution dispositions. |
| REQ-03-01-005 | The schema expresses missing solve-required or rule-check-required material values as explicit diagnostics or completeness findings, not defaults. | OPS-K-DATA-2; AB-00-06 | Fixture contains omitted values, incomplete status, and blocking `MATERIAL_PROPERTY_MISSING` diagnostics. |
| REQ-03-01-006 | The schema supports quarantine/escalation status for suspected protected material content. | OPS-K-IP-3 | Schema includes `protected_suspected` redistribution/value states and protected-content diagnostic code. |
| REQ-03-01-007 | The schema preserves versioned, provenance-preserving, schema-governed serialization behavior where material data is serialized. | AB-00-04 | Schema validation is present; round-trip persistence integration remains downstream `TBD`. |
| REQ-03-01-008 | The deliverable does not treat agent-generated setup text as engineering authority or source data. | OPS-K-AGENT-4 | Fixture provenance labels invented/schema-slot evidence and omits engineering values. |
| REQ-03-01-009 | The material property dimension vocabulary stays aligned with accepted PKG-02 dimensions. | DEL-02-01; DEL-02-02; PKG-02 audit finding resolution | `tests/test_material_schema.py` asserts material dimensions are a subset of canonical dimensions and exclude retired aliases. |

## Standards

| Standard or policy source | Use in this setup evidence |
|---|---|
| OpenPipeStress CONTRACT | Governs protected-content, provenance, privacy, data, unit, governance, and agent-output constraints. |
| SOFTWARE_DECOMP revision 0.7 | Provides package/deliverable scope and architecture basis IDs. |
| External engineering standards | Referenced only as possible source categories; exact text, tables, values, and clauses are not locally available and remain `TBD`. |

## Verification

Current verification evidence includes:

- `python3 tests/test_material_schema.py` validates required identity, unit, provenance, redistribution, privacy, completeness, diagnostic, open-decision, and fixture fields.
- The test asserts no schema `default` keys are present, preserving no-silent-default behavior.
- The invented fixture omits material engineering values and records an incomplete solve-required material record with blocking diagnostics.
- The test checks that protected/code-compliance phrases and standards identifiers from the forbidden list are absent from schema and fixture text.
- The test checks material dimensions against the accepted PKG-02 dimension vocabulary and retired aliases.
- Round-trip persistence integration, public source catalog acceptance, fixture value policy, interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions remain `TBD`.

## Documentation

Implemented product evidence:

- `schemas/material.schema.yaml`
- `fixtures/material/invented_material_library_valid.json`
- `tests/test_material_schema.py`

Still-open review evidence:

- protected-content and redistribution review disposition
- human review of package audit findings
- dependency satisfaction and downstream readiness disposition
