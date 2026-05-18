# Specification: DEL-06-01 Rule-pack schema

## Scope

This deliverable defines setup evidence for a future rule-pack schema covering metadata, required inputs, formula declarations, user-supplied allowables, pass/fail/incomplete criteria, checksum fields, provenance/source notes, redistribution status, and professional-boundary markings.

This setup pass does not implement `schemas/rule_pack.schema.yaml`, update `docs/SPEC.md`, create public examples, define an expression grammar, implement a sandboxed evaluator, publish protected rule content, or claim engineering code compliance.

## Requirements

| ID | Requirement | Evidence basis | Verification approach |
|---|---|---|---|
| REQ-06-01-001 | The future schema shall use JSON Schema 2020-12 conventions for public schema/interchange artifacts. | ContextBudgetQA row DEL-06-01; AB-00-04 | Schema meta-validation in a later implementation pass. |
| REQ-06-01-002 | The future schema shall include rule-pack identity, schema version, rule-pack version, lifecycle/status, and source-note metadata. | SOW-042; OPS-K-RULE-3 | Required-field schema validation. |
| REQ-06-01-003 | The future schema shall require provenance metadata for rule-pack values, allowable slots, formulas, and source bases. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-RULE-3 | Positive and negative provenance fixtures. |
| REQ-06-01-004 | The future schema shall record public/private classification and redistribution status for each rule pack and public-example candidate. | SOW-042; OPS-K-RULE-3; OPS-K-PRIV-1 | Redistribution-status fixture validation. |
| REQ-06-01-005 | The future schema shall include checksum metadata identifying algorithm, hash scope, hash value, and canonicalization basis for JSON payloads. | SOW-042; AB-00-04 | Canonical JSON/JCS-compatible hash fixture validation. |
| REQ-06-01-006 | The future schema shall support required-input declarations with units, dimensional categories, source/provenance requirements, and missing-value findings. | SOW-016; OPS-K-DATA-2; OPS-K-UNIT-1 | Missing-input and unit-mismatch fixtures. |
| REQ-06-01-007 | The future schema shall support formula declaration slots without embedding protected standards text, protected formulas, copied code equations, or arbitrary executable code. | SOW-016; OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-2 | Protected-content review and evaluator-boundary review. |
| REQ-06-01-008 | The future schema shall support user-supplied allowable slots with units, provenance, source notes, redistribution status, and completeness status. | SOW-016; OPS-K-DATA-1; OPS-K-DATA-3 | Allowable-slot negative fixture validation. |
| REQ-06-01-009 | The future schema shall represent pass/fail/incomplete criteria for user-rule checks while preserving the distinction between user-rule checked and professionally approved. | OBJ-005; OPS-K-MECH-2; OPS-K-AUTH-1 | Status-model and report-wording review. |
| REQ-06-01-010 | The future schema shall emit or support structured diagnostics for rule-check blocking, provenance warning, unit mismatch, protected-content warning, evaluator error, and redistribution warning conditions. | AB-00-06; OPS-K-DATA-2; OPS-K-IP-3 | Diagnostic fixture validation. |
| REQ-06-01-011 | The future schema shall require public examples to use invented non-code values and explicit non-engineering notices. | OPS-K-RULE-1; SOW-016 note | Public-example protected-content gate. |
| REQ-06-01-012 | The future schema shall not treat agent-generated setup text as engineering authority, standards authority, legal conclusion, or professional approval. | OPS-K-AGENT-1; OPS-K-AGENT-4 | Review checklist. |

## Standards

| Standard or policy source | Use in this setup evidence |
|---|---|
| OpenPipeStress CONTRACT | Governs protected-content, provenance, privacy, unit, rule-pack, professional-boundary, and agent-output constraints. |
| SOFTWARE_DECOMP revision 0.4 | Provides package/deliverable scope, accepted architecture basis, open issues, and remaining `TBD` boundaries. |
| JSON Schema 2020-12 | Required baseline for future public schema/interchange artifacts. Exact file layout and code-generation tooling remain `TBD`. |
| Canonical JSON / JCS-compatible hashing | Required checksum basis where JSON payloads are hashed. Exact implementation library remains `TBD`. |
| External engineering standards | May be referenced by user-owned private rule packs, but their protected text, tables, examples, formulas, and values are not public project content. |

## Verification

Future implementation verification should include:

- JSON Schema 2020-12 meta-validation for the rule-pack schema artifact.
- Positive fixtures for identity/version metadata, checksum metadata, required input declarations, provenance/source notes, redistribution status, unit metadata, and rule-check criteria.
- Negative fixtures for missing units, missing provenance, missing source notes, missing redistribution status, missing required inputs, checksum mismatch, and private/public classification gaps.
- Protected-content review confirming public fixtures do not contain standards text, copied protected formulas, proprietary allowables, protected tables, or project-specific engineering values.
- Canonical JSON/JCS-compatible hash verification for JSON rule-pack payloads and manifest hash behavior for any non-JSON assets.
- Status wording review confirming no output claims certification, sealing, engineering approval, or code compliance for reliance.

## Documentation

Expected future product artifacts remain:

- `schemas/rule_pack.schema.yaml`
- `docs/SPEC.md` rule-pack schema section update
- invented non-code fixture notes, if examples are later authorized
- checksum/provenance validation notes
- protected-content and redistribution review evidence

