# Datasheet: DEL-16-04 Agent rationale and professional-boundary controls

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-16-04 | `_CONTEXT.md` |
| Name | Agent rationale and professional-boundary controls | `_CONTEXT.md` |
| Package ID | PKG-16 | `_CONTEXT.md` |
| Package Name | Model Operation and Agent Proposal Framework | `_CONTEXT.md` |
| Type | SECURITY_CONTROL | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
| Scope Coverage | SOW-070 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4 |
| Objective Support | OBJ-015, OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
| Anticipated Artifacts | agent rationale record; professional-boundary guard tests | `_CONTEXT.md` |
| Context Envelope | S | `_CONTEXT.md` |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Deliverable purpose | Capture agent rationale and unresolved assumptions while preventing certification, approval, or code-compliance claims. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7, DEL-16-04 |
| Scope item requirement | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4, SOW-070 |
| Professional boundary | Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance. | `docs/CONTRACT.md` section 1, OPS-K-AUTH-1 |
| Agent authority | Agent outputs are drafts or proposals until accepted by a human gate. | `docs/CONTRACT.md` section 1, OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 |
| Status vocabulary boundary | Automatic software statuses are limited to software findings and must not include human approval, code-compliance, certification, sealing, or approval-equivalent language. | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.3 |
| Missing information handling | Missing data and assumptions must be surfaced; unknown engineering values remain `TBD`. | `INIT.md` root instructions; `docs/SPEC.md` section 12 |
| Protected content posture | Public artifacts must not contain protected standards content, proprietary commercial data, or private project/rule data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |

## Conditions

| Condition | Status |
|---|---|
| This deliverable does not authorize hidden model mutation or autonomous engineering acceptance. | FACT: PKG-16 package exclusion in `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md` section 6. |
| Agent output cannot become accepted engineering work by itself. | FACT: `_CONTEXT.md` Context Envelope Notes and Context Budget QA note. |
| Human acceptance, if represented, is external, human-actor-owned, and bound to reviewed evidence hashes. | FACT: `docs/SPEC.md` sections 4.3 and 9. |
| Current rationale implementation path is established for this slice. | FACT: `core/model_operations/agent_rationale/engine.py` provides `record_agent_rationale()` and `canonical_json()` for DEL-16-04 rationale records. |
| Current focused guard-test path is established for this slice. | FACT: `tests/test_agent_rationale_boundary.py` covers deterministic rationale records, decision-support posture, no accepted-state mutation, visible TBD diagnostics, unresolved assumptions, prohibited-claim blocking, copied-context scanning, enum-style authority tokens, and lowercase approved coordination-context false-positive behavior. |
| No project-specific engineering values, code clauses, code-compliance conclusions, or professional approval evidence are established by this implementation slice. | FACT: implementation and fixtures use invented/public metadata and professional-boundary flags; downstream human review dispositions remain external. |
| Standalone rationale JSON Schema, final UI/agent workflow presentation, broader persistence/application-service behavior, and human review dispositions remain unresolved. | TBD for downstream Type 2 work or human ruling. |

## Construction

| Construct | Conservative Definition |
|---|---|
| Agent rationale record | Current Python implementation builds deterministic DEL-16-04 rationale records from DEL-16-01 operation envelopes plus optional audit trail, validation context, source, actor, rationale text, assumptions, affected references, audit references, timestamp, and accepted-state input. It records rationale as decision-support metadata only and computes stable rationale ID/hash values. Standalone rationale schema remains TBD. |
| Professional-boundary controls | Current implementation sets explicit professional-boundary booleans, prevents rationale from creating accepted operation records, mutating accepted model state, or bypassing user acceptance, and emits blocking diagnostics for prohibited authority claims in rationale or copied operation/audit/validation context. Final UI/API/report presentation remains TBD. |
| Guard tests | Current focused test harness is `tests/test_agent_rationale_boundary.py`; adjacent evidence includes `tests/test_operation_audit_trail.py`, `tests/test_operation_validation_preview.py`, and `tests/test_model_operation_schema.py`. Broader persistence/application behavior remains TBD. |
| Dependency evidence surface | `Dependencies.csv` is an approved DAG-006 mirror with ACTIVE upstream rows for architecture-basis deliverables, professional responsibility policy, user acceptance/audit trail, and security threat model. The mirror is not rewritten by this setup pass. |

## References

| Reference | Used For |
|---|---|
| `_CONTEXT.md` | Deliverable identity, scope, objectives, artifacts, package boundary, architecture basis injection. |
| `_REFERENCES.md` | Reference inventory and source boundary for this setup pass. |
| `_DEPENDENCIES.md`; `Dependencies.csv` | Approved DAG-006 local mirror/evidence surface and upstream context; historical DAG-002 row IDs remain preserved evidence, not current graph authority. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-070, OBJ-015, OBJ-018, PKG-16, DEL-16-04 decomposition context. |
| `docs/CONTRACT.md` | Binding invariants for professional authority, agent authority, no invention, and conflict surfacing. |
| `docs/DIRECTIVE.md` | Founding professional-boundary principles and stop rules. |
| `docs/TYPES.md` | Deliverable type, status vocabulary, and epistemic labels. |
| `docs/SPEC.md` | Analysis-boundary, persistence, report-section, and acceptance semantics relevant to professional-boundary controls. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content, private-data, and contribution boundary. |
| `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` | Type 2 draft/proposal authority and deliverable execution rules. |
| `core/model_operations/agent_rationale/engine.py` | Current implementation evidence for deterministic rationale records and professional-boundary diagnostics. |
| `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py`; `schemas/model_operation.schema.json` | Read-only upstream/adjacent implementation evidence for audit-trail, validation-preview, and operation-schema boundaries consumed by the rationale tests. |
| `tests/test_agent_rationale_boundary.py`; `tests/test_operation_audit_trail.py`; `tests/test_operation_validation_preview.py`; `tests/test_model_operation_schema.py` | Focused validation evidence for this slice and adjacent model-operation controls. |
| `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json` | Invented fixture evidence used by the focused tests; no protected project payload is introduced. |
