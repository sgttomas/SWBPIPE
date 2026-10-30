# SWBPIPE Contributor Guide

External contribution intake remains closed until the owner activates it with
the required contributor/legal arrangements. The project license is MIT;
maintainer acceptance is not professional engineering approval.

For authorized development, start with Root `AGENTS.md` and
[LOOP_INIT](../../loop/LOOP_INIT.md). Read the relevant deliverable’s
ScopeOfWork, Design and dependency conditions; use the Root deliverable CLI
to find connected work. No sealed brief, lifecycle transition or handoff packet
is required. The PR states what changed, why, checks and open decisions.

## Data And Provenance Boundary

Public contribution content may include open mechanics, schemas, workflow
documentation, blank templates, invented examples, original examples, and
public-domain or permissively licensed records with documented provenance.

Public contribution content must not include:

- protected standards text, tables, figures, examples, commentary, or copied
  code-derived formulas;
- material allowables, stress-intensification values, flexibility factors,
  dimensional tables, rating tables, or code-specific acceptance values copied
  from protected sources;
- proprietary vendor catalogs, commercial software examples, commercial report
  templates, or benchmark files without documented redistribution rights;
- private project models, owner standards, company design bases, client data,
  private rule packs, or private material/component libraries;
- real secrets, credentials, private paths, tokens, or logs containing private
  engineering data.

When source status is unclear, leave the field as `TBD`, mark the risk, and
route the contribution for review. Do not infer rights from public
availability, usefulness, common industry practice, or a source being easy to
find online.

For public data, examples, report templates, benchmark cases, or documentation
excerpts, use the source-rights and review surfaces in
[`CONTRIBUTING.md`](../../CONTRIBUTING.md),
[`governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`](../../governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md),
and
[`governance/CONTRIBUTION_REVIEW_CHECKLIST.md`](../../governance/CONTRIBUTION_REVIEW_CHECKLIST.md).
The final project-wide legal mechanism remains `TBD` until the human project
authority records it.

## Verification and decisions

Run focused checks for the consequential behaviour being changed. The required
Root `harness` result aggregates selected hosted checks; do not repeat full
local suites just to produce another record. Release candidates receive their
actual product checks and separate owner acceptance.

Keep source, unit, diagnostic, privacy and professional boundaries intact.
Missing engineering data is explicit, never silently defaulted. Changes to
commitments or acceptance criteria, releases and risk acceptance remain owner
reserved. Report unavailable or failed verification honestly in the PR.

See the [developer guide](../developer_guide/index.md), [product requirements](../PRD.md),
[contract](../CONTRACT.md) and [build/release guide](../BUILD_AND_RELEASE.md).
