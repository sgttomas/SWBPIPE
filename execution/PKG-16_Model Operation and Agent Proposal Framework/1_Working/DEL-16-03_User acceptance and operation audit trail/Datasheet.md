# Datasheet: DEL-16-03 User acceptance and operation audit trail

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-16-03 | `_CONTEXT.md` |
| Deliverable name | User acceptance and operation audit trail | `_CONTEXT.md` |
| Package ID | PKG-16 | `_CONTEXT.md` |
| Package name | Model Operation and Agent Proposal Framework | `_CONTEXT.md` |
| Deliverable type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-16-03 |
| Scope items | SOW-069, SOW-070 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 and section 7 |
| Objective support | OBJ-015 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-16-03 |

## Attributes

| Attribute | Current value | Source / note |
|---|---|---|
| Intended artifact | Operation audit log | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-16-03 |
| Intended test artifact | Acceptance workflow tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-16-03 |
| Operation dispositions in scope | Accepted and rejected operations | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7 PKG-16 deliverables |
| Required audit subjects | Affected entities, actor/source metadata, timestamps, assumptions | `_CONTEXT.md` |
| Required accepted-operation preservation | Operation history, rationale, assumptions, affected entities, audit metadata | SOW-070 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
| Required pre-application workflow context | Schema validation, constraint validation, diff preview, controlled application through the model engine | SOW-069 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
| Default autonomy posture | User acceptance is required unless later explicitly changed | `_CONTEXT.md`; OI-016 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 12 |
| Professional-boundary posture | Operation history supports design iteration without professional approval claims | SOW-070 note in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `docs/DIRECTIVE.md` section 3 |

## Conditions

| Condition | Status |
|---|---|
| Current audit-trail implementation path | Established for this slice at `core/model_operations/audit_trail/engine.py`. |
| Current validation-preview dependency path | Established read-only dependency at `core/model_operations/validation_preview/engine.py`. |
| Current operation schema and fixture basis | Established upstream evidence at `schemas/model_operation.schema.json`, `fixtures/model_operations/invented_operation_set_valid.json`, and `fixtures/model_operations/invented_accepted_model_state.json`. |
| Explicit user acceptance requirement | Established: accepted records require `acceptance_signal.accepted is True`, `decision == accept`, and `actor_type == user`; agent-only acceptance is held for user acceptance. |
| Blocked validation behavior | Established: accepted records require passed schema, constraint, and unit validation, generated diff preview, and `application_status: not_applied`; blocking validation prevents accepted status. |
| Model-state hash requirement | Established: accepted records require a current accepted model-state hash and matching model-state current-hash precondition. |
| Accepted-state mutation behavior | Established: audit recording does not apply operations or mutate accepted model state; outputs expose `accepted_model_state_unchanged`. |
| Rejected-record behavior | Established: rejected operations are recorded as rejected audit records and set `accepted_model_state_mutated` to `false`. |
| Missing input diagnostics | Established: missing validation outcome, diff-preview reference, user decision, timestamp, and rationale are surfaced as visible `TBD` diagnostics or placeholder values. |
| Deterministic record shape | Established: records are sorted by canonical JSON, hashes are derived from stable JSON serialization, and focused tests compare canonical output stability. |
| Professional/compliance claims | Established: current audit output carries boundary flags and focused tests reject prohibited professional, certification, sealing, authentication, approval, and code-compliance wording. |
| Durable persistence container or storage mechanism | TBD; current implementation returns an in-memory audit payload and does not define long-term storage. |
| Long-term retention policy | TBD; rejected operations are recorded in the current payload, but durable retention duration and disposal rules are not defined. |
| Final actor identity model beyond current fields | TBD; current fields include `actor_type`, `actor_ref`, and `source_role`, but final identity/authentication semantics are not defined. |
| Timestamp precision policy beyond current fixture evidence | TBD; current fixture/test evidence uses ISO-like UTC strings, but project-wide precision/clock policy is not defined. |
| Operation application outside this slice | TBD; this slice records audit decisions and explicitly does not apply operations. |
| Human review dispositions | TBD; review finding disposition fields remain subject to the human disposition gate. |
| Exact dependency versions | TBD per `_CONTEXT.md` architecture basis injection. |
| Protected or proprietary engineering data in public examples | Not permitted by `docs/DIRECTIVE.md` section 3 and `docs/IP_AND_DATA_BOUNDARY.md` sections 3-5. |

## Construction

DEL-16-03 is a backend feature-slice control surface for recording operation acceptance outcomes and audit metadata. The current implementation is the deterministic audit-trail module at `core/model_operations/audit_trail/engine.py`, exercised by `tests/test_operation_audit_trail.py`. It consumes structured operation envelopes, validation/diff-preview evidence, acceptance or rejection signals, actor/source metadata, timestamps, rationale, audit metadata, and accepted model-state references.

The module records accepted, rejected, and held-for-user-acceptance outcomes without applying operations or mutating accepted model state. Accepted audit records require explicit user acceptance, nonblocking validation/preview evidence, hash-bound diff-preview evidence, and a current accepted model-state hash that matches the operation precondition. Missing inputs remain visible as `TBD` diagnostics. Durable persistence, retention policy, final actor identity semantics, timestamp precision policy, operation application, and human review dispositions remain outside the established implementation facts for this slice.

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Dependencies.csv`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `core/model_operations/audit_trail/engine.py`
- `core/model_operations/validation_preview/engine.py`
- `schemas/model_operation.schema.json`
- `fixtures/model_operations/invented_operation_set_valid.json`
- `fixtures/model_operations/invented_accepted_model_state.json`
- `tests/test_operation_audit_trail.py`
- `tests/test_operation_validation_preview.py`
- `tests/test_model_operation_schema.py`
