---
doc_id: OPS-CONTRIBUTING
doc_kind: governance.contributor_workflow
status: draft
created: 2026-05-01
deliverable_id: DEL-01-03
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: OPS-IP-DATA-BOUNDARY
  - rel: uses_template
    to: OPS-CONTRIBUTOR-CERTIFICATION-TEMPLATE
---

# Contributing to OpenPipeStress

OpenPipeStress accepts contributions only within the project boundaries in
`docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and
`docs/PROFESSIONAL_BOUNDARY.md`.

New contributors should start with the tutorial path in
`docs/contributor_guide/index.md` before changing files. The tutorial is a
navigation aid only; the governance documents above, an assigned sealed brief,
and human project-authority decisions remain controlling when they conflict.

This workflow is project governance. It is not legal advice, professional
engineering approval, certification, sealing, standards-body endorsement, or a
code-compliance determination.

## 1. Contribution Scope

Contributions may include code, schemas, documentation, tests, examples,
templates, benchmark cases, or data records when they respect the public/private
data boundary.

Public contributions must not include:

- protected standards text, tables, figures, examples, commentary, or copied
  formulas;
- material allowables, SIF/flexibility tables, protected dimensional/rating
  tables, or code-specific acceptance values copied from protected sources;
- proprietary vendor catalog data, commercial software examples, report
  templates, or benchmark files without documented redistribution rights;
- private project models, owner standards, company design bases, private rule
  packs, or private material/component libraries.

When rights or source status are unclear, leave the value as `TBD` and route
the contribution for review. Do not infer redistribution rights from public
availability, usefulness, or common industry use.

## 2. Required Contributor Certification

Every public data, example, template, report, benchmark, rule-pack example, or
library-record contribution must include a completed contributor certification
record using `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` or an approved
review form with the same fields.

At minimum, the contributor must record:

| Field | Requirement |
|---|---|
| Contributor identity | Person or organization submitting the contribution. |
| Changed content | Paths, records, or artifacts covered by the certification. |
| Source name and location | Origin of the submitted material, or `TBD` if still under review. |
| Source license / redistribution basis | Public-domain, permissive license, original, invented, private-only, permission record, `unknown`, or `TBD`. |
| Contributor statement | Statement that the contributor has the right to submit the material for repository review under the project contribution process. |
| Redistribution status | `public_permissive`, `private_only`, `unknown`, `protected_suspected`, or `rejected`. |
| Protected-content screen | Confirmation that protected standards/proprietary content was not copied, or an escalation note. |
| Private-data screen | Confirmation that private project, rule-pack, material, component, owner-standard, and company data are excluded unless intentionally submitted with documented rights. |

The final project-wide mechanism, such as DCO, CLA, or another legal instrument,
remains `TBD` until the human project authority records a decision.

## 3. Review Routing

Maintainers review contributions through
`governance/CONTRIBUTION_REVIEW_CHECKLIST.md` or an equivalent record that
preserves the same evidence fields.

Use this routing:

| Evidence state | Route |
|---|---|
| Source, rights, certification, and redistribution evidence are complete | Maintainer review for protected-content, private-data, product-claim, and test evidence. |
| Source, rights, or certification evidence is missing | Keep as `pending`, request evidence, or reject if evidence cannot be provided. |
| Redistribution status is `unknown` | Do not accept as public data; request clarification or reject. |
| Redistribution status is `private_only` | Keep outside public examples and bundled public data. |
| Redistribution status is `protected_suspected` | Stop review, quarantine metadata, and escalate for human/legal review. |
| Contribution includes product or report claims | Check against `docs/PROFESSIONAL_BOUNDARY.md` before merge. |

Maintainer acceptance is repository governance only. It does not approve a
piping calculation, certify engineering work, or determine code compliance for
professional reliance.

## 4. Rejection And Quarantine Rules

A public contribution must be rejected, quarantined, or kept private-only when:

- required source, rights, certification, or redistribution evidence is missing;
- protected standards or proprietary source content is suspected;
- private project, rule-pack, material, component, owner-standard, company, or
  client data may be exposed;
- a public example uses non-invented or uncleared engineering values;
- the contribution claims certification, sealing, endorsement, professional
  approval, or code compliance for reliance.

Quarantine records must describe metadata and routing only. Do not copy
suspected protected text, tables, examples, or values into public notes while
the issue is unresolved.

## 5. Tests And Evidence

Contributions must include tests, scans, or explicit deferrals appropriate to
the changed surface. Examples include schema validation, protected-content
screening, provenance checks, unit tests, documentation review, or report
notice review.

Missing tests or unresolved review questions remain explicit `TBD` items until
a maintainer records an accepted deferral or the contributor supplies the
evidence.
