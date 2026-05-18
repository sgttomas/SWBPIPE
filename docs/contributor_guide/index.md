---
doc_id: OPS-CONTRIBUTOR-GUIDE
doc_kind: guide.contributor_onboarding
status: draft
created: 2026-05-09
deliverable_id: DEL-11-05
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-CONTRIBUTING
  - rel: explains
    to: OPS-AGENTIC-DEVELOPMENT-WORKFLOW
---

# Contributor Onboarding Guide

This guide is a tutorial path through the existing OpenPipeStress governance
and deliverable workflow. It does not replace `CONTRIBUTING.md`,
`docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`, or
`docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`. If this guide conflicts with an
assigned sealed brief, a governing document, or a human project-authority
instruction, stop and surface the conflict instead of guessing.

This guide is project workflow documentation. It is not legal advice,
professional engineering approval, sealing, standards-body endorsement, or a
code-compliance determination.

## First-Hour Path

Use this sequence before changing files.

| Step | Read | Why |
|---|---|---|
| 1 | [`INIT.md`](../../INIT.md) | Establish the required reading order and project boundaries. |
| 2 | [`AGENTS.md`](../../AGENTS.md) | Understand Type 1 routing roles, Type 2 execution roles, and the dispatch rule. |
| 3 | [`docs/DIRECTIVE.md`](../DIRECTIVE.md) | Review founding intent, product boundaries, and stop rules. |
| 4 | [`docs/CONTRACT.md`](../CONTRACT.md) | Check invariant IDs for IP, data, governance, privacy, and agent behavior. |
| 5 | [`docs/TYPES.md`](../TYPES.md) | Confirm package, deliverable, status, and professional-boundary vocabulary. |
| 6 | [`docs/IP_AND_DATA_BOUNDARY.md`](../IP_AND_DATA_BOUNDARY.md) | Confirm public/private data, provenance, and quarantine rules. |
| 7 | [`docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`](../AGENTIC_DEVELOPMENT_WORKFLOW.md) | Confirm the Type 1/Type 2 workflow, evidence expectations, and review gate. |
| 8 | [`execution/_Decomposition/SOFTWARE_DECOMP.md`](../../execution/_Decomposition/SOFTWARE_DECOMP.md) and [`docs/_Registers`](../_Registers/) | Confirm the active package/deliverable identity and scope rows. |
| 9 | Your assigned sealed brief and deliverable folder | Confirm the exact write scope, acceptance criteria, and verification commands. |

Do not begin implementation from memory. Work from the current files in the
repository and the current assignment.

## Repository Map

OpenPipeStress uses a flat package and deliverable decomposition:

```text
execution/_Decomposition/SOFTWARE_DECOMP.md
docs/_Registers/*.csv
execution/PKG-XX_.../1_Working/DEL-XX-YY_.../
```

Common surfaces:

| Path | Use |
|---|---|
| `docs/` | Governed project docs, architecture notes, validation strategy, and guide surfaces. |
| `docs/_Registers/` | Machine-readable scope, deliverable, and context-budget rows. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Current package/deliverable working surface. |
| `execution/PKG-*/1_Working/DEL-*/` | Deliverable working folders and local document kits. |
| `schemas/`, `core/`, `api/`, `validation/`, `examples/` | Product artifacts owned by specific deliverables. |
| `governance/` | Contribution review, maintainer, and source-rights workflow records. |
| `tools/validation/` and package-local test commands | Deterministic evidence helpers. |

The package ID owns the first two digits of the deliverable ID. For example,
`DEL-11-05` belongs to `PKG-11`. IDs persist across name changes; do not
renumber, reuse, or reinterpret them in contributor work.

## Package And Deliverable Work

Before editing, record the work identity:

| Field | Source |
|---|---|
| `DeliverableID` | Sealed brief and deliverable `_CONTEXT.md`. |
| `PackageID` | Sealed brief, `_CONTEXT.md`, and `docs/_Registers/Deliverables.csv`. |
| Scope items and objectives | `docs/_Registers/Deliverables.csv` and `docs/_Registers/ScopeLedger.csv`. |
| Applicable invariants | `docs/CONTRACT.md` and the sealed brief. |
| Acceptance criteria | Sealed brief or deliverable `_CONTEXT.md`. |
| Allowed write scope | Sealed brief or human assignment. |
| Verification commands | Sealed brief, local `Procedure.md`, or changed-surface test docs. |

If a local context file names an older decomposition revision and the sealed
brief names a newer accepted basis, follow the active assignment and record the
mismatch in the handoff. Do not update lifecycle, dependency, DAG, blocker,
candidate, evidence, release, or authority records unless they are explicitly
inside the write scope.

## Sealed Type 2 Execution

A Type 2 contributor or agent executes one bounded deliverable. The normal
sequence is:

1. Confirm `DeliverableID`, `PackageID`, scope items, objectives, invariants,
   acceptance criteria, and write scope.
2. Inspect the current worktree. Treat unrelated changes as other-worker or
   user work; do not revert or rewrite them.
3. Read the deliverable packet: `_CONTEXT.md`, `_REFERENCES.md`,
   `_DEPENDENCIES.md`, `_STATUS.md`, `Specification.md`, `Guidance.md`, and
   `Procedure.md` where present.
4. Make only the requested changes in the allowed paths.
5. Run tests, validators, scans, or documentation checks appropriate to the
   changed surface.
6. Record evidence: changed files, commands, results, warnings, unresolved
   `TBD`s, and anything intentionally deferred.
7. Hand off for review. Type 2 output remains draft work until the appropriate
   human or review gate accepts it.

Do not silently expand a Type 2 task across package boundaries. If the work
requires a protected source, a private project file, a legal mechanism decision,
a release decision, a maintainer-authority decision, or a change to stable
scope, stop and route the issue through the assigned review or CHANGE path.

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

## Tests And Evidence

Run the smallest useful checks for the files you changed. Typical evidence:

| Changed surface | Useful checks |
|---|---|
| Markdown docs | Link/path check, spelling or terminology review where available, `git diff --check`. |
| Schemas or JSON fixtures | Schema validators and provenance checks named by the owning deliverable. |
| Rust core | Package-specific `cargo test` commands and any validation fixtures listed by the deliverable. |
| TypeScript or GUI | Package scripts, Vitest, Playwright, screenshot checks, or documented deferrals. |
| Examples or report templates | Protected-content, private-data, provenance, and product-claim review. |
| Release-impacting changes | Release quality gates where a human-governed release path applies. |

If a check is impractical, unavailable, or out of scope, record that fact. Do
not replace a failed or skipped check with unsupported assurance language.

## Review And CHANGE Gates

Review checks whether a contribution stayed in scope, preserved the data
boundary, produced adequate evidence, and avoided claims beyond the available
evidence. Maintainer or reviewer acceptance of a repository contribution is
project governance only.

Route these issues instead of resolving them inside a bounded contribution:

| Issue | Route |
|---|---|
| Scope, package boundary, stable ID, or deliverable split change | CHANGE or SOFTWARE_DECOMP path. |
| Protected-content or unclear redistribution status | Contribution review and human/legal escalation path. |
| Private project, owner, client, library, rule-pack, or secret exposure | Security/privacy and contribution review path. |
| License, contributor mechanism, release authority, maintainer quorum, CI provider, or release-label decision | Human project-authority path. |
| Professional reliance, code interpretation, or project-specific engineering acceptance wording | Professional-boundary review path. |

Preserve existing `TBD`s for license, release authority, CI policy, dependency
versions, and other human-governed decisions unless the assignment explicitly
grants authority to change them.

## Contribution Expectations

- Work from current files, not cached assumptions.
- Keep edits inside the assigned write scope.
- Prefer small, reviewable changes with explicit evidence.
- Keep public examples invented or otherwise cleared for redistribution.
- Preserve unit, provenance, diagnostic, privacy, and professional-boundary
  controls when touching APIs, adapters, schemas, reports, or rule packs.
- Surface conflicts, missing inputs, stale assumptions, and unclear rights.
- Use `TBD` for unknowns rather than inventing values, sources, legal
  conclusions, or release decisions.

## Handoff Template

Use this structure for the final handoff when no project-specific template is
provided:

```text
Deliverable: DEL-XX-YY / PKG-XX
Changed files:
- path

Commands run:
- command -> result

Data/provenance boundary:
- no protected standards, proprietary source material, private project data,
  or real secrets introduced
- unresolved items: TBD or none

Review notes:
- warnings, assumptions, deferrals, or open issues
```

The handoff is evidence for review. It does not by itself change lifecycle
state, release status, project policy, or engineering reliance status.
