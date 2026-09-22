---
doc_id: OPS-CONTRIBUTION-REVIEW-CHECKLIST
doc_kind: governance.review_checklist
status: draft
created: 2026-05-01
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: OPS-IP-DATA-BOUNDARY
  - rel: informs
    to: OPS-MAINTAINERS
  - rel: uses_template
    to: OPS-CONTRIBUTOR-CERTIFICATION-TEMPLATE
---

# Contribution Review Checklist

This checklist is a project-governance intake record for public repository contributions. It is not legal advice, professional engineering approval, certification, sealing, standards-body endorsement, or a code-compliance claim.

Use this checklist before accepting contributions that add or modify public data, examples, report templates, benchmark cases, import fixtures, documentation excerpts, rule-pack examples, component/material/library records, or generated artifacts that may expose private or protected content.

## Current Authority Basis

This draft checklist is aligned to
the accepted `execution/_Decomposition/SOFTWARE_DECOMP.md` and the approved
graph selected by `execution/_DAG/_LATEST.md`. The project license has been selected as
`PolyForm-Noncommercial-1.0.0`; this checklist records contribution-source
rights and redistribution evidence separately from that project license notice.

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

| Field | Review value |
|---|---|
| Review ID | `TBD` |
| Contribution title | `TBD` |
| Contributor | `TBD` |
| Reviewer | `TBD` |
| Related deliverable / issue / PR | `TBD` |
| Changed paths | `TBD` |
| Contribution type | `code`, `schema`, `documentation`, `example`, `benchmark`, `data`, `template`, `other` |
| Project license notice | `PolyForm-Noncommercial-1.0.0` |
| Contributor certification record | `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` or equivalent |
| Review date | `TBD` |

## 2. Source And Rights Evidence

| Field | Review value |
|---|---|
| Source name | `TBD` |
| Source location | `TBD` |
| Source type | `invented`, `original`, `public_domain`, `permissive_license`, `private_user_supplied`, `proprietary_with_permission`, `unknown`, `protected_suspected` |
| Source license / redistribution basis | `TBD` |
| Contributor certification | `TBD` |
| Contributor certification statement accepted for review | `yes`, `no`, `TBD` |
| Redistribution status | `public_permissive`, `private_only`, `unknown`, `protected_suspected`, `rejected` |
| Evidence attachment / note | `TBD` |

Acceptance requires a recorded source, redistribution basis, contributor certification, and reviewer disposition. `unknown`, `protected_suspected`, or missing evidence cannot be accepted as public data.

Use `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` for the default
contributor certification record unless a maintainer-approved form preserves
the same source, rights, redistribution, protected-content, private-data, and
review fields. The project license is `PolyForm-Noncommercial-1.0.0`; the final
project-wide contributor legal mechanism remains `TBD`.

## 3. Protected-Content Screen

| Check | Result | Notes |
|---|---|---|
| No standards-body code text, commentary, figures, or examples copied into public content | `pending` | `TBD` |
| No copied material allowable tables, SIF tables, flexibility-factor tables, or protected dimensional/rating tables | `pending` | `TBD` |
| No copied commercial software examples, report templates, benchmark files, or proprietary formulas without permission | `pending` | `TBD` |
| No proprietary vendor catalog data without documented redistribution rights | `pending` | `TBD` |
| Public examples use invented, original, public-domain, or permissively licensed values only | `pending` | `TBD` |
| Documentation excerpts are original summaries or permissively licensed quotations within project policy | `pending` | `TBD` |

Allowed result values are `pass`, `fail`, `pending`, `not_applicable`, and `escalated`.

## 4. Private-Data Screen

| Check | Result | Notes |
|---|---|---|
| No private project model, owner standard, company design basis, or client data is included | `pending` | `TBD` |
| No private material, component, rule-pack, allowable, SIF, flexibility, or load-combination data is included | `pending` | `TBD` |
| Telemetry, issue logs, screenshots, and reports exclude private engineering data by default | `pending` | `TBD` |
| Private-only content is stored outside public examples and marked private-only where referenced | `pending` | `TBD` |

## 5. Engineering And Product-Claim Screen

| Check | Result | Notes |
|---|---|---|
| No claim of professional engineering approval, certification, sealing, authentication, or code compliance for reliance | `pending` | `TBD` |
| No standards-body endorsement or official approval claim | `pending` | `TBD` |
| Missing engineering values remain `TBD` or explicit findings rather than defaults | `pending` | `TBD` |
| Mechanics results, user rule checks, and human acceptance remain distinct | `pending` | `TBD` |

## 6. Tests And Evidence

| Field | Review value |
|---|---|
| Required tests / scans run | `TBD` |
| Protected-content scan result | `TBD` |
| Schema / provenance validation result | `TBD` |
| Contributor certification template review | `TBD` |
| Documentation review result | `TBD` |
| Explicit deferrals | `TBD` |

## 7. Disposition

| Field | Review value |
|---|---|
| Review disposition | `pending`, `accepted`, `rejected`, `quarantined`, `private_only`, `escalated`, `TBD` |
| Disposition rationale | `TBD` |
| Required follow-up | `TBD` |
| Human/legal escalation needed | `yes`, `no`, `TBD` |
| Approving maintainer / authority | `TBD` |

`accepted` is allowed only when source rights, protected-content risk, private-data risk, and product-claim review are closed or explicitly deferred by a recorded maintainer decision.

## 8. Quarantine Record

Complete this section when any protected or private content risk is unresolved.

| Field | Review value |
|---|---|
| Quarantine status | `not_required`, `required`, `complete`, `TBD` |
| Quarantine path | `quarantine/protected-content/` or maintainer-approved equivalent |
| Issue / decision record | `TBD` |
| Escalation owner | `TBD` |
| Resolution | `TBD` |

Quarantined material must not be copied into public examples, converted into public defaults, or used as public fixture data while the review remains unresolved.
