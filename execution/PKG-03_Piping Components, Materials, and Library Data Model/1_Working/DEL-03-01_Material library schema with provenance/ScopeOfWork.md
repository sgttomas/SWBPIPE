---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-01
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-017]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-01` in service of project scope [SOW-017] and package objectives [OBJ-004].

- **OUT-001** — A material-library schema contract covering temperature-dependent properties, allowable slots, provenance, redistribution status, privacy, completeness, and explicit diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-01 Material library schema with provenance

> #### Datasheet: DEL-03-01 Material library schema with provenance
>

### CLM-002 — PDU-024 Version Integration

> ##### PDU-024 Version Integration
>
> - Owning material schema remains `schemas/material.schema.yaml`.
> - Project-carried material data consumes DEL-02-05 model-document version handling, whose accepted current family is `0.2.0`.
> - No material fixture migration, material serializer, or new migration status is added by this tranche.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-01 |
> | Package ID | PKG-03 |
> | Package | Piping Components, Materials, and Library Data Model |
> | Type | DATA_MODEL_CHANGE |
> | Scope item | SOW-017 |
> | Objective | OBJ-004 |
> | Anticipated artifacts | `schemas/material.schema.yaml`; material editor fixtures |
> | Current evidence status | Schema, fixture, and test implementation evidence present; deliverable remains `IN_PROGRESS` pending review and dependency disposition. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Required setup meaning | Source |
> |---|---|---|
> | Material library scope | Private material libraries with temperature-dependent properties, allowables, and provenance fields. | ScopeLedger row SOW-017 |
> | Data boundary | Public repository must not bundle protected standards tables, material allowables, copied examples, proprietary data, or paraphrased protected tables. | CONTRACT OPS-K-IP-1, OPS-K-IP-3; SOW-017 notes |
> | Provenance fields | Material, component, allowable, and rule-pack values carry provenance fields. | CONTRACT OPS-K-DATA-3 |
> | Redistribution status | Public data contributions require source, provenance, license or redistribution status, contributor certification, and review disposition. | CONTRACT OPS-K-IP-2 |
> | Unit handling | Imported values, exported values, and calculations using material values must be unit-aware and dimensionally checked. | CONTRACT OPS-K-UNIT-1 |
> | Missing required values | Missing solve-required or rule-check-required material values must become explicit findings, not silent defaults. | CONTRACT OPS-K-DATA-2 |
> | Private data handling | Private material data must not be transmitted or committed publicly by default. | CONTRACT OPS-K-PRIV-1 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - `schemas/material.schema.yaml` defines structure for temperature-dependent properties and allowables, but it does not provide actual engineering values.
> - `fixtures/material/invented_material_library_valid.json` is a public schema fixture with omitted engineering values, `TBD` source/license disposition, and explicit blocking diagnostics for missing solve-required data.
> - Code-specific values and allowables remain user-supplied or lawfully imported private data, not bundled public defaults.
> - Architecture basis requires JSON Schema 2020-12, deterministic/versioned/provenance-preserving persistence, unit awareness, diagnostics, adapter validation, and protected-content/provenance gates where relevant.
> - Public material source catalog, public fixture value policy, temperature interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions remain `TBD`.
>

### CLM-006 — Construction

> ##### Construction
>
> The implemented material schema and fixture evidence cover these record groups:
>
> | Record group | Implemented evidence and remaining disposition |
> |---|---|
> | Library identity | `MaterialLibrary` requires `library_id`, `name`, `library_scope`, `privacy_class`, `provenance`, and `review_status`. |
> | Material identity | `MaterialRecord` requires `material_id`, `name`, `material_family`, privacy/redistribution fields, properties, allowables, completeness, provenance, and review status. |
> | Property definitions | `MaterialPropertyDefinition` declares property kind, dimension ID, quantity kind, unit requirement, required-for context, public fixture policy, and review status. |
> | Property values | `MaterialPropertyValue` carries property kind, value status, required-for context, provenance, review status, and optional unit-bearing quantity value. |
> | Allowable slots | `MaterialAllowableSlot` records slot metadata, value status, public repository value policy, required-for context, provenance, and review status without supplying protected/code-specific values. |
> | Provenance | `Provenance` requires source name/location/license, contributor, contributor certification, redistribution status, and review status. Fixture source/license disposition remains `TBD`. |
> | Redistribution and privacy | Schema enums record public, private, unknown, rejected, suspected-protected, and `TBD` dispositions. Human acceptance of public material data remains separate. |
> | Completeness flags | `CompletenessRule` and `MaterialCompletenessFinding` record required property kinds, missing behavior, status, and diagnostic code. |
> | Diagnostics | `MaterialDiagnostic` records code, class, severity, source, affected reference, message, remediation, and provenance. The fixture emits `MATERIAL_PROPERTY_MISSING` as a blocking solve diagnostic. |
> | Open decisions | `OpenDecision` preserves unresolved public fixture, source catalog, allowable storage, and interpolation policy topics. |
>

### CLM-007 — References

> ##### References
>
> - docs/_Registers/Deliverables.csv row DEL-03-01
> - docs/_Registers/ScopeLedger.csv row SOW-017
> - docs/_Registers/ContextBudgetQA.csv row DEL-03-01
> - execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
> - docs/CONTRACT.md invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1..4

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-03-01 Material library schema with provenance

> #### Specification: DEL-03-01 Material library schema with provenance
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable records implementation evidence for a material-library schema covering private material records, temperature-dependent properties, allowable slots, provenance fields, redistribution status, and completeness flags.
>
> The implemented evidence includes `schemas/material.schema.yaml`, `fixtures/material/invented_material_library_valid.json`, and `tests/test_material_schema.py`. It does not define engineering allowable values, reproduce standards content, claim code compliance, certify engineering suitability, resolve human review dispositions, or close dependency satisfaction.
>

### CLM-010 — PDU-024 Persistence Integration Boundary

> ##### PDU-024 Persistence Integration Boundary
>
> When material data is carried in a project model, downstream DEL-02-05 persistence uses the accepted model-document `0.2.0` family and established current/stale/unsupported/newer/failed version checks. This integration evidence does not add a DEL-03-01-owned serializer, migrate the invented material fixture, or close the separately recorded material round-trip persistence deferral.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence basis | Verification approach |
> |---|---|---|---|
> | REQ-03-01-001 | The schema supports material records with property slots, explicit dimensions, quantity kind, unit-required flags, and unit-bearing quantity values. | SOW-017; OPS-K-UNIT-1 | `tests/test_material_schema.py` validates schema and fixture structure. |
> | REQ-03-01-002 | The schema supports allowable-value slots without bundling protected or code-specific allowable tables in public artifacts. | SOW-017 note; OPS-K-IP-1; OPS-K-DATA-1 | Test coverage checks public repository value policy and forbidden public-data text. |
> | REQ-03-01-003 | The schema requires provenance metadata for material libraries, material records, property values, allowable slots, and diagnostics. | OPS-K-DATA-3; OPS-K-IP-2 | Test coverage asserts required provenance fields. |
> | REQ-03-01-004 | The schema records public/private classification and redistribution status for material library data. | OPS-K-IP-2; OPS-K-PRIV-1; OPS-K-GOV-4 | Schema enums and fixture validation cover privacy and redistribution dispositions. |
> | REQ-03-01-005 | The schema expresses missing solve-required or rule-check-required material values as explicit diagnostics or completeness findings, not defaults. | OPS-K-DATA-2; AB-00-06 | Fixture contains omitted values, incomplete status, and blocking `MATERIAL_PROPERTY_MISSING` diagnostics. |
> | REQ-03-01-006 | The schema supports quarantine/escalation status for suspected protected material content. | OPS-K-IP-3 | Schema includes `protected_suspected` redistribution/value states and protected-content diagnostic code. |
> | REQ-03-01-007 | The schema preserves versioned, provenance-preserving, schema-governed serialization behavior where material data is serialized. | AB-00-04 | Schema validation is present and project-carried material data consumes DEL-02-05's accepted `0.2.0` version contract; a dedicated material serialization round-trip harness remains downstream `TBD`. |
> | REQ-03-01-008 | The deliverable does not treat agent-generated setup text as engineering authority or source data. | OPS-K-AGENT-4 | Fixture provenance labels invented/schema-slot evidence and omits engineering values. |
> | REQ-03-01-009 | The material property dimension vocabulary stays aligned with accepted PKG-02 dimensions. | DEL-02-01; DEL-02-02; PKG-02 audit finding resolution | `tests/test_material_schema.py` asserts material dimensions are a subset of canonical dimensions and exclude retired aliases. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or policy source | Use in this setup evidence |
> |---|---|
> | OpenPipeStress CONTRACT | Governs protected-content, provenance, privacy, data, unit, governance, and agent-output constraints. |
> | SOFTWARE_DECOMP revision 0.7 | Provides package/deliverable scope and architecture basis IDs. |
> | External engineering standards | Referenced only as possible source categories; exact text, tables, values, and clauses are not locally available and remain `TBD`. |
>

### CLM-013 — Verification

> ##### Verification
>
> Current verification evidence includes:
>
> - `python3 tests/test_material_schema.py` validates required identity, unit, provenance, redistribution, privacy, completeness, diagnostic, open-decision, and fixture fields.
> - The test asserts no schema `default` keys are present, preserving no-silent-default behavior.
> - The invented fixture omits material engineering values and records an incomplete solve-required material record with blocking diagnostics.
> - The test checks that protected/code-compliance phrases and standards identifiers from the forbidden list are absent from schema and fixture text.
> - The test checks material dimensions against the accepted PKG-02 dimension vocabulary and retired aliases.
> - Round-trip persistence integration, public source catalog acceptance, fixture value policy, interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions remain `TBD`.
>

### CLM-014 — Documentation

> ##### Documentation
>
> Implemented product evidence:
>
> - `schemas/material.schema.yaml`
> - `fixtures/material/invented_material_library_valid.json`
> - `tests/test_material_schema.py`
>
> Still-open review evidence:
>
> - protected-content and redistribution review disposition
> - human review of package audit findings
> - dependency satisfaction and downstream readiness disposition

- **AC-001** — The contract preserves accepted material-data requirements and boundaries, including unit awareness, protected-content and redistribution controls, explicit missing-value findings, and unresolved policy decisions without supplying engineering values or professional approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-03-01 Material library schema with provenance

> #### Procedure: DEL-03-01 Material library schema with provenance
>

### CLM-016 — PDU-024 Integration Check

> ##### PDU-024 Integration Check
>
> 1. Validate material records against the DEL-03-01 schema and provenance rules.
> 2. When material records are carried by a project model, consume DEL-02-05 version-check and migration evidence rather than a local version literal.
> 3. Preserve explicit stale/unsupported/newer/failed project-version diagnostics and do not rewrite the material fixture as part of model-document compatibility handling.
>

### CLM-017 — Purpose

> ##### Purpose
>
> Describe the maintenance and review procedure for the DEL-03-01 material-library schema, invented fixture, and validation evidence while preserving protected-content, provenance, unit, privacy, and agent-output boundaries.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Sealed DEL-03-01 brief and write scope.
> - `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, register rows, SOFTWARE_DECOMP revision 0.7, and approved DAG-006 context.
> - Current evidence files: `schemas/material.schema.yaml`, `fixtures/material/invented_material_library_valid.json`, and `tests/test_material_schema.py`.
> - Human-approved rules for any public fixture source, license, redistribution status, and review disposition.
> - No protected material tables or proprietary library data in the working folder.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm `_STATUS.md` is not `ISSUED` and that the intended tranche is evidence reconciliation, not lifecycle promotion.
> 2. Read the DEL-03-01 context, register rows, SOW-017, OBJ-004, and applicable CONTRACT invariants.
> 3. Inspect `schemas/material.schema.yaml` for identity, property, allowable, provenance, redistribution, completeness, diagnostic, and open-decision coverage.
> 4. Inspect `fixtures/material/invented_material_library_valid.json` to confirm it remains invented/schema-shape evidence with omitted engineering values and explicit diagnostics.
> 5. Run `python3 tests/test_material_schema.py` and record the result in a deliverable-local run record.
> 6. Run a stale-language check against the active four-document kit for phrases that incorrectly describe the implemented schema as only setup/future evidence.
> 7. Preserve unresolved public source, fixture value, interpolation, allowable storage, dependency satisfaction, and human review disposition items as `TBD`.
> 8. Route public data acceptance, quarantine disposition, package audit finding disposition, dependency satisfaction, and lifecycle transitions through separate human/reconciliation gates.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Material schema test | `python3 tests/test_material_schema.py` passes. |
> | Stale active-doc language | Active four-document kit no longer says implementation evidence is only setup/future work. |
> | Protected-content scan | No protected standards text, material allowable tables, copied examples, or proprietary material data is introduced. |
> | Provenance validation | Every governed material value has source/provenance and rights status, or produces an explicit finding. |
> | Unit validation | Material property dimensions remain aligned with the accepted PKG-02 dimension vocabulary. |
> | Missing-value behavior | Missing solve-required or rule-check-required values produce explicit diagnostics. |
> | Privacy check | Private libraries are not transmitted or committed publicly by default. |
> | Review disposition check | `Review_Findings.csv` remains pending human disposition unless a separate human gate authorizes edits. |
>

### CLM-021 — Records

> ##### Records
>
> - Schema review notes: deliverable-local run records.
> - Fixture provenance review: `TBD` until human/source review accepts a public fixture policy.
> - Protected-content review disposition: `TBD`.
> - Human rulings on public fixture sources and package audit findings: `TBD`.
> - Dependency register and run records in this deliverable folder.

- **VER-001** — Validate the contract and review source parity, schema and fixture coverage, unit and provenance boundaries, missing-value diagnostics, protected-content controls, deterministic persistence compatibility, and unresolved human-review items.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-03-01 Material library schema with provenance

> #### Guidance: DEL-03-01 Material library schema with provenance
>

### CLM-023 — PDU-024 Guidance

> ##### PDU-024 Guidance
>
> Treat model-document compatibility as downstream DEL-02-05 authority. Material consumers must not duplicate a current model-version literal or reinterpret migration statuses. Preserve the material schema/provenance contract and surface downstream stale, unsupported, newer, or failed project-version diagnostics without coercing material records.
>

### CLM-024 — Purpose

> ##### Purpose
>
> This deliverable exists so OpenPipeStress can support piping-specific private material libraries without shipping protected material tables, proprietary commercial data, or unreviewed public data. The schema should make provenance, rights, units, and completeness visible enough for later validation, diagnostics, and review gates.
>

### CLM-025 — Principles

> ##### Principles
>
> - Treat material values and allowables as governed data, not as free public defaults.
> - Separate schema slots from data content: defining an `allowable` field is allowed; populating public tables of protected allowables is not.
> - Preserve source and rights metadata with every material value that could affect solving, rule checking, reporting, or downstream reliance.
> - Use `TBD` for unresolved public source catalogs, accepted fixture value policy, interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions.
> - Prefer explicit warnings and blocked states over silent fallbacks when required material data is absent or untrusted.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The implemented schema already distinguishes private/user-supplied values, public-permissive reviewed values, invented non-engineering fixture evidence, suspected protected content, and unresolved `TBD` states. Private records may carry user-entered or lawfully imported values, while public repository fixtures still require documented redistribution rights and review disposition before any real values are accepted.
>
> The current invented fixture is schema-shape evidence. Its source/license and redistribution fields intentionally remain `TBD`, and that `TBD` status is not accepted public material data. The fixture omits engineering values and carries blocking diagnostics to demonstrate missing-data behavior.
>
> The schema is structured so importers and adapters have provenance, redistribution, unit, and diagnostic fields to preserve. Concrete import/export formats and adapter behavior remain downstream work.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Tension | Current position |
> |---|---|---|
> | Public examples | Useful for tests and documentation, but risky if derived from protected standards or proprietary libraries. | The current fixture is invented/schema-shape evidence with omitted values. Real public material values require later source and redistribution review. |
> | Required field strictness | Strict requirements improve safety but may block partial private libraries. | Required-for-solving/checking values produce explicit completeness findings and diagnostics when absent. Additional policy strictness remains downstream. |
> | Source citations | Detailed source pointers improve traceability but can expose protected content if mishandled. | Store source/provenance pointers and rights status; do not reproduce protected text/tables. |
> | Allowable values | Needed for some checks, but code-specific tables are protected or user-governed. | Provide schema slots and diagnostics; do not bundle public allowable tables. |
>

### CLM-028 — Examples

> ##### Examples
>
> The current public fixture is `fixtures/material/invented_material_library_valid.json`. It is not accepted material data for engineering use: it is an invented schema fixture with omitted values, `TBD` source/license disposition, and explicit blocking diagnostics.
>
> Any later material editor fixture that supplies values must use invented non-engineering values or rights-cleared public-permissive data with documented review disposition. It must not include protected material allowable tables, copied standards examples, proprietary library data, or invented values presented as engineering data.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict found in accessible setup sources. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-017 OBJ-004 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
