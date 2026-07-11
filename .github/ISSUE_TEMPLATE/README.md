---
doc_id: OPS-PUBLIC-ISSUE-TEMPLATES
doc_kind: governance.public_issue_templates
status: draft
created: 2026-07-10
deliverable_id: DEL-01-03
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: OPS-IP-DATA-BOUNDARY
  - rel: informed_by
    to: OPS-CONTRIBUTING
---

# Public Issue Templates (Future Public Repository)

This directory holds the OpenPipeStress **public issue templates** named as a
PRD §22.6 deliverable ("Public issue templates"; completion-plan row E6 in
`plans/PLAN_2026-06-17_prd_completion.md`). They are GitHub issue forms:

| File | Purpose |
|---|---|
| `config.yml` | Disables blank issues so every report passes through a form carrying the data-boundary caution. |
| `bug_report.yml` | Defect / incorrect-behavior reports. |
| `question.yml` | Usage, build, and understanding questions. |
| `documentation_feedback.yml` | Unclear, incorrect, or missing documentation. |

## Where these templates live and where they apply

The public OpenPipeStress repository **does not exist yet**; its creation is
an owner-gated publication act. These templates are therefore authored
in-project at `projects/chirality-piping/.github/ISSUE_TEMPLATE/` and are
**inert here** — GitHub only reads `.github/` at a repository root, so nothing
in this directory changes the behavior of the private monorepo. They are
intended to ship with the future public export, landing at the public
repository's root `.github/ISSUE_TEMPLATE/`.

## Contribution posture (controlling governance)

- **`DEC-027`** (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12): external
  contributions are **not accepted at this time**; maintainer quorum is one;
  no contributor legal mechanism (CLA/DCO or equivalent) is stood up. The
  templates therefore state that code contributions are closed and that the
  tracker is for issue reports and questions only.
- **`D-07b`** (contributor intake if/when contributions open) is
  **AWAITING_RULING**. No CONTRIBUTING intake mechanism, contributor legal
  mechanism, or pull-request workflow is created by these templates, and none
  may be added to them until that decision is ruled by the human project
  authority.
- **PRD §17.5 pre-release legal review** remains pending: before public
  release, legal counsel review of documentation language, disclaimers, and
  contribution policy — including these templates — is required. Authoring
  these files creates no release, publication, or legal-clearance claim.

## Content constraints carried by the forms

- Every form carries the public/private data-boundary caution
  (`docs/IP_AND_DATA_BOUNDARY.md`): no protected standards content (code
  text, tables, figures, examples, commentary, material allowables, SIF
  tables, flexibility-factor tables) and no private project data (real
  models, private libraries, private rule packs, proprietary vendor data,
  client-identifying information) in reports, and required confirmation
  checkboxes to that effect.
- No form promises review SLAs, response times, support levels, fixes, or
  releases.
- No form makes or implies release-readiness, professional approval,
  certification, sealing, authentication, standards-body endorsement, or
  code-compliance claims, and each states that this tracker is not an
  engineering-advice channel.

This README is project governance documentation, not legal advice.
