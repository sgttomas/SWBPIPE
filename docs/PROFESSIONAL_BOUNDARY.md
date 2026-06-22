---
doc_id: OPS-PROFESSIONAL-BOUNDARY
doc_kind: governance.policy
status: draft
created: 2026-05-01
deliverable_id: DEL-01-04
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: informs
    to: OPS-SPEC
---

# Professional Boundary and Product Claims Policy

## 1. Purpose

This policy defines the professional-responsibility boundary for
OpenPipeStress product, report, documentation, release, and agent claims.

OpenPipeStress is decision-support software for transparent piping mechanics,
analysis-grade design-engine workflows, and stress-model authoring. It may
compute mechanics, evaluate user-supplied rule packs, record diagnostics,
support design-authoring and comparison workflows, and assemble auditable
reports. It does not replace competent professional judgment for a
project-specific piping calculation.

This policy is project governance. It is not legal advice, professional
engineering approval, certification, sealing, standards-body endorsement, or a
code-compliance determination.

## 2. Current Authority Basis

This draft policy is aligned to
`execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` and approved
`DAG-007` coordination authority. The project license has been selected as
`PolyForm-Noncommercial-1.0.0`; this is a project governance notice and not
legal advice, professional approval, or code-compliance evidence.

The final contributor legal mechanism, maintainer roster/quorum, release
authority, legal-review authority, security contact, release-label vocabulary,
human-acceptance workflow, and jurisdiction-specific professional-practice
wording remain `TBD` until recorded by the human project authority or qualified
reviewer.

## 3. Authority Boundaries

| Surface | What it may state | What it must not state |
|---|---|---|
| Solver | Mechanical solve status, numerical results, diagnostics, warnings, and limitations. | That a piping design is professionally approved or code-compliant for reliance. |
| Rule pack | User-defined check results using recorded user inputs, formulas, and allowables. | That the software has authenticated, sealed, or professionally approved the design. |
| Report | Inputs, versions, hashes, provenance, warnings, assumptions, results, rule-pack references, limitations, and review notices. | That the report itself certifies, seals, endorses, or approves engineering work. |
| Design-engine workflow | Model-authoring support, comparison views, handoff packages, export packages, warnings, assumptions, and review aids. | That design-authoring, export, or comparison output is professional acceptance or external solver validation. |
| Agent output | Drafts, proposals, evidence summaries, checks, and open issues. | That agent output is accepted policy, accepted engineering work, or professional approval without a human gate. |
| Release notes | Implemented scope, validation evidence, known limitations, data-boundary constraints, and professional-boundary notices. | That a release is suitable for project-specific reliance without competent human review. |

## 4. Permitted Claims

OpenPipeStress materials may claim supported behavior when the claim is backed
by implementation, tests, documentation, or recorded evidence. Permitted claims
include:

- mechanics results computed from a recorded model and load basis;
- user-rule check results computed from user-supplied rule-pack definitions and
  user-supplied data;
- diagnostics, warnings, missing-input findings, assumptions, and limitations;
- design-authoring, comparison, handoff, and export support when described as
  review support or workflow evidence rather than professional approval;
- source, provenance, redistribution, version, checksum, and hash records;
- validation or regression evidence for software behavior;
- report generation for competent human review;
- local-first handling of private project, rule-pack, material, and component
  data when implemented and tested.

Claims must be specific to the feature, version, data, and evidence available.
Unsupported claims remain `TBD`, `ASSUMPTION`, or `PROPOSAL`.

## 5. Prohibited Claims

OpenPipeStress software, reports, agents, examples, releases, and public
documentation must not state or imply that the project:

- certifies, seals, approves, authenticates, or endorses engineering work;
- declares engineering code compliance for reliance;
- provides standards-body approval or official interpretation;
- replaces the engineer of record or other competent professional reviewer;
- removes the user's responsibility to supply applicable code, owner,
  material, component, load, SIF, flexibility, allowable, and project data;
- makes public example data suitable for real project use;
- turns user-rule pass/fail results into professional approval;
- treats design-engine, comparison, handoff, export, or external-prover
  metadata as automatic professional validation;
- provides legal advice or jurisdiction-specific professional-practice advice.

Prohibited language is unacceptable even when convenient for marketing, release
notes, generated reports, examples, or UI labels.

## 6. Status Vocabulary

Product and report language must preserve these distinctions:

| Status concept | Meaning |
|---|---|
| Mechanics solved | The numerical mechanics problem reached a recorded solve state. |
| User-rule checked | A user-supplied rule pack evaluated recorded inputs and results. |
| Human review required | Competent review remains required before professional reliance. |
| Human accepted for project | An external human acceptance record exists for a specific model, rule-pack, and report basis. |

Software must not emit automatic professional-approval or automatic
code-compliance statuses. Human acceptance records are external governance or
project records, not solver-generated conclusions.

## 7. Human Acceptance Records

If OpenPipeStress stores or references a human acceptance record, that record
must:

- identify the human acceptance authority or external record location;
- bind to the applicable model hash, rule-pack checksum, report hash, software
  version, and relevant input manifest;
- identify the scope and limitations of the acceptance;
- become stale or invalid for reliance when any bound model, rule-pack, report,
  input manifest, or relevant software basis changes;
- remain separate from solver, rule-pack, and report-generation outputs.

The exact acceptance-record storage workflow remains `TBD` until implemented by
the relevant persistence, report, and governance deliverables.

## 8. Report Notice Requirements

Generated reports and public report templates must include notices that:

- identify OpenPipeStress as decision-support software;
- distinguish mechanics results from user-rule checks and human professional
  acceptance;
- state that code-specific and project-specific data are supplied by the user
  or by user-controlled private sources;
- disclose warnings, assumptions, missing data, provenance notes, versions,
  hashes, and limitations;
- state that competent human review is required before professional reliance;
- describe design-authoring, comparison, handoff, export, and external-prover
  metadata as workflow evidence only, where included;
- avoid protected standards text, protected tables, proprietary engineering
  values, private project data, and private rule-pack content unless the user
  intentionally includes private material in a private report workflow.

The baseline notice template is `docs/report_notice_template.md`.

## 9. Product-Claim Review Checklist

Before merging public-facing claims, generated notices, report templates,
examples, release notes, or documentation, maintainers must check:

| Check | Required result |
|---|---|
| Evidence support | Claim is supported by implementation, tests, source evidence, or an explicit human ruling. |
| Professional boundary | Claim does not state or imply certification, sealing, endorsement, approval, or code compliance for reliance. |
| Status separation | Mechanics solve, user-rule check, missing data, and human acceptance remain separate. |
| Protected content | Claim or notice does not reproduce protected standards text, tables, figures, examples, code-derived formulas, or proprietary data. |
| Private data | Claim or report path does not expose private project, rule-pack, material, component, owner-standard, or company data by default. |
| Limitations | Known limitations, validation status, data-boundary constraints, and professional-responsibility limitations are visible. |

If a claim is ambiguous, stop and route it to human review before publication.

## 10. Open TBDs

| TBD ID | Item | Owner |
|---|---|---|
| PB-TBD-001 | Jurisdiction-specific legal or professional-practice wording, if any. | Qualified human/legal/professional reviewer |
| PB-TBD-002 | Exact storage and invalidation workflow for human acceptance records. | Future persistence/report/governance deliverables |
| PB-TBD-003 | Release-label vocabulary and final release policy language. | Human project authority |
| PB-TBD-004 | Legal-review authority for product-claim and professional-practice wording. | Human project authority |
| PB-TBD-005 | Final acceptance or revision of this draft policy and report notice template. | Human project authority |
