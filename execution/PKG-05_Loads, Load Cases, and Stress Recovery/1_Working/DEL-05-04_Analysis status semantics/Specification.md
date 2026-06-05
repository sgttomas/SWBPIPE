# Specification: DEL-05-04 Analysis status semantics

## Scope

This deliverable specifies the analysis-status semantics used by result envelopes and related boundaries. It covers status distinctions among mechanics solved, rule-pack checked or blocked by incomplete rule inputs, incomplete solve data, and human-approved/not-human-approved records. Implemented evidence exists for the schema vocabulary, result/API fields, boundary tests, and downstream consumers. Lifecycle promotion, release readiness, GUI acceptance workflow ownership, report rendering acceptance, and professional approval remain outside this deliverable-local kit.

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
| REQ-05-04-010 | Tests and future release gates shall prove mechanics solved, rule checked, rule inputs incomplete, human review required, and human-approved record states cannot collapse into a single automatic compliance flag. | OPS-K-AUTH-1; OPS-K-MECH-2; AB-00-08 |
| REQ-05-04-011 | Public examples and deliverable artifacts shall not embed protected standards text, protected tables, code formulas, proprietary values, private rule values, or professional approval claims. | OPS-K-IP-1; `docs/architecture/analysis_status_semantics.md` Protected Data Boundary |
| REQ-05-04-012 | Status semantics shall use `TBD` for unresolved human acceptance workflow ownership, storage location, UI presentation, and non-JSON hash edge cases. | OPS-K-AGENT-1; `docs/architecture/analysis_status_semantics.md` Remaining TBDs |
| REQ-05-04-013 | Result-envelope design shall identify at least the software status, diagnostic references, human-review notice, and optional external human acceptance record reference without implying approval. | `_SEMANTIC_LENSING.md` F-002; AB-00-03; AB-00-06; `schemas/analysis_status.schema.yaml`; `schemas/results.schema.yaml` |
| REQ-05-04-014 | Future release gates shall include tests and wording checks that prevent automatic approval, automatic code-compliance claims, and stale human-acceptance reuse after content changes. | `_SEMANTIC_LENSING.md` A-001 and X-001; OPS-K-AUTH-1; OPS-K-AUTH-2 |

## Standards

No external protected standard text is introduced by this deliverable-local kit. Governing local standards are the project invariant catalog, type vocabulary, technical specification, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-05-04-001 | `tests/test_analysis_status_schema.py` and `tests/test_analysis_boundary_schema.py` confirm separate automatic software status, authority-model, and external human-record structures. |
| REQ-05-04-002 | Schema and downstream tests confirm `MECHANICS_SOLVED` is an automatic status and does not require rule-pack checked status. |
| REQ-05-04-003 | Boundary/result schemas and downstream UX tests preserve `MODEL_INCOMPLETE` as a missing-input status; product-specific emitting workflows remain separate deliverable scope. |
| REQ-05-04-004 | `tests/test_analysis_boundary_schema.py`, `core/rules/completeness_checker/src/lib.rs`, and downstream result/run tests preserve `RULE_INPUTS_INCOMPLETE` alongside mechanics status. |
| REQ-05-04-005 | Rule-pack/evaluator and result/report tests preserve checked/failed user-rule outcomes without compliance claims. |
| REQ-05-04-006 | `tests/test_results_schema.py`, `tests/test_report_generator_contract.py`, `tests/test_report_sections_contract.py`, and runner/report modules require or surface `HUMAN_REVIEW_REQUIRED`. |
| REQ-05-04-007 | `tests/test_analysis_status_schema.py`, `tests/test_analysis_boundary_schema.py`, `tests/test_results_schema.py`, and `tests/test_api_boundary_contract.py` exclude `HUMAN_APPROVED_FOR_PROJECT` from automatic/API/result statuses. |
| REQ-05-04-008 | `schemas/analysis_status.schema.yaml` defines hash-bound `HumanAcceptanceRecord`; workflow ownership, storage, and UI presentation remain TBD. |
| REQ-05-04-009 | `schemas/analysis_boundary.schema.yaml`, `schemas/results.schema.yaml`, report/result export modules, and tests preserve diagnostics, provenance, and boundary notices. |
| REQ-05-04-010 | Focused schema/API tests prevent `CODE_COMPLIANT`, certification, sealing, approval, or equivalent automatic statuses. |
| REQ-05-04-011 | Protected-content and product-claims review of examples, docs, and report-facing wording. |
| REQ-05-04-012 | Open-item review requiring unresolved human workflow details to remain explicit `TBD`s. |
| REQ-05-04-013 | Schema/API review and tests confirm status-envelope fields remain distinct and do not imply approval. |
| REQ-05-04-014 | Negative tests for automatic approval/compliance plus stale-hash acceptance reuse. |

## Documentation

Required local artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`, and `_run_records/`. These local artifacts summarize and cite implemented evidence; they are not lifecycle approval records.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No conflict identified during current evidence alignment. | N/A | N/A | N/A | N/A | TBD |
