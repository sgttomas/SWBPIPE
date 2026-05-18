# Specification: DEL-05-04 Analysis status semantics

## Scope

This deliverable specifies the setup boundary for analysis status semantics in result envelopes. It covers status distinctions among mechanics solved, rule-pack checked or blocked by incomplete rule inputs, incomplete solve data, and human-approved/not-human-approved records. It does not implement schemas, API code, tests, GUI flows, report rendering, or a human approval workflow.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-05-04-001 | Status semantics shall clearly distinguish mechanics-solved state, rule-pack evaluation state, incomplete-data findings, and human acceptance state. | SOW-047; AB-00-03 |
| REQ-05-04-002 | `MECHANICS_SOLVED` shall mean mechanics outputs were computed for a specific model snapshot and shall not imply rule-pack completeness or code compliance. | `docs/TYPES.md` section 4; OPS-K-MECH-2 |
| REQ-05-04-003 | `MODEL_INCOMPLETE` shall represent missing solve-required physical inputs and shall block mechanics solving for the affected subject. | `docs/architecture/analysis_status_semantics.md` Usage Rules; OPS-K-DATA-2 |
| REQ-05-04-004 | `RULE_INPUTS_INCOMPLETE` shall represent missing rule-pack-required user/code/project data and may coexist with `MECHANICS_SOLVED`. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Usage Rules |
| REQ-05-04-005 | `USER_RULE_CHECKED` and `USER_RULE_FAILED` shall describe user-rule-pack computation outcomes without declaring professional code compliance. | `docs/architecture/analysis_status_semantics.md` Authority Boundary; OPS-K-AUTH-1 |
| REQ-05-04-006 | `HUMAN_REVIEW_REQUIRED` shall be available for reportable result envelopes to preserve the professional-review boundary. | `docs/architecture/analysis_status_semantics.md` Usage Rules; OPS-K-REPORT-1 |
| REQ-05-04-007 | Software shall not automatically emit `HUMAN_APPROVED_FOR_PROJECT`; that value is reserved for a separate human acceptance record. | `docs/architecture/analysis_status_semantics.md` Authority Boundary; OPS-K-AUTH-1 |
| REQ-05-04-008 | Any future human acceptance record shall bind to specific reviewed model, result, rule-pack, report, and manifest hashes and shall not survive content changes. | OPS-K-AUTH-2; `docs/architecture/analysis_status_semantics.md` Human Acceptance Records |
| REQ-05-04-009 | Result/status envelopes shall preserve diagnostic provenance needed for rule-pack references, warnings, assumptions, limitations, and report audit. | AB-00-06; OPS-K-REPORT-1 |
| REQ-05-04-010 | Future tests shall prove mechanics solved, rule checked, rule inputs incomplete, human review required, and human-approved record states cannot collapse into a single automatic compliance flag. | OPS-K-AUTH-1; OPS-K-MECH-2; AB-00-08 |
| REQ-05-04-011 | Public examples and setup artifacts shall not embed protected standards text, protected tables, code formulas, proprietary values, private rule values, or professional approval claims. | OPS-K-IP-1; `docs/architecture/analysis_status_semantics.md` Protected Data Boundary |
| REQ-05-04-012 | Status semantics shall use `TBD` for unresolved human acceptance workflow ownership, storage location, UI presentation, and non-JSON hash edge cases. | OPS-K-AGENT-1; `docs/architecture/analysis_status_semantics.md` Remaining TBDs |
| REQ-05-04-013 | Future result-envelope design shall identify at least the software status, diagnostic references, human-review notice, and optional external human acceptance record reference, with final field names `TBD`. | `_SEMANTIC_LENSING.md` F-002; AB-00-03; AB-00-06 |
| REQ-05-04-014 | Future release gates shall include tests and wording checks that prevent automatic approval, automatic code-compliance claims, and stale human-acceptance reuse after content changes. | `_SEMANTIC_LENSING.md` A-001 and X-001; OPS-K-AUTH-1; OPS-K-AUTH-2 |

## Standards

No external protected standard text is introduced by this setup. Governing local standards are the project invariant catalog, type vocabulary, technical specification, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-05-04-001 | Schema/API review confirming separate status fields or status dimensions for mechanics, rule checks, incomplete data, and human acceptance. |
| REQ-05-04-002 | Unit or schema tests proving `MECHANICS_SOLVED` can exist without rule-pack checked status. |
| REQ-05-04-003 | Validation test emitting `MODEL_INCOMPLETE` for missing solve-required physical data. |
| REQ-05-04-004 | Rule completeness test emitting `RULE_INPUTS_INCOMPLETE` after a mechanics solve when required rule inputs are missing. |
| REQ-05-04-005 | Rule-pack tests showing checked/failed outcomes preserve user-rule provenance and do not emit compliance claims. |
| REQ-05-04-006 | Report/result-envelope test showing `HUMAN_REVIEW_REQUIRED` is surfaced for reportable results. |
| REQ-05-04-007 | Negative test proving software cannot automatically emit `HUMAN_APPROVED_FOR_PROJECT`. |
| REQ-05-04-008 | Future human-record tests binding acceptance records to hashes; current implementation details remain TBD. |
| REQ-05-04-009 | Diagnostic/result-envelope inspection for code, class, severity, source, affected object, message, remediation, and provenance fields. |
| REQ-05-04-010 | Regression tests preventing `CODE_COMPLIANT` or equivalent automatic compliance statuses. |
| REQ-05-04-011 | Protected-content and product-claims review of examples, docs, and report-facing wording. |
| REQ-05-04-012 | Open-item review requiring unresolved human workflow details to remain explicit `TBD`s. |
| REQ-05-04-013 | Schema/API review confirming status-envelope fields remain distinct and do not imply approval. |
| REQ-05-04-014 | Negative tests for automatic approval/compliance plus stale-hash acceptance reuse. |

## Documentation

Required local setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No conflict identified during setup. | N/A | N/A | N/A | N/A | TBD |
