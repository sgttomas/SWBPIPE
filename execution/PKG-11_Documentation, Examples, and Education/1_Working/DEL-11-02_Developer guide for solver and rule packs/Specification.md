# Specification: DEL-11-02 Developer guide for solver and rule packs

## Scope

This deliverable defines setup evidence for a future developer guide covering solver architecture, rule-pack schema expectations, test discipline, and contribution boundaries for OpenPipeStress.

This setup pass does not edit `docs/developer_guide/index.md`, create examples, implement source code, define protected code rules, select a solver numerical library, select a rule expression grammar, update repository-level documentation, or move artifacts to `ISSUED`.

The guide is documentation for contributors. It must help developers inspect and extend the open mechanics and rule-pack infrastructure while preserving the boundary between public mechanics and user-supplied/protected code data.

## Requirements

| ID | Requirement | Evidence basis | Verification approach |
|---|---|---|---|
| REQ-11-02-001 | The future developer guide shall explain the layered architecture: GUI/application services/domain core/solver/loads/stress/rules/reports/adapters, with layer responsibilities and no-bypass adapter constraints. | AB-00-02; AB-00-07; `docs/SPEC.md` section 1 | Documentation review against architecture basis rows. |
| REQ-11-02-002 | The future guide shall describe the solver as a 3D centerline/frame mechanics engine with six degree-of-freedom nodes and deterministic result behavior. | SOW-005; `docs/SPEC.md` section 4; OPS-K-MECH-1 | Review confirms wording does not imply solid-model default or code compliance. |
| REQ-11-02-003 | The future guide shall distinguish solver mechanics from user-rule checking and professional approval. | OPS-K-MECH-2; OPS-K-AUTH-1; `docs/TYPES.md` analysis statuses | Status wording review. |
| REQ-11-02-004 | The future guide shall document that code-specific load combinations, code formulas, allowables, SIF/flexibility factors, and protected dimensional values are user-supplied/private or lawfully imported, not public defaults. | OPS-K-IP-1; OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content and data-boundary review. |
| REQ-11-02-005 | The future guide shall describe rule-pack schema expectations: identity, version, source notice, redistribution status, checksum, required inputs, variables, checks, and report notice. | `docs/SPEC.md` section 6; OPS-K-RULE-3 | Documentation review against rule-pack schema baseline. |
| REQ-11-02-006 | The future guide shall describe the evaluator boundary as sandboxed, unit-aware, deterministic, and incapable of arbitrary code execution. | OPS-K-RULE-2; OPS-K-UNIT-1; `docs/SPEC.md` section 6 | Rule-engine architecture review. |
| REQ-11-02-007 | The future guide shall require unit-aware data flow across solver inputs, rule-pack variables, imports, exports, reports, and tests. | OPS-K-UNIT-1; SOW-025 | Unit discipline checklist in guide review. |
| REQ-11-02-008 | The future guide shall explain structured diagnostics and result envelopes for solve-blocking, rule-check-blocking, provenance, assumptions, nonlinear behavior, and IP-boundary warnings. | AB-00-06; `docs/SPEC.md` section 7 | Diagnostic terminology review. |
| REQ-11-02-009 | The future guide shall define test discipline for solver, load/stress, nonlinear supports, rule packs, reports, protected-content/provenance gates, and regression evidence. | AB-00-08; `docs/VALIDATION_STRATEGY.md`; OPS-K-SOLVER-1 | Test-section review against validation strategy. |
| REQ-11-02-010 | The future guide shall explain contributor boundaries: no protected standards text/tables/figures/examples, no private project data, provenance required for public data, and quarantine/escalation for suspected protected content. | OPS-K-IP-1; OPS-K-IP-2; OPS-K-IP-3; OPS-K-PRIV-1 | Contribution-boundary review. |
| REQ-11-02-011 | The future guide shall avoid protected code examples, protected formulas, proprietary commercial examples, or invented values that could be mistaken for design guidance. | SOW-033; OPS-K-RULE-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content lint plus human review. |
| REQ-11-02-012 | The future guide shall mark unresolved implementation details as `TBD` rather than choosing a dependency version, numerical library, expression grammar, CI threshold, or physical project package format without human approval. | SOFTWARE_DECOMP section 8.2; OPS-K-AGENT-1 | TBD scan and human-ruling review. |
| REQ-11-02-013 | The future guide shall state that agent outputs, setup artifacts, generated examples, and software results remain drafts or decision support until accepted by appropriate human review. | OPS-K-AGENT-4; OPS-K-AUTH-1 | Professional-boundary wording review. |
| REQ-11-02-014 | This setup deliverable shall keep all writes within the DEL-11-02 execution folder and shall not edit the final `docs/developer_guide/index.md` artifact. | Human brief write scope | Git path review. |

## Standards

| Standard or policy source | Use in this setup evidence |
|---|---|
| OpenPipeStress CONTRACT | Governs IP, data, privacy, rule-pack, unit, professional-authority, and agent-output constraints. |
| OpenPipeStress SPEC | Provides current architecture, solver, rule-pack, GUI-warning, report, and V&V baseline for developer-guide requirements. |
| SOFTWARE_DECOMP revision 0.4 | Provides package/deliverable scope, architecture basis injection, objectives, and remaining `TBD` decisions. |
| IP and Data Boundary Policy | Governs public/private content rules, provenance, and quarantine behavior. |
| Validation Strategy | Governs test families and release-quality expectations to describe in the guide. |
| External engineering standards | May be referenced as user-owned/private design bases; protected text, tables, examples, formulas, and values are not public guide content. |

## Verification

Future guide review should include:

- Confirm the guide covers solver architecture, rule-pack schema, test discipline, and contribution boundaries.
- Confirm the guide preserves the distinction among mechanics solved, user-rule checked, and human-approved states.
- Confirm no public guide text contains protected standards text, protected examples, code-derived formulas, protected tables, proprietary vendor data, or private project data.
- Confirm rule-pack examples, if later added, are invented and clearly non-code.
- Confirm the guide explains unit-aware and provenance-aware contribution requirements.
- Confirm test sections align with validation strategy families and AB-00-08.
- Confirm no unresolved implementation detail is silently selected; unresolved items remain `TBD`.
- Confirm the anticipated artifact path remains outside this setup write scope until a later authorized session edits it.

## Documentation

Expected future product artifact:

- `docs/developer_guide/index.md`

Required supporting evidence for this setup deliverable:

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

