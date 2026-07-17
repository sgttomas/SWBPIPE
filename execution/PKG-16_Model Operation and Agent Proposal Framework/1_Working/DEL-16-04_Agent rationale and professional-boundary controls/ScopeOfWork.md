---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-04
package_id: PKG-16
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-070]
package_objective_refs: [OBJ-015, OBJ-018]
---

# Scope of Work — DEL-16-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-16-04` in service of project scope [SOW-070] and package objectives [OBJ-015, OBJ-018].

- **OUT-001** — A deterministic agent-rationale and professional-boundary control contract that preserves review context while keeping agent output decision-support only is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-16-04 Agent rationale and professional-boundary controls

> #### Datasheet: DEL-16-04 Agent rationale and professional-boundary controls
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-16-04 | `_CONTEXT.md` |
> | Name | Agent rationale and professional-boundary controls | `_CONTEXT.md` |
> | Package ID | PKG-16 | `_CONTEXT.md` |
> | Package Name | Model Operation and Agent Proposal Framework | `_CONTEXT.md` |
> | Type | SECURITY_CONTROL | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
> | Scope Coverage | SOW-070 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4 |
> | Objective Support | OBJ-015, OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
> | Anticipated Artifacts | agent rationale record; professional-boundary guard tests | `_CONTEXT.md` |
> | Context Envelope | S | `_CONTEXT.md` |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Deliverable purpose | Capture agent rationale and unresolved assumptions while preventing certification, approval, or code-compliance claims (PRD §21.2). | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7, DEL-16-04 |
> | Scope item requirement | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4, SOW-070 |
> | Professional boundary | Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance (PRD §21.2). | `docs/CONTRACT.md` section 1, OPS-K-AUTH-1 |
> | Agent authority | Agent outputs are drafts or proposals until accepted by a human gate. | `docs/CONTRACT.md` section 1, OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 |
> | Status vocabulary boundary | Automatic software statuses are limited to software findings and must not include human approval, code-compliance, certification, sealing, or approval-equivalent language (PRD §21.2). | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.3 |
> | Missing information handling | Missing data and assumptions must be surfaced; unknown engineering values remain `TBD`. | `INIT.md` root instructions; `docs/SPEC.md` section 12 |
> | Protected content posture | Public artifacts must not contain protected standards content, proprietary commercial data, or private project/rule data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | This deliverable does not authorize hidden model mutation or autonomous engineering acceptance. | FACT: PKG-16 package exclusion in `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md` section 6. |
> | Agent output cannot become accepted engineering work by itself. | FACT: `_CONTEXT.md` Context Envelope Notes and Context Budget QA note. |
> | Human acceptance, if represented, is external, human-actor-owned, and bound to reviewed evidence hashes. | FACT: `docs/SPEC.md` sections 4.3 and 9. |
> | Current rationale implementation path is established for this slice. | FACT: `core/model_operations/agent_rationale/engine.py` provides `record_agent_rationale()` and `canonical_json()` for DEL-16-04 rationale records. |
> | Current focused guard-test path is established for this slice. | FACT: `tests/test_agent_rationale_boundary.py` covers deterministic rationale records, decision-support posture, no accepted-state mutation, visible TBD diagnostics, unresolved assumptions, prohibited-claim blocking, copied-context scanning, enum-style authority tokens, and lowercase approved coordination-context false-positive behavior. |
> | No project-specific engineering values, code clauses, code-compliance conclusions, or professional approval evidence (PRD §21.2) are established by this implementation slice. | FACT: implementation and fixtures use invented/public metadata and professional-boundary flags; downstream human review dispositions remain external. |
> | Standalone rationale JSON Schema, final UI/agent workflow presentation, broader persistence/application-service behavior, and human review dispositions remain unresolved. | TBD for downstream Type 2 work or human ruling. |
>

### CLM-005 — Construction

> ##### Construction
>
> | Construct | Conservative Definition |
> |---|---|
> | Agent rationale record | Current Python implementation builds deterministic DEL-16-04 rationale records from DEL-16-01 operation envelopes plus optional audit trail, validation context, source, actor, rationale text, assumptions, affected references, audit references, timestamp, and accepted-state input. It records rationale as decision-support metadata only and computes stable rationale ID/hash values. Standalone rationale schema remains TBD. |
> | Professional-boundary controls | Current implementation sets explicit professional-boundary booleans, prevents rationale from creating accepted operation records, mutating accepted model state, or bypassing user acceptance, and emits blocking diagnostics for prohibited authority claims in rationale or copied operation/audit/validation context. Final UI/API/report presentation remains TBD. |
> | Guard tests | Current focused test harness is `tests/test_agent_rationale_boundary.py`; adjacent evidence includes `tests/test_operation_audit_trail.py`, `tests/test_operation_validation_preview.py`, and `tests/test_model_operation_schema.py`. Broader persistence/application behavior remains TBD. |
> | Dependency evidence surface | `Dependencies.csv` is an approved DAG-006 mirror with ACTIVE upstream rows for architecture-basis deliverables, professional responsibility policy, user acceptance/audit trail, and security threat model. The mirror is not rewritten by this setup pass. |
>

### CLM-006 — References

> ##### References
>
> | Reference | Used For |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, scope, objectives, artifacts, package boundary, architecture basis injection. |
> | `_REFERENCES.md` | Reference inventory and source boundary for this setup pass. |
> | `_DEPENDENCIES.md`; `Dependencies.csv` | Approved DAG-006 local mirror/evidence surface and upstream context; historical DAG-002 row IDs remain preserved evidence, not current graph authority. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-070, OBJ-015, OBJ-018, PKG-16, DEL-16-04 decomposition context. |
> | `docs/CONTRACT.md` | Binding invariants for professional authority, agent authority, no invention, and conflict surfacing. |
> | `docs/DIRECTIVE.md` | Founding professional-boundary principles and stop rules. |
> | `docs/TYPES.md` | Deliverable type, status vocabulary, and epistemic labels. |
> | `docs/SPEC.md` | Analysis-boundary, persistence, report-section, and acceptance semantics relevant to professional-boundary controls. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content, private-data, and contribution boundary. |
> | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` | Type 2 draft/proposal authority and deliverable execution rules. |
> | `core/model_operations/agent_rationale/engine.py` | Current implementation evidence for deterministic rationale records and professional-boundary diagnostics. |
> | `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py`; `schemas/model_operation.schema.json` | Read-only upstream/adjacent implementation evidence for audit-trail, validation-preview, and operation-schema boundaries consumed by the rationale tests. |
> | `tests/test_agent_rationale_boundary.py`; `tests/test_operation_audit_trail.py`; `tests/test_operation_validation_preview.py`; `tests/test_model_operation_schema.py` | Focused validation evidence for this slice and adjacent model-operation controls. |
> | `fixtures/model_operations/invented_operation_set_valid.json`; `fixtures/model_operations/invented_accepted_model_state.json` | Invented fixture evidence used by the focused tests; no protected project payload is introduced. |

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-16-04 Agent rationale and professional-boundary controls

> #### Specification: DEL-16-04 Agent rationale and professional-boundary controls
>

### CLM-008 — Scope

> ##### Scope
>
> This deliverable covers the security-control specification surface for recording agent rationale and unresolved assumptions while preserving the professional boundary. It is scoped to DEL-16-04 in PKG-16 and SOW-070.
>
> Included:
>
> - rationale and assumption recording expectations for accepted model operations;
> - professional-boundary controls that prevent agent/software claims of certification, approval, authentication, sealing, professional reliance, or code compliance (PRD §21.2);
> - guard-test expectations for that boundary.
>
> Excluded:
>
> - implementation of model-operation schemas owned by DEL-16-01;
> - implementation of validation/diff preview owned by DEL-16-02;
> - implementation of user acceptance and operation audit trail owned by DEL-16-03;
> - professional responsibility/product-claims policy authorship owned by DEL-01-04;
> - standalone rationale JSON Schema, final UI/agent workflow presentation, broader persistence/application-service behavior outside this slice, dependency versions, and human review dispositions, which remain TBD unless assigned by later Type 2 work or human ruling.
>
> Current implementation evidence for this deliverable-local slice is `core/model_operations/agent_rationale/engine.py` with focused tests in `tests/test_agent_rationale_boundary.py`. That implementation consumes DEL-16-01 operation envelopes and optional DEL-16-03 audit / DEL-16-02 validation context as read-only rationale context; it does not implement those sibling deliverables.
>

### CLM-009 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-16-04-01 | The deliverable shall remain bounded to DEL-16-04, PKG-16, SOW-070, OBJ-015, and OBJ-018. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 4-7 | Review document metadata and scope references. |
> | REQ-16-04-02 | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4, SOW-070 | Current `record_agent_rationale()` output preserves rationale text/status, assumptions, validation context, affected entities, audit references, audit context, and operation context; standalone schema remains TBD. |
> | REQ-16-04-03 | Agent outputs shall remain drafts or proposals until accepted by a human gate. | `docs/CONTRACT.md` section 1, OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 | `tests/test_agent_rationale_boundary.py` confirms rationale records are decision-support only, create no accepted operation record, do not bypass user acceptance, and do not mutate accepted model state. |
> | REQ-16-04-04 | Software and agents shall not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance (PRD §21.2). | `docs/CONTRACT.md` section 1, OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 3-5 | `tests/test_agent_rationale_boundary.py` confirms prohibited rationale language emits blocking professional-boundary diagnostics. |
> | REQ-16-04-05 | Automatic status vocabulary shall not include `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED`, or equivalent professional/code-compliance language (PRD §21.2). | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.3 | Current tests confirm enum-style authority tokens in copied context are blocked and lowercase approved coordination context is not treated as professional approval. Adjacent schema tests confirm forbidden operation statuses remain excluded from `model_operation.schema.json`. |
> | REQ-16-04-06 | Any human acceptance record represented by the product shall be external, human-actor-owned, and bound to reviewed payload hashes; it shall not be software-generated professional approval. | `docs/SPEC.md` sections 4.3 and 9 | Current rationale record stores audit context/references as context and sets `rationale_creates_audit_acceptance`/accepted-record creation flags false; final human-disposition workflow remains TBD. |
> | REQ-16-04-07 | Missing data, unresolved assumptions, warnings, limitations, and `TBD` values shall remain explicit findings and shall not be converted into silent defaults. | `docs/DIRECTIVE.md` section 2.4; `docs/SPEC.md` sections 4.3, 9, and 12 | Current tests confirm missing audit, validation, source, actor, rationale text, and timestamp inputs emit visible `TBD_VISIBLE` diagnostics and unresolved assumptions remain counted/reportable. |
> | REQ-16-04-08 | Public artifacts shall not introduce protected standards text, code-specific values, proprietary data, private project data, or private rule-pack payloads. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/CONTRACT.md` section 1, OPS-K-IP-1 through OPS-K-IP-3 | Protected-content and data-boundary review; exact linter integration TBD. |
> | REQ-16-04-09 | Plugin, adapter, persistence, report, and application-service paths that touch this control surface shall preserve schema validation, provenance, private-data, protected-content, diagnostics, hashes, and professional-boundary checks. | `docs/SPEC.md` sections 4.4 and 4.5 | Rationale implementation path is established for this slice; plugin, adapter, persistence, report, and broader application-service integration remain TBD until concrete paths are assigned. |
> | REQ-16-04-10 | Guard tests shall cover both positive preservation of rationale/assumptions and negative blocking of professional/code-compliance claim language. | `_CONTEXT.md` Anticipated Artifacts; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4 | Current focused guard tests exist in `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage remains TBD. |
>

### CLM-010 — Standards

> ##### Standards
>
> No external engineering standard text is accessible or required for this setup pass. The governing standards for this deliverable-local draft are the project governance and technical sources listed in `_REFERENCES.md`.
>
> | Standard or Authority | Applicability |
> |---|---|
> | `docs/CONTRACT.md` | Binding invariants for authority boundary, agent proposal status, no invention, and protected-content handling. |
> | `docs/DIRECTIVE.md` | Project professional-responsibility principles and stop rules. |
> | `docs/TYPES.md` | Status vocabulary and epistemic labels. |
> | `docs/SPEC.md` | Analysis-boundary, report-section, persistence, plugin/adapter, and acceptance semantics. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
> | External piping codes or standards | Location TBD; no code text, clause-level rule, acceptance value, or compliance claim is derived here. |
>

### CLM-011 — Verification

> ##### Verification
>
> | Verification Item | Method | Acceptance Signal |
> |---|---|---|
> | Scope conformance | Review against `_CONTEXT.md`, Deliverables register row, and decomposition DEL-16-04 row. | No scope expansion beyond agent rationale and professional-boundary controls. |
> | Rationale and assumption preservation | Focused test review of `tests/test_agent_rationale_boundary.py` and implementation review of `core/model_operations/agent_rationale/engine.py`. | Operation-related rationale, unresolved assumptions, affected entities, validation context, audit references, and audit metadata are retained or explicitly marked TBD. |
> | Prohibited claim prevention | Focused guard tests for rationale text and copied operation/audit/validation context; future UI/API/report guard coverage remains TBD. | Prohibited authority language yields blocking diagnostics and does not become a permitted rationale status. |
> | Human acceptance separation | Review current rationale flags and adjacent audit-trail tests. | Rationale output is decision-support only and does not create accepted operation records, bypass user acceptance, or mutate accepted model state; final human-disposition workflow remains TBD. |
> | Protected-content/data boundary | Protected-content and private-data checks. | No protected standards content or private project/rule data added to public artifacts. |
> | Dependency preservation | Check approved DAG-006 local mirror. | Existing approved rows remain ACTIVE unless later changed by RECONCILIATION plus CHANGE approval. |
>

### CLM-012 — Documentation

> ##### Documentation
>
> Required deliverable artifacts:
>
> - `agent rationale record` - current implementation path is `core/model_operations/agent_rationale/engine.py`; standalone schema and final persistence path remain TBD.
> - `professional-boundary guard tests` - current focused test path is `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage remains TBD.
>
> Setup artifacts produced by this workflow:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
>
> These documents cite current implementation/test evidence for the bounded DEL-16-04 slice only. They do not claim lifecycle transition or engineering acceptance. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

- **AC-001** — The contract preserves operation context/history, rationale, unresolved assumptions, affected entities, audit and validation context/references, source/actor metadata, timestamps, diagnostics, provenance, stable rationale identity/hash, visible TBD findings, explicit human-review and external hash-bound human-authority boundaries, copied-context claim scanning, prohibited authority-language blocking, no accepted-record creation, no acceptance bypass, no accepted-state mutation, protected/private-data controls, and unresolved standalone schema, persistence, UI/API/report, broader application, dependency, and human-disposition matters.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-16-04 Agent rationale and professional-boundary controls

> #### Procedure: DEL-16-04 Agent rationale and professional-boundary controls
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-014 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-16-04-DECL-004`.
>

### CLM-015 — Purpose

> ##### Purpose
>
> Define a conservative procedure for reviewing the current bounded DEL-16-04 rationale implementation/test slice and for producing or reviewing later DEL-16-04 artifacts. This procedure is document-level guidance only; it does not implement product code or claim engineering acceptance.
>

### CLM-016 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source |
> |---|---|
> | Confirm deliverable identity, package, scope, objectives, and artifact expectations from `_CONTEXT.md`. | `_CONTEXT.md` |
> | Use the accepted revision 0.7 decomposition for SOW-070, OBJ-015, OBJ-018, PKG-16, and DEL-16-04. | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Apply project authority invariants for professional boundary and agent proposal status. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4 |
> | Apply status vocabulary boundaries and epistemic labels. | `docs/TYPES.md` sections 4 and 5 |
> | Apply data-boundary and protected-content constraints. | `docs/IP_AND_DATA_BOUNDARY.md` |
> | Preserve the approved DAG-006 local dependency mirror unless RECONCILIATION plus CHANGE approval supersedes it. | `_DEPENDENCIES.md`; `Dependencies.csv` |
> | Use the current rationale implementation and focused tests as bounded evidence for this slice. | `core/model_operations/agent_rationale/engine.py`; `tests/test_agent_rationale_boundary.py` |
> | Use adjacent PKG-16 schema/audit/preview implementation and tests as read-only context for boundaries consumed by rationale tests. | `schemas/model_operation.schema.json`; `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py`; adjacent focused tests |
> | Treat standalone rationale schema, final UI/agent workflow presentation, broader persistence/application behavior, dependency versions, and human review dispositions as TBD until later Type 2 work or human ruling resolves them. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 12 |
>

### CLM-017 — Steps

> ##### Steps
>
> 1. Confirm the work is still bounded to DEL-16-04 and SOW-070.
> 2. Review source authority in this order: `_REFERENCES.md` listed governing sources, `_CONTEXT.md`, approved local `Dependencies.csv`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
> 3. Identify the rationale-record categories supported by sources and implementation: operation history/context, rationale, assumptions, affected entities, audit metadata/references, validation context, source/actor metadata, diagnostics, provenance, and stable rationale ID/hash.
> 4. Confirm the record posture remains decision-support only: no accepted operation record creation, no accepted-model-state mutation, no user-acceptance bypass, and explicit human-review requirement.
> 5. Mark unsupported schema fields, standalone schema path, persistence path, UI/agent workflow presentation, broader application behavior, and human dispositions as `TBD` rather than inventing them.
> 6. Identify prohibited professional-boundary outputs from `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `PROHIBITED_CLAIM_PATTERNS` in `core/model_operations/agent_rationale/engine.py`.
> 7. Check that rationale text and copied operation/audit/validation/source/actor/reference context produce blocking diagnostics for prohibited authority language.
> 8. Check that lowercase coordination references such as approved DAG authority are not misclassified as professional approval (PRD §21.2) unless they use blocked authority language.
> 9. Check that assumptions, warnings, limitations, missing context, and `TBD` values remain explicit findings.
> 10. Check that no protected standards text, code-specific values, private project data, or private rule-pack payloads are introduced.
> 11. Check dependency handling against the local DAG-006 mirror; do not retire, delete, or reclassify approved ACTIVE rows during this setup pass.
> 12. Record unresolved issues as `TBD`, `ASSUMPTION`, or `PROPOSAL` using `docs/TYPES.md` epistemic labels.
>

### CLM-018 — Verification

> ##### Verification
>
> | Check | Expected Result |
> |---|---|
> | Scope check | Documents and rationale artifacts reference DEL-16-04, PKG-16, SOW-070, OBJ-015, and OBJ-018 without expanding into sibling deliverables. |
> | Rationale preservation check | Operation history, rationale, assumptions, affected entities, and audit metadata are present as required categories or explicitly marked TBD pending schema work. |
> | Non-acceptance check | Current rationale output is decision-support only and does not create accepted operation records, bypass user acceptance, or mutate accepted model state. |
> | Prohibited-claim check | Automatic output cannot state or imply certification, sealing, approval, authentication, professional reliance, external validation authority, autonomous engineering acceptance, or code compliance (PRD §21.2). |
> | Status-vocabulary check | Automatic statuses remain within the permitted vocabulary from `docs/TYPES.md` and `docs/SPEC.md`. |
> | Human-acceptance check | Any human acceptance reference is external, human-actor-owned, and hash-bound; it is not generated by software. |
> | Copied-context check | Prohibited authority language in copied operation, audit, or validation context is blocked before rationale is treated as captured for user review. |
> | Visible-TBD check | Missing audit context, validation context, source metadata, actor metadata, rationale text, and timestamp remain visible `TBD_VISIBLE` diagnostics. |
> | Data-boundary check | No protected standards text, proprietary data, private project data, or private rule-pack payload is introduced. |
> | Dependency preservation check | `Dependencies.csv` approved DAG-006 rows remain ACTIVE and unchanged by this setup pass. |
>

### CLM-019 — Records

> ##### Records
>
> The expected records for this deliverable are:
>
> - agent rationale record: current implementation path `core/model_operations/agent_rationale/engine.py`; standalone schema and persistence path TBD;
> - professional-boundary guard tests: current focused path `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage TBD;
> - unresolved assumptions and `TBD` decisions surfaced in deliverable documents;
> - validation evidence from focused implementation tests;
> - this setup document kit and semantic/lensing metadata.

- **VER-001** — Validate the contract and review source parity, rationale/audit/validation/source/actor context preservation, deterministic identity, visible TBD and unresolved-assumption behavior, all prohibited-claim and copied-context guards including false-positive boundaries, external human-owned acceptance separation, no application or state mutation, protected-data posture, retained downstream TBDs, and absence of autonomous acceptance or professional-authority claims.

## Governing Values and Decisions — Axiology

### CLM-020 — Guidance: DEL-16-04 Agent rationale and professional-boundary controls

> #### Guidance: DEL-16-04 Agent rationale and professional-boundary controls
>

### CLM-021 — Purpose

> ##### Purpose
>
> This deliverable exists to keep agent-assisted model operation workflows auditable without allowing agent output to become accepted engineering work by itself. The source basis is DEL-16-04, SOW-070, OBJ-015, OBJ-018, and the project authority boundaries in `docs/CONTRACT.md`, `docs/TYPES.md`, and `docs/SPEC.md`.
>

### CLM-022 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Proposal is not acceptance | Treat agent rationale as proposal/supporting explanation until a human gate accepts the related work. | `docs/CONTRACT.md` OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 |
> | Preserve rationale and assumptions | Do not collapse rationale, unresolved assumptions, affected entities, and audit metadata into unstructured text that cannot support reproducible review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-070 |
> | Block professional claims | Controls should prevent certification, approval, sealing, authentication, professional reliance, and code-compliance claim language (PRD §21.2) from being generated as software/agent authority. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
> | Surface unknowns | Missing or unsupported facts should remain `TBD` or explicit assumptions rather than silent defaults. | `docs/DIRECTIVE.md` section 2.4; `docs/SPEC.md` section 12 |
> | Keep protected data out | Rationale and tests must not quote or paraphrase protected standards content, proprietary data, private project data, or private rule-pack payloads into public artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
> | Preserve hash-bound human authority | If human acceptance records are referenced, treat them as external, human-owned, and bound to reviewed payload hashes. | `docs/SPEC.md` sections 4.3 and 9 |
> | Keep rationale non-applying | Treat `record_agent_rationale()` output as deterministic decision-support metadata only; it must not create accepted operation records, mutate accepted model state, or bypass user acceptance. | `core/model_operations/agent_rationale/engine.py`; `tests/test_agent_rationale_boundary.py` |
>

### CLM-023 — Considerations

> ##### Considerations
>
> | Topic | Consideration |
> |---|---|
> | Record shape | The current Python record shape is established in `core/model_operations/agent_rationale/engine.py` for this slice: deliverable/package/scope metadata, operation set ref, audit context, source, actor, timestamp, rationale posture, assumptions, validation context, affected entities, audit references, operation context, diagnostics, accepted-state unchanged flag, professional boundary, provenance, rationale ID, and rationale hash. Standalone JSON Schema and final persistence path remain TBD. |
> | Guard-test surface | Current focused guards are in `tests/test_agent_rationale_boundary.py`. They cover deterministic records, no accepted-state mutation, no accepted-operation creation, no user-acceptance bypass, missing-context/TBD visibility, unresolved assumptions, prohibited rationale claims, copied-context claims, enum-style authority tokens, and lowercase approved coordination-context false-positive behavior. Broader UI/API/report coverage remains TBD. |
> | Dependency context | The approved DAG-006 mirror identifies architecture-basis deliverables, professional responsibility policy, user acceptance/audit trail, and security threat model as upstream context. This setup pass preserves that mirror rather than reclassifying it. |
> | Rationale privacy | Rationale may mention project context or user inputs. Future implementation must respect private-data and protected-content controls before public export. |
> | Human review | Human review is a boundary, not a wording detail. A UI label or report phrase must not imply human acceptance unless backed by an external, human-owned, hash-bound record. |
> | Copied context | Rationale capture scans rationale text plus copied operation, audit, validation, source, actor, affected-reference, and audit-reference context for prohibited authority language. Blocking diagnostics should be treated as wording/authority blockers, not engineering findings. |
>

### CLM-024 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative Direction |
> |---|---|
> | Detailed rationale vs. private-data exposure | Preserve enough rationale for reproducible model-state review, but redact or keep private any protected, proprietary, or project-private content. |
> | Strict prohibited-claim filters vs. false positives | Prefer blocking or flagging ambiguous professional/code-compliance language until a human resolves the wording. |
> | Flexible notes vs. structured review | Allow notes only when structured fields preserve operation history, assumptions, affected entities, audit metadata, and review status. |
> | Human acceptance references vs. software statuses | Keep human acceptance external and hash-bound; do not convert it into an automatic software status. |
>

### CLM-025 — Examples

> ##### Examples
>
> Source-backed vocabulary examples:
>
> | Category | Examples from Sources |
> |---|---|
> | Permitted automatic software statuses | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED` (`docs/TYPES.md` section 4). |
> | Prohibited automatic status or claim language | `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED`, or equivalent professional/code-compliance language (PRD §21.2; `docs/TYPES.md` section 4). |
> | Required epistemic labels | `FACT`, `ASSUMPTION`, `PROPOSAL`, `TBD` (`docs/TYPES.md` section 5). |
> | Current rationale blocking diagnostics | `RATIONALE-AUTHORITY-COMPLIANCE-BLOCKED`, `RATIONALE-AUTHORITY-CERTIFICATION-BLOCKED`, `RATIONALE-AUTHORITY-SEALING-BLOCKED`, `RATIONALE-AUTHORITY-AUTHENTICATION-BLOCKED`, `RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED`, `RATIONALE-AUTHORITY-EXTERNAL-VALIDATION-BLOCKED`, and `RATIONALE-AUTHORITY-AUTONOMOUS-ACCEPTANCE-BLOCKED` from `core/model_operations/agent_rationale/engine.py` (enforcing PRD §21.2). |
> | Current visible TBD diagnostics | Missing audit context, validation context, source metadata, actor metadata, rationale text, and timestamp are emitted as `TBD_VISIBLE` diagnostics in `record_agent_rationale()`. |
>
> The authorized implementation/test evidence uses invented fixtures only. No project-specific engineering example, standards clause, acceptance value, or professional/code-compliance conclusion is established by this slice.
>

### CLM-026 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None | No cross-source conflict identified in the slices used for this setup pass. | N/A | N/A | N/A | N/A | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-070 OBJ-015 OBJ-018 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
