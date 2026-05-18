# Specification: DEL-07-05 Results viewer

## Scope

This deliverable specifies the setup basis for a GUI results viewer that presents reviewable analysis outputs for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and user-rule ratios. It is a UX/UI slice under PKG-07 and supports OBJ-006 and OBJ-007.

This setup does not implement GUI source code, tests, schemas, exporter code, solver logic, stress recovery, rule-pack evaluation, report generation, or protected engineering data.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| REQ-07-05-001 | The viewer shall present result categories listed in SOW-023 when those categories are present in validated result envelopes. | SOW-023; `docs/SPEC.md` sections 7-8 |
| REQ-07-05-002 | The viewer shall preserve the architecture boundary: GUI-facing code consumes application-service result envelopes and shall not bypass domain-core unit checks, provenance checks, diagnostics, or public/private data boundaries. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 1 |
| REQ-07-05-003 | Displayed numerical result values shall be unit-aware and dimensionally labeled; missing or incompatible units shall be shown as diagnostics rather than silently converted or defaulted. | OPS-K-UNIT-1 and OPS-K-DATA-2 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` section 3 |
| REQ-07-05-004 | The viewer shall distinguish mechanics result status, rule-input incompleteness, user-rule-check status, failed user-rule status, and human-review-required status; it shall not expose automatic `CODE_COMPLIANT` status. | `docs/TYPES.md` section 4; OPS-K-AUTH-1 |
| REQ-07-05-005 | Rule-pack ratios shall be displayed only when the required user-supplied rule-pack inputs and checksum/provenance status support the calculation; otherwise the ratio surface shall show an unavailable or blocked state with a diagnostic. | SOW-023 note; OPS-K-DATA-1/2; OPS-K-RULE-1/2/3 |
| REQ-07-05-006 | Result review shall keep warnings, assumptions, missing data, solver diagnostics, nonlinear/convergence state, provenance gaps, and IP-boundary warnings visible with the result context they qualify. | `docs/SPEC.md` section 7; OPS-K-DATA-2; OPS-K-AUTH-1 |
| REQ-07-05-007 | Result displays intended for reports or exports shall retain traceability to model version/hash, solver version, rule-pack name/version/checksum, units, warnings, assumptions, and limitations when those envelope fields exist. | OBJ-007; SOW-039; SOW-046; `docs/SPEC.md` section 8 |
| REQ-07-05-008 | Public setup artifacts shall not introduce protected standards text, protected tables, proprietary formulas, code-specific allowables, stress limits, SIF/flexibility values, load-combination defaults, or certification language. | OPS-K-IP-1/2/3; OPS-K-AUTH-1; `docs/DIRECTIVE.md` sections 3-5 |
| REQ-07-05-009 | The future implementation shall include UI tests or equivalent review evidence for category availability, unit labels, warning/blocked states, status separation, and report/export handoff signals. | DEL-07-05 anticipated artifacts; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` sections 4-5 |

## Standards

No external engineering code or standards text is introduced by this deliverable. The controlling references for this setup are the OpenPipeStress governance and technical documents listed in `_REFERENCES.md`.

Any future rule-check wording, stress ratio threshold, allowable, code category, or formula must come from a user-supplied or lawfully imported private rule pack and must preserve provenance and redistribution status.

## Verification

| Verification ID | Method | Expected evidence |
|---|---|---|
| VER-07-05-001 | Document review | Four-document kit exists and matches DEL-07-05 scope. |
| VER-07-05-002 | Boundary review | No protected code data, proprietary thresholds, or certification claims appear in setup artifacts. |
| VER-07-05-003 | Dependency-register validation | `Dependencies.csv` validates against v3.1 schema. |
| VER-07-05-004 | Semantic setup review | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist with complete matrix/lens coverage. |
| VER-07-05-005 | Future implementation test | UI test coverage for result categories, units, diagnostics, ratio blocking, status separation, and export handoff remains TBD until source implementation exists. |

## Documentation

Required setup records for this deliverable are:

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

## Acceptance Notes

This setup can be marked `SEMANTIC_READY` only after the four-document pass, semantic matrix build, lens register, Pass 3 consistency sweep, dependency extraction, and local validation gates complete. `SEMANTIC_READY` is a development lifecycle state only; it is not product implementation, code compliance, professional approval, or an issued deliverable.
