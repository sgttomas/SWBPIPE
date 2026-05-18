# Procedure: DEL-07-04 Missing-data warning and blocking UX

## Purpose

This procedure defines how future implementation work should use the setup artifact for missing-data warning and blocking UX. It is an operational guide for the deliverable, not an implementation script.

## Prerequisites

- Confirm the active sealed scope is DEL-07-04 / PKG-07.
- Read `_CONTEXT.md`, `docs/CONTRACT.md`, `docs/SPEC.md` section 7, `docs/TYPES.md` section 4, and the applicable architecture basis rows AB-00-03, AB-00-05, and AB-00-06.
- Confirm upstream setup evidence for DEL-04-06, DEL-05-04, and DEL-06-03 before writing implementation code.
- Confirm no protected standards text, protected tables, copied formulas, proprietary values, private rule packs, or certification/compliance claims are introduced.

## Steps

1. Classify missing or weak data by warning class: `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, or `IP_BOUNDARY_WARNING`.
2. Map `SOLVE_BLOCKING` conditions to `MODEL_INCOMPLETE` or equivalent incomplete mechanics-solve status without inventing defaults.
3. Map `RULE_CHECK_BLOCKING` conditions to `RULE_INPUTS_INCOMPLETE` or equivalent rule-check-blocked status while preserving valid mechanics results when available.
4. Preserve diagnostic fields from result envelopes: code, class, severity, source, affected object, message, remediation, and provenance where applicable.
5. Present warnings in the GUI where users edit data, run solves, review results, and prepare reports/exports.
6. Keep warning state synchronized through application-service commands and job/result envelopes; do not mutate domain readiness directly from transient UI state.
7. Check wording for professional-boundary compliance before any user-facing message is accepted.
8. Check public examples, screenshots, exports, report previews, and fixtures for protected-content and private-data boundary risks.

## Verification

Future verification should confirm:

- each warning class can be rendered from a representative diagnostic envelope;
- all six warning classes remain distinct in tests and are not collapsed into a generic alert state;
- solve-blocking and rule-check-blocking behavior remain distinct;
- missing values are surfaced as `TBD`, diagnostic findings, or blocked/qualified results rather than silent defaults;
- provenance, assumption, nonlinear, and IP-boundary warnings survive result review and report/export handoff;
- keyboard, non-color, and assistive-technology paths can identify warning class, severity, affected object, and remediation;
- warning text does not claim certification, approval, sealing, authentication, official endorsement, or code compliance.
- `IP_BOUNDARY_WARNING` blocks, qualifies, quarantines, or routes public contribution/report/export risk without exposing protected/private data.

## Records

Maintain these local setup records:

- four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`;
- semantic artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`;
- dependency artifacts: `Dependencies.csv`, `_DEPENDENCIES.md`;
- run records under `_run_records/`;
- lifecycle record in `_STATUS.md`.
