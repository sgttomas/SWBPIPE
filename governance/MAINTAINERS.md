---
doc_id: OPS-MAINTAINERS
doc_kind: governance.maintainer_policy
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: informed_by
    to: OPS-DIRECTIVE
---

# MAINTAINERS - OpenPipeStress Governance Baseline

This file defines the initial maintainer policy skeleton for OpenPipeStress. It is a governance artifact, not a legal opinion, professional engineering approval, certification, sealing, or code-compliance claim.

## 1. Project Status

| Field | Current value |
|---|---|
| Project intent | Free and open-source piping stress analysis platform |
| License | `TBD` |
| Contributor certification mechanism | Interim project certification template in `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`; final DCO/CLA/other mechanism remains `TBD` |
| Maintainer roster | `TBD` |
| Release authority | `TBD` |
| Security contact | `TBD` |
| Human project authority record | `TBD` |

No contributor, maintainer, agent, or release note may state a final license or final governance authority until the corresponding `TBD` is resolved by the human project authority.

## 2. Maintainer Responsibilities

Maintainers are responsible for project stewardship within the boundaries in `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and `docs/VALIDATION_STRATEGY.md`.

Maintainers must:

- preserve stable package, deliverable, scope, objective, decision, issue, and risk IDs;
- enforce the open-mechanics/private-code-data boundary;
- require source, provenance, license or redistribution status, contributor certification, and review disposition for public data contributions;
- quarantine suspected protected standards content, proprietary data, or private project data before merge or release;
- keep missing engineering, legal, governance, or release choices as `TBD`;
- require tests, validation evidence, or explicit deferrals appropriate to the changed surface;
- prevent product claims that imply certification, endorsement, sealing, authentication, or automatic code compliance;
- record accepted governance decisions in public artifacts or decision records.

## 3. Contribution Review Gates

Every public contribution must pass these gates before merge or have an explicit recorded maintainer deferral:

| Gate | Required check |
|---|---|
| Scope and identity | Change maps to an accepted deliverable, issue, or maintainer-approved maintenance task. |
| IP and provenance | Contribution contains no protected standards text, copied standards tables, proprietary vendor data, private rule packs, or private project data without documented redistribution rights. |
| Data boundary | Public examples use invented, original, public-domain, or permissively licensed data only. |
| Tests and evidence | Required tests, validation checks, documentation updates, or explicit deferrals are present. |
| Product claims | User-facing text does not claim certification, approval, endorsement, sealing, or code compliance for reliance. |
| Privacy | Telemetry, bug reports, examples, and exports do not include private engineering data by default. |

Public data contributions must also record these evidence slots before acceptance:

| Evidence slot | Required value |
|---|---|
| Source identity | Source name and source location, or `TBD` if the source is still under review. |
| Redistribution basis | License or redistribution status, including whether the data is public, private-only, unknown, or suspected protected content. |
| Contributor certification | Contributor statement that the data is original, invented, public-domain, permissively licensed, or otherwise redistributable. |
| Review disposition | `pending`, `accepted`, `rejected`, `quarantined`, or `TBD` with a reason. |
| Quarantine status | Required when protected standards content, proprietary data, or private project data is suspected. |
| Private-data risk | Review note confirming whether the contribution can expose private project, rule-pack, material, component, owner-standard, or company design-basis data. |

The default contributor certification record is
`governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`. The default review record
for these gates is `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`. A maintainer
may use a project issue, pull-request template, or release checklist instead
only when it preserves the same evidence fields and disposition states.

## 4. Release Policy Skeleton

A public release must not be published until maintainers complete a release review covering:

- release scope, changed surfaces, and any excluded or deferred work;
- selected license and notices; if the license remains `TBD`, no public source release may be published without a recorded human exception;
- validation status and known limitations;
- data-boundary constraints and professional-responsibility notices;
- protected-content and provenance review for public examples, templates, and bundled data;
- private-data and telemetry review;
- reproducibility evidence such as version, commit, build, schema, model, and rule-pack checksum behavior where applicable;
- human review of release notes and product claims.

Release maturity labels remain `TBD`. No release label may imply professional reliance, code compliance, certification, endorsement, or engineering approval.

## 5. Decision Records

Governance decisions that affect license, contributor certification, maintainer authority, release criteria, public-data acceptance, professional-responsibility language, security reporting, or private-data handling must be recorded with:

- decision ID;
- date;
- decision status;
- decision owner or human authority;
- affected documents or packages;
- rationale and source references;
- reconsideration trigger.

The ADR location and numbering convention remain `TBD`.

## 6. Open Governance Questions

| ID | Question | Status |
|---|---|---|
| GOV-TBD-001 | Which open-source license will the project use? | `TBD` |
| GOV-TBD-002 | Will the project use a Developer Certificate of Origin, contributor license agreement, or another contributor certification mechanism? | `TBD` |
| GOV-TBD-003 | Who are the initial maintainers and what quorum is required for policy changes? | `TBD` |
| GOV-TBD-004 | What release signing and artifact-retention process will be used? | `TBD` |
| GOV-TBD-005 | What security disclosure channel and response process will be used? | `TBD` |
| GOV-TBD-006 | What public-data sources, if any, are acceptable beyond invented, original, public-domain, or permissively licensed examples? | `TBD` |

## 7. Stop Rules

Maintainers must pause review and escalate when:

- a contribution may contain protected standards text, tables, figures, examples, copied formulas, or proprietary data;
- a contribution lacks source, provenance, license or redistribution status, or contributor certification;
- a user-facing claim could be read as certification, endorsement, sealing, authentication, or professional code-compliance approval;
- a proposed public release has unresolved protected-content, private-data, validation, or product-claim issues;
- a governance decision would resolve a `TBD` without a recorded human ruling.
