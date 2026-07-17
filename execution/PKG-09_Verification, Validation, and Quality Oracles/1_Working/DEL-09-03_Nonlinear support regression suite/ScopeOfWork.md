---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-03
package_id: PKG-09
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-026]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-09-03` in service of project scope [SOW-026] and package objectives [OBJ-008].

- **OUT-001** — A nonlinear-support regression-suite contract covering active-set, gap, friction, lift-off, convergence, non-convergence, and deterministic rerun evidence is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-09-03 Nonlinear support regression suite

> #### Datasheet: DEL-09-03 Nonlinear support regression suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-09-03-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-09-03 |
> | Name | Nonlinear support regression suite |
> | Package ID | PKG-09 |
> | Package Name | Verification, Validation, and Quality Oracles |
> | Deliverable Type | TEST_SUITE |
> | Scope Coverage | SOW-026 |
> | Objective Support | OBJ-008 |
> | Context Envelope | M |
> | Current Setup Role | Documented setup surface only; no regression implementation in this pass. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Source-grounded value |
> |---|---|
> | Intended test area | Active-set, gap, friction, lift-off convergence, and regression cases. |
> | Anticipated final artifact areas | `validation/benchmarks/nonlinear`; regression tests. |
> | Source boundary | Benchmark sources must be public, original, or permissive. |
> | Data boundary | Protected standards text, code tables, protected examples, proprietary values, and commercial benchmark data without rights are excluded. |
> | Solver dependency | Depends on nonlinear solver maturity. |
> | Diagnostics dependency | Nonlinear support behavior must expose convergence, active-state, gap, lift-off, friction-state, iteration-count, tolerance, and non-convergence warning information. |
> | Units expectation | Inputs, checks, and result comparisons must be unit-aware. |
> | Professional boundary | Regression success is software-quality evidence only. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
>

### CLM-005 — Conditions

> ##### Conditions
>
> The suite is scoped to verification and regression coverage for nonlinear support behavior in the open mechanics solver. It is not a substitute for project-specific validation or competent professional review.
>
> The current task is setup/document production only. It does not create benchmark source files, test fixtures, solver code, or final numerical convergence tolerances.
>

### CLM-006 — Construction

> ##### Construction
>
> The future suite should be organized around source-qualified case definitions, expected diagnostic/result-envelope observations, deterministic rerun criteria, and release-gate integration. Case content remains `TBD` until the nonlinear support solver and related diagnostics are mature enough to support defensible regression criteria.
>

### CLM-007 — References

> ##### References
>
> | Source | Used for |
> |---|---|
> | `INIT.md` | Open mechanics, data boundary, and professional-responsibility boundaries. |
> | `AGENTS.md` | Type 2 sealed deliverable execution and write-scope constraints. |
> | `docs/CONTRACT.md` | Invariants for IP/data boundaries, unit safety, nonlinear diagnostics, and agent authority. |
> | `docs/TYPES.md` | TEST_SUITE type, analysis-status vocabulary, and data/provenance terms. |
> | `docs/SPEC.md` | Nonlinear support diagnostics, numerical quality, V&V mechanics, and repository layout target. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-09 package scope, DEL-09-03 deliverable entry, SOW-026, and OBJ-008. |
> | `docs/_Registers/Deliverables.csv` | Deliverable identity, artifacts, scope, objective, and risk note. |
> | `docs/_Registers/ScopeLedger.csv` | SOW-026 scope statement and public/original/permissive benchmark-source note. |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-09-03 Nonlinear support regression suite

> #### Specification: DEL-09-03 Nonlinear support regression suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-09-03-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the setup surface for a future nonlinear support regression suite covering active-set, gap, friction, lift-off convergence, and regression cases for OpenPipeStress.
>
> In scope for this setup pass:
>
> - document the regression-suite purpose, boundaries, dependencies, and acceptance surface;
> - preserve public/original/permissive benchmark-source constraints;
> - identify diagnostics and result-envelope information the future tests must observe;
> - record `TBD` items that depend on nonlinear solver maturity.
>
> Out of scope for this setup pass:
>
> - implementing nonlinear solver behavior;
> - creating benchmark source files or regression tests;
> - copying protected standards examples, code text, tables, formulas, or proprietary commercial cases;
> - inventing final numerical convergence tolerances;
> - asserting certification, professional approval, or code compliance (PRD §21.2).
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source basis | Verification approach |
> |---|---|---|---|
> | REQ-09-03-001 | The future suite shall cover active-set, gap, friction, lift-off convergence, and regression behavior. | DEL-09-03 register description; SOW-026; `docs/SPEC.md` section 4.4 and 9. | Confirm case catalog maps each behavior category to at least one source-qualified case once implementation begins. |
> | REQ-09-03-002 | Benchmark and regression case sources shall be public, original, or permissively licensed, with provenance recorded before public contribution. | SOW-026 note; OPS-K-IP-1/2/3; OPS-K-DATA-1/2/3. | Review case-source records and reject or quarantine suspected protected or proprietary content. |
> | REQ-09-03-003 | The suite shall not bundle protected standards data, protected examples, code-derived tables, proprietary component values, or commercial benchmark cases without redistribution rights. | `INIT.md`; `docs/DIRECTIVE.md`; OPS-K-IP-1/2/3. | Protected-content/provenance review before any case enters the public repository. |
> | REQ-09-03-004 | Nonlinear support regression checks shall observe diagnostic/result-envelope fields for active/inactive state, gaps, lift-off, friction state, convergence tolerance, iteration count, and non-convergence warnings when those fields exist. | `docs/SPEC.md` section 4.4; OPS-K-SOLVER-2; architecture basis AB-00-06. | Schema and test review against the diagnostics/result-envelope contract. |
> | REQ-09-03-005 | Regression comparisons shall be deterministic for the same model, units, solver version, and applicable rule-pack inputs. | `docs/SPEC.md` section 4.5; OBJ-008. | Repeat-run checks once solver and runner support exist. |
> | REQ-09-03-006 | All case definitions, inputs, and expected observations shall be unit-aware and dimensionally checkable. | OPS-K-UNIT-1; `docs/SPEC.md` sections 3, 4.5, and 9. | Unit/schema validation before case acceptance. |
> | REQ-09-03-007 | Final numerical convergence tolerances and pass/fail thresholds shall remain `TBD` until nonlinear solver maturity provides evidence for defensible values. | DEL-09-03 risk note; OPS-K-AGENT-1. | Human review of solver evidence and tolerance proposal before implementation. |
> | REQ-09-03-008 | Regression outcomes shall be reported as software verification evidence only and shall not claim code compliance, certification, sealing, approval, or professional reliance (PRD §21.2). | OPS-K-AUTH-1; `docs/TYPES.md` analysis-status vocabulary; `docs/DIRECTIVE.md` section 4.2. | Review report text and test labels for prohibited authority claims. |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code text or protected standard is imported by this setup deliverable.
>
> | Standard or governance source | Status |
> |---|---|
> | OpenPipeStress invariant catalog | Applicable through `docs/CONTRACT.md`. |
> | OpenPipeStress technical specification | Applicable through `docs/SPEC.md`. |
> | External protected design codes | Not used as public source material; any future user-private use remains outside this public setup artifact. |
>

### CLM-013 — Acceptance Criteria

> ##### Acceptance Criteria
>
> This setup deliverable is acceptable when:
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and remain within the deliverable write scope;
> - `_SEMANTIC.md` is generated and audited without final-cell algebra/operator leaks;
> - `_SEMANTIC_LENSING.md` covers matrices A, B, C, F, D, X, and E and records warranted gaps without inventing content;
> - `Dependencies.csv` is parseable with the v3.1 dependency schema;
> - `_DEPENDENCIES.md` summarizes active anchors, constraints, warnings, and run history;
> - `_STATUS.md` records `SEMANTIC_READY` only after the setup sequence completes;
> - no benchmark source files, regression tests, solver code, protected examples, or final convergence tolerances are introduced.
>

### CLM-014 — Deferred Implementation Decisions

> ##### Deferred Implementation Decisions
>
> | Decision Slot | Current setup disposition | Required future evidence |
> |---|---|---|
> | Nonlinear convergence tolerances | `TBD`; not invented in setup. | Nonlinear solver maturity evidence, repeat-run behavior, and human-reviewed tolerance proposal. |
> | Diagnostic/result-envelope field names | `TBD`; categories are named but exact schema fields are not fixed here. | Applicable schema/solver contract for active state, gap/lift-off, friction state, iteration count, tolerance basis, and non-convergence warnings. |
> | Release-gate command names | `TBD`; no implementation commands are defined in setup. | Actual regression runner, CI entry points, and validation command names from later implementation work. |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Required setup artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_*.md`
> - `_STATUS.md`

- **AC-001** — The contract preserves source-qualified public/original/permissive cases, unit-aware observations, active and friction state, gap and lift-off, iteration and tolerance basis, diagnostics, solver-maturity dependencies, explicit TBD thresholds, and the software-verification-only authority boundary.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-09-03 Nonlinear support regression suite

> #### Procedure: DEL-09-03 Nonlinear support regression suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-09-03-DECL-004`.
>

### CLM-018 — Purpose

> ##### Purpose
>
> Define the operating procedure for producing the future nonlinear support regression suite while preserving the current setup boundary. This procedure is not an implementation runbook for solver code or test files.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> - Sealed TASK context for DEL-09-03.
> - Applicable invariants from `docs/CONTRACT.md`, including OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-SOLVER-2, and OPS-K-AGENT-1..4.
> - DEL-09-03 register scope: SOW-026 and OBJ-008.
> - Nonlinear solver maturity sufficient to define intended behavior and defensible expected observations.
> - Diagnostics/result-envelope contract sufficient to record convergence, active state, gap/lift-off/friction state, iteration count, tolerance basis, and non-convergence warnings.
> - Public/original/permissive source basis for each future case.
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the working folder is the DEL-09-03 deliverable folder and that writes are deliverable-local.
> 2. Maintain the four-document kit describing scope, requirements, guidance, and future execution procedure.
> 3. Define future case categories only at the setup level: active-set, gap, friction, lift-off, convergence, and non-convergence behavior.
> 4. For each future case, require a source/provenance record before implementation. Mark source gaps as `TBD`.
> 5. Do not use protected standards examples, code tables, proprietary commercial software outputs, or vendor data without documented redistribution rights.
> 6. Defer final numerical convergence tolerances until nonlinear solver evidence is available and reviewed.
> 7. When implementation is later authorized, create case fixtures and regression tests only in the appropriate validation/test locations named by the decomposition.
> 8. Validate future tests through unit/schema checks, deterministic reruns, diagnostics/result-envelope review, and protected-content/provenance review.
> 9. Report regression outcomes as software-quality evidence only; do not claim code compliance, certification, sealing, approval, or professional reliance (PRD §21.2).
>

### CLM-021 — Verification

> ##### Verification
>
> For this setup pass, verify:
>
> - the four-document kit exists;
> - semantic matrix and lensing artifacts exist and are internally structured;
> - dependency artifacts exist and use the v3.1 schema;
> - `_STATUS.md` records `SEMANTIC_READY` only after setup artifacts are generated;
> - no files were written outside the assigned deliverable folder;
> - no nonlinear solver code, regression tests, benchmark source files, protected examples, or final convergence tolerances were introduced.
>
> For a later implementation pass, verification must include:
>
> - unit-aware case schema validation;
> - public/original/permissive provenance review;
> - deterministic repeat-run checks;
> - active-state and convergence diagnostic checks;
> - protected-content scan and human review gate.
>
> Concrete validation command names are `TBD` until the regression runner, CI entry points, and nonlinear solver diagnostics exist. Do not substitute placeholder commands as if they were accepted release gates.
>

### CLM-022 — Records

> ##### Records
>
> Maintain these setup records in the deliverable folder:
>
> - four-document kit;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - `_DEPENDENCIES.md`;
> - `_run_records/TASK_RUN_*.md`;
> - `_STATUS.md`.

- **VER-001** — Validate the contract and review source parity, nonlinear behavior-category coverage, provenance and protected-content exclusions, unit and deterministic rerun requirements, diagnostic/result-envelope categories, retained solver-maturity and tolerance TBDs, and prohibited authority claims.

## Governing Values and Decisions — Axiology

### CLM-023 — Guidance: DEL-09-03 Nonlinear support regression suite

> #### Guidance: DEL-09-03 Nonlinear support regression suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-024 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-09-03-DECL-003`.
>

### CLM-025 — Purpose

> ##### Purpose
>
> This deliverable prepares the document and semantic foundation for nonlinear support regression testing. The suite is intended to help future solver work demonstrate repeatable behavior for active-set, gap, friction, lift-off, convergence, and non-convergence diagnostics without importing protected examples or overstating the meaning of test success.
>

### CLM-026 — Principles

> ##### Principles
>
> - Keep the suite evidence-first: every future case needs a source/provenance record or an explicit original/invented basis.
> - Treat missing convergence thresholds as `TBD`; do not invent final values during setup.
> - Separate solver verification from user rule checks and professional approval.
> - Exercise diagnostics as part of regression behavior, not only displacement or reaction magnitudes.
> - Keep unit handling visible in fixtures, comparisons, and expected observations.
> - Prefer small, original mechanics cases for public regression coverage unless a public/permissive source can be documented.
>

### CLM-027 — Considerations

> ##### Considerations
>
> Nonlinear support behavior is sensitive to solver maturity. A regression suite created too early can freeze incidental behavior instead of intended behavior. This setup therefore records the categories and gates but defers final cases, tolerances, and pass/fail thresholds until the nonlinear active-set solver and diagnostics contract are mature enough to support them.
>
> The suite should distinguish at least these future evidence categories:
>
> | Category | Setup guidance |
> |---|---|
> | Active-set state | Verify active/inactive state reporting when the solver exposes it. |
> | Gap closure/opening | Verify state transition and diagnostic reporting without inventing protected benchmark values. |
> | Lift-off | Verify one-way support disengagement behavior using original or permissive cases. |
> | Friction state | Verify stick/slip or equivalent reported state only after the solver defines the state vocabulary. |
> | Convergence | Record iteration counts, tolerance basis, and non-convergence warnings through the result envelope. |
> | Regression stability | Compare deterministic outputs for the same model, units, solver version, and input manifest. |
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> Early setup can identify coverage categories and source rules, but it cannot provide authoritative numerical thresholds. Later implementation should avoid broad, opaque acceptance bands unless they are justified by solver evidence, unit behavior, and repeatability analysis.
>
> Regression tests should be specific enough to detect solver drift while avoiding dependence on protected standards data or commercial software results. Public/original examples are preferable where they can test mechanics and diagnostics directly.
>

### CLM-029 — Examples

> ##### Examples
>
> Specific regression case definitions are `TBD`. No example model, benchmark file, commercial comparison, or code-derived case is introduced by this setup pass.
>

### CLM-030 — Open Issues

> ##### Open Issues
>
> | Issue ID | Topic | Status |
> |---|---|---|
> | OI-09-03-001 | Final nonlinear convergence tolerances and pass/fail thresholds. | TBD pending nonlinear solver maturity and human review. |
> | OI-09-03-002 | Exact diagnostic/result-envelope field names for active state, friction state, and non-convergence warning records. | TBD pending applicable schema/solver contract. |
> | OI-09-03-003 | Public/original/permissive source list for future nonlinear support regression cases. | TBD; no protected examples may be copied or paraphrased into public artifacts. |
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were identified in the setup pass. Open issues above are gaps caused by deferred solver maturity, not contradictions between sources.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 OBJ-008 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
