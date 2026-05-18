# Datasheet: DEL-07-04 Missing-data warning and blocking UX

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-04 |
| Package | PKG-07 Graphical User Interface and Engineering Workflow |
| Type | UX_UI_SLICE |
| Scope items | SOW-022 |
| Objectives | OBJ-006, OBJ-011 |
| Anticipated artifacts | warning system UI; UX tests |
| Setup status | Setup/document-production only; no GUI source files, tests, schemas, package manifests, or repo-level docs are edited. |

## Attributes

This deliverable defines the setup boundary for a future GUI warning and blocking workflow. The future UX must distinguish data that blocks a mechanics solve from data that blocks or qualifies a user-rule check, while keeping provenance, assumptions, nonlinear uncertainty, IP/data-boundary risks, and professional review visible.

| Attribute | Required treatment | Source basis |
|---|---|---|
| Solve-required missing data | Classify as `SOLVE_BLOCKING` and prevent or invalidate the affected mechanics solve rather than supplying defaults. | `docs/SPEC.md` section 7; OPS-K-DATA-2 |
| Rule-check-required missing data | Classify as `RULE_CHECK_BLOCKING`; mechanics results may exist, but user-rule pass/fail or ratios remain unavailable or qualified. | `docs/SPEC.md` section 7; `docs/TYPES.md` analysis-status vocabulary |
| Provenance weakness | Classify as `PROVENANCE_WARNING` when a value exists but source/provenance is missing or weak. | OPS-K-DATA-3; AB-00-06 |
| User/model assumptions | Classify as `ASSUMPTION_WARNING` and keep the assumption visible through result review/report handoff. | SOW-022; OBJ-006 |
| Nonlinear uncertainty | Classify as `NONLINEAR_WARNING` when convergence or active-state uncertainty affects interpretation. | OPS-K-SOLVER-2; `docs/SPEC.md` section 4.4 and section 7 |
| IP or private-data risk | Classify as `IP_BOUNDARY_WARNING`; do not move protected/private data into public artifacts. | OPS-K-IP-1/2/3; `docs/IP_AND_DATA_BOUNDARY.md` quarantine rule |
| Professional boundary | Never present software output as certified, sealed, approved, authenticated, or professionally code-compliant. | OPS-K-AUTH-1; OBJ-011 |
| Warning display access | Warning class, severity, affected object, message, and remediation must be available through text/assistive paths and not color-only signaling. | `_SEMANTIC_LENSING.md` C-001; `docs/SPEC.md` section 7 |

## Conditions

| Condition | Setup ruling |
|---|---|
| Silent engineering defaults | Prohibited. Unknown engineering values remain `TBD` or become explicit diagnostics. |
| Protected standards/code data | Not included. No code text, code tables, copied formulas, material allowables, SIF/flexibility tables, or proprietary values are introduced. |
| Result-envelope fields | Future warnings must preserve at least code, class, severity, source, affected object, message, remediation, and provenance where applicable. |
| Analysis state separation | Future UX must preserve `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and `HUMAN_REVIEW_REQUIRED` distinctions; it must not auto-emit `CODE_COMPLIANT`. |
| GUI architecture | Future GUI mutation and state changes route through application-service command/query/job result envelopes and preserve durable/transient state separation. |
| Surface placement | Exact editor, solve-runner, results-view, report-preview, and export-control placement remains implementation-level `TBD`; this setup defines behavior and boundaries only. |

## Construction

The setup artifact is a document kit and local evidence bundle. It does not create visual components, application state, tests, schemas, or source-code contracts.

Future implementation work is expected to consume:

- architecture basis AB-00-03 for command/query/job result envelopes and status separation;
- architecture basis AB-00-05 for GUI state, editing, and diagnostic preservation;
- architecture basis AB-00-06 for diagnostics, warning classes, and result-envelope fields;
- DEL-04-06 for solver diagnostic producers;
- DEL-05-04 for analysis status semantics;
- DEL-06-03 for rule-pack required-input completeness signals.

## References

- `_CONTEXT.md` for deliverable identity, scope, objectives, and architecture-basis injection.
- `docs/CONTRACT.md` for invariants OPS-K-DATA-1/2/3, OPS-K-RULE-1/2/3, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV, and OPS-K-AGENT-1..4.
- `docs/SPEC.md` section 7 for GUI warnings and warning classes.
- `docs/TYPES.md` section 4 for analysis-status vocabulary.
- `docs/DIRECTIVE.md` sections 2 and 5 for missing-data, professional-boundary, and stop-rule treatment.
- `docs/IP_AND_DATA_BOUNDARY.md` for provenance and quarantine rules.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` rows SOW-022, OBJ-006, OBJ-011, and AB-00-03/05/06.
