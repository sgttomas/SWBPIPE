# Guidance: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

## Purpose

This deliverable keeps solve execution reviewable from the GUI without moving solver authority into the GUI. The user should be able to see that a solve is running, what the service reports about its progress, whether cancellation was requested or completed, and which diagnostics qualify the result.

The setup is intentionally contract-facing. It prepares a future GUI slice to consume application-service jobs and diagnostics/result envelopes while preserving missing-data, provenance, unit, privacy, and professional-responsibility boundaries.

## Principles

| Principle | Guidance |
|---|---|
| Service boundary first | GUI components initiate commands/jobs and observe job/envelope state. They do not own solver lifecycle semantics. |
| Report what is supplied | Progress should reflect service-provided phases, counts, statuses, or messages. If only indeterminate progress is available, the UI should show indeterminate progress rather than fabricate precision. |
| Cancellation is a request | Cancellation UX should distinguish requested, accepted, completed, failed, and not-supported states only when the service contract supplies those states. Exact enum names remain `TBD`. |
| Diagnostics stay attached | Warnings and errors should remain linked to the run, affected object, source, severity, and remediation fields where available. |
| Missing data is visible | Solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warnings should remain visible instead of becoming generic failure messages. |
| Professional boundary remains visible | A completed mechanics solve is not a code-compliance certificate. Human review remains required for professional reliance. |

## Considerations

Background solves often have phases that are meaningful to users even when exact completion percentages are not reliable. A future UI can show service-reported states such as queued, validating, solving, recovering results, checking user rules, writing report/export artifacts, canceled, failed, or complete only if those states are supplied by the job contract. These labels are examples of UX categories, not fixed contract enums.

Cancellation should preserve auditability. A canceled run may still produce diagnostics, partial records, or a terminal envelope explaining what was stopped and what was not computed. The GUI should present that terminal envelope rather than removing the run from view.

Diagnostic presentation should support both quick triage and detailed review. A user needs to know which diagnostics block solving, which block only rule checks, which are provenance or assumption warnings, and which reflect nonlinear/numerical uncertainty. The future UI should keep diagnostic class and severity visible without converting warnings into professional judgments.

Result/export readiness should be traceable. Where upstream contracts supply hashes, solver versions, model versions, rule-pack checksums, warning sets, and limitations, the solve UX should preserve those signals for reports and result exports. Where those fields are missing, the setup should mark them `TBD` rather than inventing them.

## Trade-offs

| Decision area | Recommended posture | Risk if ignored |
|---|---|---|
| Percent progress | Prefer contract-supplied percentages; otherwise use indeterminate or phase-based progress | False precision can mislead users and reviewers |
| Cancellation feedback | Show request and final job state separately | Users may believe a solve stopped cleanly when the service did not confirm it |
| Diagnostic density | Provide summary and detail surfaces | Hiding details weakens reviewability; flooding the main surface weakens usability |
| Result status wording | Use mechanics/rule/human-review status vocabulary | Users may infer code compliance or professional approval from software output |
| Error recovery | Preserve terminal envelopes and run records | Failed/canceled solves become unreviewable |

## Examples

The following are non-normative UX examples only:

- If the job contract supplies no percentage, show an indeterminate progress indicator plus the current service-reported phase.
- If cancellation is requested, disable duplicate cancel requests while continuing to display diagnostics and the final terminal envelope when received.
- If a solve completes but rule-pack inputs are missing, present the mechanics result separately from the rule-check-blocking diagnostics.
- If a nonlinear warning is returned, keep it visible with the affected object and remediation text supplied by the diagnostic record.

## Open Issues

| ID | Issue | Owner |
|---|---|---|
| OI-07-07-001 | Exact job state enum, progress payload, and cancellation terminal states remain implementation-level `TBD` until the application-service contract is materialized. | Future implementation TASK |
| OI-07-07-002 | Exact diagnostic filtering, grouping, and detail layout remain `TBD` until GUI design work is authorized. | Future GUI TASK |
| OI-07-07-003 | Exact report/export handoff fields remain `TBD` until result export and report artifacts are implemented. | PKG-08 / future interface TASK |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflicts were identified during setup. | N/A | N/A | N/A | N/A | N/A |
