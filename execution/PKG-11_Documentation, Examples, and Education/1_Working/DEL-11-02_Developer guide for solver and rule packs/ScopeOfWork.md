---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-02
package_id: PKG-11
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-11-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-11-02` in service of project scope [SOW-033] and package objectives [OBJ-001, OBJ-002].

- **OUT-001** — A developer-guide contract for governed solver and rule-pack extension, testing, diagnostics, provenance, and integration boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-11-02 Developer guide for solver and rule packs

> #### Datasheet: DEL-11-02 Developer guide for solver and rule packs
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-11-02 |
> | Package ID | PKG-11 |
> | Package | Documentation, Examples, and Education |
> | Type | DOC_UPDATE |
> | Primary anticipated artifact | `docs/developer_guide/index.md` |
> | Setup artifact location | `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs/` |
> | Scope item | SOW-033 |
> | Objectives | OBJ-001, OBJ-002 |
> | Context envelope | M |
> | Current setup status | Prepared for semantic setup and dependency extraction |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | Documentation audience | Solver contributors, rule-pack contributors, reviewers, and maintainers. |
> | Guide subject | Developer-facing explanation of solver architecture, rule-pack schema expectations, test discipline, and contribution boundaries. |
> | Architecture basis | AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08 from SOFTWARE_DECOMP revision 0.7. |
> | Runtime baseline referenced | Rust core/application services; schema-first command/query/job result envelopes; JSON Schema 2020-12; Cargo/Vitest/Playwright/validation/protected-content gates where applicable. |
> | Solver boundary | The guide must present the solver as open mechanics for a 3D centerline/frame model, not as a code-compliance authority. |
> | Rule-pack boundary | The guide must present rule packs as user-supplied/private design-basis artifacts with provenance, checksums, and redistribution status. |
> | Contribution boundary | Contributors may improve mechanics, schemas, tests, docs, and permissively sourced/invented examples; they must not add protected standards data, private project data, or misleading approval claims. |
> | Professional boundary | Software output and agent artifacts are drafts/decision support until accepted by human review; they do not certify, seal, approve, authenticate, or declare engineering compliance. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Requirement for this deliverable |
> |---|---|
> | Protected content | Do not copy standards-body text, tables, examples, protected formulas, protected dimensional tables, material allowables, SIF/flexibility tables, or proprietary commercial data. |
> | Examples | Any example shown in the future guide must be invented, non-code, and marked as educational only. |
> | Missing values | Unknown implementation details, engineering values, tolerances, expression grammar choices, numerical solver choices, and CI thresholds remain `TBD` unless a human-approved source resolves them. |
> | Unit handling | Guide requirements must reinforce unit-aware model data, rule expressions, imports, exports, diagnostics, and tests. |
> | Diagnostics | Guide requirements must use structured diagnostics and result-envelope language without implying compliance approval. |
> | No-bypass baseline | Plugins, adapters, rule-pack tooling, and public APIs cannot bypass units, provenance, diagnostics, sandboxing, report controls, or data-boundary checks. |
>

### CLM-006 — Construction

> ##### Construction
>
> The future developer guide should be organized around these content groups:
>
> | Content group | Required coverage |
> |---|---|
> | Architecture map | Layer responsibilities for GUI/application services/domain core/solver/rules/reports/adapters, with no-bypass constraints. |
> | Solver architecture | Centerline/frame model, six degree-of-freedom node model, straight elements, component interfaces, loads, stress recovery, nonlinear support status, deterministic diagnostics, and test obligations. |
> | Rule-pack architecture | Schema metadata, required inputs, user-supplied variables, checks, provenance, redistribution status, checksum handling, sandboxing, unit checks, and incomplete-input behavior. |
> | Test discipline | Unit tests, schema tests, solver benchmarks, stress recovery regression, nonlinear convergence traces, rule-pack evaluator tests, report reproducibility tests, protected-content/provenance gates, and review evidence. |
> | Contribution boundaries | What contributors may add, what must be quarantined, how to label assumptions/TBDs, and when to escalate to human/legal/professional review. |
> | Review and acceptance | How developer changes move from draft evidence through review without claiming engineering reliance. |
>

### CLM-007 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `INIT.md` | Bootstrap boundaries: open mechanics, private standards data, rule checks vs professional approval, centerline analysis vs local FEA. |
> | `AGENTS.md` | TASK dispatch boundaries and documentation package role. |
> | `docs/DIRECTIVE.md` | Founding intent, stop rules, non-negotiable product principles. |
> | `docs/CONTRACT.md` | Invariants for IP, data, rule packs, units, privacy, authority, and agent boundaries. |
> | `docs/TYPES.md` | Canonical statuses, terms, provenance labels, and domain-object vocabulary. |
> | `docs/SPEC.md` | Architecture, solver, rule-pack, report, and V&V baseline for guide content. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data rules and quarantine behavior. |
> | `docs/VALIDATION_STRATEGY.md` | Verification and validation families and release gate expectations. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Scope, objectives, package context, architecture basis, and open issues. |
> | `docs/_Registers/Deliverables.csv` | DEL-11-02 identity and anticipated artifact. |
> | `docs/_Registers/ScopeLedger.csv` | SOW-033 scope mapping and no-protected-examples note. |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-11-02 Developer guide for solver and rule packs

> #### Specification: DEL-11-02 Developer guide for solver and rule packs
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-02-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines setup evidence for a future developer guide covering solver architecture, rule-pack schema expectations, test discipline, and contribution boundaries for OpenPipeStress.
>
> This setup pass does not edit `docs/developer_guide/index.md`, create examples, implement source code, define protected code rules, select a solver numerical library, select a rule expression grammar, update repository-level documentation, or move artifacts to `ISSUED`.
>
> The guide is documentation for contributors. It must help developers inspect and extend the open mechanics and rule-pack infrastructure while preserving the boundary between public mechanics and user-supplied/protected code data.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence basis | Verification approach |
> |---|---|---|---|
> | REQ-11-02-001 | The future developer guide shall explain the layered architecture: GUI/application services/domain core/solver/loads/stress/rules/reports/adapters, with layer responsibilities and no-bypass adapter constraints. | AB-00-02; AB-00-07; `docs/SPEC.md` section 1 | Documentation review against architecture basis rows. |
> | REQ-11-02-002 | The future guide shall describe the solver as a 3D centerline/frame mechanics engine with six degree-of-freedom nodes and deterministic result behavior. | SOW-005; `docs/SPEC.md` section 4; OPS-K-MECH-1 | Review confirms wording does not imply solid-model default or code compliance. |
> | REQ-11-02-003 | The future guide shall distinguish solver mechanics from user-rule checking and professional approval. | OPS-K-MECH-2; OPS-K-AUTH-1; `docs/TYPES.md` analysis statuses | Status wording review. |
> | REQ-11-02-004 | The future guide shall document that code-specific load combinations, code formulas, allowables, SIF/flexibility factors, and protected dimensional values are user-supplied/private or lawfully imported, not public defaults. | OPS-K-IP-1; OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content and data-boundary review. |
> | REQ-11-02-005 | The future guide shall describe rule-pack schema expectations: identity, version, source notice, redistribution status, checksum, required inputs, variables, checks, and report notice. | `docs/SPEC.md` section 6; OPS-K-RULE-3 | Documentation review against rule-pack schema baseline. |
> | REQ-11-02-006 | The future guide shall describe the evaluator boundary as sandboxed, unit-aware, deterministic, and incapable of arbitrary code execution. | OPS-K-RULE-2; OPS-K-UNIT-1; `docs/SPEC.md` section 6 | Rule-engine architecture review. |
> | REQ-11-02-007 | The future guide shall require unit-aware data flow across solver inputs, rule-pack variables, imports, exports, reports, and tests. | OPS-K-UNIT-1; SOW-025 | Unit discipline checklist in guide review. |
> | REQ-11-02-008 | The future guide shall explain structured diagnostics and result envelopes for solve-blocking, rule-check-blocking, provenance, assumptions, nonlinear behavior, and IP-boundary warnings. | AB-00-06; `docs/SPEC.md` section 7 | Diagnostic terminology review. |
> | REQ-11-02-009 | The future guide shall define test discipline for solver, load/stress, nonlinear supports, rule packs, reports, protected-content/provenance gates, and regression evidence. | AB-00-08; `docs/VALIDATION_STRATEGY.md`; OPS-K-SOLVER-1 | Test-section review against validation strategy. |
> | REQ-11-02-010 | The future guide shall explain contributor boundaries: no protected standards text/tables/figures/examples, no private project data, provenance required for public data, and quarantine/escalation for suspected protected content. | OPS-K-IP-1; OPS-K-IP-2; OPS-K-IP-3; OPS-K-PRIV-1 | Contribution-boundary review. |
> | REQ-11-02-011 | The future guide shall avoid protected code examples, protected formulas, proprietary commercial examples, or invented values that could be mistaken for design guidance. | SOW-033; OPS-K-RULE-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content lint plus human review. |
> | REQ-11-02-012 | The future guide shall mark unresolved implementation details as `TBD` rather than choosing a dependency version, numerical library, expression grammar, CI threshold, or physical project package format without human approval. | SOFTWARE_DECOMP section 8.2; OPS-K-AGENT-1 | TBD scan and human-ruling review. |
> | REQ-11-02-013 | The future guide shall state that agent outputs, setup artifacts, generated examples, and software results remain drafts or decision support until accepted by appropriate human review. | OPS-K-AGENT-4; OPS-K-AUTH-1 | Professional-boundary wording review. |
> | REQ-11-02-014 | This setup deliverable shall keep all writes within the DEL-11-02 execution folder and shall not edit the final `docs/developer_guide/index.md` artifact. | Human brief write scope | Git path review. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or policy source | Use in this setup evidence |
> |---|---|
> | OpenPipeStress CONTRACT | Governs IP, data, privacy, rule-pack, unit, professional-authority, and agent-output constraints. |
> | OpenPipeStress SPEC | Provides current architecture, solver, rule-pack, GUI-warning, report, and V&V baseline for developer-guide requirements. |
> | SOFTWARE_DECOMP revision 0.7 | Provides package/deliverable scope, architecture basis injection, objectives, and remaining `TBD` decisions. |
> | IP and Data Boundary Policy | Governs public/private content rules, provenance, and quarantine behavior. |
> | Validation Strategy | Governs test families and release-quality expectations to describe in the guide. |
> | External engineering standards | May be referenced as user-owned/private design bases; protected text, tables, examples, formulas, and values are not public guide content. |
>

### CLM-013 — Verification

> ##### Verification
>
> Future guide review should include:
>
> - Confirm the guide covers solver architecture, rule-pack schema, test discipline, and contribution boundaries.
> - Confirm the guide preserves the distinction among mechanics solved, user-rule checked, and human-approved states.
> - Confirm no public guide text contains protected standards text, protected examples, code-derived formulas, protected tables, proprietary vendor data, or private project data.
> - Confirm rule-pack examples, if later added, are invented and clearly non-code.
> - Confirm the guide explains unit-aware and provenance-aware contribution requirements.
> - Confirm test sections align with validation strategy families and AB-00-08.
> - Confirm no unresolved implementation detail is silently selected; unresolved items remain `TBD`.
> - Confirm the anticipated artifact path remains outside this setup write scope until a later authorized session edits it.
>

### CLM-014 — Documentation

> ##### Documentation
>
> Expected future product artifact:
>
> - `docs/developer_guide/index.md`
>
> Required supporting evidence for this setup deliverable:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`

- **AC-001** — The contract preserves architecture and extension-point descriptions, schema and unit invariants, deterministic test and benchmark expectations, rule-pack provenance and protected-content boundaries, diagnostics and failure behavior, current implementation evidence, visible unresolved interfaces, and the separation of software contribution evidence from engineering approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-11-02 Developer guide for solver and rule packs

> #### Procedure: DEL-11-02 Developer guide for solver and rule packs
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-02-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure defines how a future authorized session should produce or refresh the developer guide artifact for solver and rule-pack contributors while preserving OpenPipeStress architecture, IP, data, unit, test, and professional-responsibility boundaries.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source or status |
> |---|---|
> | Sealed DEL-11-02 brief and explicit write scope | Required before editing the final guide artifact. |
> | Current decomposition and registers | `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`. |
> | Governing invariants | `docs/CONTRACT.md`, especially IP, data, unit, rule-pack, privacy, authority, and agent invariants. |
> | Architecture basis | AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08. |
> | Source references | `INIT.md`, `docs/DIRECTIVE.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/VALIDATION_STRATEGY.md`, and workflow docs. |
> | Human authority for unresolved decisions | Required for license/contributor certification, solver numerical library, rule expression grammar, dependency versions, CI thresholds, and other `TBD` decisions. |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the active write scope.
>    - For this setup session, do not edit `docs/developer_guide/index.md`.
>    - For a future guide-authoring session, verify the human explicitly authorizes that path.
>
> 2. Build the guide outline from the required content groups.
>    - Architecture map.
>    - Solver architecture.
>    - Rule-pack architecture.
>    - Test discipline.
>    - Contribution boundaries.
>    - Review and acceptance.
>
> 3. Source every non-trivial requirement from local governing material.
>    - Prefer `docs/SPEC.md` for technical architecture and baseline mechanics.
>    - Prefer `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` for boundary constraints.
>    - Prefer `docs/VALIDATION_STRATEGY.md` for test families and release gates.
>    - Prefer SOFTWARE_DECOMP revision 0.7 for scope, objectives, and architecture basis.
>
> 4. Draft solver sections conservatively.
>    - Explain centerline/frame mechanics, six degree-of-freedom nodes, loads, stress recovery, diagnostics, and test hooks.
>    - Do not copy protected standards formulas, protected examples, code tables, or proprietary commercial examples.
>    - Keep exact tolerances, solver numerical library, and performance thresholds as `TBD` unless resolved by approved source.
>
> 5. Draft rule-pack sections conservatively.
>    - Explain schema shape, required inputs, variables, checks, provenance, redistribution status, checksums, report notices, and missing-input behavior.
>    - Explain sandboxing and unit-aware deterministic evaluation.
>    - Do not provide protected code equations, allowables, SIF/flexibility factors, protected tables, or private rule-pack content.
>
> 6. Draft test-discipline sections.
>    - Map solver and rule-pack changes to deterministic evidence families.
>    - Include protected-content and provenance gates for public examples, fixtures, docs, reports, and templates.
>    - Distinguish mechanics verification from validation and professional reliance.
>
> 7. Draft contribution-boundary sections.
>    - Define permitted public contributions and prohibited public content.
>    - Include quarantine/escalation steps for suspected protected or private data.
>    - Require `TBD`, `ASSUMPTION`, and `PROPOSAL` labels where warranted.
>
> 8. Run local review checks before proposing the guide for review.
>    - Check scope and anticipated artifact path.
>    - Scan for protected-content risk and certification/compliance overclaims.
>    - Confirm unit/provenance/test/professional boundaries are visible.
>    - Confirm unresolved implementation choices remain `TBD`.
>
> 9. Record evidence.
>    - Keep setup and drafting evidence in the deliverable folder.
>    - Record validation commands and results.
>    - Do not move anything to `ISSUED` without human review.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
> | Scope check | Documentation-only setup; no source code, examples, repo-level docs, or final guide artifact edited in this session. |
> | Boundary check | No protected standards text/tables/examples/formulas/proprietary data; no professional approval claims. |
> | Coverage check | Solver architecture, rule-pack schema, test discipline, and contribution boundaries are all addressed. |
> | TBD check | Unresolved implementation choices are marked `TBD`. |
> | Dependency check | `Dependencies.csv` is v3.1 schema-valid and `_DEPENDENCIES.md` counts match. |
> | Semantic setup | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and are lens-only evidence. |
>

### CLM-021 — Records

> ##### Records
>
> Maintain these records in this deliverable folder:
>
> - four-document setup kit;
> - semantic matrix file and lensing register;
> - dependency register and dependency index;
> - `_STATUS.md` lifecycle history;
> - `_run_records/*` for the five required setup invocations;
> - validation command results in the final TASK response.

- **VER-001** — Validate the contract and review source parity, solver and rule-pack extension coverage, architecture and schema boundaries, units/provenance/protected-content controls, deterministic tests and diagnostics, retained interface TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-11-02 Developer guide for solver and rule packs

> #### Guidance: DEL-11-02 Developer guide for solver and rule packs
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-02-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> The developer guide should give contributors enough structure to extend OpenPipeStress without weakening its governing boundaries. It should explain how the open solver mechanics, user-supplied rule packs, tests, reports, and adapter boundaries fit together, while making clear that protected standards data and professional approval remain outside the public software authority.
>

### CLM-025 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Open mechanics first | Describe the solver as an auditable mechanics engine for global centerline/frame analysis. Avoid wording that makes the solver appear to implement a proprietary code. |
> | Code-neutral rule checks | Present rule packs as user-owned design-basis artifacts. Public content may define schemas and evaluator mechanics, not protected rule content. |
> | No silent defaults | Contributor-facing docs should remind implementers that missing solve-required or rule-check-required values become explicit diagnostics, not hidden assumptions. |
> | Unit discipline | Any developer path touching model data, rule variables, imports, exports, reports, or tests must preserve unit awareness and dimensional checks. |
> | Provenance discipline | Material, component, rule, allowable, SIF, flexibility, report, and public data values require source/provenance and redistribution status. |
> | Test before reliance | Solver and rule-engine changes need deterministic tests before release use; documentation should state the expected evidence families. |
> | Human authority | The guide must not suggest that software, maintainers, or agents certify, seal, approve, authenticate, or declare project-specific engineering compliance. |
>

### CLM-026 — Considerations

> ##### Considerations
>

### CLM-027 — Solver Architecture

> ###### Solver Architecture
>
> The guide should explain the solver at a level suitable for contributors:
>
> - domain objects and unit-aware model data;
> - six degree-of-freedom node semantics;
> - local/global element orientation concepts without protected formulas;
> - stiffness assembly and sparse solve boundary at a conceptual level;
> - loads, stress recovery, result envelopes, and diagnostics;
> - nonlinear support status reporting where applicable;
> - interaction with rule packs through mechanical outputs and required-input mappings.
>
> Solver documentation should not include protected code equations or code-specific acceptance categories unless those are supplied by the user in private rule packs.
>

### CLM-028 — Rule-Pack Architecture

> ###### Rule-Pack Architecture
>
> The guide should explain the rule-pack artifact as a data contract, not as bundled standards content. The safe baseline includes identity, version, source notice, redistribution status, checksum, required input declarations, variables, checks, unit metadata, and report notice fields.
>
> The guide should make the evaluator boundary explicit: rule expressions are declarative, sandboxed, unit-aware, deterministic, and unable to execute arbitrary code. Exact expression grammar and implementation library remain `TBD` until a sealed implementation deliverable and human approval resolve them.
>

### CLM-029 — Test Discipline

> ###### Test Discipline
>
> The guide should point contributors to evidence categories rather than rely on trust:
>
> - unit/schema tests for domain objects and units;
> - deterministic solver benchmarks for frame mechanics and transforms using rights-cleared fixtures;
> - stress recovery and load application regression tests;
> - nonlinear support convergence and active-state traces;
> - rule-pack required-input, unit-mismatch, unsafe-expression, and invented-example tests;
> - report reproducibility and checksum tests;
> - protected-content and provenance gates for public docs, examples, fixtures, and reports.
>

### CLM-030 — Contribution Boundaries

> ###### Contribution Boundaries
>
> Developer documentation should give contributors simple stop rules:
>
> - do not contribute protected standards text, tables, figures, examples, protected code-derived formulas, protected dimensional tables, or proprietary vendor data;
> - do not add private project data, owner standards, company rule packs, or private component/material libraries to public examples;
> - quarantine and escalate suspected protected content;
> - label unknowns as `TBD`;
> - label inferences as `ASSUMPTION` and proposals as `PROPOSAL`;
> - keep mechanics outputs, user-rule checks, and human professional acceptance separate.
>

### CLM-031 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred direction |
> |---|---|
> | Helpful examples vs protected-content risk | Use invented non-code examples or public/permissive mechanics examples only. |
> | Detailed formulas vs IP boundary | Explain architecture and verification expectations without copying protected standards formulas or tables. |
> | Fast contribution vs review discipline | Require provenance, test evidence, and protected-content review before accepting public data or rule-pack examples. |
> | Flexible plugins vs no-bypass governance | Allow extension points only when units, provenance, diagnostics, sandboxing, privacy, and report controls remain mandatory. |
> | Mechanics solve vs rule check | Let the solver compute open mechanics; keep acceptability logic in user/private rule packs. |
>

### CLM-032 — Examples

> ##### Examples
>
> Safe example themes for the future guide:
>
> - a non-code invented rule-pack field map that uses placeholder names and no engineering values;
> - a mechanics-only test fixture described as an original/public-domain frame case, with expected values held in validation evidence rather than copied from standards;
> - a contributor checklist for adding a solver feature without changing rule-pack semantics;
> - a quarantine example that shows process fields and decisions without reproducing suspected protected content.
>
> Unsafe example themes:
>
> - copied standards text, tables, clause examples, or formulas;
> - material allowable tables or dimensional tables copied from standards;
> - private company rule packs, owner standards, project models, or commercial software benchmark files;
> - report snippets that imply official compliance, certification, approval, or professional sealing by the software.
>

### CLM-033 — Open Decisions

> ##### Open Decisions
>
> | Decision | Status |
> |---|---|
> | Exact solver numerical library | `TBD` |
> | Rule expression grammar/library | `TBD` |
> | Exact dependency versions | `TBD` |
> | CI provider, coverage thresholds, and performance thresholds | `TBD` |
> | Physical project package/container | `TBD` |
> | License and contributor certification mechanism | `TBD` until human project authority records it |
>

### CLM-034 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were identified during setup. If later guide drafting discovers conflict between architecture basis, implementation practice, legal/data-boundary policy, or validation requirements, record it here rather than silently resolving it.
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | TBD | None recorded. | TBD | TBD | TBD | TBD | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 OBJ-002 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
