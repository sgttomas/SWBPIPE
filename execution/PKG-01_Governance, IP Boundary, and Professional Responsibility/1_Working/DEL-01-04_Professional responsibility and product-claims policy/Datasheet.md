# Datasheet: DEL-01-04 Professional responsibility and product-claims policy

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-01-04 |
| Name | Professional responsibility and product-claims policy |
| Package ID | PKG-01 |
| Package | Governance, IP Boundary, and Professional Responsibility |
| Type | DOC_UPDATE |
| Scope Item | SOW-034 |
| Objective | OBJ-011 |
| Decomposition Basis | docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 |
| Local Status | Draft setup artifact; not human-issued |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary purpose | Define permitted/prohibited claims, report disclaimers, and human approval boundaries. | _CONTEXT.md; Deliverables.csv row DEL-01-04 |
| Professional boundary | Software and agents do not certify, approve, seal, authenticate, or declare engineering code compliance for reliance. | CONTRACT.md OPS-K-AUTH-1; ScopeLedger.csv SOW-034 |
| Solver/rule boundary | The solver computes mechanics; rule packs evaluate user-defined acceptability; professional compliance remains human judgment. | CONTRACT.md OPS-K-MECH-2 |
| Report boundary | Reports must disclose provenance, warnings, assumptions, limitations, and professional-responsibility notices. | CONTRACT.md OPS-K-REPORT-1; SPEC.md Section 8 |
| Release boundary | Public releases must disclose scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | CONTRACT.md OPS-K-GOV-3 |
| Agent authority | Agent outputs are drafts/proposals until accepted by a human gate. | CONTRACT.md OPS-K-AGENT-4; AGENTIC_DEVELOPMENT_WORKFLOW.md |

## Conditions

| Condition | Draft Policy Value |
|---|---|
| Permitted software/product claims | May describe implemented mechanics, user-rule evaluation, diagnostics, validation evidence, provenance capture, and report generation when supported by source evidence. |
| Prohibited software/product claims | Must not claim certification, sealing, professional approval, official code compliance, standards endorsement, or readiness for reliance without competent human review. |
| Unknown or unsupported claims | Mark `TBD`, `ASSUMPTION`, or `PROPOSAL`; do not convert uncertainty into product language. |
| Protected standards/code data | Do not reproduce protected standards text, tables, figures, code-derived formulas, examples, allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. |
| Human acceptance records | If used, bind to specific model/rule/report hashes and do not survive content changes without re-review. |

## Construction

| Local Artifact | Intended Repo-Level Artifact Discussed | Construction Note |
|---|---|---|
| Datasheet.md | docs/PROFESSIONAL_BOUNDARY.md | Structured identity and policy attributes only. |
| Specification.md | docs/PROFESSIONAL_BOUNDARY.md; report notice template | Normative draft requirements for later repo-level policy work. |
| Guidance.md | docs/PROFESSIONAL_BOUNDARY.md | Rationale and examples for product-claims boundaries. |
| Procedure.md | report notice template | Operational workflow for drafting/reviewing claims and notices. |

## References

- INIT.md
- AGENTS.md
- docs/README.md
- docs/DIRECTIVE.md
- docs/CONTRACT.md
- docs/TYPES.md
- docs/SPEC.md
- docs/AGENTIC_DEVELOPMENT_WORKFLOW.md
- agents/AGENT_PREPARATION.md
- docs/_Decomposition/SOFTWARE_DECOMP.md
- docs/_Registers/Deliverables.csv
- docs/_Registers/ScopeLedger.csv
- docs/_Registers/ContextBudgetQA.csv
