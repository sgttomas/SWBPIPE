---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-04
package_id: PKG-01
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-034, SOW-064]
package_objective_refs: [OBJ-011, OBJ-018]
---

# Scope of Work — DEL-01-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-01-04` in service of project scope [SOW-034, SOW-064] and package objectives [OBJ-011, OBJ-018].

- **OUT-001** — A professional-responsibility and product-claims policy and report-notice requirements defining permitted evidence claims, prohibited reliance claims, human approval boundaries, and bounded analysis-grade design-engine language.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-01-04 Professional responsibility and product-claims policy

> #### Datasheet: DEL-01-04 Professional responsibility and product-claims policy
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-01-04 |
> | Name | Professional responsibility and product-claims policy |
> | Package ID | PKG-01 |
> | Package | Governance, IP Boundary, and Professional Responsibility |
> | Type | DOC_UPDATE |
> | Scope Items | SOW-034, SOW-064 |
> | Objectives | OBJ-011, OBJ-018 |
> | Decomposition Basis | execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 |
> | DAG Context | execution/_DAG/DAG-006/ approved active graph authority |
> | Local Status | IN_PROGRESS; draft governance/report-notice evidence, not issued or professionally accepted |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary purpose | Define permitted/prohibited claims, report disclaimers, human approval boundaries, and product-claim boundaries for the design-engine scope. | _CONTEXT.md; Deliverables.csv row DEL-01-04 |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | CONTRACT.md OPS-K-AUTH-1; ScopeLedger.csv SOW-034 |
> | Design-engine scope boundary | OpenPipeStress may be framed as an analysis-grade piping design engine and stress-model authoring environment with a full internal solver, but reliance remains non-authoritative without competent human review. | ScopeLedger.csv SOW-064; SOFTWARE_DECOMP.md revision 0.7 |
> | Solver/rule boundary | The solver computes mechanics; rule packs evaluate user-defined acceptability; professional compliance remains human judgment. | CONTRACT.md OPS-K-MECH-2 |
> | Report boundary | Reports must disclose provenance, warnings, assumptions, limitations, and professional-responsibility notices. | CONTRACT.md OPS-K-REPORT-1; SPEC.md Section 8 |
> | Release boundary | Public releases must disclose scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | CONTRACT.md OPS-K-GOV-3 |
> | Agent authority | Agent outputs are drafts/proposals until accepted by a human gate. | CONTRACT.md OPS-K-AGENT-4; AGENT_WORKING_ITEMS.md |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Draft Policy Value |
> |---|---|
> | Permitted software/product claims | May describe implemented mechanics, user-rule evaluation, diagnostics, validation evidence, provenance capture, and report generation when supported by source evidence. |
> | Prohibited software/product claims | Must not claim certification, sealing, professional approval, official code compliance, standards endorsement, or readiness for reliance without competent human review (PRD §21.2). |
> | Unknown or unsupported claims | Mark `TBD`, `ASSUMPTION`, or `PROPOSAL`; do not convert uncertainty into product language. |
> | Protected standards/code data | Do not reproduce protected standards text, tables, figures, code-derived formulas, examples, allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. |
> | Human acceptance records | If used, bind to specific model/rule/report hashes and do not survive content changes without re-review. |
>

### CLM-005 — Construction

> ##### Construction
>
> | Local Artifact | Intended Repo-Level Artifact Discussed | Construction Note |
> |---|---|---|
> | Datasheet.md | docs/PROFESSIONAL_BOUNDARY.md | Structured identity and policy attributes only. |
> | Specification.md | docs/PROFESSIONAL_BOUNDARY.md; docs/report_notice_template.md | Normative requirements and acceptance boundaries for draft repo-level policy surfaces. |
> | Guidance.md | docs/PROFESSIONAL_BOUNDARY.md | Rationale and examples for product-claims boundaries. |
> | Procedure.md | docs/report_notice_template.md | Operational workflow for drafting/reviewing claims and notices. |
>

### CLM-006 — References

> ##### References
>
> - INIT.md
> - AGENTS.md
> - docs/README.md
> - docs/DIRECTIVE.md
> - docs/CONTRACT.md
> - docs/TYPES.md
> - docs/SPEC.md
> - agents/AGENT_PREPARATION.md
> - execution/_Decomposition/SOFTWARE_DECOMP.md
> - execution/_DAG/DAG-006/
> - docs/_Registers/Deliverables.csv
> - docs/_Registers/ScopeLedger.csv
> - docs/_Registers/ContextBudgetQA.csv
> - docs/PROFESSIONAL_BOUNDARY.md
> - docs/report_notice_template.md
>

### CLM-007 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The professional-responsibility and product-claims policy is current under SOFTWARE_DECOMP revision 0.8 and DAG-007 coordination. It preserves the distinction between computational evidence, professional judgment, validation, and approval; no engineering ruling is created here.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-01-04 Professional responsibility and product-claims policy

> #### Specification: DEL-01-04 Professional responsibility and product-claims policy
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable-local specification defines requirements and acceptance boundaries for the draft professional-boundary policy and report notice template. It covers product-claims language, software/agent authority limits, report notices, and human approval boundaries for OpenPipeStress.
>
> This current-basis refresh does not edit repo-level policy files, implement product behavior, provide legal advice, certify engineering work, issue project policy, or determine project-specific code compliance.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-01-04-R01 | Policy language shall state that OpenPipeStress software and agents do not certify, seal, approve, authenticate, or declare engineering code compliance for reliance (PRD §21.2). | OPS-K-AUTH-1; SOW-034 | Text review for prohibited-claim absence and explicit notice presence. |
> | DEL-01-04-R02 | Policy language shall distinguish mechanics solve, user-rule-pack check, and competent human professional approval (PRD §21.2). | OPS-K-MECH-2; TYPES.md Analysis-status vocabulary | Cross-check terms against `MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT`. |
> | DEL-01-04-R03 | Report notice language shall say that code-specific data is user-supplied and that professional reliance requires competent human review. | SPEC.md Section 8; OPS-K-REPORT-1 | Report-template checklist includes both notices. |
> | DEL-01-04-R04 | Product and release claims shall disclose scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | OPS-K-GOV-3 | Release/review checklist references these disclosure categories. |
> | DEL-01-04-R05 | Human acceptance records, if implemented, shall bind to specific model, rule-pack, and report hashes and require re-review after content changes. | OPS-K-AUTH-2 | Acceptance-record schema or procedure includes hash binding and invalidation rule. |
> | DEL-01-04-R06 | Draft and generated outputs shall remain proposals until accepted by a human gate. | OPS-K-AGENT-4; AGENTIC_DEVELOPMENT_WORKFLOW.md | Lifecycle/status language avoids issued/accepted claims unless human approval exists. |
> | DEL-01-04-R07 | Unknown policy choices, legal interpretations, or jurisdiction-specific professional obligations shall be marked `TBD` and escalated for human/legal/professional review. | OPS-K-AGENT-1; DIRECTIVE.md stop rules | Review confirms no legal conclusion is stated as fact. |
> | DEL-01-04-R08 | Public notices and examples shall not reproduce protected standards content or proprietary engineering data. | OPS-K-IP-1; OPS-K-REPORT-2 | Protected-content review gate. |
> | DEL-01-04-R09 | Repo-level professional-boundary and report-notice text shall remain draft until a human review gate accepts it; local evidence must not present draft text as issued policy. | _CONTEXT.md write-scope limit; OPS-K-AGENT-4 | Review confirms this local kit does not present repo-level text as issued policy. |
> | DEL-01-04-R10 | Product claims may frame OpenPipeStress as an analysis-grade piping design engine and stress-model authoring environment only when acceptance and professional judgment remain with the responsible engineer. | SOW-064; OBJ-018; DIRECTIVE.md §1-§3 | Claim scan confirms design-engine language does not imply certification, sealing, authentication, or code compliance (PRD §21.2). |
>

### CLM-011 — Standards

> ##### Standards
>
> | Reference | Applicability | Location |
> |---|---|---|
> | docs/CONTRACT.md | Governing invariant catalog for authority, reporting, governance, and agent boundaries. | Repo-local |
> | docs/DIRECTIVE.md | Founding intent and stop rules, including human authority and data-boundary principles. | Repo-local |
> | docs/TYPES.md | Vocabulary for analysis statuses, epistemic labels, and professional approval. | Repo-local |
> | docs/SPEC.md | Report and architecture requirements that notices must align with. | Repo-local |
> | docs/IP_AND_DATA_BOUNDARY.md | Protected-content and private-data rules that public report notices and claims must preserve. | Repo-local |
> | docs/PROFESSIONAL_BOUNDARY.md | Existing draft professional-boundary policy surface for this deliverable. | Repo-local |
> | docs/report_notice_template.md | Existing draft report notice template surface for this deliverable. | Repo-local |
> | Protected engineering codes/standards | Not a source for this draft unless lawfully available and explicitly authorized; do not reproduce. | TBD |
>

### CLM-012 — Verification

> ##### Verification
>
> | Check | Method | Pass Condition |
> |---|---|---|
> | Prohibited claim scan | Search policy/report notice text for certification, approval, seal, authentication, compliance-for-reliance, official endorsement, or equivalent language (PRD §21.2). | Any such term is absent or clearly negated as outside software authority. |
> | Status vocabulary scan | Compare policy/report text to TYPES.md. | No automatic `CODE_COMPLIANT` status or equivalent is introduced. |
> | Report notice completeness | Review report-template candidate language. | Includes software/version/provenance/warnings/limitations and human-review notice categories. |
> | Human gate trace | Review acceptance-record references. | Human approval, if present, is hash-bound and draft/proposal status remains clear. |
> | Protected-content screen | Review public examples/notices. | No protected standards text, tables, figures, examples, or proprietary data are reproduced. |
> | Human wording approval | Review proposed repo-level notice language. | Exact notice text is accepted by the human project authority before repo-level publication. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> Draft repo-level artifacts already present:
>
> - `docs/PROFESSIONAL_BOUNDARY.md`
> - `docs/report_notice_template.md`
>
> This refresh records those artifacts as draft evidence only. Final issuance, legal/professional wording, and lifecycle acceptance remain human-gated.
>

### CLM-014 — Open TBDs

> ##### Open TBDs
>
> | TBD ID | Item | Resolution Owner |
> |---|---|---|
> | DEL-01-04-TBD-01 | Final acceptance or revision of `docs/PROFESSIONAL_BOUNDARY.md`. | Human project authority |
> | DEL-01-04-TBD-02 | Final acceptance or revision of `docs/report_notice_template.md`. | Human project authority |
> | DEL-01-04-TBD-03 | Jurisdiction-specific legal/professional-practice language, if any. | Qualified human/legal/professional reviewer |
> | DEL-01-04-TBD-04 | Exact storage and invalidation workflow for hash-bound human acceptance records. | Future persistence/report/governance deliverables |

- **AC-001** — The policy preserves the distinctions among mechanics results, user-rule results, validation evidence, competent human judgment, and approval; prohibits software or agent reliance claims; and retains all recorded TBDs and human gates for SOW-034 and SOW-064.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-01-04 Professional responsibility and product-claims policy

> #### Procedure: DEL-01-04 Professional responsibility and product-claims policy
>

### CLM-016 — Purpose

> ##### Purpose
>
> Define the local procedure for producing and checking professional-boundary policy language and report notice text for OpenPipeStress.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Sealed deliverable context for DEL-01-04.
> - Applicable sources: docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7, approved DAG-006 context, and register rows for DEL-01-04, SOW-034, and SOW-064.
> - No protected standards/code text or proprietary engineering data in the drafting input.
> - Human/legal/professional rulings available for jurisdiction-specific language: `TBD`.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Confirm the target artifact is policy/report-notice language, not solver, GUI, schema, or report-generator implementation.
> 2. Extract the required professional boundary from SOW-034 and OPS-K-AUTH-1.
> 3. Extract related report, governance, and agent boundaries from OPS-K-REPORT-1, OPS-K-REPORT-2, OPS-K-GOV-3, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-MECH-2.
> 4. Extract the SOW-064 design-engine/product-scope boundary and keep it subordinate to the non-authoritative professional reliance boundary.
> 5. Draft permitted-claim language that describes only supported behavior: mechanics computation, user-rule evaluation, warnings, provenance, assumptions, limitations, design-authoring support, and human-review support.
> 6. Draft prohibited-claim language that excludes certification, sealing, approval, authentication, official endorsement, and compliance-for-reliance claims by software or agents (PRD §21.2).
> 7. Review `docs/report_notice_template.md` for notice language that states code-specific data is user-supplied and professional reliance requires competent human review.
> 8. Mark legal conclusions, jurisdiction-specific obligations, unresolved acceptance workflow details, and final acceptance of notice wording as `TBD`.
> 9. Check drafts against protected-content boundaries before publication or release use.
> 10. Route any repo-level policy/report-notice revision or issuance decision to human review before any `ISSUED` lifecycle state or project-policy acceptance claim.
> 11. If human acceptance records are introduced, bind each record to the relevant model, rule-pack, and report hashes and require re-review after content changes.
>

### CLM-019 — Verification

> ##### Verification
>
> | Check | Action |
> |---|---|
> | Scope check | Confirm edits are limited to the authorized deliverable or later authorized repo-level target. |
> | Prohibited-claim check | Confirm no affirmative certification, sealing, approval, authentication, endorsement, or compliance-for-reliance claim remains (PRD §21.2). |
> | Status check | Confirm mechanics, user-rule, missing-data, and human-approval states remain distinct. |
> | Provenance check | Confirm report notice language requires source/provenance disclosure where engineering reliance may be affected. |
> | Publication-readiness screen | Confirm exact notice wording, protected-content checks, and human acceptance status before repo-level publication. |
> | Human-gate check | Confirm draft/proposal status remains visible until human acceptance. |
>

### CLM-020 — Records

> ##### Records
>
> - Deliverable-local four-document kit.
> - `_SEMANTIC.md` semantic matrix output.
> - `_SEMANTIC_LENSING.md` enrichment register.
> - `Dependencies.csv` and `_DEPENDENCIES.md` dependency extraction outputs.
> - `_run_records/TASK_RUN_*.md` evidence record.
> - `_run_records/WORKING_ITEMS_RUN_*.md` current-basis refresh record.
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The professional-responsibility and product-claims policy is current under SOFTWARE_DECOMP revision 0.8 and DAG-007 coordination. It preserves the distinction between computational evidence, professional judgment, validation, and approval; no engineering ruling is created here.

- **VER-001** — Compare the converted contract source markers and parity report against all four legacy source documents, then inspect the matrix and derived checklist for the non-authoritative professional-reliance boundary and exact SOW-034, SOW-064, OBJ-011, and OBJ-018 traceability.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-01-04 Professional responsibility and product-claims policy

> #### Guidance: DEL-01-04 Professional responsibility and product-claims policy
>

### CLM-023 — Purpose

> ##### Purpose
>
> This guidance explains how to draft professional-boundary and product-claims language for OpenPipeStress without overstating what software, agents, rule packs, reports, or releases can warrant.
>
> The central boundary is: OpenPipeStress may assist analysis, expose evidence, compute mechanics, evaluate user-defined rule packs, and produce auditable reports; it does not replace competent professional judgment.
>

### CLM-024 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Use precise authority language | Say what the software computes or records. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
> | Separate statuses | Keep `MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT` distinct. |
> | Preserve human decision rights | A human project authority may accept a development artifact; a competent engineering professional may accept a project calculation. These are different gates. |
> | Make missing data visible | Missing code data, provenance, assumptions, and warnings are findings, not silent defaults. |
> | Keep public examples safe | Public notices and examples use original, invented, or permissively sourced content only. |
> | Keep design-engine claims bounded | The current product scope may describe a design engine and stress-model authoring environment; acceptance and professional judgment remain with the responsible engineer. |
>

### CLM-025 — Considerations

> ##### Considerations
>
> - Product language should prefer "decision support", "mechanics result", "rule-pack result", "diagnostic", "warning", "provenance", and "human review required" over conclusory compliance language.
> - Report notices should be attached to the report output and not hidden only in documentation.
> - A release maturity claim can describe validation evidence and limitations, but it cannot imply approval for a specific piping design.
> - Legal or jurisdiction-specific professional-practice wording is `TBD` until a qualified human authority provides it.
> - Draft repo-level notice wording exists in `docs/report_notice_template.md`; final acceptance, jurisdiction-specific wording, and release-policy language remain human-gated.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred Resolution |
> |---|---|
> | Clear marketing language vs. professional boundary precision | Prefer precision. Ambiguous claims create unacceptable reliance risk. |
> | Automated report convenience vs. human review | Reports may assemble evidence, but reliance remains a human decision. |
> | User-rule pass/fail vs. code compliance | Treat user-rule pass/fail as a computational result using user-supplied inputs, not a professional compliance declaration. |
> | Draft agent output vs. accepted governance | Treat agent output as draft/proposal until human acceptance. |
> | Draft repo-level policy vs. accepted governance | Treat `docs/PROFESSIONAL_BOUNDARY.md` and `docs/report_notice_template.md` as draft evidence until a human gate accepts or revises them. |
>

### CLM-027 — Examples

> ##### Examples
>
> | Safer Draft Claim | Avoid |
> |---|---|
> | "The report includes mechanical results, user-rule check results, warnings, assumptions, provenance notes, and limitations for competent human review." | "The report certifies code compliance." |
> | "The rule pack evaluated user-defined checks using the recorded inputs." | "The software approved the design." |
> | "A competent human must review the model, data, rule basis, warnings, and report before professional reliance." | "No further engineering review is required." |
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in accessible setup materials. | TBD | TBD | TBD | TBD | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-034 SOW-064 OBJ-011 OBJ-018 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
