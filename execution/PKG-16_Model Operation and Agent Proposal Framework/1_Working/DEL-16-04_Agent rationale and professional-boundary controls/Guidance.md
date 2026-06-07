# Guidance: DEL-16-04 Agent rationale and professional-boundary controls

## Purpose

This deliverable exists to keep agent-assisted model operation workflows auditable without allowing agent output to become accepted engineering work by itself. The source basis is DEL-16-04, SOW-070, OBJ-015, OBJ-018, and the project authority boundaries in `docs/CONTRACT.md`, `docs/TYPES.md`, and `docs/SPEC.md`.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Proposal is not acceptance | Treat agent rationale as proposal/supporting explanation until a human gate accepts the related work. | `docs/CONTRACT.md` OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 |
| Preserve rationale and assumptions | Do not collapse rationale, unresolved assumptions, affected entities, and audit metadata into unstructured text that cannot support reproducible review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-070 |
| Block professional claims | Controls should prevent certification, approval, sealing, authentication, professional reliance, and code-compliance claim language from being generated as software/agent authority. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
| Surface unknowns | Missing or unsupported facts should remain `TBD` or explicit assumptions rather than silent defaults. | `docs/DIRECTIVE.md` section 2.4; `docs/SPEC.md` section 12 |
| Keep protected data out | Rationale and tests must not quote or paraphrase protected standards content, proprietary data, private project data, or private rule-pack payloads into public artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
| Preserve hash-bound human authority | If human acceptance records are referenced, treat them as external, human-owned, and bound to reviewed payload hashes. | `docs/SPEC.md` sections 4.3 and 9 |
| Keep rationale non-applying | Treat `record_agent_rationale()` output as deterministic decision-support metadata only; it must not create accepted operation records, mutate accepted model state, or bypass user acceptance. | `core/model_operations/agent_rationale/engine.py`; `tests/test_agent_rationale_boundary.py` |

## Considerations

| Topic | Consideration |
|---|---|
| Record shape | The current Python record shape is established in `core/model_operations/agent_rationale/engine.py` for this slice: deliverable/package/scope metadata, operation set ref, audit context, source, actor, timestamp, rationale posture, assumptions, validation context, affected entities, audit references, operation context, diagnostics, accepted-state unchanged flag, professional boundary, provenance, rationale ID, and rationale hash. Standalone JSON Schema and final persistence path remain TBD. |
| Guard-test surface | Current focused guards are in `tests/test_agent_rationale_boundary.py`. They cover deterministic records, no accepted-state mutation, no accepted-operation creation, no user-acceptance bypass, missing-context/TBD visibility, unresolved assumptions, prohibited rationale claims, copied-context claims, enum-style authority tokens, and lowercase approved coordination-context false-positive behavior. Broader UI/API/report coverage remains TBD. |
| Dependency context | The approved DAG-006 mirror identifies architecture-basis deliverables, professional responsibility policy, user acceptance/audit trail, and security threat model as upstream context. This setup pass preserves that mirror rather than reclassifying it. |
| Rationale privacy | Rationale may mention project context or user inputs. Future implementation must respect private-data and protected-content controls before public export. |
| Human review | Human review is a boundary, not a wording detail. A UI label or report phrase must not imply human acceptance unless backed by an external, human-owned, hash-bound record. |
| Copied context | Rationale capture scans rationale text plus copied operation, audit, validation, source, actor, affected-reference, and audit-reference context for prohibited authority language. Blocking diagnostics should be treated as wording/authority blockers, not engineering findings. |

## Trade-offs

| Trade-off | Conservative Direction |
|---|---|
| Detailed rationale vs. private-data exposure | Preserve enough rationale for reproducible model-state review, but redact or keep private any protected, proprietary, or project-private content. |
| Strict prohibited-claim filters vs. false positives | Prefer blocking or flagging ambiguous professional/code-compliance language until a human resolves the wording. |
| Flexible notes vs. structured review | Allow notes only when structured fields preserve operation history, assumptions, affected entities, audit metadata, and review status. |
| Human acceptance references vs. software statuses | Keep human acceptance external and hash-bound; do not convert it into an automatic software status. |

## Examples

Source-backed vocabulary examples:

| Category | Examples from Sources |
|---|---|
| Permitted automatic software statuses | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED` (`docs/TYPES.md` section 4). |
| Prohibited automatic status or claim language | `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED`, or equivalent professional/code-compliance language (`docs/TYPES.md` section 4). |
| Required epistemic labels | `FACT`, `ASSUMPTION`, `PROPOSAL`, `TBD` (`docs/TYPES.md` section 5). |
| Current rationale blocking diagnostics | `RATIONALE-AUTHORITY-COMPLIANCE-BLOCKED`, `RATIONALE-AUTHORITY-CERTIFICATION-BLOCKED`, `RATIONALE-AUTHORITY-SEALING-BLOCKED`, `RATIONALE-AUTHORITY-AUTHENTICATION-BLOCKED`, `RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED`, `RATIONALE-AUTHORITY-EXTERNAL-VALIDATION-BLOCKED`, and `RATIONALE-AUTHORITY-AUTONOMOUS-ACCEPTANCE-BLOCKED` from `core/model_operations/agent_rationale/engine.py`. |
| Current visible TBD diagnostics | Missing audit context, validation context, source metadata, actor metadata, rationale text, and timestamp are emitted as `TBD_VISIBLE` diagnostics in `record_agent_rationale()`. |

The authorized implementation/test evidence uses invented fixtures only. No project-specific engineering example, standards clause, acceptance value, or professional/code-compliance conclusion is established by this slice.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No cross-source conflict identified in the slices used for this setup pass. | N/A | N/A | N/A | N/A | TBD |
