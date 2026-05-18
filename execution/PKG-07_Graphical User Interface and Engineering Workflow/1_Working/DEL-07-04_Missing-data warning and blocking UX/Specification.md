# Specification: DEL-07-04 Missing-data warning and blocking UX

## Scope

This deliverable specifies the setup boundary for a future GUI warning and blocking workflow. It covers warning classification, blocking and qualifying behavior, result-envelope visibility, and professional/IP boundary wording for missing or weak data. It does not implement GUI components, application state, schemas, tests, package manifests, or product source code.

## Requirements

| ReqID | Requirement | Source basis | Verification approach |
|---|---|---|---|
| R-DEL-07-04-001 | The future GUI shall preserve the warning classes `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING`. | `docs/SPEC.md` section 7; AB-00-06 | UI/result-envelope tests inspect class values from representative diagnostics. |
| R-DEL-07-04-002 | Missing solve-required physical input shall block or invalidate the affected mechanics solve and shall not be replaced by silent defaults. | SOW-022; OPS-K-DATA-2; `docs/TYPES.md` `MODEL_INCOMPLETE` | UX tests show solve action/result state is blocked or marked incomplete with remediation. |
| R-DEL-07-04-003 | Missing rule-check-required user/code/project data shall block or qualify rule-pack check results while preserving any valid mechanics-solve result. | SOW-022; `docs/TYPES.md` `RULE_INPUTS_INCOMPLETE`; DEL-06-03 setup evidence | UX tests show mechanics results can remain reviewable while rule-check ratios/pass-fail are unavailable or qualified. |
| R-DEL-07-04-004 | The GUI shall present diagnostics through result-envelope fields that include code, class, severity, source, affected object, message, remediation, and provenance where applicable. | AB-00-06; DEL-00-06 Specification REQ-06-01 | Contract/UI tests verify required diagnostic fields are rendered or available to assistive tooling. |
| R-DEL-07-04-005 | The warning workflow shall preserve analysis-status distinctions among incomplete model, mechanics solved, rule inputs incomplete, user-rule checked/failed, human review required, and human acceptance records. | AB-00-03; DEL-05-04; `docs/TYPES.md` section 4 | Status-mapping tests prevent collapse into a single automatic pass/compliance state. |
| R-DEL-07-04-006 | Provenance weakness shall remain visible as `PROVENANCE_WARNING`; value presence alone is insufficient to hide source/provenance risk. | OPS-K-DATA-3; `docs/IP_AND_DATA_BOUNDARY.md` section 4 | UX tests show missing/weak provenance is visible for relevant materials, components, rule values, and report/export surfaces. |
| R-DEL-07-04-007 | Assumptions shall remain visible as `ASSUMPTION_WARNING` and shall not be treated as authenticated engineering approval. | OBJ-006; OPS-K-AUTH-1 | Review workflow tests show assumption warnings survive result review and report handoff. |
| R-DEL-07-04-008 | Nonlinear convergence or active-state uncertainty shall remain visible as `NONLINEAR_WARNING` and shall not be hidden behind nominal result display. | OPS-K-SOLVER-2; `docs/SPEC.md` section 4.4 | Solver-result UX tests expose convergence/active-state uncertainty when present. |
| R-DEL-07-04-009 | Suspected protected/private content in public contribution, export, or report flows shall produce `IP_BOUNDARY_WARNING` and route to quarantine or human review rather than public output. | OPS-K-IP-1/2/3; `docs/IP_AND_DATA_BOUNDARY.md` section 5 | Protected-content/export tests verify warning and block/quarantine behavior. |
| R-DEL-07-04-010 | GUI warning text shall not claim certification, sealing, approval, authentication, official endorsement, or professional code compliance. | OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 2 and 5 | Product-claims wording review and negative tests exclude automatic `CODE_COMPLIANT` or equivalent claims. |
| R-DEL-07-04-011 | GUI state changes that affect solve or rule-check readiness shall route through application-service commands and preserve diagnostics across undo/redo where applicable. | AB-00-05; DEL-00-05 Specification REQ-05-03/05 | Future interaction tests verify warning state updates after editable model changes. |
| R-DEL-07-04-012 | Setup artifacts and future public examples shall use invented/non-code data only and shall not embed protected standards text, copied formulas, protected tables, or proprietary values. | OPS-K-IP-1; OPS-K-RULE-1 | Protected-content review confirms setup and future fixtures remain clean. |
| R-DEL-07-04-013 | Warning class, severity, affected object, message, and remediation shall be exposed through text/assistive paths and shall not rely on color-only signaling. | `_SEMANTIC_LENSING.md` C-001; SOW-036 adjacency via DEL-07-06 | Accessibility-oriented UX tests confirm non-color and assistive access to warning meaning. |

## Standards

No standards-body formulas, allowables, text, examples, or protected data are included. Code-specific values and rule-pack data remain user-supplied or privately imported with provenance. Any future private project use of licensed standards data remains the user's responsibility and must not be committed as public project content.

## External Inputs

| Input | Required from | Notes |
|---|---|---|
| Application-service result envelopes and status separation | AB-00-03 / DEL-00-03 | Needed before implementation can bind warnings to command/query/job results. |
| GUI state and diagnostic preservation architecture | AB-00-05 / DEL-00-05 | Needed for undo/redo, editing, and transient/durable state boundaries. |
| Diagnostics and warning-class contract | AB-00-06 / DEL-00-06 | Required source for warning classes and diagnostic field set. |
| Solver diagnostics | DEL-04-06 | Required for solve-blocking and nonlinear warning producers. |
| Analysis status semantics | DEL-05-04 | Required for status distinctions and no-compliance boundary. |
| Rule-pack required-input completeness signals | DEL-06-03 | Required for rule-check-blocking missing-data behavior. |

## Verification

Future implementation verification must include:

- UI tests for each warning class listed in R-DEL-07-04-001;
- per-class tests proving all six classes remain distinct rather than collapsing into generic alerts;
- tests that `SOLVE_BLOCKING` prevents or invalidates the affected mechanics solve without inventing defaults;
- tests that `RULE_CHECK_BLOCKING` blocks/qualifies rule-check results without erasing mechanics results;
- tests that provenance, assumption, nonlinear, and IP-boundary warnings remain visible through result review and report/export handoff;
- accessibility-oriented checks that warning severity, class, message, affected object, and remediation are available without color-only signaling;
- wording checks that no warning, report preview, or status surface claims certification, approval, sealing, authentication, official endorsement, or professional code compliance;
- protected-content checks for public examples, exported artifacts, and report templates touched by future implementation.
- explicit `IP_BOUNDARY_WARNING` tests for public contribution, export, and report-preview paths using invented fixtures only.

## Documentation

Required setup artifacts for this deliverable are:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

Implementation artifacts listed in the register (`warning system UI`, `UX tests`) remain future work outside this setup session's write scope.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during P1/P2 setup. | N/A | N/A | N/A | N/A | TBD |
