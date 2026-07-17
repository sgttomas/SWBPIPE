---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-02
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-006]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-02` in service of project scope [SOW-006] and package objectives [OBJ-003].

- **OUT-001** — A straight-pipe-element contract covering local stiffness, explicit section-property integration, weight hooks, boundary metadata, spanned loads and axial effects, unit-aware end/station resultant recovery, and deterministic solver verification is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-02 Straight pipe element

> #### Datasheet: DEL-04-02 Straight pipe element
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-04-02 |
> | Package ID | PKG-04 |
> | Package | Solver Core and Numerical Methods |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-006 |
> | Objective | OBJ-003 |
> | Context envelope | M |
> | Anticipated artifacts | straight pipe element; solver tests |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | Element family | Straight pipe centerline/frame element |
> | Analysis model boundary | 3D centerline/frame mechanics with six degrees of freedom per node; shell/solid FEA remains a local-analysis handoff path. |
> | Covered mechanics | Local stiffness, section-property integration, weight hooks, and element force recovery. |
> | Excluded mechanics | Code compliance decisions, rule-pack acceptability, protected standard formulas/tables, and repo-bundled protected dimensional or material values. |
> | Unit policy | All inputs, intermediate values, and outputs are unit-aware and dimensionally checked. Exact unit API is `TBD`. |
> | Data provenance policy | Pipe dimensions, material values, and other solve inputs are user-supplied or lawfully imported private/project data. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Solver numerical library | `TBD` from architecture/implementation decision. |
> | Section-property source contract | `TBD`; must connect to user/project data or lawful library inputs. |
> | Weight-load integration contract | `TBD`; must remain a hook to load-case behavior, not a hidden default load application. |
> | Element force recovery conventions | `TBD`; must be unit-aware and consistent with solver result envelopes. |
> | Test fixture data | Must be synthetic, public-domain, or otherwise cleared for repository use. |
>

### CLM-006 — Construction

> ##### Construction
>
> The setup kit describes the future implementation boundary only. It does not implement solver code, choose element dimensions, choose material values, encode protected formulas, or create repo-level tests.
>
> The future element is expected to receive validated geometry, section properties, material/mechanical inputs, and load hooks through governed domain/service contracts. Missing solve-required values must produce explicit findings rather than silent defaults.
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-04-02`.
> - `docs/_Registers/ScopeLedger.csv` row `SOW-006`.
> - `docs/_Registers/ContextBudgetQA.csv` row `DEL-04-02`.
> - `docs/CONTRACT.md` invariants listed in the sealed brief.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, and `AB-00-08`.
>

### CLM-008 — Open Setup Questions

> ##### Open Setup Questions
>
> | Question | Status |
> |---|---|
> | Which upstream schema owns straight-pipe section-property inputs? | `TBD` |
> | Which solver-kernel interface owns local-to-global transformation and assembly handoff? | `TBD` |
> | Which load engine interface receives or invokes weight hooks? | `TBD` |
> | Which deterministic verification cases are accepted for the element without protected data? | `TBD` |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-04-02 Straight pipe element

> #### Specification: DEL-04-02 Straight pipe element
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the implemented straight pipe element backend slice. It covers straight pipe local stiffness, section-property integration, weight hooks, boundary metadata, end/station resultant recovery, spanned loads, and axial effects within the global 3D centerline/frame solver architecture.
>
> Current evidence lives in `core/solver/straight_pipe` with deterministic unit and witness-backed benchmark coverage. This documentation refresh does not edit solver code or fixtures, introduce protected formulas or tables, or make certification/compliance claims.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-04-02-RQ-001 | The straight pipe element shall fit the 3D centerline/frame solver model and shall not bypass the solver kernel boundary. | SOW-006; OPS-K-MECH-1; AB-00-02 | Architecture/module-boundary review once implementation paths are selected. |
> | DEL-04-02-RQ-002 | Local stiffness behavior shall be derived only from open mechanics and user/project/lawfully imported input values. | SOW-006; OPS-K-IP-1; OPS-K-DATA-1 | Protected-content review and fixture review. |
> | DEL-04-02-RQ-003 | Section-property integration shall require explicit section-property inputs or validated upstream calculations; missing solve-required properties shall produce explicit findings. | SOW-006; OPS-K-DATA-2; OPS-K-UNIT-1 | Negative tests for missing properties and units; exact cases `TBD`. |
> | DEL-04-02-RQ-004 | Weight hooks shall expose the information needed for load-case application without silently applying hidden load defaults. | SOW-006; OPS-K-DATA-2; AB-00-03 | Load-interface tests once the primitive load contract is accepted. |
> | DEL-04-02-RQ-005 | Element force recovery shall return unit-aware mechanical result components suitable for downstream stress recovery, without encoding code stress checks. | SOW-006; OPS-K-MECH-2; OPS-K-UNIT-1 | Solver result-envelope tests and downstream interface review. |
> | DEL-04-02-RQ-006 | Solver changes shall include deterministic verification tests before release use. | OPS-K-SOLVER-1; AB-00-08 | Deterministic solver tests using synthetic or cleared inputs. |
> | DEL-04-02-RQ-007 | Diagnostics shall use governed result-envelope concepts and shall not claim professional approval, certification, or compliance (PRD §21.2). | AB-00-06; OPS-K-AGENT-4 | Diagnostic/result-envelope review. |
>

### CLM-012 — Standards

> ##### Standards
>
> No protected standard text, protected formulas, protected dimensional tables, material allowables, or proprietary commercial data are available in this deliverable-local setup context. Any future code or standard basis must be introduced only as a lawful private/project input or non-protected pointer with provenance. Clause-level requirements are `TBD`.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification area | Minimum setup expectation |
> |---|---|
> | Solver boundary | Tests and reviews must show the element is a solver component, not a rule-pack or compliance component. |
> | Unit safety | Tests must cover dimensional checking for section properties, stiffness-related inputs, weight-related inputs, and recovered forces. |
> | Missing inputs | Tests must show explicit findings for missing solve-required values. |
> | Force recovery | Tests must show deterministic recovered mechanical result components; exact benchmark cases are `TBD`. |
> | IP/data boundary | Test data must be synthetic, public-domain, or otherwise cleared for redistribution. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Current implementation artifacts are:
>
> - `core/solver/straight_pipe`;
> - its deterministic crate tests and rights-safe straight-pipe mechanics witnesses.
>
> Module and test paths are now evidenced in-tree. Production numerical-library
> integration, final governed per-kind tolerances, and full solver-to-result-envelope
> binding remain `TBD` or separately recorded residual work.
>

### CLM-015 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A |

- **AC-001** — The contract preserves the accepted straight-pipe mechanics and interface boundaries, including explicit units and lawful input provenance, no hidden load or engineering defaults, rights-cleared fixtures, mechanics-only outputs, and the unresolved governed solver-to-result-envelope integration.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-04-02 Straight pipe element

> #### Procedure: DEL-04-02 Straight pipe element
>

### CLM-017 — Purpose

> ##### Purpose
>
> Define the bounded maintenance and verification procedure for the implemented straight pipe element without expanding its mechanics scope.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Confirm the task names `DEL-04-02` and its authorized write scope.
> - Confirm upstream architecture basis constraints from `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, and `AB-00-08`.
> - Confirm the current `core/solver/straight_pipe` unit/domain and frame-kernel interfaces; unresolved integration items remain `TBD`.
> - Confirm all example dimensions, material values, and fixtures are synthetic, public-domain, or otherwise cleared.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Re-read `_CONTEXT.md`, `Specification.md`, and `_DEPENDENCIES.md`.
> 2. Identify the accepted module boundary for the straight pipe element and its relation to the global frame kernel.
> 3. Identify required input contracts for section properties, material/mechanical values, units, weight hooks, diagnostics, and result envelopes.
> 4. Maintain only the authorized straight-pipe local element behavior; do not broaden into stress-code or professional-acceptance logic.
> 5. Add explicit findings for missing solve-required values and unit mismatches; do not apply silent defaults.
> 6. Add deterministic solver tests with synthetic or cleared inputs.
> 7. Verify recovered element forces remain mechanical results and do not claim code compliance.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Scope boundary | Implementation changes are limited to the approved implementation scope for DEL-04-02. |
> | Unit behavior | Unit-aware tests pass and dimensional mismatches fail explicitly. |
> | Missing input behavior | Missing solve-required properties produce findings. |
> | Solver result behavior | Element force recovery is deterministic for accepted synthetic/cleared cases. |
> | IP/data boundary | No protected tables, formulas, examples, material allowables, or proprietary data are added. |
>

### CLM-021 — Records

> ##### Records
>
> - Implementation or maintenance run records for authorized code work.
> - Solver test results.
> - Fixture provenance notes.
> - Protected-content review evidence where applicable.
> - Any human rulings for `TBD` items.

- **VER-001** — Validate the contract and review source parity, straight-pipe behavior and frame-kernel boundaries, section and weight interfaces, spanned-load and resultant coverage, dimensional checks, explicit findings, protected-content controls, and result-envelope residuals.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-04-02 Straight pipe element

> #### Guidance: DEL-04-02 Straight pipe element
>

### CLM-023 — Purpose

> ##### Purpose
>
> This deliverable exists to isolate the straight pipe element as one bounded solver feature slice. It should give the global 3D frame solver a governed, testable element behavior while keeping section data, units, diagnostics, load hooks, and downstream force recovery explicit.
>

### CLM-024 — Principles

> ##### Principles
>
> - Keep mechanics computation separate from rule-pack acceptability and human compliance judgment.
> - Treat dimensions, material values, and protected standard-derived values as external governed inputs, never public bundled defaults.
> - Prefer explicit `TBD` and explicit diagnostics over silent assumptions when solve-required values are missing.
> - Preserve unit and provenance information through element inputs and outputs.
> - Keep weight behavior as an interface hook until the load-case contract determines how loads are formed and applied.
>

### CLM-025 — Considerations

> ##### Considerations
>
> The implemented element consumes explicit section properties and mechanics inputs through the frame-kernel boundary, carries unit/boundary metadata, and supports deterministic force/resultant recovery and spanned-load behavior. Full application-service result-envelope production remains separate residual work.
>
> Verification should use open, synthetic, or cleared test cases. Any hand-check examples must avoid copying protected standard examples, tables, or protected formula presentations.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Topic | Guidance |
> |---|---|
> | Completeness vs data boundary | Record missing section or material values as findings rather than providing public defaults. |
> | Element scope vs solver kernel scope | Keep local element behavior here; keep global assembly, transforms, and sparse solve responsibilities in the solver kernel deliverable unless the accepted implementation boundary says otherwise. |
> | Load hooks vs load application | Expose weight-related information without creating hidden primitive load behavior inside the element. |
> | Force recovery vs stress checks | Recover mechanical element forces; downstream stress recovery and rule checks remain separate deliverables. |
>

### CLM-027 — Examples

> ##### Examples
>
> Rights-safe deterministic examples exist in the straight-pipe mechanics witness
> and benchmark families. They are software verification evidence, not design
> examples or professional acceptance, and must not be replaced with protected
> standards content.
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | None | No setup conflict found. | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-006 OBJ-003 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
