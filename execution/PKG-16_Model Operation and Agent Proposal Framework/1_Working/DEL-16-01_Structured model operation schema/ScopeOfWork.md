---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-01
package_id: PKG-16
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-069]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-16-01` in service of project scope [SOW-069] and package objectives [OBJ-015].

- **OUT-001** — A schema-first structured model-operation contract for GUI and agent proposals, with invented fixtures and explicit mutation, model-basis, unit, provenance, diagnostic, and professional-boundary controls, is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-16-01 Structured model operation schema

> #### Datasheet: DEL-16-01 Structured model operation schema
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-16-01 |
> | Name | Structured model operation schema |
> | Package ID | PKG-16 |
> | Package Name | Model Operation and Agent Proposal Framework |
> | Type | DATA_MODEL_CHANGE |
> | Scope Coverage | SOW-069 |
> | Objective Support | OBJ-015 |
> | Implemented Schema Artifact | `schemas/model_operation.schema.json` |
> | Implemented Fixture Evidence | `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json` |
> | Lifecycle note | Deliverable-local status remains governed by `_STATUS.md`; this datasheet records implementation evidence only. |
>
> Sources: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Scope Coverage", "Objective Support"; `schemas/model_operation.schema.json`; `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`.
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Operation route | GUI and agent edits are represented as structured model operations. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `_CONTEXT.md` Context Envelope |
> | Mutation boundary | The schema contract status fixes `mutation_route = structured_operations_only` and `direct_model_mutation_allowed = false`. | `schemas/model_operation.schema.json` `$defs.OperationContractStatus`; `tests/test_model_operation_schema.py` |
> | Operation categories in scope | `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`. | `schemas/model_operation.schema.json` `$defs.OperationKind`; `tests/test_model_operation_schema.py` |
> | Change kinds represented | `add_object`, `remove_object`, `set_field`, `move_geometry`, `reconnect`, `update_constraint`, `update_load`, `update_support`, and `attach_design_knowledge`. | `schemas/model_operation.schema.json` `$defs.OperationChange`; `tests/test_model_operation_schema.py` |
> | Schema/interchange baseline | JSON Schema 2020-12. | `schemas/model_operation.schema.json` `$schema`; `tests/test_model_operation_schema.py` |
> | Root envelope fields | `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set`. | `schemas/model_operation.schema.json` root `required`; `tests/test_model_operation_schema.py` |
> | Operation record fields | `operation_id`, `operation_kind`, `operation_status`, `author_type`, `target_refs`, `preconditions`, `changes`, `validation`, `diagnostics`, `diff_preview_refs`, `assumptions`, `provenance`, and `professional_boundary`. | `schemas/model_operation.schema.json` `$defs.ModelOperationRecord`; `tests/test_model_operation_schema.py` |
> | Model-basis binding | Operation sets require `model_basis` with `model_ref`, `canonical_model_role = physical_source_of_truth`, `physical_source_of_truth_ref`, `accepted_model_state_ref`, and `accepted_model_state_hash`. Operation preconditions require current hashes including `payload_scope = model_state_record`. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis` and `$defs.OperationPrecondition`; fixtures; `tests/test_model_operation_schema.py` |
> | Unit boundary | Unit-bearing payloads require explicit unit metadata and dimension checks; missing-unit behavior is `emit_diagnostic`. Quantity dimensions use the canonical dimension enum currently encoded in the schema. | `schemas/model_operation.schema.json` `$defs.UnitRequirements` and `$defs.Quantity`; `tests/test_model_operation_schema.py` |
> | Downstream validation surfaces | Schema fields include validation state, diagnostics, diff-preview references, and application status, but validator/diff behavior is owned by DEL-16-02. | `schemas/model_operation.schema.json`; `tests/test_operation_validation_preview.py`; `Dependencies.csv` downstream row `DEL-16-01-D001` |
> | Professional boundary | Operation records and operation sets require `human_review_required = true` and negative software claims for compliance, certification, sealing, approval, and authentication. | `schemas/model_operation.schema.json` `$defs.ProfessionalBoundary`; fixtures; `tests/test_model_operation_schema.py` |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Exact schema file path | Established as `schemas/model_operation.schema.json`. |
> | Exact implemented root and operation field layout | Established by `schemas/model_operation.schema.json` and covered by `tests/test_model_operation_schema.py`. |
> | Operation fixture path | Established as `fixtures/model_operations/invented_operation_set_valid.json`. |
> | Accepted model-state fixture path | Established as `fixtures/model_operations/invented_accepted_model_state.json`. |
> | Exact operation granularity for all future physical model edits | Still TBD beyond the current operation/change enums and fixture coverage. |
> | Exact persistence granularity and broader hash partitioning | Still TBD beyond the implemented `accepted_model_state_hash` and per-operation `required_current_hashes` hooks. |
> | Validation/diff-preview behavior | Downstream DEL-16-02; current schema only provides fields and evidence hooks. |
> | User acceptance and audit trail behavior | Downstream DEL-16-03; current schema records boundary hooks only. |
> | Human review dispositions | Review finding dispositions remain governed by `Review_Findings.csv` and are not changed here. |
> | Protected standards or proprietary data | Public fixtures remain invented examples with provenance and privacy classifications; no protected standards content or private project data is asserted. |
>
> Sources: `schemas/model_operation.schema.json`; fixtures under `fixtures/model_operations/`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1, OPS-K-AUTH-1, OPS-K-AGENT-4.
>

### CLM-005 — Construction

> ##### Construction
>
> The deliverable is currently materialized as a schema-first data model plus invented fixtures:
>
> | Artifact | Construction Status | Notes |
> |---|---|---|
> | `schemas/model_operation.schema.json` | Implemented evidence present | Strict JSON Schema 2020-12 contract for proposed model operations. It uses `additionalProperties: false` at object definitions inspected and has no schema defaults in the focused test. |
> | `fixtures/model_operations/invented_operation_set_valid.json` | Implemented evidence present | Invented public example operation set covering required operation and change categories, with provenance, validation placeholders, diff-preview references where present, precondition hashes, and professional-boundary fields. |
> | `fixtures/model_operations/invented_accepted_model_state.json` | Implemented evidence present | Invented accepted physical model-state fixture with `state_hash = sha256:invented-state-001`, matching operation-set model-basis and required-current-hash evidence. |
> | `tests/test_model_operation_schema.py` | Implemented evidence present | Focused stdlib test for schema structure, operation/change enums, model-basis requirements, unit dimensions, professional-boundary negatives, and fixture validation. |
> | `tests/test_operation_validation_preview.py` | Read-only adjacent evidence | DEL-16-02 validation/diff-preview behavior test consumes the DEL-16-01 schema and fixtures; behavior ownership remains downstream. |
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, objective, architecture-basis injection.
> - `_REFERENCES.md` - source list for this setup pass.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 current decomposition basis, SOW-069, OBJ-015, PKG-16, DEL-16-01, architecture basis.
> - `docs/CONTRACT.md` - project invariants, including data boundary, professional boundary, and agent non-invention constraints.
> - `docs/TYPES.md` - canonical object registry and schema boundary notes.
> - `docs/SPEC.md` - technical specification slices for domain objects, persistence, viewport command intents, and professional boundaries.
> - `docs/IP_AND_DATA_BOUNDARY.md` - public/private/protected-content boundary.
> - `Dependencies.csv` - approved local dependency mirror/evidence surface, read only for this pass.
> - `schemas/model_operation.schema.json` - implemented schema evidence.
> - `fixtures/model_operations/invented_operation_set_valid.json` and `fixtures/model_operations/invented_accepted_model_state.json` - invented fixture evidence.
> - `tests/test_model_operation_schema.py` and `tests/test_operation_validation_preview.py` - focused validation evidence.

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-16-01 Structured model operation schema

> #### Specification: DEL-16-01 Structured model operation schema
>

### CLM-008 — Scope

> ##### Scope
>
> This deliverable covers the data-model contract for structured model operations used by GUI and agent proposal workflows. The current implementation evidence is `schemas/model_operation.schema.json` plus invented fixtures under `fixtures/model_operations/`.
>
> In scope:
>
> - A JSON Schema 2020-12 envelope for proposed model operation sets.
> - Structured operation records for add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge edits.
> - Operation/change fields that preserve target references, preconditions, proposed changes, validation state placeholders, diagnostics, diff-preview references, assumptions, provenance, unit metadata, model-basis hashes, and professional-boundary flags.
> - Invented public fixture coverage for the operation kinds and change kinds established by the schema and focused tests.
>
> Out of scope:
>
> - The operation validator, deterministic diff preview, and invalid-operation blocking behavior owned by DEL-16-02.
> - User acceptance and audit trail implementation owned by DEL-16-03.
> - Agent rationale and professional-boundary workflow controls owned by DEL-16-04, except for schema fields needed to preserve the boundary.
> - Exact persistence package/container granularity, broad hash partitioning beyond current schema hooks, GUI runtime behavior, lifecycle promotion, review-finding dispositions, and professional/code-compliance conclusions.
>
> Sources: `_CONTEXT.md`; `schemas/model_operation.schema.json`; `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`; `Dependencies.csv` downstream rows for DEL-16-02 and DEL-16-03.
>

### CLM-009 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-16-01-R001 | The schema shall represent GUI and agent edits as structured model operations before they can alter accepted model state. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `_CONTEXT.md`; `schemas/model_operation.schema.json` `$defs.OperationContractStatus` | `tests/test_model_operation_schema.py` checks `mutation_route = structured_operations_only` and `direct_model_mutation_allowed = false`; `tests/test_operation_validation_preview.py` checks preview generation without mutating accepted state. |
> | DEL-16-01-R002 | The schema shall cover the operation kinds `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`. | `_CONTEXT.md` Description; `schemas/model_operation.schema.json` `$defs.OperationKind` | `tests/test_model_operation_schema.py` checks the enum and fixture coverage. |
> | DEL-16-01-R003 | Operation records shall preserve enough structure to support schema validation, constraint validation, diff preview, and controlled application by downstream services. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `schemas/model_operation.schema.json` `$defs.OperationValidation`, `$defs.Diagnostic`, `$defs.DiffPreviewRef` | Schema review; `tests/test_operation_validation_preview.py` is adjacent DEL-16-02 behavior evidence, not DEL-16-01 ownership. |
> | DEL-16-01-R004 | Operation records shall not encode autonomous engineering acceptance, certification, sealing, authentication, approval, or code-compliance claims. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4; `schemas/model_operation.schema.json` `$defs.ProfessionalBoundary` | `tests/test_model_operation_schema.py` checks negative professional-boundary fields and forbidden status/text sets; `tests/test_operation_validation_preview.py` checks output boundary language. |
> | DEL-16-01-R005 | The public schema and fixtures shall not include protected standards text, protected tables, proprietary catalog values, private project data, or engineering defaults. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1; `docs/IP_AND_DATA_BOUNDARY.md`; fixture provenance fields | Fixture inspection; fixture provenance uses invented public example status. This is evidence only, not legal/professional approval. |
> | DEL-16-01-R006 | The schema shall align with JSON Schema 2020-12 as the accepted public schema/interchange baseline. | `schemas/model_operation.schema.json` root `$schema`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/TYPES.md` section 8 | `tests/test_model_operation_schema.py` validates the schema document and asserts the `$schema` URI. |
> | DEL-16-01-R007 | Operation records shall reference canonical model/domain objects rather than redefine full physical model structure. | `docs/SPEC.md` sections 1 and 3; `schemas/model_operation.schema.json` `$defs.Reference`, `$defs.OperationSet`, `$defs.OperationPrecondition` | Schema review for reference fields; tests assert model-basis and target/precondition reference structure. |
> | DEL-16-01-R008 | Operation records shall expose diagnostics/provenance boundary hooks without silently defaulting missing engineering inputs. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3; `schemas/model_operation.schema.json` `$defs.Diagnostic`, `$defs.Provenance`, `$defs.Quantity`, `$defs.UnitRequirements` | `tests/test_model_operation_schema.py` checks no `default` keys, provenance requirements, unit requirements, and canonical quantity dimensions. |
> | DEL-16-01-R009 | Operation records shall preserve proposal status and downstream review/application boundaries. | `schemas/model_operation.schema.json` `$defs.OperationStatus`, `$defs.OperationValidation`, `$defs.OperationContractStatus` | Tests verify allowed statuses include `proposed`, `schema_validated`, `blocked_by_diagnostics`, `ready_for_user_review`, and `rejected`, while forbidden authority statuses are absent. Exact human disposition workflow remains DEL-16-03/TBD. |
> | DEL-16-01-R010 | Operation sets shall bind operations to a model basis with accepted model-state reference and accepted model-state hash. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis`; `fixtures/model_operations/invented_accepted_model_state.json` | `tests/test_model_operation_schema.py` checks required model-basis fields and per-operation `required_current_hashes` containing `payload_scope = model_state_record`. Broader hash partitioning remains TBD. |
>

### CLM-010 — Standards

> ##### Standards
>
> | Standard or Control | Applicability |
> |---|---|
> | JSON Schema 2020-12 | Governing schema/interchange baseline for `schemas/model_operation.schema.json`. |
> | OPS contract invariants | Binding project controls for source fidelity, IP/data boundary, unit/provenance behavior, professional boundary, and agent non-invention. |
> | Protected-content and private-data policy | Applies to public fixtures and schema defaults; current fixtures are marked invented public examples. |
> | Engineering design codes | Not directly used by this deliverable; no code clause text, standards tables, or engineering compliance conclusions are asserted. |
>

### CLM-011 — Verification

> ##### Verification
>
> | Verification Item | Method | Current Evidence |
> |---|---|---|
> | Schema parses and validates as JSON Schema 2020-12 | Focused schema validation in test helper | `python3 tests/test_model_operation_schema.py` |
> | Operation categories represented | Schema enum and invented operation fixture coverage | `tests/test_model_operation_schema.py` asserts required operation kinds are present in schema and fixture. |
> | Change categories represented | Schema enum and invented operation fixture coverage | `tests/test_model_operation_schema.py` asserts required change kinds are present. |
> | No direct mutation route | Contract status and adjacent preview behavior | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks preview without mutating accepted state and rejects direct mutation request. |
> | Unit metadata and dimensions | Schema unit requirements and quantity dimension enum | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks missing unit metadata and unknown dimension block preview. |
> | Model-basis hash binding | Accepted state fixture and operation precondition hashes | `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks hash mismatch blocks preview. |
> | Professional-boundary terms absent or constrained | Schema and fixture checks | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`. |
> | Cross-deliverable validation handoff | Read-only adjacent DEL-16-02 test evidence | `tests/test_operation_validation_preview.py`; behavior remains owned by DEL-16-02. |
>

### CLM-012 — Documentation

> ##### Documentation
>
> Current implementation/evidence records:
>
> - `schemas/model_operation.schema.json`
> - `fixtures/model_operations/invented_operation_set_valid.json`
> - `fixtures/model_operations/invented_accepted_model_state.json`
> - `tests/test_model_operation_schema.py`
> - `tests/test_operation_validation_preview.py` as adjacent downstream behavior evidence
>
> Residual TBDs:
>
> - exact persistence granularity and broader hash partitioning beyond implemented model-state hash hooks;
> - complete operation granularity for future physical model edit classes beyond current enum/fixture coverage;
> - validation/diff-preview behavior owned by DEL-16-02;
> - user acceptance, audit trail, and human review dispositions owned by DEL-16-03 and review governance;
> - agent rationale/professional-boundary workflow behavior owned by DEL-16-04.

- **AC-001** — The contract preserves the JSON Schema 2020-12 envelope, all declared operation and change categories, structured-only mutation route, target and precondition references, accepted-model-state hashes, unit and dimension requirements, diagnostics, diff-preview and downstream acceptance hooks, assumptions, provenance, invented-public fixture boundaries, professional non-authority, sibling ownership limits, and unresolved persistence, hash-partitioning, operation-granularity, autonomy, and human-disposition questions without creating direct mutation or acceptance.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-16-01 Structured model operation schema

> #### Procedure: DEL-16-01 Structured model operation schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-014 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-16-01-DECL-004`.
>

### CLM-015 — Purpose

> ##### Purpose
>
> Provide the bounded procedure for maintaining and verifying the structured model operation schema. This procedure records current implementation evidence and preserves downstream ownership boundaries.
>

### CLM-016 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source / Status |
> |---|---|
> | Deliverable context and decomposition basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
> | Scope and objective mapping | `_CONTEXT.md` rows `SOW-069` and `OBJ-015`; schema root constants |
> | Architecture basis constraints | `_CONTEXT.md` "Architecture Basis Injection"; JSON Schema 2020-12 and canonical hash-basis notes |
> | Approved dependency mirror | `Dependencies.csv`; preserve active rows unless a later approved workflow says otherwise |
> | Schema artifact | Implemented at `schemas/model_operation.schema.json` |
> | Operation fixture | Implemented at `fixtures/model_operations/invented_operation_set_valid.json` |
> | Accepted model-state fixture | Implemented at `fixtures/model_operations/invented_accepted_model_state.json` |
> | Focused DEL-16-01 validation | `tests/test_model_operation_schema.py` |
> | Adjacent downstream validation evidence | `tests/test_operation_validation_preview.py`; behavior owned by DEL-16-02 |
>

### CLM-017 — Steps

> ##### Steps
>
> 1. Confirm the operation schema remains scoped to DEL-16-01 and SOW-069 by checking root constants for `deliverable_id`, `package_id`, `scope_item`, and `objectives`.
> 2. Preserve JSON Schema 2020-12 as the public schema/interchange baseline.
> 3. Preserve the schema-first operation envelope: `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set`.
> 4. Preserve required operation-set structure: project reference, model reference, `model_basis`, operations, diagnostics, provenance, and professional boundary.
> 5. Preserve model-basis hooks for physical source-of-truth model role, accepted model-state reference, and accepted model-state hash.
> 6. Preserve operation record structure for operation identity/kind/status/author, target references, preconditions, changes, validation state, diagnostics, diff-preview refs, assumptions, provenance, and professional boundary.
> 7. Preserve the declared operation kinds: `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`.
> 8. Preserve the declared change kinds: `add_object`, `remove_object`, `set_field`, `move_geometry`, `reconnect`, `update_constraint`, `update_load`, `update_support`, and `attach_design_knowledge`.
> 9. Keep operation data separate from direct persisted mutation. The schema contract status must retain `structured_operations_only` and `direct_model_mutation_allowed = false`.
> 10. Keep unit-bearing payload behavior explicit: unit metadata required, dimension check required, and missing unit behavior `emit_diagnostic`.
> 11. Keep public fixtures invented and schema-focused. Do not embed protected standards text, protected numeric tables, proprietary catalog values, private project data, or engineering defaults.
> 12. Keep downstream behavior boundaries explicit: validation/diff preview in DEL-16-02, acceptance/audit trail in DEL-16-03, and agent rationale/professional-boundary workflow in DEL-16-04.
> 13. Record unresolved issues for exact persistence granularity, broader hash partitioning, future operation granularity, and human review dispositions.
>

### CLM-018 — Verification

> ##### Verification
>
> | Check | Expected Result |
> |---|---|
> | Schema validation check | `python3 tests/test_model_operation_schema.py` exits 0. |
> | Adjacent validation-preview check | `python3 tests/test_operation_validation_preview.py` exits 0; result is downstream behavior evidence only. |
> | Category fixture check | The invented operation fixture covers all declared operation kinds and change kinds. |
> | Boundary check | Operation records do not claim certification, sealing, approval, authentication, professional approval, engineering acceptance, or code compliance. |
> | Mutation-route check | Contract status disallows direct model mutation; adjacent preview behavior does not mutate accepted state. |
> | Source-fidelity check | Requirements trace to `_CONTEXT.md`, decomposition, governance docs, schema, fixtures, tests, or the approved local dependency mirror. Unsupported details remain TBD or ASSUMPTION. |
> | Dependency mirror check | Existing local dependency rows remain ACTIVE and are not reclassified by this procedure. |
>

### CLM-019 — Records

> ##### Records
>
> Current records for this deliverable/evidence slice:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `MEMORY.md`
> - `schemas/model_operation.schema.json`
> - `fixtures/model_operations/invented_operation_set_valid.json`
> - `fixtures/model_operations/invented_accepted_model_state.json`
> - `tests/test_model_operation_schema.py`
> - `tests/test_operation_validation_preview.py`
> - `_run_records/TASK_RUN_*.md`
>
> Records not changed by this procedure:
>
> - `_STATUS.md`
> - `_REVIEW.md`
> - `Review_Findings.csv`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - schemas, fixtures, core code, and tests

- **VER-001** — Validate the contract and review source parity, complete operation/change taxonomy and record fields, structured-only mutation route, model-basis/current-hash and unit boundaries, diagnostics/provenance and invented-fixture posture, sibling ownership separations, retained CT-001 through CT-003 and other TBDs, and absence of hidden mutation, protected content, or professional-authority claims.

## Governing Values and Decisions — Axiology

### CLM-020 — Guidance: DEL-16-01 Structured model operation schema

> #### Guidance: DEL-16-01 Structured model operation schema
>

### CLM-021 — Purpose

> ##### Purpose
>
> This deliverable makes model edits explicit, reviewable, and controllable before they change accepted model state. The current schema evidence is `schemas/model_operation.schema.json`; current invented fixtures are under `fixtures/model_operations/`.
>
> Sources: `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069, OBJ-015, PKG-16; `_CONTEXT.md`; `schemas/model_operation.schema.json`; `tests/test_model_operation_schema.py`.
>

### CLM-022 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Operation records are proposals until downstream controls run | Treat GUI and agent edits as operation data, not as direct persisted-project mutation. The schema contract sets `direct_model_mutation_allowed = false`. |
> | Use the implemented envelope | Current operation documents use `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set` at the root. |
> | Use the implemented operation record structure | Current records include operation identity/kind/status/author, target references, preconditions, changes, validation, diagnostics, diff-preview refs, assumptions, provenance, and professional-boundary fields. |
> | Preserve source boundaries | Public fixtures and examples must remain invented or otherwise cleared, with provenance, redistribution, review, and privacy-classification fields. |
> | Prefer references over duplicate model structure | Use `Reference` objects and model-basis/precondition hashes rather than embedding the full canonical physical model in operation records. |
> | Expose missing inputs | Missing unit metadata or other solve/rule-required information should surface as diagnostics or blocked validation, never hidden defaults. |
> | Keep validation ownership separate | DEL-16-01 defines schema structure. DEL-16-02 owns validator and diff-preview behavior even when its tests consume DEL-16-01 fixtures. |
> | Keep audit and acceptance ownership separate | DEL-16-03 owns acceptance and operation audit trail behavior. DEL-16-01 records boundary hooks only. |
>
> Sources: `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-AGENT-1, OPS-K-AGENT-4; `schemas/model_operation.schema.json`; `tests/test_operation_validation_preview.py`.
>

### CLM-023 — Considerations

> ##### Considerations
>
> - The current schema establishes the concrete path `schemas/model_operation.schema.json`, root envelope fields, operation/change enums, reference/precondition/hash hooks, provenance requirements, validation placeholders, unit requirements, and professional-boundary flags.
> - The fixture `fixtures/model_operations/invented_operation_set_valid.json` covers the declared operation kinds and change kinds using invented public example data.
> - The fixture `fixtures/model_operations/invented_accepted_model_state.json` provides the invented physical source-of-truth model state and `sha256:invented-state-001` hash referenced by operation-set model basis and preconditions.
> - Exact persistence granularity, physical project package/container behavior, and broad hash partitioning remain TBD beyond the current model-state hash hooks.
> - Runtime validation, deterministic diff preview, blocking behavior, and controlled application remain DEL-16-02 behavior. `tests/test_operation_validation_preview.py` is useful adjacent evidence but does not transfer ownership to DEL-16-01.
> - User acceptance, audit trail records, and human review dispositions remain DEL-16-03/review-governance behavior. Existing `Review_Findings.csv` human dispositions are not changed by this evidence-alignment pass.
> - Agent operation autonomy remains bounded by proposal status, downstream user acceptance, and professional-boundary controls.
>

### CLM-024 — Trade-offs

> ##### Trade-offs
>
> | Topic | Current Position |
> |---|---|
> | Broad operation enum vs specialized records | The implemented schema uses an `OperationKind` enum plus structured `OperationChange` records. More granular future edit classes remain TBD. |
> | Patch-like payload vs command-like payload | The implemented schema supports `value_kind` values including `structured_patch`, `quantity`, `reference_list`, and `text`; it does not settle a universal patch grammar. |
> | Embedding snapshots vs referencing targets | Current schema uses target references, model-basis references, accepted model-state hash, and required current hashes rather than embedding full model snapshots. |
> | Agent-authored operations | Agent authoring is represented by `author_type = agent`, but records remain proposals and carry required professional-boundary negatives. |
> | Fixtures | Current fixtures are invented public examples with provenance and privacy classification. They are schema evidence, not engineering defaults. |
>

### CLM-025 — Examples

> ##### Examples
>
> Current example evidence is the invented operation set at `fixtures/model_operations/invented_operation_set_valid.json`. It includes:
>
> - `op:add-node` using `operation_kind = add` and `change_kind = add_object`;
> - `op:resize` using `operation_kind = modify` and `change_kind = set_field`;
> - `op:move-node` using `operation_kind = move` and `change_kind = move_geometry`;
> - `op:delete-load` using `operation_kind = delete` and `change_kind = remove_object`;
> - `op:reconnect-component` using `operation_kind = reconnect` and `change_kind = reconnect`;
> - `op:update-constraint`, `op:update-load`, `op:update-support`, and `op:attach-design-knowledge` for the remaining declared categories.
>
> These examples remain invented fixture evidence only. They must not be treated as professional approval, code compliance, or reusable engineering defaults.
>

### CLM-026 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | CT-001 | Operation granularity is required by SOW-069, but exact future edit-class granularity is not fully settled beyond the current schema enums. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `schemas/model_operation.schema.json` `$defs.OperationKind` and `$defs.OperationChange` | Specification Requirements; Procedure Steps | Treat current enums/fixtures as implemented DEL-16-01 evidence; keep broader operation granularity TBD. | TBD |
> | CT-002 | Agent edits are in scope, but autonomy level is unresolved beyond proposal and human-review boundaries. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `schemas/model_operation.schema.json` `OperationAuthorType`, `OperationContractStatus`, and `ProfessionalBoundary` | Specification Requirements; Procedure Verification | Allow `author_type = agent` operation proposals only; preserve downstream acceptance/audit and professional-boundary controls. | TBD |
> | CT-003 | Hash binding is implemented for accepted model state, but broader persistence/hash partitioning remains unsettled. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis` and `$defs.OperationPrecondition` | `_CONTEXT.md` Architecture Basis Injection still lists physical project package/container as TBD | Datasheet Conditions; Specification R010; Procedure Verification | Record current model-state hash hooks as fact; keep exact persistence granularity and broader hash partitioning TBD. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
