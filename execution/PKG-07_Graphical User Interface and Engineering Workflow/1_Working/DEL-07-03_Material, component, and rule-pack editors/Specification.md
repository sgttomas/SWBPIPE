# Specification: DEL-07-03 Material, component, and rule-pack editors

## Scope

This deliverable specifies the setup documentation for future GUI editor panels and validation UI tests covering material, section, component, load-case, support/restraint, rule-pack reference, and private-library editing within `PKG-07`.

In scope:

- Editor workflow requirements for user/private engineering data entry and review.
- Missing-data, provenance, unit, rule-pack-reference, and private-library validation expectations.
- Boundaries between GUI editing, application-service commands, domain schemas, rule-pack evaluation, diagnostics, and private data handling.
- Setup artifacts only: document kit, semantic artifacts, dependencies, run records, and status evidence.

Out of scope:

- GUI source implementation, tests, package manifests, schemas, or repo-level documentation.
- Bundled protected standards content, proprietary material/component/rule data, or silent engineering defaults.
- Selection of exact GUI component libraries, state libraries, rule expression grammar/library, dependency versions, public API transport, or physical project package/container.
- Any claim that the GUI certifies, seals, approves, authenticates, or declares engineering code compliance.

Sources: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Architecture Basis Injection"; `docs/_Registers/Deliverables.csv` row `DEL-07-03`; `docs/_Registers/ScopeLedger.csv` row `SOW-021`; `docs/CONTRACT.md` invariants `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-AUTH-1`, `OPS-K-AGENT-4`.

## Requirements

| Req ID | Requirement | Source |
|---|---|---|
| DEL-07-03-R-001 | The setup artifacts shall preserve `DEL-07-03` as the sole production unit and shall not expand into GUI source implementation. | `_CONTEXT.md`; `AGENTS.md` dispatch rule; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4 |
| DEL-07-03-R-002 | The editor workflow shall expose materials, sections, components, load cases, supports, rule packs, and private libraries as editable/reviewable GUI concepts under SOW-021. | `docs/_Registers/ScopeLedger.csv` row `SOW-021`; `docs/SPEC.md` section 7 |
| DEL-07-03-R-003 | The material editor shall require provenance/source status for material records and shall not provide protected public material allowables or code tables. | `docs/TYPES.md` sections 7 and 8; `docs/CONTRACT.md` `OPS-K-DATA-1`, `OPS-K-DATA-3`, `OPS-K-IP-1` |
| DEL-07-03-R-004 | The section and component editors shall treat dimensions, weights, centers of gravity, SIFs, flexibility factors, stiffnesses, and proprietary catalog values as user-supplied or lawfully imported data with provenance. | `docs/SPEC.md` sections 3 and 4.3; `docs/DIRECTIVE.md` section 3 |
| DEL-07-03-R-005 | The load-case editor shall support unit-aware editor input for load cases while leaving code-specific load combinations to user project inputs or rule packs. | `docs/SPEC.md` sections 3 and 5; `docs/CONTRACT.md` `OPS-K-UNIT-1`, `OPS-K-DATA-1` |
| DEL-07-03-R-006 | The support/restraint editor shall surface fields needed to represent support/restraint behavior and shall not hide nonlinear active-state uncertainty or non-convergence findings. | `docs/SPEC.md` sections 3, 4.4, and 7 |
| DEL-07-03-R-007 | The rule-pack reference editor shall expose rule-pack identity, version, checksum, source notice, redistribution status, required inputs, and missing required-input findings. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` `OPS-K-RULE-1`, `OPS-K-RULE-2`, `OPS-K-RULE-3` |
| DEL-07-03-R-008 | Editor mutations shall be modeled as application-service command interactions, not direct bypasses of domain validation, unit checks, provenance checks, or public/private data boundaries. | `docs/SPEC.md` section 1; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-02`, `AB-00-03`, `AB-00-07` |
| DEL-07-03-R-009 | Editor state shall separate durable project/model data from transient session, selection, validation, and job-progress state; exact state-management library remains TBD. | `_CONTEXT.md` "Still TBD"; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-05` |
| DEL-07-03-R-010 | Editor validation shall surface missing data early using diagnostic/result-envelope warning classes and shall not silently introduce engineering defaults. | `docs/SPEC.md` section 7; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06`; `docs/CONTRACT.md` `OPS-K-DATA-2` |
| DEL-07-03-R-011 | Private material, component, project, and rule-pack data shall remain local/user-controlled by default and shall not be committed or transmitted publicly by this deliverable. | `docs/CONTRACT.md` `OPS-K-PRIV-1`; `docs/DIRECTIVE.md` sections 3 and 6 |
| DEL-07-03-R-012 | UI wording and output states shall distinguish editable user data, mechanics solve readiness, user-rule-check readiness, and human professional review without implying code compliance. | `docs/TYPES.md` section 4; `docs/DIRECTIVE.md` section 2.2; `docs/CONTRACT.md` `OPS-K-AUTH-1`, `OPS-K-MECH-2` |
| DEL-07-03-R-013 | The future implementation brief shall split this deliverable or request human approval if the editor scope becomes too broad for one bounded GUI slice. | `_CONTEXT.md` "Context Budget QA"; `docs/_Registers/ContextBudgetQA.csv` row `DEL-07-03` |

## Standards

No protected engineering code or standards text is used as an authority in this setup artifact. Governing standards for this setup pass are the public project governance and decomposition documents listed in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md`.

Any future editor behavior that depends on an engineering code, owner standard, vendor catalog, material allowable, SIF/flexibility value, or rule formula shall treat that content as user-supplied/private unless redistribution rights and provenance are documented. Source location for future standards data is `TBD` and must not be inferred here.

## Verification

| Verification ID | Requirement(s) | Verification approach |
|---|---|---|
| DEL-07-03-V-001 | R-001 | Confirm changed paths remain within the assigned deliverable folder and no `ISSUED` path exists. |
| DEL-07-03-V-002 | R-002 to R-007 | Review the four-document kit for all editor surface names: materials, sections, components, load cases, supports, rule packs, and private libraries. |
| DEL-07-03-V-003 | R-003 to R-007, R-010 | Confirm all missing engineering values are `TBD`, user-supplied, or provenance-marked rather than defaulted. |
| DEL-07-03-V-004 | R-008 to R-009 | Confirm application-service command routing and durable/transient state split are recorded as constraints, with exact state library `TBD`. |
| DEL-07-03-V-005 | R-010 to R-012 | Confirm warning classes and professional-boundary text are present and no compliance/certification claim appears. |
| DEL-07-03-V-006 | R-013 | Confirm scope split risk is visible in the datasheet, guidance, procedure, and `_STATUS.md`/run records where applicable. |
| DEL-07-03-V-007 | Dependency setup | Validate `Dependencies.csv` with `tools/validation/validate_dependencies_schema.py` and enum/ID checks. |
| DEL-07-03-V-008 | Semantic setup | Confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and run records exist, and `_STATUS.md` is `SEMANTIC_READY` only after setup gates pass. |

## Documentation

Required deliverable-local setup artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_2026-04-30_1040_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1041_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1042_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1043_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1044_dependency-extract.md`

Future implementation artifacts named by the register remain anticipated, not implemented here: editor panels and validation UI tests.
