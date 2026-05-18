# Specification: DEL-08-03 Warnings, assumptions, and provenance report section

## Scope

This deliverable specifies the report section that exposes missing data, warnings, assumptions, user-supplied values, and source/provenance notes for OpenPipeStress calculation reports.

In scope:

- report-facing requirements for warning, assumption, missing-data, and provenance disclosure;
- integration expectations for diagnostics/result-envelope data supplied by GUI/core/rule/report workflows;
- protected-content and professional-responsibility guardrails for public report templates;
- acceptance criteria and evidence expected from future implementation work.

Out of scope:

- implementation of report renderer code in this setup session;
- report generator implementation owned by DEL-08-01;
- audit manifest/model hash implementation owned by DEL-08-02;
- result export format owned by DEL-08-04;
- protected-content linter implementation owned by DEL-08-05;
- creation or bundling of code-specific values, protected standards content, proprietary formulas, or professional acceptance records.

## Requirements

| Requirement ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-08-03-REQ-001 | The report section shall expose missing solve-required and rule-check-required data as explicit report findings. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` Section 8 | Report fixture or snapshot includes missing-data findings without default values |
| DEL-08-03-REQ-002 | The report section shall render warning classes supplied by diagnostics/result envelopes, including solve blocking, rule-check blocking, provenance, assumption, nonlinear, and IP-boundary warnings where present. | `docs/SPEC.md` Section 7; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Fixture covers each supported warning class and preserves class/severity/code |
| DEL-08-03-REQ-003 | Each rendered warning or assumption shall preserve machine-readable trace data when supplied: code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Schema/snapshot test confirms trace fields are retained or explicitly marked `TBD` |
| DEL-08-03-REQ-004 | User-supplied values and rule-pack references shall be identified as user-supplied or private where applicable, with source/provenance notes and rule-pack identity/checksum references when supplied by upstream artifacts. | `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-DATA-3, OPS-K-RULE-3; `docs/SPEC.md` Section 8 | Report fixture shows source/provenance and rule-pack ref/checksum without embedding protected content |
| DEL-08-03-REQ-005 | The report section shall include a professional-responsibility notice stating that professional reliance requires competent human review and that software output is decision support. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` Section 8 | Snapshot or template check finds the notice and rejects certification/approval language |
| DEL-08-03-REQ-006 | The report section shall not reproduce protected code text, protected standards tables, copyrighted examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data in public templates/examples. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` Section 7 | Protected-content lint and review evidence pass for public templates/examples |
| DEL-08-03-REQ-007 | The report section shall preserve unit context for any rendered values and shall not display unit-bearing values without their units when units are supplied. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` Section 8 | Report fixture includes unit-bearing values with units and rejects unitless display where units exist |
| DEL-08-03-REQ-008 | The report section shall distinguish mechanics-solved, user-rule-checked, incomplete-data, and human-review-required states without presenting software-generated code compliance. | `docs/TYPES.md` Section 4; `docs/CONTRACT.md` OPS-K-MECH-2 and OPS-K-AUTH-1 | Status fixture avoids `CODE_COMPLIANT` and includes human-review-required boundary language |
| DEL-08-03-REQ-009 | The report section shall support reproducibility by referencing model/report metadata supplied by audit manifest work, including model hash, software/solver version, and rule-pack checksum when present. | `docs/SPEC.md` Section 8; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 | Integration fixture consumes manifest fields by reference or marks unavailable fields `TBD` |
| DEL-08-03-REQ-010 | Missing source/provenance shall itself be reportable as a provenance warning rather than silently omitted. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3; `docs/SPEC.md` Section 7 | Fixture with `UNKNOWN_SOURCE` or missing source emits a provenance warning |
| DEL-08-03-REQ-011 | If automated protected-content lint is unavailable during early implementation, the report-section change shall remain subject to explicit human review and shall record the unavailable lint gate as an open verification item. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` Review checklist | Review evidence records lint status and human review requirement |
| DEL-08-03-REQ-012 | Report-section tests or review checks shall reject language that claims software certification, approval, endorsement, sealing, authentication, or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Section 4.2 | Snapshot/lint/review check covers prohibited claim language |

## Standards

No protected standards text is required or authorized for this deliverable. Governing standards-body content is intentionally not embedded in public artifacts. Applicable project standards are the OpenPipeStress governance artifacts listed in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md`.

## Verification

Future implementation work should provide at least these checks:

| Check | Purpose |
|---|---|
| Warning fixture coverage | Verifies rendering for each supported warning class without reclassification |
| Missing-data fixture | Verifies absent solve/rule data appears as a finding and not a default |
| Provenance fixture | Verifies source/provenance fields, private/public status, and `UNKNOWN_SOURCE` handling |
| Professional-boundary snapshot | Verifies the notice is present and prohibited certification/compliance language is absent |
| Protected-content lint | Verifies public templates/examples do not embed protected standards content |
| Lint fallback review | Verifies unavailable protected-content lint is recorded as an open verification item and does not advance silently |
| Prohibited-claim check | Verifies report text avoids certification, approval, endorsement, sealing, authentication, and automatic code-compliance claims |
| Unit display check | Verifies unit-bearing values are rendered with units |
| Reproducibility reference check | Verifies manifest/hash/rule-pack checksum fields are referenced when supplied |

## Documentation

Required setup artifacts for this run:

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

Future production artifacts anticipated by the register:

- report provenance section;
- tests.
