---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-02
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-006]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-02

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-02` in service of project scope [SOW-006] and package objectives [OBJ-003].

- **OUT-001** — A straight-pipe-element contract covering local stiffness, explicit section-property integration, weight hooks, boundary metadata, spanned loads and axial effects, unit-aware end/station resultant recovery, and deterministic solver verification is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-02 Straight pipe element

> #### Datasheet: DEL-04-02 Straight pipe element
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

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
> | Unit policy | All inputs, intermediate values, and outputs are unit-aware and dimensionally checked. Current metadata boundary is `StraightPipeBoundaryMetadata` over `FrameKernelUnitBasis`; product normalization requires its own evidence. |
> | Data provenance policy | Pipe dimensions, material values, and other solve inputs are user-supplied or lawfully imported private/project data. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Solver numerical library | DEC-023 strategy; live/default policy DEC-050/053. |
> | Section-property source contract | Explicit `StraightPipeSectionProperties` inputs must connect to user/project data or lawful library inputs. |
> | Weight-load integration contract | Explicit mass/weight-per-length hooks connect to the load owner; product self-weight generation remains user-input mechanics, never a hidden default. |
> | Element force recovery conventions | Current local end-resultant and displacement/force recovery conventions are in `core/solver/straight_pipe/README.md`; units and result-envelope consistency remain required. |
> | Test fixture data | Must be synthetic, public-domain, or otherwise cleared for repository use. |
>

### CLM-006 — Construction

> ##### Construction
>
> The contract describes the implemented straight-pipe boundary in `core/solver/straight_pipe/`. It supplies no element dimensions, material defaults or protected values; implementation tests remain candidate-bound.
>
> The element must receive validated geometry, section properties, material/mechanical inputs, and load hooks through governed domain/service contracts. Missing solve-required values must produce explicit findings rather than silent defaults.
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-04-02`.
> - `docs/_Registers/ScopeLedger.csv` row `SOW-006`.
> - `docs/_Registers/ContextBudgetQA.csv` row `DEL-04-02`.
> - `docs/CONTRACT.md` invariants listed in the sealed brief.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` (accepted authority through the decision register) architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, and `AB-00-08`.
>

### CLM-008 — Open Setup Questions

> ##### Open Setup Questions
>
> | Question | Status |
> |---|---|
> | Which upstream schema owns straight-pipe section-property inputs? | DEL-03-08 owns calculation; DEL-03-02 owns schema; `StraightPipeSectionProperties` is the primitive input contract. |
> | Which solver-kernel interface owns local-to-global transformation and assembly handoff? | `core/solver/frame_kernel/` owns primitive transforms/assembly; current product integration follows the accepted decomposition. |
> | Which load engine interface receives or invokes weight hooks? | `core/loads/primitive_loads/` and the product self-weight path; explicit input and no-default obligations remain. |
> | Which deterministic verification cases are accepted for the element without protected data? | Current straight-pipe crate witnesses are implementation verification evidence; professional/independent validation remains separate. |

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
> | DEL-04-02-RQ-003 | Section-property integration shall require explicit section-property inputs or validated upstream calculations; missing solve-required properties shall produce explicit findings. | SOW-006; OPS-K-DATA-2; OPS-K-UNIT-1 | Negative tests for missing properties and units; current cases are the straight-pipe section-validation and blocking-error tests, with results bound to their candidate. |
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
> | Force recovery | Tests must show deterministic recovered mechanical result components; current witnesses are the axial, transverse-bending and end-resultant tests in `core/solver/straight_pipe/src/lib.rs`; engineering validation remains separate. |
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
> Module and test paths are evidenced in-tree. The product/runner route supplies bounded envelope-binding evidence through `core/product_physics/`; DEC-023/050/053 governs the sparse path. Complete per-kind numeric evidence and report vocabulary remain separately assessed; no whole-envelope or engineering acceptance is inferred.
>

### CLM-015 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A |

- **AC-001** — The contract preserves the accepted straight-pipe mechanics and interface boundaries, including explicit units and lawful input provenance, no hidden load or engineering defaults, rights-cleared fixtures, mechanics-only outputs, and the governed product/runner envelope boundary, with uncovered per-kind evidence and report-vocabulary integration retained.

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
> 1. Re-read `_CONTEXT.md`, `ScopeOfWork.md` (requirements), and `_DEPENDENCIES.md`.
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
> The implemented element consumes explicit section properties and mechanics inputs through the frame-kernel boundary, carries unit/boundary metadata, and supports deterministic force/resultant recovery and spanned-load behavior. Bounded product/runner envelope realization is evidenced through `core/product_physics/`; remaining coverage and report vocabulary must be assessed separately.
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

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-04-02:1** — Locate or obtain a protected-content review bound to the straight-pipe source/test candidate and retain fresh scope parity as an evidence task. (R5 continuation claim references: DEL-04-02:SOW#CLM-021; production-and-verification-method-praxeology/VER-001.)
