# Datasheet: DEL-13-02 Constraint entity and provenance model

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-02 | `_CONTEXT.md` |
| Name | Constraint entity and provenance model | `_CONTEXT.md` |
| Package ID | PKG-13 | `_CONTEXT.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Anticipated artifact | `schemas/constraint.schema.json` | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Anticipated artifact | constraint provenance model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope coverage | SOW-068, SOW-067 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective support | OBJ-014, OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objective-to-Deliverable Mapping` |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |

## Attributes

| Attribute | Current source-grounded value |
|---|---|
| Constraint categories in scope | connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data (`SOW-068`) |
| Related design knowledge inputs | endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata (`SOW-067`) |
| Provenance posture | Constraint records must identify user/project/import/agent/source provenance where known (`_CONTEXT.md`; `docs/_Registers/Deliverables.csv`) |
| Model relationship | The physical model is the editable source of truth and carries typed references to design knowledge, constraints, diagnostics, unresolved assumptions, and traceability (`docs/SPEC.md#3 Domain model and schema`) |
| Schema baseline | JSON Schema 2020-12 is the accepted schema basis from the architecture injection and `docs/TYPES.md#Domain object vocabulary`; concrete schema vocabulary remains `TBD` |
| Unit posture | Unit-bearing physical values must carry explicit unit metadata unless explicitly dimensionless; missing or ambiguous unit metadata is a diagnostic (`docs/SPEC.md#4 Unit system and dimensional analysis`) |
| Professional-boundary posture | Constraint entities and outputs must not encode software-generated professional approval, certification, sealing, authentication, or code-compliance labels (`docs/CONTRACT.md`; `docs/SPEC.md#4.3 Analysis status and authority boundary`) |
| IP/data posture | Public artifacts must not bundle protected owner standards, protected code criteria, proprietary project data, copied standards text, protected tables, or code-specific values (`INIT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13`) |

## Conditions

| Condition | Handling |
|---|---|
| Source of constraints | Constraints are represented from available user/project/import/agent/source design knowledge. Protected owner or code standards are not bundled as public defaults. |
| Missing required data | Missing data is represented as an explicit constraint/diagnostic finding with provenance rather than silently defaulted. |
| Validation result role | Constraint failures are validation messages, not hidden report prose or agent text (`SOW-068`). |
| Engineering acceptance | Constraint records may support review, but they do not provide professional acceptance or code-compliance certification. |
| Dependency inputs | The approved local dependency mirror records upstream architecture basis, design knowledge schema, canonical domain model schema, unit contract, persistence contract, and professional-boundary policy as active predecessor context. |
| Exact field names and enum values | `TBD` pending implementation-level schema design and human review. |

## Construction

The deliverable is a data-model production unit. The source-grounded construction target is a constraint schema plus provenance model that can describe constraint records, constraint category, affected model/design-knowledge references, source/provenance metadata, unresolved/missing-data state, and validation-message support.

Concrete JSON Schema structure, required properties, `$id`, `$schema`, enum spellings, version fields, reusable `$defs`, and any example payloads are `TBD`. Any future public examples must be invented or otherwise cleared for redistribution and must not contain protected standards data, owner standards, proprietary project data, code-specific limits, or professional-approval claims.

## References

- `_CONTEXT.md` - deliverable identity, scope, artifacts, context envelope, architecture-basis injection.
- `_REFERENCES.md` - deliverable-local reference index.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-002 mirror/evidence surface for active predecessor context.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.5 package and deliverable basis.
- `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/ContextBudgetQA.csv` - machine-readable deliverable, scope, and context rows.
- `INIT.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md` - governing boundaries for schema, provenance, unit safety, privacy/IP, and professional responsibility.
