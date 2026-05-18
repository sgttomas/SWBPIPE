# Guidance: DEL-07-04 Missing-data warning and blocking UX

## Purpose

This deliverable prepares the warning and blocking UX surface for future GUI implementation. The key distinction is that a mechanics solve, a user-rule check, and a professional acceptance decision are different states. The GUI must help users see missing data, assumptions, provenance weakness, nonlinear uncertainty, and IP/private-data risks without filling engineering gaps silently.

## Principles

- Missing solve-required or rule-check-required values are findings, not hidden defaults.
- `SOLVE_BLOCKING` and `RULE_CHECK_BLOCKING` are not interchangeable: the former affects mechanics solve readiness; the latter affects rule-check readiness.
- Result displays may show mechanics outputs only with the relevant diagnostics and limitations visible.
- Warnings should carry actionable remediation, affected object identity, source/provenance where applicable, and severity.
- Warning UX must preserve the professional responsibility boundary: software assists analysis but does not certify, approve, seal, authenticate, or declare code compliance for reliance.
- Public-facing examples, report previews, and exports must preserve protected-data and privacy boundaries.
- `HUMAN_REVIEW_REQUIRED` is the appropriate reminder for professional reliance; it is not a substitute for a human approval record and must not be presented as software approval.

## Considerations

| Warning class | UX implication | Boundary to preserve |
|---|---|---|
| `SOLVE_BLOCKING` | Disable, block, or invalidate the affected solve path and point to the missing physical input. | Do not invent geometry, material, support, load, section, or component values. |
| `RULE_CHECK_BLOCKING` | Keep mechanics results reviewable when valid, but block or qualify user-rule-check results and ratios. | Do not imply code pass/fail, compliance, or professional acceptance. |
| `PROVENANCE_WARNING` | Show that the value exists but its source, license, or review status is missing, weak, or unresolved. | Do not treat a value as public or reliable simply because it is present. |
| `ASSUMPTION_WARNING` | Keep user/model assumptions visible during editing, solving, result review, and reporting. | Do not upgrade assumptions into approved engineering facts. |
| `NONLINEAR_WARNING` | Surface convergence, active-set, gap, lift-off, or friction-state uncertainty when present. | Do not hide uncertainty behind clean-looking result visuals. |
| `IP_BOUNDARY_WARNING` | Warn, block, quarantine, or route to human review for public contribution/report/export risk. | Do not copy protected standards or private project data into public artifacts. |

## Trade-offs

- Blocking too early can interrupt modeling, but blocking too late can make incomplete engineering states look valid. Prefer allowing model editing while clearly blocking solve or rule-check actions that lack required data.
- A warning list alone is insufficient for engineering review. Future UX should also mark affected model objects/results so the user can locate the issue.
- Severity and class should not rely on color alone; future accessibility work in DEL-07-06 should cover keyboard navigation, screen-reader labels, contrast, and review workflow usability.
- Implementation should avoid duplicating warning logic in UI state. The GUI should consume application-service diagnostics/result envelopes and present them faithfully.
- Exact placement and copy for editor panels, solve execution, result review, report preview, and export controls remain `TBD` for future GUI implementation. This setup fixes warning semantics, not component layout.

## Examples

The following are invented behavioral examples, not standards-derived cases:

- A component has enough user-entered geometry for mechanics but lacks a private rule-pack required allowable. The GUI may show mechanics results while marking the rule check as blocked with `RULE_CHECK_BLOCKING`.
- A support definition is missing stiffness or restraint direction needed by the solver. The GUI blocks the affected solve path with `SOLVE_BLOCKING` and leaves the missing field visible.
- A value is present but has `source_location = TBD` or unresolved redistribution status. The GUI shows `PROVENANCE_WARNING` or `IP_BOUNDARY_WARNING` depending on the affected workflow.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during setup. | N/A | N/A | N/A | N/A | TBD |
