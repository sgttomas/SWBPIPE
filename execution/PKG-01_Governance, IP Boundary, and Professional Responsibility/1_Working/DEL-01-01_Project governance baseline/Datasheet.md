# Datasheet: DEL-01-01 Project governance baseline

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-01-01 |
| Package ID | PKG-01 |
| Package Name | Governance, IP Boundary, and Professional Responsibility |
| Deliverable Name | Project governance baseline |
| Type | DOC_UPDATE |
| Scope Items | SOW-001, SOW-048 |
| Objectives | OBJ-001, OBJ-002 |
| Context Envelope | M |
| Decomposition Basis | docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 |
| Status | Draft deliverable-local setup kit |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Product stance | Free and open-source piping stress analysis platform | SOW-001; docs/DIRECTIVE.md Section 6 |
| License selection | TBD | OPS-K-GOV-1; SOW-048 notes |
| Governance surfaces | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `governance/MAINTAINERS.md` | _CONTEXT.md anticipated artifacts |
| Policy scope | Governance baseline, maintainer policy skeleton, release/governance boundary language | DEL-01-01 register row; SOW-048 |
| Public data boundary | Protected standards text, tables, figures, examples, protected formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data are not public repository content | OPS-K-IP-1; docs/DIRECTIVE.md Sections 3 and 4 |
| Professional authority boundary | Software and agents do not certify, seal, approve, authenticate, or declare engineering code compliance for reliance | OPS-K-AUTH-1; docs/DIRECTIVE.md Section 6 |
| Agent authority boundary | Type 2 outputs are drafts/proposals until accepted by a human gate | OPS-K-AGENT-4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md |

## Conditions

| Condition | Status |
|---|---|
| No repo-level artifacts are edited by this setup run | Applied |
| No protected standards/code data is reproduced | Applied |
| No legal conclusion beyond draft policy language is made | Applied |
| Maintainer roster, release authority, quorum, signing, and license choice | TBD |
| Public release maturity labels and validation disclosure format | TBD |
| Human project authority for governance decisions | TBD |

## Governance Decision Surface

| Decision Surface | Current Value | Authority Needed |
|---|---|---|
| Open-source license | TBD | Human project authority |
| Maintainer roster | TBD | Human project authority |
| Maintainer quorum or approval model | TBD | Human project authority |
| Release authority | TBD | Human project authority |
| Release signing or provenance process | TBD | Human project authority |
| Legal review process for license/IP questions | TBD | Human project authority |

## Construction

This deliverable-local kit describes the baseline content that future human-approved edits may apply to the governance artifacts. It does not itself amend `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, or `governance/MAINTAINERS.md`.

The baseline is constructed from:

- scope item SOW-001: public free/open-source platform intent;
- scope item SOW-048: license, governance, release, and maintainer policy obligation;
- objective OBJ-001: auditable, inspectable, extensible platform;
- objective OBJ-002: protected-standards and user-supplied-data separation;
- contract invariants for hierarchy, identifiers, IP boundary, professional authority, governance, and agent execution.

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `docs/TYPES.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
