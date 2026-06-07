# Specification: DEL-16-04 Agent rationale and professional-boundary controls

## Scope

This deliverable covers the security-control specification surface for recording agent rationale and unresolved assumptions while preserving the professional boundary. It is scoped to DEL-16-04 in PKG-16 and SOW-070.

Included:

- rationale and assumption recording expectations for accepted model operations;
- professional-boundary controls that prevent agent/software claims of certification, approval, authentication, sealing, professional reliance, or code compliance;
- guard-test expectations for that boundary.

Excluded:

- implementation of model-operation schemas owned by DEL-16-01;
- implementation of validation/diff preview owned by DEL-16-02;
- implementation of user acceptance and operation audit trail owned by DEL-16-03;
- professional responsibility/product-claims policy authorship owned by DEL-01-04;
- standalone rationale JSON Schema, final UI/agent workflow presentation, broader persistence/application-service behavior outside this slice, dependency versions, and human review dispositions, which remain TBD unless assigned by later Type 2 work or human ruling.

Current implementation evidence for this deliverable-local slice is `core/model_operations/agent_rationale/engine.py` with focused tests in `tests/test_agent_rationale_boundary.py`. That implementation consumes DEL-16-01 operation envelopes and optional DEL-16-03 audit / DEL-16-02 validation context as read-only rationale context; it does not implement those sibling deliverables.

## Requirements

| ReqID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-16-04-01 | The deliverable shall remain bounded to DEL-16-04, PKG-16, SOW-070, OBJ-015, and OBJ-018. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 4-7 | Review document metadata and scope references. |
| REQ-16-04-02 | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4, SOW-070 | Current `record_agent_rationale()` output preserves rationale text/status, assumptions, validation context, affected entities, audit references, audit context, and operation context; standalone schema remains TBD. |
| REQ-16-04-03 | Agent outputs shall remain drafts or proposals until accepted by a human gate. | `docs/CONTRACT.md` section 1, OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 | `tests/test_agent_rationale_boundary.py` confirms rationale records are decision-support only, create no accepted operation record, do not bypass user acceptance, and do not mutate accepted model state. |
| REQ-16-04-04 | Software and agents shall not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance. | `docs/CONTRACT.md` section 1, OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 3-5 | `tests/test_agent_rationale_boundary.py` confirms prohibited rationale language emits blocking professional-boundary diagnostics. |
| REQ-16-04-05 | Automatic status vocabulary shall not include `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED`, or equivalent professional/code-compliance language. | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.3 | Current tests confirm enum-style authority tokens in copied context are blocked and lowercase approved coordination context is not treated as professional approval. Adjacent schema tests confirm forbidden operation statuses remain excluded from `model_operation.schema.json`. |
| REQ-16-04-06 | Any human acceptance record represented by the product shall be external, human-actor-owned, and bound to reviewed payload hashes; it shall not be software-generated professional approval. | `docs/SPEC.md` sections 4.3 and 9 | Current rationale record stores audit context/references as context and sets `rationale_creates_audit_acceptance`/accepted-record creation flags false; final human-disposition workflow remains TBD. |
| REQ-16-04-07 | Missing data, unresolved assumptions, warnings, limitations, and `TBD` values shall remain explicit findings and shall not be converted into silent defaults. | `docs/DIRECTIVE.md` section 2.4; `docs/SPEC.md` sections 4.3, 9, and 12 | Current tests confirm missing audit, validation, source, actor, rationale text, and timestamp inputs emit visible `TBD_VISIBLE` diagnostics and unresolved assumptions remain counted/reportable. |
| REQ-16-04-08 | Public artifacts shall not introduce protected standards text, code-specific values, proprietary data, private project data, or private rule-pack payloads. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/CONTRACT.md` section 1, OPS-K-IP-1 through OPS-K-IP-3 | Protected-content and data-boundary review; exact linter integration TBD. |
| REQ-16-04-09 | Plugin, adapter, persistence, report, and application-service paths that touch this control surface shall preserve schema validation, provenance, private-data, protected-content, diagnostics, hashes, and professional-boundary checks. | `docs/SPEC.md` sections 4.4 and 4.5 | Rationale implementation path is established for this slice; plugin, adapter, persistence, report, and broader application-service integration remain TBD until concrete paths are assigned. |
| REQ-16-04-10 | Guard tests shall cover both positive preservation of rationale/assumptions and negative blocking of professional/code-compliance claim language. | `_CONTEXT.md` Anticipated Artifacts; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4 | Current focused guard tests exist in `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage remains TBD. |

## Standards

No external engineering standard text is accessible or required for this setup pass. The governing standards for this deliverable-local draft are the project governance and technical sources listed in `_REFERENCES.md`.

| Standard or Authority | Applicability |
|---|---|
| `docs/CONTRACT.md` | Binding invariants for authority boundary, agent proposal status, no invention, and protected-content handling. |
| `docs/DIRECTIVE.md` | Project professional-responsibility principles and stop rules. |
| `docs/TYPES.md` | Status vocabulary and epistemic labels. |
| `docs/SPEC.md` | Analysis-boundary, report-section, persistence, plugin/adapter, and acceptance semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
| External piping codes or standards | Location TBD; no code text, clause-level rule, acceptance value, or compliance claim is derived here. |

## Verification

| Verification Item | Method | Acceptance Signal |
|---|---|---|
| Scope conformance | Review against `_CONTEXT.md`, Deliverables register row, and decomposition DEL-16-04 row. | No scope expansion beyond agent rationale and professional-boundary controls. |
| Rationale and assumption preservation | Focused test review of `tests/test_agent_rationale_boundary.py` and implementation review of `core/model_operations/agent_rationale/engine.py`. | Operation-related rationale, unresolved assumptions, affected entities, validation context, audit references, and audit metadata are retained or explicitly marked TBD. |
| Prohibited claim prevention | Focused guard tests for rationale text and copied operation/audit/validation context; future UI/API/report guard coverage remains TBD. | Prohibited authority language yields blocking diagnostics and does not become a permitted rationale status. |
| Human acceptance separation | Review current rationale flags and adjacent audit-trail tests. | Rationale output is decision-support only and does not create accepted operation records, bypass user acceptance, or mutate accepted model state; final human-disposition workflow remains TBD. |
| Protected-content/data boundary | Protected-content and private-data checks. | No protected standards content or private project/rule data added to public artifacts. |
| Dependency preservation | Check approved DAG-006 local mirror. | Existing approved rows remain ACTIVE unless later changed by RECONCILIATION plus CHANGE approval. |

## Documentation

Required deliverable artifacts:

- `agent rationale record` - current implementation path is `core/model_operations/agent_rationale/engine.py`; standalone schema and final persistence path remain TBD.
- `professional-boundary guard tests` - current focused test path is `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage remains TBD.

Setup artifacts produced by this workflow:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`

These documents cite current implementation/test evidence for the bounded DEL-16-04 slice only. They do not claim lifecycle transition, engineering acceptance, professional approval, certification, sealing, authentication, or code compliance.
