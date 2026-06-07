# Specification: DEL-16-01 Structured model operation schema

## Scope

This deliverable covers the data-model contract for structured model operations used by GUI and agent proposal workflows. The current implementation evidence is `schemas/model_operation.schema.json` plus invented fixtures under `fixtures/model_operations/`.

In scope:

- A JSON Schema 2020-12 envelope for proposed model operation sets.
- Structured operation records for add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge edits.
- Operation/change fields that preserve target references, preconditions, proposed changes, validation state placeholders, diagnostics, diff-preview references, assumptions, provenance, unit metadata, model-basis hashes, and professional-boundary flags.
- Invented public fixture coverage for the operation kinds and change kinds established by the schema and focused tests.

Out of scope:

- The operation validator, deterministic diff preview, and invalid-operation blocking behavior owned by DEL-16-02.
- User acceptance and audit trail implementation owned by DEL-16-03.
- Agent rationale and professional-boundary workflow controls owned by DEL-16-04, except for schema fields needed to preserve the boundary.
- Exact persistence package/container granularity, broad hash partitioning beyond current schema hooks, GUI runtime behavior, lifecycle promotion, review-finding dispositions, and professional/code-compliance conclusions.

Sources: `_CONTEXT.md`; `schemas/model_operation.schema.json`; `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`; `Dependencies.csv` downstream rows for DEL-16-02 and DEL-16-03.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-16-01-R001 | The schema shall represent GUI and agent edits as structured model operations before they can alter accepted model state. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `_CONTEXT.md`; `schemas/model_operation.schema.json` `$defs.OperationContractStatus` | `tests/test_model_operation_schema.py` checks `mutation_route = structured_operations_only` and `direct_model_mutation_allowed = false`; `tests/test_operation_validation_preview.py` checks preview generation without mutating accepted state. |
| DEL-16-01-R002 | The schema shall cover the operation kinds `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`. | `_CONTEXT.md` Description; `schemas/model_operation.schema.json` `$defs.OperationKind` | `tests/test_model_operation_schema.py` checks the enum and fixture coverage. |
| DEL-16-01-R003 | Operation records shall preserve enough structure to support schema validation, constraint validation, diff preview, and controlled application by downstream services. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `schemas/model_operation.schema.json` `$defs.OperationValidation`, `$defs.Diagnostic`, `$defs.DiffPreviewRef` | Schema review; `tests/test_operation_validation_preview.py` is adjacent DEL-16-02 behavior evidence, not DEL-16-01 ownership. |
| DEL-16-01-R004 | Operation records shall not encode autonomous engineering acceptance, certification, sealing, authentication, approval, or code-compliance claims. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4; `schemas/model_operation.schema.json` `$defs.ProfessionalBoundary` | `tests/test_model_operation_schema.py` checks negative professional-boundary fields and forbidden status/text sets; `tests/test_operation_validation_preview.py` checks output boundary language. |
| DEL-16-01-R005 | The public schema and fixtures shall not include protected standards text, protected tables, proprietary catalog values, private project data, or engineering defaults. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1; `docs/IP_AND_DATA_BOUNDARY.md`; fixture provenance fields | Fixture inspection; fixture provenance uses invented public example status. This is evidence only, not legal/professional approval. |
| DEL-16-01-R006 | The schema shall align with JSON Schema 2020-12 as the accepted public schema/interchange baseline. | `schemas/model_operation.schema.json` root `$schema`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/TYPES.md` section 8 | `tests/test_model_operation_schema.py` validates the schema document and asserts the `$schema` URI. |
| DEL-16-01-R007 | Operation records shall reference canonical model/domain objects rather than redefine full physical model structure. | `docs/SPEC.md` sections 1 and 3; `schemas/model_operation.schema.json` `$defs.Reference`, `$defs.OperationSet`, `$defs.OperationPrecondition` | Schema review for reference fields; tests assert model-basis and target/precondition reference structure. |
| DEL-16-01-R008 | Operation records shall expose diagnostics/provenance boundary hooks without silently defaulting missing engineering inputs. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3; `schemas/model_operation.schema.json` `$defs.Diagnostic`, `$defs.Provenance`, `$defs.Quantity`, `$defs.UnitRequirements` | `tests/test_model_operation_schema.py` checks no `default` keys, provenance requirements, unit requirements, and canonical quantity dimensions. |
| DEL-16-01-R009 | Operation records shall preserve proposal status and downstream review/application boundaries. | `schemas/model_operation.schema.json` `$defs.OperationStatus`, `$defs.OperationValidation`, `$defs.OperationContractStatus` | Tests verify allowed statuses include `proposed`, `schema_validated`, `blocked_by_diagnostics`, `ready_for_user_review`, and `rejected`, while forbidden authority statuses are absent. Exact human disposition workflow remains DEL-16-03/TBD. |
| DEL-16-01-R010 | Operation sets shall bind operations to a model basis with accepted model-state reference and accepted model-state hash. | `schemas/model_operation.schema.json` `$defs.OperationModelBasis`; `fixtures/model_operations/invented_accepted_model_state.json` | `tests/test_model_operation_schema.py` checks required model-basis fields and per-operation `required_current_hashes` containing `payload_scope = model_state_record`. Broader hash partitioning remains TBD. |

## Standards

| Standard or Control | Applicability |
|---|---|
| JSON Schema 2020-12 | Governing schema/interchange baseline for `schemas/model_operation.schema.json`. |
| OPS contract invariants | Binding project controls for source fidelity, IP/data boundary, unit/provenance behavior, professional boundary, and agent non-invention. |
| Protected-content and private-data policy | Applies to public fixtures and schema defaults; current fixtures are marked invented public examples. |
| Engineering design codes | Not directly used by this deliverable; no code clause text, standards tables, or engineering compliance conclusions are asserted. |

## Verification

| Verification Item | Method | Current Evidence |
|---|---|---|
| Schema parses and validates as JSON Schema 2020-12 | Focused schema validation in test helper | `python3 tests/test_model_operation_schema.py` |
| Operation categories represented | Schema enum and invented operation fixture coverage | `tests/test_model_operation_schema.py` asserts required operation kinds are present in schema and fixture. |
| Change categories represented | Schema enum and invented operation fixture coverage | `tests/test_model_operation_schema.py` asserts required change kinds are present. |
| No direct mutation route | Contract status and adjacent preview behavior | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks preview without mutating accepted state and rejects direct mutation request. |
| Unit metadata and dimensions | Schema unit requirements and quantity dimension enum | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks missing unit metadata and unknown dimension block preview. |
| Model-basis hash binding | Accepted state fixture and operation precondition hashes | `fixtures/model_operations/invented_accepted_model_state.json`; `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py` checks hash mismatch blocks preview. |
| Professional-boundary terms absent or constrained | Schema and fixture checks | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py`. |
| Cross-deliverable validation handoff | Read-only adjacent DEL-16-02 test evidence | `tests/test_operation_validation_preview.py`; behavior remains owned by DEL-16-02. |

## Documentation

Current implementation/evidence records:

- `schemas/model_operation.schema.json`
- `fixtures/model_operations/invented_operation_set_valid.json`
- `fixtures/model_operations/invented_accepted_model_state.json`
- `tests/test_model_operation_schema.py`
- `tests/test_operation_validation_preview.py` as adjacent downstream behavior evidence

Residual TBDs:

- exact persistence granularity and broader hash partitioning beyond implemented model-state hash hooks;
- complete operation granularity for future physical model edit classes beyond current enum/fixture coverage;
- validation/diff-preview behavior owned by DEL-16-02;
- user acceptance, audit trail, and human review dispositions owned by DEL-16-03 and review governance;
- agent rationale/professional-boundary workflow behavior owned by DEL-16-04.
