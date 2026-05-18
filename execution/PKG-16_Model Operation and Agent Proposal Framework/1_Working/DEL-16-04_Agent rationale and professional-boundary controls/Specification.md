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
- exact code modules, schema files, dependency versions, and test harness paths, which remain TBD unless assigned by later Type 2 work.

## Requirements

| ReqID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-16-04-01 | The deliverable shall remain bounded to DEL-16-04, PKG-16, SOW-070, OBJ-015, and OBJ-018. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 4-7 | Review document metadata and scope references. |
| REQ-16-04-02 | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 4, SOW-070 | Future guard tests or schema review confirm those categories exist; exact fields TBD. |
| REQ-16-04-03 | Agent outputs shall remain drafts or proposals until accepted by a human gate. | `docs/CONTRACT.md` section 1, OPS-K-AGENT-4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1 | Guard test or review confirms no agent output path bypasses human gate semantics. |
| REQ-16-04-04 | Software and agents shall not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance. | `docs/CONTRACT.md` section 1, OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 3-5 | Prohibited-claim tests reject automatic professional/code-compliance claim language. |
| REQ-16-04-05 | Automatic status vocabulary shall not include `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED`, or equivalent professional/code-compliance language. | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.3 | Status enum tests or schema review confirm only permitted automatic statuses are emitted. |
| REQ-16-04-06 | Any human acceptance record represented by the product shall be external, human-actor-owned, and bound to reviewed payload hashes; it shall not be software-generated professional approval. | `docs/SPEC.md` sections 4.3 and 9 | Review confirms acceptance references are external/hash-bound; exact record schema TBD. |
| REQ-16-04-07 | Missing data, unresolved assumptions, warnings, limitations, and `TBD` values shall remain explicit findings and shall not be converted into silent defaults. | `docs/DIRECTIVE.md` section 2.4; `docs/SPEC.md` sections 4.3, 9, and 12 | Guard tests confirm assumptions/TBDs are preserved and reportable. |
| REQ-16-04-08 | Public artifacts shall not introduce protected standards text, code-specific values, proprietary data, private project data, or private rule-pack payloads. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/CONTRACT.md` section 1, OPS-K-IP-1 through OPS-K-IP-3 | Protected-content and data-boundary review; exact linter integration TBD. |
| REQ-16-04-09 | Plugin, adapter, persistence, report, and application-service paths that touch this control surface shall preserve schema validation, provenance, private-data, protected-content, diagnostics, hashes, and professional-boundary checks. | `docs/SPEC.md` sections 4.4 and 4.5 | Integration review when concrete paths are assigned; current implementation surface TBD. |
| REQ-16-04-10 | Guard tests shall cover both positive preservation of rationale/assumptions and negative blocking of professional/code-compliance claim language. | `_CONTEXT.md` Anticipated Artifacts; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4 | Future professional-boundary guard tests exist and cite this specification. |

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
| Rationale and assumption preservation | Schema/test review once implementation exists. | Operation-related rationale, unresolved assumptions, affected entities, and audit metadata are retained or explicitly marked TBD. |
| Prohibited claim prevention | Guard tests for automatic statuses, UI/API/report strings, and agent outputs where applicable. | No automatic certification, approval, sealing, authentication, professional reliance, or code-compliance output. |
| Human acceptance separation | Review records or schemas once implementation exists. | Human acceptance is external, actor-owned, and hash-bound; software does not self-approve. |
| Protected-content/data boundary | Protected-content and private-data checks. | No protected standards content or private project/rule data added to public artifacts. |
| Dependency preservation | Check approved DAG-002 local mirror. | Existing approved rows remain ACTIVE unless later changed by RECONCILIATION plus CHANGE approval. |

## Documentation

Required deliverable artifacts:

- `agent rationale record` - exact product artifact path and schema TBD.
- `professional-boundary guard tests` - exact test files and assertions TBD.

Setup artifacts produced by this workflow:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`

No implementation evidence, product code, or engineering acceptance evidence is claimed by these setup artifacts.
