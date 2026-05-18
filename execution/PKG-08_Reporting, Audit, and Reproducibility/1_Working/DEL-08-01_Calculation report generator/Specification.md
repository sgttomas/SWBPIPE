# Specification: DEL-08-01 Calculation report generator

## Scope

This deliverable defines the calculation report generator behavior for OpenPipeStress reporting. It covers report assembly requirements for model input summaries, load cases, results, warnings, assumptions, source/provenance notes, rule-pack references, checksums, and limitations.

This setup session does not implement renderer source, report templates outside this deliverable, tests, schemas, or repo-level artifacts. Those remain future implementation work under sealed deliverable scope and review.

## Requirements

| ID | Requirement | SourceRef | Verification |
|---|---|---|---|
| R-08-01-001 | The report generator shall assemble auditable reports that include inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations. | ScopeLedger.csv row SOW-024; CONTRACT.md OPS-K-REPORT-1; SPEC.md section 8 | Inspect generated report contract/template behavior in future implementation tests. |
| R-08-01-002 | Reports shall include software version, solver version, model hash, input manifest, unit system, provenance summary, load cases/combinations, mechanical results, warnings, assumptions, missing data, rule-pack name/version/checksum, user-supplied data notice, and professional-review notice where those inputs exist. | SPEC.md section 8 | Report fixture/golden-output tests verify presence and omission rules. |
| R-08-01-003 | Public report templates and examples shall not reproduce protected code text, protected standards tables, protected figures, proprietary formulas, protected examples, protected dimensional tables, or proprietary commercial data. | CONTRACT.md OPS-K-IP-1, OPS-K-REPORT-2; SPEC.md section 8 | Protected-content report gate and human review before public release. |
| R-08-01-004 | Reports shall not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance. | CONTRACT.md OPS-K-AUTH-1; TYPES.md sections 4 and 8; DIRECTIVE.md sections 2 and 6 | Text review/lint gate checks prohibited claim language. |
| R-08-01-005 | Reports shall distinguish mechanics solved, user-rule checked, rule inputs incomplete, and human review required states. | TYPES.md section 4; SOFTWARE_DECOMP.md AB-00-03 and AB-00-06 | Status fields in report fixtures are checked against analysis-status vocabulary. |
| R-08-01-006 | Reports shall preserve unit context for unit-bearing inputs and results. | CONTRACT.md OPS-K-UNIT-1; SPEC.md sections 3 and 8 | Unit-aware fixture tests verify displayed and structured units. |
| R-08-01-007 | Reports shall disclose diagnostics using result-envelope-compatible warning classes including solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warnings when present. | SPEC.md section 7; SOFTWARE_DECOMP.md AB-00-06 | Diagnostic fixture tests verify warning class pass-through and report rendering. |
| R-08-01-008 | Reports shall treat rule packs as user/private design-basis artifacts and expose only safe metadata such as identity, version, checksum, source notice, redistribution status, required-input status, and user-provided report notices. | SPEC.md section 6; CONTRACT.md OPS-K-DATA-1, OPS-K-RULE-3 | Rule-pack report fixtures verify metadata display and protected-content exclusion. |
| R-08-01-009 | The report generator shall use schema-first command/query/job result envelopes and application-service boundaries; adapters/plugins must not bypass units, provenance, diagnostics, sandboxing, or report controls. | SOFTWARE_DECOMP.md AB-00-02, AB-00-03, AB-00-07 | Architecture review verifies service boundary conformance before implementation acceptance. |
| R-08-01-010 | Report reproducibility tests shall verify deterministic output for the same model/result/rule-pack inputs, versions, and report settings, subject to explicitly documented volatile fields. | SPEC.md section 9; CONTRACT.md OPS-K-REPORT-1 | Future tests compare normalized report outputs and audit manifests. |

## Standards

No protected standards text, protected tables, protected formulas, or proprietary code examples are used as source content for this deliverable. Any future user-private report template or rule-pack notice remains user responsibility and must be handled outside public-template distribution unless redistribution rights are documented.

## Verification

The future implementation should be accepted only after these checks exist and pass:

- Four-document setup artifacts exist for this deliverable.
- `Dependencies.csv` is valid v3.1 and every ACTIVE row has evidence.
- Report fixtures cover mechanics-only, rule-inputs-incomplete, user-rule-checked, warnings-present, and protected-content-risk cases.
- Report reproducibility tests normalize volatile metadata and verify stable report payloads.
- Protected-content and prohibited-claim gates scan public report templates/examples.
- Human review confirms that the report generator does not assert compliance, certification, sealing, or project acceptance.

## Documentation

Required setup documentation for this deliverable:

- `Datasheet.md` records identity, boundaries, conditions, and reference basis.
- `Specification.md` records requirements and verification targets.
- `Guidance.md` records rationale, principles, trade-offs, and open questions.
- `Procedure.md` records execution and future implementation workflow.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` record semantic setup evidence only.
- `Dependencies.csv` and `_DEPENDENCIES.md` record information-flow dependencies.
- `_run_records/*` record setup sequence evidence.

## Acceptance Criteria For This Setup Session

- No file outside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/` is edited.
- No renderer source, external report template, test, schema, or repo-level artifact is modified.
- Setup artifacts are source-grounded to local governance, decomposition, register, and context files.
- No protected standards content or certification/compliance claim is introduced.
- `_STATUS.md` reports `SEMANTIC_READY` only after the four-document kit, semantic artifacts, dependency register, and validation checks are complete.

