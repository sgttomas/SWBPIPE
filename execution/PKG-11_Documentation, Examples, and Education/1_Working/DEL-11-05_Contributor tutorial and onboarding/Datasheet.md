# Datasheet: DEL-11-05 Contributor tutorial and onboarding

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-11-05 |
| Package ID | PKG-11 |
| Package | Documentation, Examples, and Education |
| Type | DOC_UPDATE |
| Scope item | SOW-033 |
| Objectives | OBJ-001; OBJ-002 |
| Context envelope | S |
| Lifecycle target for setup | SEMANTIC_READY after setup gates pass |

## Attributes

| Attribute | Source | Value |
|---|---|---|
| Primary purpose | _CONTEXT.md; Deliverables.csv row DEL-11-05 | Create an onboarding path for new contributors using package/deliverable decomposition and governance docs. |
| Production surface | Human brief | Deliverable-local setup/document production only. Repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, documentation outside this deliverable, source code, and repo-level artifacts are excluded from this session. |
| Intended audience | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md section 1; AGENTS.md project-specific TASK profiles | New contributors and Type 2 TASK workers who need a safe first path through OpenPipeStress governance, decomposition, and bounded execution. |
| Documentation targets | _CONTEXT.md; Deliverables.csv row DEL-11-05 | Future CONTRIBUTING tutorial material and future AGENTIC_DEVELOPMENT_WORKFLOW onboarding material, staged here only. |
| Data boundary | INIT.md; docs/DIRECTIVE.md sections 3-5; docs/IP_AND_DATA_BOUNDARY.md sections 2-6 | Public onboarding must not introduce protected standards text, copied tables, protected examples, proprietary vendor data, private rule packs, owner standards, or company design bases. |
| Professional boundary | docs/CONTRACT.md OPS-K-AUTH-1; docs/TYPES.md section 4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md section 4 | Onboarding must not state or imply that software, agents, maintainers, or contributors certify, approve, seal, authenticate, or declare engineering code compliance for reliance. |
| Agent boundary | AGENTS.md dispatch rule; docs/SPEC.md sections 10-11 | Type 2 work uses one sealed deliverable, explicit write scope, applicable invariants, acceptance criteria, and evidence. |
| Architecture-basis context | _CONTEXT.md Architecture Basis Injection | Applicable architecture basis IDs are AB-00-01, AB-00-02, AB-00-06, AB-00-07, and AB-00-08; they inform contributor routing but do not mark PKG-00 as ISSUED. |

## Conditions

| Condition | Handling |
|---|---|
| No protected examples | Use invented, public-domain, or permissively licensed examples only; otherwise record `TBD` or quarantine per policy. |
| No public defaults for code data | Contributor material must preserve user-supplied/private rule-pack and project-data boundaries. |
| No certification claims | Use "human review", "maintainer review", "acceptance gate", or "draft/proposal" language as appropriate; avoid code-compliance certification claims. |
| No out-of-scope edits | This setup deliverable does not edit repo-level onboarding files. Future integration must be a separate approved task or human action. |
| Unknowns | Record `TBD`; do not invent policy, legal conclusions, engineering values, or source citations. |

## Construction

The onboarding path should be constructed as a contributor journey with the following local drafting components:

| Component | Role | Boundary |
|---|---|---|
| Orientation | Explains OpenPipeStress intent, agent roles, decomposition hierarchy, and sealed-deliverable execution. | Must remain descriptive and source-grounded. |
| Safe contribution checklist | Lists protected-data, provenance, privacy, and professional-responsibility checks before public contribution. | Does not replace legal or professional review. |
| Type 2 task walkthrough | Shows how a contributor reads `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and register rows before working. | Does not authorize cross-deliverable edits. |
| Evidence and review handoff | Describes expected outputs, validation evidence, warnings, and review readiness. | Does not mark any artifact accepted or ISSUED. |

## References

| Reference | Sections used | Use |
|---|---|---|
| INIT.md | Required reading order; agent rule | Bootstrap and stop-rule context. |
| AGENTS.md | Agent posture; primary agents; dispatch rule | Role and dispatch boundaries. |
| docs/README.md | Document map; agent use package | Source navigation. |
| docs/DIRECTIVE.md | Sections 1-6 | Project intent, boundaries, and stop rules. |
| docs/CONTRACT.md | Invariant index | Binding invariants. |
| docs/TYPES.md | Sections 1-9 | IDs, statuses, epistemic labels, authority vocabulary. |
| docs/SPEC.md | Sections 1, 6-11 | Architecture boundary, public examples, reports, agent mechanics, acceptance semantics. |
| docs/IP_AND_DATA_BOUNDARY.md | Sections 2-7 | Public/private data and quarantine policy. |
| docs/VALIDATION_STRATEGY.md | Sections 1, 4-5 | Validation boundary and permitted benchmark/example sources. |
| docs/AGENTIC_DEVELOPMENT_WORKFLOW.md | Sections 1-6 | Type 1/Type 2 workflow and review checklist. |
| docs/_Decomposition/SOFTWARE_DECOMP.md | Revision 0.4; SOW-033; DEL-11-05 | Scope and objectives. |
| docs/_Registers/Deliverables.csv | Row DEL-11-05 | Deliverable identity. |
| docs/_Registers/ScopeLedger.csv | Row SOW-033 | Scope item basis. |
| execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md | Explicit holds | Architecture-basis handling and ISSUED hold. |
