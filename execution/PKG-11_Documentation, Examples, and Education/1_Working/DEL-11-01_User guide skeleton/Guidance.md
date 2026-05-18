# Guidance: DEL-11-01 User guide skeleton

## Purpose

This guidance explains how to interpret the user guide skeleton without blurring the OpenPipeStress data, rule-check, and professional responsibility boundaries. The user guide should help a user move through setup, modeling, solving, rule checking, reports, and limitations while making missing data and human decision points visible.

## Principles

| Principle | Guidance |
|---|---|
| Start with the authority boundary. | The guide should explain what OpenPipeStress computes and what remains user-supplied or human-reviewed before it teaches button-by-button workflows. |
| Keep setup factual. | Packaging, install steps, storage containers, external transports, and dependency versions remain `TBD` until accepted elsewhere. Do not invent setup commands. |
| Teach the centerline model. | Modeling guidance should orient users around projects, units, nodes, elements, components, supports, loads, combinations, and provenance. |
| Keep mechanics and rule checks separate. | A mechanics solve produces displacements, forces, moments, reactions, stresses, and diagnostics. A user rule pack can evaluate those results only when required private/user data is present. |
| Keep private data private. | Rule packs, owner requirements, licensed code values, vendor data, and private libraries belong in user-controlled/private paths unless redistribution rights are documented. |
| Keep reports auditable. | Report guidance should emphasize manifests, model hashes, solver versions, warnings, assumptions, provenance, checksums, results, limitations, and human review notices. |
| Keep limitations visible. | The guide should state when a result is incomplete, when local analysis may be needed, and when validation or professional review is outside software authority. |

## Considerations

The user guide has to serve multiple audiences without making the product sound more mature or authoritative than the current evidence supports:

- New users need a clear workflow from project creation through reports.
- Experienced engineers need explicit status, warning, provenance, and limitation semantics.
- Maintainers need a guide structure that can grow as implementation and validation evidence mature.
- Educators need invented or public/permissive examples that do not rely on protected standards data.

The guide should avoid presenting a future GUI, CLI, or report behavior as already implemented unless that behavior is supported by an accepted implementation deliverable. Where behavior is planned but unresolved, use `TBD` or "future section" language.

## Trade-offs

| Trade-off | Preferred handling |
|---|---|
| Helpfulness vs false precision | Provide a section slot and the governing boundary; leave exact commands or screenshots as `TBD` until implemented. |
| Engineering clarity vs protected content risk | Explain open mechanics and data responsibilities without copying code text, protected examples, or proprietary tables. |
| Workflow simplicity vs warning visibility | Do not hide missing data, provenance gaps, rule-check blockers, or nonlinear diagnostics to make the guide look simpler. |
| Public examples vs realism | Use invented or permissively sourced examples and label them as educational, non-code examples. |
| Report confidence vs professional reliance | Reports can support review; they do not replace competent human acceptance for project use. |

## Examples

Acceptable guide examples:

- A mechanics-only walkthrough using invented material and section values, clearly marked as non-code educational data.
- A rule-pack workflow that uses invented demonstration values and shows missing-input blockers.
- A report review checklist that names provenance, warnings, hashes, checksums, and limitations without quoting protected source material.
- A troubleshooting example that explains `RULE_CHECK_BLOCKING` or `PROVENANCE_WARNING` without supplying proprietary values.

Excluded guide examples:

- Copied protected standards tables, figures, examples, formulas, or commentary.
- Commercial software examples, manuals, screenshots, or benchmark files without redistribution permission.
- Vendor catalog or owner data without documented public redistribution rights.
- Any wording that implies software certification, endorsement, engineering seal, official approval, or automatic code compliance.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No source conflict identified in setup pass. | Not applicable | Not applicable | Not applicable | Not applicable | TBD |
