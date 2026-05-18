# Specification: DEL-01-04 Professional responsibility and product-claims policy

## Scope

This deliverable-local specification defines draft requirements for a future professional-boundary policy and report notice template. It covers product-claims language, software/agent authority limits, report notices, and human approval boundaries for OpenPipeStress.

This setup artifact does not edit repo-level policy files, implement product behavior, provide legal advice, certify engineering work, or determine project-specific code compliance.

## Requirements

| Req ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-01-04-R01 | Policy language shall state that OpenPipeStress software and agents do not certify, seal, approve, authenticate, or declare engineering code compliance for reliance. | OPS-K-AUTH-1; SOW-034 | Text review for prohibited-claim absence and explicit notice presence. |
| DEL-01-04-R02 | Policy language shall distinguish mechanics solve, user-rule-pack check, and competent human professional approval. | OPS-K-MECH-2; TYPES.md Analysis-status vocabulary | Cross-check terms against `MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT`. |
| DEL-01-04-R03 | Report notice language shall say that code-specific data is user-supplied and that professional reliance requires competent human review. | SPEC.md Section 8; OPS-K-REPORT-1 | Report-template checklist includes both notices. |
| DEL-01-04-R04 | Product and release claims shall disclose scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | OPS-K-GOV-3 | Release/review checklist references these disclosure categories. |
| DEL-01-04-R05 | Human acceptance records, if implemented, shall bind to specific model, rule-pack, and report hashes and require re-review after content changes. | OPS-K-AUTH-2 | Acceptance-record schema or procedure includes hash binding and invalidation rule. |
| DEL-01-04-R06 | Draft and generated outputs shall remain proposals until accepted by a human gate. | OPS-K-AGENT-4; AGENTIC_DEVELOPMENT_WORKFLOW.md | Lifecycle/status language avoids issued/accepted claims unless human approval exists. |
| DEL-01-04-R07 | Unknown policy choices, legal interpretations, or jurisdiction-specific professional obligations shall be marked `TBD` and escalated for human/legal/professional review. | OPS-K-AGENT-1; DIRECTIVE.md stop rules | Review confirms no legal conclusion is stated as fact. |
| DEL-01-04-R08 | Public notices and examples shall not reproduce protected standards content or proprietary engineering data. | OPS-K-IP-1; OPS-K-REPORT-2 | Protected-content review gate. |
| DEL-01-04-R09 | Exact repo-level professional-boundary wording and report notice text shall remain `TBD` until an authorized repo-level task and human review gate approve them. | _CONTEXT.md write-scope limit; OPS-K-AGENT-4 | Review confirms this local kit does not present repo-level text as issued policy. |

## Standards

| Reference | Applicability | Location |
|---|---|---|
| docs/CONTRACT.md | Governing invariant catalog for authority, reporting, governance, and agent boundaries. | Repo-local |
| docs/DIRECTIVE.md | Founding intent and stop rules, including human authority and data-boundary principles. | Repo-local |
| docs/TYPES.md | Vocabulary for analysis statuses, epistemic labels, and professional approval. | Repo-local |
| docs/SPEC.md | Report and architecture requirements that notices must align with. | Repo-local |
| Protected engineering codes/standards | Not a source for this draft unless lawfully available and explicitly authorized; do not reproduce. | TBD |

## Verification

| Check | Method | Pass Condition |
|---|---|---|
| Prohibited claim scan | Search policy/report notice text for certification, approval, seal, authentication, compliance-for-reliance, official endorsement, or equivalent language. | Any such term is absent or clearly negated as outside software authority. |
| Status vocabulary scan | Compare policy/report text to TYPES.md. | No automatic `CODE_COMPLIANT` status or equivalent is introduced. |
| Report notice completeness | Review report-template candidate language. | Includes software/version/provenance/warnings/limitations and human-review notice categories. |
| Human gate trace | Review acceptance-record references. | Human approval, if present, is hash-bound and draft/proposal status remains clear. |
| Protected-content screen | Review public examples/notices. | No protected standards text, tables, figures, examples, or proprietary data are reproduced. |
| Human wording approval | Review proposed repo-level notice language. | Exact notice text is accepted by the human project authority before repo-level publication. |

## Documentation

Anticipated repo-level artifacts for a later authorized task:

- `docs/PROFESSIONAL_BOUNDARY.md`
- report notice template

For this setup workflow, those artifacts are discussed only in this deliverable-local kit.

## Open TBDs

| TBD ID | Item | Resolution Owner |
|---|---|---|
| DEL-01-04-TBD-01 | Final text of `docs/PROFESSIONAL_BOUNDARY.md`. | Human project authority after authorized repo-level task |
| DEL-01-04-TBD-02 | Final report notice template wording. | Human project authority after authorized repo-level task |
| DEL-01-04-TBD-03 | Jurisdiction-specific legal/professional-practice language, if any. | Qualified human/legal/professional reviewer |
