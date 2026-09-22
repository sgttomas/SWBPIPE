---
doc_id: OPS-CONTRIBUTOR-CERTIFICATION-TEMPLATE
doc_kind: governance.certification_template
status: draft
created: 2026-05-01
deliverable_id: DEL-01-03
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: OPS-IP-DATA-BOUNDARY
  - rel: used_by
    to: OPS-CONTRIBUTING
---

# Contributor Certification Template

This template records contributor source, rights, redistribution, and review
evidence for public SWBPIPE contributions. It is project governance. It
is not legal advice, professional engineering approval, certification, sealing,
standards-body endorsement, or a code-compliance determination.

## Current Authority Basis

This draft template is aligned to
the accepted `execution/_Decomposition/SOFTWARE_DECOMP.md` and the approved
graph selected by `execution/_DAG/_LATEST.md`. The project license has been selected as
`MIT`; source-license and redistribution fields below
record the submitted material's basis and do not replace that project license
notice.

DEC-027 records a quorum of one: the sole human project authority is the sole
maintainer and release authority. External contribution intake remains closed.
DEC-079 adopts the D-07b sole-maintainer review and protected-content screening
skeleton and records “process defined; intake closed”; activation requires a
future owner decision. The final contributor legal instrument remains `TBD`
until that activation decision, with legal advice obtained by the owner. No
legal instrument or release is authorized by this current-basis correction.

Legal-review authority, security contact, release-label vocabulary,
human-acceptance workflow, and jurisdiction-specific professional-practice
wording retain their own unresolved decisions and qualified-review routes.
See DEC-027/DEC-079 in `execution/_Decomposition/SOFTWARE_DECOMP.md` and
`execution/_Coordination/_DECISIONS/D-07b_contributor_intake_mechanism.md` §10.
Blank per-contribution fields remain unfilled evidence slots.

## 1. Contribution Identity

| Field | Value |
|---|---|
| Certification ID | `TBD` |
| Contribution title | `TBD` |
| Contributor name / organization | `TBD` |
| Contributor contact or account | `TBD` |
| Related issue / PR / deliverable | `TBD` |
| Covered paths / records | `TBD` |
| Contribution type | `code`, `schema`, `documentation`, `example`, `benchmark`, `data`, `template`, `rule_pack_example`, `other` |
| Date submitted | `TBD` |

## 2. Source And Rights Statement

| Field | Value |
|---|---|
| Source name | `TBD` |
| Source location | `TBD` |
| Source type | `invented`, `original`, `public_domain`, `permissive_license`, `private_user_supplied`, `proprietary_with_permission`, `unknown`, `protected_suspected` |
| Source license / redistribution basis | `TBD` |
| Permission record or license note | `TBD` |
| Redistribution status | `public_permissive`, `private_only`, `unknown`, `protected_suspected`, `rejected` |

Contributor statement:

> I certify that, to the best of my knowledge, I have the right to submit the
> covered contribution for SWBPIPE repository review under the recorded
> source and redistribution basis. I have not knowingly copied protected
> standards text, protected tables, protected examples, proprietary vendor data,
> private project data, private rule packs, owner standards, company design
> bases, or commercial software material into the public contribution unless the
> recorded evidence shows that redistribution is permitted.

| Field | Value |
|---|---|
| Contributor signature / account affirmation | `TBD` |
| Certification date | `TBD` |

The project license is `MIT`. The final project-wide
contributor mechanism, such as DCO, CLA, or another legal instrument, remains
`TBD`.

## 3. Protected-Content Screen

| Check | Contributor response | Notes |
|---|---|---|
| No protected standards text, commentary, figures, or examples are copied | `pending` | `TBD` |
| No protected tables, allowables, SIF/flexibility factors, dimensions, ratings, or copied formulas are included | `pending` | `TBD` |
| No proprietary vendor catalog data or commercial software examples/templates are included without permission | `pending` | `TBD` |
| Public examples use invented, original, public-domain, or permissively licensed values only | `pending` | `TBD` |

Allowed response values are `yes`, `no`, `not_applicable`, and `needs_review`.

## 4. Private-Data Screen

| Check | Contributor response | Notes |
|---|---|---|
| No private project, client, owner-standard, or company design-basis data is included | `pending` | `TBD` |
| No private material, component, rule-pack, allowable, SIF, flexibility, or load-combination data is included | `pending` | `TBD` |
| Screenshots, logs, reports, and examples exclude private engineering data by default | `pending` | `TBD` |
| Any private-only content is marked private-only and excluded from public examples or bundled data | `pending` | `TBD` |

## 5. Maintainer Review Disposition

| Field | Value |
|---|---|
| Review record | `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` or equivalent |
| Reviewer | `TBD` |
| Review disposition | `pending`, `accepted`, `rejected`, `quarantined`, `private_only`, `escalated`, `TBD` |
| Disposition rationale | `TBD` |
| Human/legal escalation needed | `yes`, `no`, `TBD` |
| Quarantine path or issue, if applicable | `TBD` |

`accepted` is allowed only when source rights, redistribution status,
contributor certification, protected-content risk, private-data risk, and
product-claim review are closed or explicitly deferred by recorded maintainer
decision.
