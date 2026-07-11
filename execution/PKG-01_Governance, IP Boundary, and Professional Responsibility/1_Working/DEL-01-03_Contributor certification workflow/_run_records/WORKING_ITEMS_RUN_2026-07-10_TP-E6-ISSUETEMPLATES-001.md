# WORKING_ITEMS RUN - TP-E6-ISSUETEMPLATES-001

## Scope

- Package: PKG-01 Governance, IP Boundary, and Professional Responsibility.
- Primary deliverable: DEL-01-03 Contributor certification workflow.
- Tranche: `TP-E6-ISSUETEMPLATES-001` — author the **public issue templates**
  named by completion-plan row E6
  (`plans/PLAN_2026-06-17_prd_completion.md`) as a PRD §22.6 deliverable gap
  ("Public issue templates"; `.github/ISSUE_TEMPLATE/` did not exist).
  Docs-only; no code, schema, app, or lifecycle surface touched.
- Run-record placement basis: row E6 maps the community/IP surface to
  DEL-01-02/03 and DEL-11-05. The existing public community-intake surface
  (`CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`,
  `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`) carries
  `deliverable_id: DEL-01-03` and its SOW basis (SOW-028 contributor
  governance / intake controls, SOW-048 governance policies) is where public
  issue intake lives, so DEL-01-03 owns this record. DEL-01-02 (boundary
  policy wording) and DEL-11-05 (onboarding docs) are informed, not changed.
- Authority context: `DEC-027` (contributions closed; D-07b gates any future
  intake), `DEC-056` rider (v0.1 R5 release-machinery deliverables including
  issue templates carried as explicit R6-entry residuals), PRD §17.5
  (pre-release legal review), PRD §22.6 (deliverable list).

## Primary Evidence

- New directory `projects/chirality-piping/.github/ISSUE_TEMPLATE/`
  (project-local; verified absent before this tranche; **inert** in the
  private monorepo because GitHub only reads repo-root `.github/`, and the
  monorepo root `.github/` was not touched):
  - `config.yml` — disables blank issues so every report passes through a
    form carrying the data-boundary caution.
  - `bug_report.yml` — defect reports: what happened / expected / steps to
    reproduce (invented data only) / version / platform / redacted logs.
  - `question.yml` — usage/build/understanding questions; states explicitly
    it is not an engineering-advice or code-compliance channel.
  - `documentation_feedback.yml` — unclear/incorrect/missing documentation;
    suggested wording must not reproduce protected content.
  - `README.md` — explains the templates ship with the future public export
    (repository creation is owner-gated and does not exist), and that the
    contribution posture is governed by `DEC-027` / `D-07b` with PRD §17.5
    legal review pending.
- Content constraints honored in every form:
  - `DEC-027`: states external code contributions are **not currently
    accepted** (issue reports and questions only); no pull-request or patch
    intake is invited; no CONTRIBUTING intake mechanism authored (D-07b is
    AWAITING_RULING).
  - No review SLAs, response times, support levels, fixes, or releases are
    promised; each form says reports are read as maintainer capacity allows
    and create no obligation.
  - F-PIP-1 posture: standard caution against including protected standards
    content (code text, tables, figures, examples, commentary, material
    allowables, SIF tables, flexibility-factor tables) or private project
    data (real models, private libraries, private rule packs, proprietary
    vendor data, client-identifying information, secrets), plus required
    data-boundary confirmation checkboxes; invented example data only.
  - F-PIP-2 posture: no release-readiness, certification, professional
    approval, sealing, authentication, endorsement, or code-compliance
    language; PRD §17.4 non-affiliation wording carried; "not an
    engineering-advice channel" stated.
- YAML issue forms validated structurally (`yaml.safe_load` plus
  required-key/body-element-type/checkbox-option checks) — all four files
  parse and conform to the GitHub issue-form element set.

## Validation

- Repo-root `PYTHONDONTWRITEBYTECODE=1 python3
  tools/practitioner_harness/harness.py self-check` — see closeout section
  for exit status at the committed head.
- DEC-025 five-surface sweep: `_COORDINATION.md` workflow step 9 requires the
  sweep before pushing on parallel agent development branches without a
  docs-only carve-out, so it is run at the committed clean head (after
  `npm ci` in `apps/desktop` for the fresh worktree) and the summary
  committed; result recorded in the closeout addendum below.

## Boundary

- F-PIP-1 held: docs-only, local-only; no network features, no private data,
  no protected standards content (the templates *name* protected categories
  in cautions only; no values, text, or tables are embedded).
- F-PIP-2 held: no release-readiness, certification, professional-approval,
  sealing, authentication, or code-compliance claim is made or implied; the
  README states authoring these files creates no release, publication, or
  legal-clearance claim.
- D-07b untouched: no CONTRIBUTING.md edits, no contributor legal mechanism,
  no intake mechanism of any kind authored; the README records that none may
  be added until the human project authority rules D-07b.
- Owner-gated acts untouched: no public repository created, no publication or
  export performed, no monorepo-root `.github/` writes, no lifecycle
  transition, no legal review claimed (PRD §17.5 review remains pending and
  human-owned).
- Open PR surfaces (#158/#160/#161/#162/#163), including
  `tools/release/export_public_openpipestress.py`, were not depended on or
  edited; wiring these templates into the public-export manifest is a
  follow-on once that surface is in main.

## Residuals

- §17.5 pre-release legal review of these templates (with all public-facing
  language) — human/counsel, before any publication.
- D-07b ruling gates any future contribution-intake wording change.
- Export wiring: include `projects/chirality-piping/.github/ISSUE_TEMPLATE/`
  at the public repository root `.github/ISSUE_TEMPLATE/` in the sanitized
  public export when that machinery (PR #161 surface / D-05b / D-06) lands
  and is exercised.
