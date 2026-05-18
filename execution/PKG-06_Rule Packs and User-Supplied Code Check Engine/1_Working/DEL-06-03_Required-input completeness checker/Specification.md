# Specification: DEL-06-03 Required-input completeness checker

## Scope

This deliverable specifies the setup boundary for a required-input completeness checker for user-defined rule packs. The future checker must connect a rule pack's declared required inputs to project/model/user-supplied data and prevent a user-rule-check status from being reported when required rule-check data is missing.

This setup run does not implement code, schemas, executable completeness rules, code-specific formulas, allowables, or standards-derived defaults.

## Requirements

| ID | Requirement | Source basis | Verification approach |
|---|---|---|---|
| R-DEL-06-03-001 | Missing rule-check-required values must be explicit findings, never silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` no silent engineering defaults | Future unit tests for missing required inputs and explicit findings. |
| R-DEL-06-03-002 | Code-specific and project-specific values must be user-supplied or privately imported, not bundled as public defaults. | `docs/CONTRACT.md` OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` private user data | Protected-content and default-data review of checker fixtures. |
| R-DEL-06-03-003 | Rule-pack values, materials, components, SIF/flexibility factors, allowables, and related inputs must carry provenance where relied upon. | `docs/CONTRACT.md` OPS-K-DATA-3; `docs/TYPES.md` provenance labels | Schema/provenance validation in future feature tests. |
| R-DEL-06-03-004 | Completeness must be machine-checkable from declarative rule-pack input declarations, not arbitrary executable code. | SOW-004; `docs/SPEC.md` rule-pack `required_inputs`; OPS-K-RULE-2 | Future tests bind rule-pack required-input declarations to missing-input diagnostics without executing arbitrary code. |
| R-DEL-06-03-005 | Missing rule-pack input must map to a rule-check-blocking condition distinct from solve-blocking physical input. | `docs/SPEC.md` warning classes; `docs/TYPES.md` `RULE_INPUTS_INCOMPLETE` | Status/diagnostic tests verify `RULE_CHECK_BLOCKING` classification. |
| R-DEL-06-03-006 | The checker must not assert code compliance, certification, approval, sealing, or professional reliance. | OPS-K-AUTH-1; `docs/TYPES.md` analysis-status vocabulary | Report/API text review and future status tests exclude automatic `CODE_COMPLIANT`. |
| R-DEL-06-03-007 | Suspected protected or proprietary data requests must be surfaced and escalated, not translated into public data. | OPS-K-IP-1/2/3; `docs/IP_AND_DATA_BOUNDARY.md` quarantine rule | Protected-content lint and quarantine-path tests in later implementation. |

## Standards

No standards-body formulas, allowables, tables, text, examples, or code-specific interpretations are included in this setup artifact. Any future private project use of licensed standards data must remain user-controlled and provenance-marked.

## External Inputs

| Input | Required from | Notes |
|---|---|---|
| Rule-pack schema required-input declarations | DEL-06-01 Rule-pack schema | Needed before executable completeness checking can be implemented. |
| Analysis status semantics | DEL-05-04 Analysis status semantics | Needed to bind missing inputs to `RULE_INPUTS_INCOMPLETE` without compliance claims. |
| Diagnostics/result envelope contract | Architecture basis AB-00-06 | Needed for `RULE_CHECK_BLOCKING` findings and remediation text. |
| User/project rule data | User-controlled private rule pack/project files | Must not be bundled into the public repository by this deliverable. |

## Verification

Future implementation verification must include:

- rule-pack missing-input tests using invented/non-code data only;
- unit-aware binding checks where the rule-pack schema declares dimensions or units;
- tests that missing code/project inputs block rule-check status but do not block the mechanics solve status by default;
- tests that no protected defaults, code-specific formulas, or material allowables are shipped;
- tests that output messages preserve the professional responsibility boundary.

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

Implementation artifacts listed in the register (`rule completeness checker`, `tests`) remain future work outside this setup session's write scope.
