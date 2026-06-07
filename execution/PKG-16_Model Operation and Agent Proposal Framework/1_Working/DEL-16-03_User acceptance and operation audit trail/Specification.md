# Specification: DEL-16-03 User acceptance and operation audit trail

## Scope

DEL-16-03 covers the backend feature slice for recording accepted/rejected model operations, affected entities, actor/source metadata, timestamps, assumptions, operation history, rationale, and audit metadata needed for reproducible model-state review.

The scope is bounded to SOW-069 and SOW-070 and supports OBJ-015. It does not implement hidden model mutation, autonomous engineering acceptance, professional approval, certification, sealing, or code-compliance claims. Exact audit-log schema, persistence details, and autonomy policy beyond the default user-acceptance posture remain TBD until a sealed implementation brief resolves them.

Current implementation evidence establishes a narrow deterministic audit-record constructor at `core/model_operations/audit_trail/engine.py`. It returns an audit payload for structured operation inputs and does not persist the payload, apply operations, or mutate accepted model state. The current upstream operation schema, validation/preview engine, fixtures, and focused tests are the evidence basis for this specification update.

## Requirements

| ID | Requirement | Source | Verification approach |
|---|---|---|---|
| DEL-16-03-REQ-001 | GUI and agent edits that reach this surface shall be represented as structured model operations before controlled application. | SOW-069 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `docs/_Registers/ScopeLedger.csv` row SOW-069 | Acceptance workflow tests confirm only structured operation inputs are accepted by the audit workflow. |
| DEL-16-03-REQ-002 | The acceptance workflow shall account for schema validation, constraint validation, unit validation, diff preview, and non-application status before an audit record may be marked accepted. | SOW-069 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py` | `tests/test_operation_audit_trail.py` asserts blocked validation prevents accepted records; `tests/test_operation_validation_preview.py` asserts invalid schema/unit/constraint/direct-mutation paths block preview and keep `application_status` as `not_applied`. |
| DEL-16-03-REQ-003 | The audit trail shall record accepted, rejected, and held-for-user-acceptance operation outcomes. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7 PKG-16; `core/model_operations/audit_trail/engine.py` | `tests/test_operation_audit_trail.py` asserts accepted, rejected, and held-for-user-acceptance dispositions and decision counts. |
| DEL-16-03-REQ-004 | The audit trail shall record affected entities, actor/source metadata, timestamps, assumptions, validation outcomes, diff-preview references, operation history, audit metadata, and explicit visible `TBD` diagnostics for missing inputs. | `_CONTEXT.md`; `core/model_operations/audit_trail/engine.py` | `tests/test_operation_audit_trail.py` asserts affected entities, actor/source metadata, unresolved assumptions, validation outcome, diff-preview ref, and visible missing-input diagnostics. |
| DEL-16-03-REQ-005 | Accepted model-operation audit records shall preserve operation history, rationale, assumptions, affected entities, audit metadata, accepted model-state references, and hash evidence needed for reproducible model-state review. | SOW-070 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `docs/_Registers/ScopeLedger.csv` row SOW-070; `core/model_operations/audit_trail/engine.py` | Focused tests confirm accepted records retain review metadata, diff-preview evidence, model-state hash references, and stable record hashes. |
| DEL-16-03-REQ-006 | The default workflow shall require explicit user acceptance before a record can be marked accepted. | `_CONTEXT.md`; OI-016 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 12; `core/model_operations/audit_trail/engine.py` | `tests/test_operation_audit_trail.py` covers that agent-only or missing acceptance signals produce `held_for_user_acceptance` with `AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED`. |
| DEL-16-03-REQ-007 | The audit trail shall not represent operation acceptance as professional approval, certification, sealing, or code compliance. | `docs/DIRECTIVE.md` section 3; `INIT.md`; `docs/TYPES.md` section 9; SOW-070 note | Tests or review checks assert terminology remains development/audit oriented and does not create professional reliance claims. |
| DEL-16-03-REQ-008 | Public examples and records shall not introduce protected standards text, code-specific values, proprietary project data, or private engineering data by default. | `docs/DIRECTIVE.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` sections 3-5; `docs/CONTRACT.md` OPS-K-PRIV and OPS-K-AGENT invariants | Protected-content/provenance review gate for fixtures and sample audit records. |
| DEL-16-03-REQ-009 | Missing data and assumptions shall be visible rather than silently defaulted. | `docs/DIRECTIVE.md` section 3; `docs/CONTRACT.md` OPS-K-AGENT-1; `docs/SPEC.md` section 12 | Tests verify unresolved assumptions and missing required inputs are recorded or surfaced as TBD. |
| DEL-16-03-REQ-010 | Accepted audit records shall require a hash-bound diff-preview reference and a current accepted model-state hash that matches the operation current-hash precondition. | `core/model_operations/audit_trail/engine.py`; `schemas/model_operation.schema.json`; `fixtures/model_operations/invented_accepted_model_state.json` | `tests/test_operation_audit_trail.py` asserts missing model-state hash prevents accepted status; the audit engine also emits blocking diagnostics for missing or mismatched operation current-hash preconditions and missing diff-preview hash. |
| DEL-16-03-REQ-011 | Audit recording shall not mutate accepted model state or apply the operation. | `_CONTEXT.md`; package exclusion for hidden mutation; `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py` | `tests/test_operation_audit_trail.py` asserts rejected records do not mutate accepted state; `tests/test_operation_validation_preview.py` asserts validation preview does not mutate accepted state and application status remains `not_applied`. |
| DEL-16-03-REQ-012 | Rejected operations shall be recorded as audit-only rejected records with accepted-state mutation set false. | `_CONTEXT.md`; `core/model_operations/audit_trail/engine.py` | `tests/test_operation_audit_trail.py` asserts rejected decision counts, rejected status, unchanged accepted state, and `accepted_model_state_mutated` false. |

## Standards

No external piping code, protected standard, or engineering acceptance standard text is locally accessible for this deliverable. Governing standards for this setup pass are the project governance documents and accepted software decomposition listed in `_REFERENCES.md`.

Architecture basis in `_CONTEXT.md` records JSON Schema 2020-12 contracts and canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. Current upstream operation records are governed by `schemas/model_operation.schema.json`, and the current audit payload uses stable JSON serialization plus SHA-256 hashes for record and audit-trail identity. A standalone persisted DEL-16-03 audit-log schema and persistence mechanism remain TBD.

## Verification

| Verification target | Required evidence |
|---|---|
| Scope alignment | DEL-16-03 ID, PKG-16 package, SOW-069/SOW-070 scope, and OBJ-015 objective match `_CONTEXT.md`, registers, and decomposition. |
| Audit-log content | Operation audit log records disposition, affected entities, actor/source metadata, timestamp, assumptions, rationale/history where accepted, and audit metadata. |
| Acceptance workflow | Tests cover accepted and rejected model-operation paths and demonstrate that default user acceptance gates proposed operations. |
| Boundary language | Review confirms no professional approval, certification, sealing, or code-compliance claim is introduced. |
| Data boundary | Review confirms no protected standards text, proprietary project data, or private engineering data appears in public fixtures or examples. |
| State nonmutation | Tests confirm audit recording and validation preview leave accepted model state unchanged and do not apply operations. |
| Hash binding | Tests and code inspection confirm accepted audit records require current accepted-state hash evidence and hash-bound preview evidence. |

## Documentation

Expected artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:

- operation audit log
- acceptance workflow tests

Current implementation and test evidence:

- `core/model_operations/audit_trail/engine.py`
- `tests/test_operation_audit_trail.py`
- `core/model_operations/validation_preview/engine.py`
- `tests/test_operation_validation_preview.py`
- `schemas/model_operation.schema.json`
- `tests/test_model_operation_schema.py`
- `fixtures/model_operations/invented_operation_set_valid.json`
- `fixtures/model_operations/invented_accepted_model_state.json`

Durable persistence container, long-term retention policy, final actor identity model beyond current fields, timestamp precision policy beyond fixture evidence, operation application outside this slice, and human review dispositions remain TBD.
