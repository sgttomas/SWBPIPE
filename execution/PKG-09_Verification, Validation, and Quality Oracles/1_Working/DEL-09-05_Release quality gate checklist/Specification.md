# Specification: DEL-09-05 Release quality gate checklist

## Scope
This specification defines the deliverable-local release quality gate checklist for solver changes, rule-engine changes, GUI releases, and report-template releases. It converts PKG-09 verification and validation obligations into process and CI gate criteria without editing CI workflows or setting final thresholds.

Out of scope:
- implementing CI jobs, tests, solver logic, GUI workflows, rule-engine logic, or report templates;
- moving artifacts to `ISSUED`;
- selecting final tolerance, coverage, performance, release-matrix, signing, or maintainer-quorum thresholds without human authority;
- asserting certification, code compliance, professional approval, or engineering reliance.

## Requirements
| ID | Requirement | Evidence |
|---|---|---|
| RQG-001 | The checklist shall route each release-impacting change to one or more gate families: solver, rule-engine, GUI, report-template, or mixed. | `Datasheet.md#Attributes`; `Procedure.md#Steps` |
| RQG-002 | Solver-change gates shall require deterministic verification/regression evidence for applicable mechanics, stress recovery, nonlinear behavior, unit-aware calculations, numerical-quality diagnostics, and warning/result-envelope behavior before release use. | `docs/SPEC.md#4.5`; `docs/SPEC.md#9`; `docs/VALIDATION_STRATEGY.md#2`; `docs/CONTRACT.md#OPS-K-SOLVER-1` |
| RQG-003 | Rule-engine gates shall require evidence for sandboxing, unit awareness, deterministic evaluation, required-input completeness, invented example data, rule-pack checksum/provenance handling, and absence of arbitrary code execution. | `docs/SPEC.md#6`; `docs/CONTRACT.md#OPS-K-RULE-1`; `docs/CONTRACT.md#OPS-K-RULE-2`; `docs/CONTRACT.md#OPS-K-RULE-3` |
| RQG-004 | GUI release gates shall require workflow evidence that missing solve data, missing rule-check data, provenance weakness, assumptions, nonlinear uncertainty, and IP-boundary warnings remain visible and cannot be collapsed into generic success states. | `docs/SPEC.md#7`; `docs/TYPES.md#4`; `docs/DIRECTIVE.md#2.2` |
| RQG-005 | Report-template gates shall require reproducibility, checksum stability, warning inclusion, provenance disclosure, professional-boundary notice, and protected-content lint evidence. | `docs/SPEC.md#8`; `docs/VALIDATION_STRATEGY.md#2`; `docs/IP_AND_DATA_BOUNDARY.md#7` |
| RQG-006 | The checklist shall preserve the distinction between mechanics verification, workflow validation, user-rule checking, and professional review. | `docs/VALIDATION_STRATEGY.md#1`; `docs/TYPES.md#4`; `docs/DIRECTIVE.md#2.2` |
| RQG-007 | A release candidate shall not be labeled engineering beta unless the release-gate evidence required by `docs/VALIDATION_STRATEGY.md#4` is present and open risks are listed and accepted by human maintainers. | `docs/VALIDATION_STRATEGY.md#4`; `docs/DIRECTIVE.md#6` |
| RQG-008 | Public release-gate artifacts shall not contain protected standards text, copied code formulas, protected examples, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary commercial data, or private user data. | `docs/CONTRACT.md#OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md#3`; `docs/DIRECTIVE.md#4.2` |
| RQG-009 | Thresholds and release-authority decisions that lack a cited human ruling shall remain `TBD` and shall not be silently selected by this deliverable. | `_CONTEXT.md#Architecture Basis Injection`; `DEL-00-08/Specification.md#Requirements`; `docs/TYPES.md#5` |
| RQG-010 | Gate records shall identify evidence commands/results, benchmark source provenance, known limitations, open risks, unresolved `TBD` items, and the human governance decision surface. | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md#5`; `docs/DIRECTIVE.md#6`; `docs/SPEC.md#11` |
| RQG-011 | Mixed changes shall run the union of applicable gate families unless the human release authority records an explicit waiver and risk disposition. | `Guidance.md#Considerations`; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md#5`; `docs/DIRECTIVE.md#6` |
| RQG-012 | Human release-governance records shall list validation status, known limitations, open risks, unresolved `TBD` decisions, and professional-boundary notices. | `docs/DIRECTIVE.md#6`; `docs/VALIDATION_STRATEGY.md#4`; `Procedure.md#Records` |

## Standards
No protected standards text or clause-level code requirements are used as source authority for this setup deliverable. Applicable project-governance sources are the local OpenPipeStress governance, specification, validation, IP/data-boundary, decomposition, register, and PKG-00 architecture-basis documents listed in `Datasheet.md#References`.

## Verification
| Requirement | Setup verification approach |
|---|---|
| RQG-001 | Confirm `Procedure.md` includes deterministic gate-family classification. |
| RQG-002 | Confirm solver gate criteria mention mechanics, stress recovery, nonlinear behavior where applicable, units, diagnostics, and deterministic regression evidence without selecting final numeric thresholds. |
| RQG-003 | Confirm rule-engine gate criteria mention sandboxing, unit awareness, required inputs, invented examples, checksums, provenance, and arbitrary-code exclusion. |
| RQG-004 | Confirm GUI gate criteria preserve the warning/status distinctions from `docs/SPEC.md` and `docs/TYPES.md`. |
| RQG-005 | Confirm report-template gate criteria include reproducibility, checksums, warnings, provenance, protected-content lint, and professional-boundary notices. |
| RQG-006 | Confirm wording separates verification, validation, user-rule checks, and professional review. |
| RQG-007 | Confirm engineering-beta wording is conditional and tied to human maintainer risk acceptance. |
| RQG-008 | Confirm no protected data, code-derived values, or private user data were introduced. |
| RQG-009 | Confirm thresholds and release-authority choices remain `TBD`. |
| RQG-010 | Confirm dependency and run-record artifacts preserve evidence and open issues. |
| RQG-011 | Confirm mixed-change examples and procedure use union routing unless a human waiver is recorded. |
| RQG-012 | Confirm release-governance record fields include validation status, limitations, risks, `TBD` decisions, and professional-boundary notice. |

## Documentation
Required setup artifacts for this deliverable:
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
- `_STATUS.md`

## Acceptance Criteria
- The deliverable remains inside the assigned write scope.
- All four production documents exist with default sections represented.
- Semantic matrix and lensing artifacts exist and do not claim engineering authority.
- `Dependencies.csv` validates against the v3.1 dependency schema.
- `Current State` is `SEMANTIC_READY` only after the setup artifacts and dependency schema validation pass.
- No CI workflows, release files outside this folder, tests, or repo-level artifacts are modified.
- No certification, code-compliance, endorsement, sealing, or professional-approval claim is made.
