# Datasheet: DEL-16-01 Structured model operation schema

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-16-01 |
| Name | Structured model operation schema |
| Package ID | PKG-16 |
| Package Name | Model Operation and Agent Proposal Framework |
| Type | DATA_MODEL_CHANGE |
| Scope Coverage | SOW-069 |
| Objective Support | OBJ-015 |
| Implemented Schema Artifact | `schemas/model_operation.schema.json` |
| Implemented Fixture Evidence | `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json` |
| Lifecycle note | Deliverable-local status remains governed by `_STATUS.md`; this datasheet records implementation evidence only. |

Sources: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Scope Coverage", "Objective Support"; `schemas/model_operation.schema.json`; `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`.

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Operation route | GUI and agent edits are represented as structured model operations. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `_CONTEXT.md` Context Envelope |
| Mutation boundary | The schema contract status fixes `mutation_route = structured_operations_only` and `direct_model_mutation_allowed = false`. | `schemas/model_operation.schema.json` `$defs.OperationContractStatus`; `tests/test_model_operation_schema.py` |
| Operation categories in scope | `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`. | `schemas/model_operation.schema.json` `$defs.OperationKind`; `tests/test_model_operation_schema.py` |
| Change kinds represented | `add_object`, `remove_object`, `set_field`, `move_geometry`, `reconnect`, `update_constraint`, `update_load`, `update_support`, and `attach_design_knowledge`. | `schemas/model_operation.schema.json` `$defs.OperationChange`; `tests/test_model_operation_schema.py` |
| Schema/interchange baseline | JSON Schema 2020-12. | `schemas/model_operation.schema.json` `$schema`; `tests/test_model_operation_schema.py` |
| Root envelope fields | `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set`. | `schemas/model_operation.schema.json` root `required`; `tests/test_model_operation_schema.py` |
| Operation record fields | `operation_id`, `operation_kind`, `operation_status`, `author_type`, `target_refs`, `preconditions`, `changes`, `validation`, `diagnostics`, `diff_preview_refs`, `assumptions`, `provenance`, and `professional_boundary`. | `schemas/model_operation.schema.json` `$defs.ModelOperationRecord`; `tests/test_model_operation_schema.py` |
| Model-basis binding | Operation sets require `model_basis` with `model_ref`, `canonical_model_role = physical_source_of_truth`, `physical_source_of_truth_ref`, `accepted_model_state_ref`, and `accepted_model_state_hash`. Operation preconditions require current hashes including `payload_scope = model_state_record`. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis` and `$defs.OperationPrecondition`; fixtures; `tests/test_model_operation_schema.py` |
| Unit boundary | Unit-bearing payloads require explicit unit metadata and dimension checks; missing-unit behavior is `emit_diagnostic`. Quantity dimensions use the canonical dimension enum currently encoded in the schema. | `schemas/model_operation.schema.json` `$defs.UnitRequirements` and `$defs.Quantity`; `tests/test_model_operation_schema.py` |
| Downstream validation surfaces | Schema fields include validation state, diagnostics, diff-preview references, and application status, but validator/diff behavior is owned by DEL-16-02. | `schemas/model_operation.schema.json`; `tests/test_operation_validation_preview.py`; `Dependencies.csv` downstream row `DEL-16-01-D001` |
| Professional boundary | Operation records and operation sets require `human_review_required = true` and negative software claims for compliance, certification, sealing, approval, and authentication. | `schemas/model_operation.schema.json` `$defs.ProfessionalBoundary`; fixtures; `tests/test_model_operation_schema.py` |

## Conditions

| Condition | Status |
|---|---|
| Exact schema file path | Established as `schemas/model_operation.schema.json`. |
| Exact implemented root and operation field layout | Established by `schemas/model_operation.schema.json` and covered by `tests/test_model_operation_schema.py`. |
| Operation fixture path | Established as `fixtures/model_operations/invented_operation_set_valid.json`. |
| Accepted model-state fixture path | Established as `fixtures/model_operations/invented_accepted_model_state.json`. |
| Exact operation granularity for all future physical model edits | Still TBD beyond the current operation/change enums and fixture coverage. |
| Exact persistence granularity and broader hash partitioning | Still TBD beyond the implemented `accepted_model_state_hash` and per-operation `required_current_hashes` hooks. |
| Validation/diff-preview behavior | Downstream DEL-16-02; current schema only provides fields and evidence hooks. |
| User acceptance and audit trail behavior | Downstream DEL-16-03; current schema records boundary hooks only. |
| Human review dispositions | Review finding dispositions remain governed by `Review_Findings.csv` and are not changed here. |
| Protected standards or proprietary data | Public fixtures remain invented examples with provenance and privacy classifications; no protected standards content or private project data is asserted. |

Sources: `schemas/model_operation.schema.json`; fixtures under `fixtures/model_operations/`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1, OPS-K-AUTH-1, OPS-K-AGENT-4.

## Construction

The deliverable is currently materialized as a schema-first data model plus invented fixtures:

| Artifact | Construction Status | Notes |
|---|---|---|
| `schemas/model_operation.schema.json` | Implemented evidence present | Strict JSON Schema 2020-12 contract for proposed model operations. It uses `additionalProperties: false` at object definitions inspected and has no schema defaults in the focused test. |
| `fixtures/model_operations/invented_operation_set_valid.json` | Implemented evidence present | Invented public example operation set covering required operation and change categories, with provenance, validation placeholders, diff-preview references where present, precondition hashes, and professional-boundary fields. |
| `fixtures/model_operations/invented_accepted_model_state.json` | Implemented evidence present | Invented accepted physical model-state fixture with `state_hash = sha256:invented-state-001`, matching operation-set model-basis and required-current-hash evidence. |
| `tests/test_model_operation_schema.py` | Implemented evidence present | Focused stdlib test for schema structure, operation/change enums, model-basis requirements, unit dimensions, professional-boundary negatives, and fixture validation. |
| `tests/test_operation_validation_preview.py` | Read-only adjacent evidence | DEL-16-02 validation/diff-preview behavior test consumes the DEL-16-01 schema and fixtures; behavior ownership remains downstream. |

## References

- `_CONTEXT.md` - deliverable identity, scope, objective, architecture-basis injection.
- `_REFERENCES.md` - source list for this setup pass.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 current decomposition basis, SOW-069, OBJ-015, PKG-16, DEL-16-01, architecture basis.
- `docs/CONTRACT.md` - project invariants, including data boundary, professional boundary, and agent non-invention constraints.
- `docs/TYPES.md` - canonical object registry and schema boundary notes.
- `docs/SPEC.md` - technical specification slices for domain objects, persistence, viewport command intents, and professional boundaries.
- `docs/IP_AND_DATA_BOUNDARY.md` - public/private/protected-content boundary.
- `Dependencies.csv` - approved local dependency mirror/evidence surface, read only for this pass.
- `schemas/model_operation.schema.json` - implemented schema evidence.
- `fixtures/model_operations/invented_operation_set_valid.json` and `fixtures/model_operations/invented_accepted_model_state.json` - invented fixture evidence.
- `tests/test_model_operation_schema.py` and `tests/test_operation_validation_preview.py` - focused validation evidence.
