---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-05
package_id: PKG-05
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-052, SOW-013]
package_objective_refs: [OBJ-003, OBJ-012]
---

# Scope of Work — DEL-05-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-05-05` in service of project scope [SOW-052, SOW-013] and package objectives [OBJ-003, OBJ-012].

- **OUT-001** — A user-load-application contract covering concentrated forces and moments, full and partial-span distributed loads, oriented straight-pipe equivalent nodal recovery, unit-aware boundary records, deterministic findings, axial-effect bridges, and downstream result hooks is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-05-05 Concentrated and distributed user load application

> #### Datasheet: DEL-05-05 Concentrated and distributed user load application
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-05-05 |
> | Package ID | PKG-05 |
> | Package Name | Loads, Load Cases, and Stress Recovery |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope Items | SOW-052, SOW-013 |
> | Objectives | OBJ-003, OBJ-012 |
> | Context Envelope | M |
> | Anticipated Artifacts | load application module; load tests; result hooks |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Evidence-grounded value |
> |---|---|
> | Primary subject | Implemented code-neutral user-load application for concentrated forces, concentrated moments, and uniform distributed user loads. |
> | Implementation surface | `core/loads/user_loads` provides the bounded Rust crate for DEL-05-05. |
> | Load boundary | General user loads are explicit mechanics inputs in addition to primitive piping load categories; code-specific combinations remain outside this crate. |
> | Unit posture | `UserLoadQuantity` carries a `LoadDimension`; boundary records require explicit unit metadata, provenance references, canonical schema binding, JCS payload references, and payload-hash references. |
> | Solver posture | Generic application emits nodal and element distributed contribution records; straight-pipe recovery emits equivalent global nodal loads for oriented straight-pipe elements. |
> | Result posture | Result recovery hooks are implemented for nodal force, nodal moment, element distributed load, and bridged element axial-effect records; final result-envelope integration remains TBD. |
> | Governance boundary | No design-code load combinations, public default factors, protected standards content, proprietary project data, rule-pack checks, or professional/code-compliance claims are introduced. |
> | Remaining TBDs | Final result-envelope/API/persistence/GUI/CLI/report integration, production tolerance policy, release thresholds, primitive axial-effect provenance beyond `load_id`, and professional reliance remain TBD. |
>

### CLM-004 — Conditions

> ##### Conditions
>
> - Must preserve the mechanics/rule-pack separation in `OPS-K-MECH-2`.
> - Must treat code-specific values and combinations as user-supplied or rule-pack supplied under `OPS-K-DATA-1`.
> - Must surface missing solve-required or rule-check-required values explicitly under `OPS-K-DATA-2`.
> - Must remain unit-aware and dimensionally checked under `OPS-K-UNIT-1`.
> - Must not introduce invented load magnitudes, default factors, code allowables, or certification claims.
>

### CLM-005 — Construction

> ##### Construction
>
> Implemented evidence from `core/loads/user_loads/README.md` and
> `core/loads/user_loads/src/lib.rs` identifies these current artifact slots:
>
> - `load application module`: `apply_user_loads` prepares explicit nodal
>   concentrated force/moment contributions and element uniform distributed-load
>   contribution records for the frame-solver boundary.
> - `straight-pipe equivalent recovery`: `apply_straight_pipe_equivalent_user_loads`
>   recovers full-span and partial-span distributed loads plus element-station
>   concentrated forces into equivalent global nodal loads for a supplied
>   `StraightPipeElement`.
> - `oriented straight-pipe handling`: global `X`, `Y`, and `Z` directions are
>   converted to global load vectors and handled by the straight-pipe helper for
>   pipes that are not aligned with global `X`.
> - `axial-effect bridge`: `apply_straight_pipe_equivalent_user_loads_with_axial_effects`
>   appends already-prepared primitive axial-effect contributions without
>   duplicating axial-effect mechanics in `user_loads`.
> - `load tests`: Rust unit tests cover concentrated force, concentrated moment,
>   distributed loads, partial-span recovery, oriented straight-pipe recovery,
>   station point-force recovery, axial-effect bridge behavior, boundary metadata,
>   deterministic ordering, and invalid/missing input findings.
> - `result hooks`: recovery hooks preserve load identity, hook kind, target
>   reference, dimension, and provenance reference for downstream stress
>   recovery, reporting, export, GUI, and headless execution work.
>
> Deterministic finding coverage includes missing targets or quantities,
> out-of-range node/element indices, invalid dimensions or directions,
> unsupported target/load-kind pairings, invalid distribution spans,
> non-positive element lengths, missing straight-pipe geometry, and nonfinite
> axial-effect inputs.
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` for sealed deliverable identity, scope, artifacts, context budget, and architecture basis.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows for SOW-013, SOW-052, OBJ-003, OBJ-012, PKG-05, DEL-05-05, and AB-00-01/02/03/06/08.
> - `docs/_Registers/Deliverables.csv` row DEL-05-05.
> - `docs/_Registers/ScopeLedger.csv` rows SOW-052 and SOW-013.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-05-05.
> - `docs/CONTRACT.md` rows OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-SOLVER-1, OPS-K-AGENT-1..4.
> - `core/loads/user_loads/README.md` for current implementation scope and
>   boundary evidence.
> - `core/loads/user_loads/src/lib.rs` for implemented API, findings, recovery
>   hooks, and unit tests.
>

### CLM-007 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. User-load application now covers nodal and distributed inputs plus the straight-pipe equivalent-recovery, oriented, and axial bridge evidenced by current implementation and tests. Other geometry/load breadth remains residual where recorded.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-05-05 Concentrated and distributed user load application

> #### Specification: DEL-05-05 Concentrated and distributed user load application
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable covers the implemented backend feature slice that applies
> explicit concentrated forces, concentrated moments, and uniform distributed user
> loads with unit-aware application records and result recovery hooks.
>
> Implemented evidence is the bounded Rust crate at `core/loads/user_loads`.
> The crate provides:
>
> - generic user-load preparation through `apply_user_loads`;
> - straight-pipe equivalent-load recovery through
>   `apply_straight_pipe_equivalent_user_loads`;
> - partial-span and oriented straight-pipe handling for translational
>   distributed loads;
> - element-station concentrated force recovery for straight-pipe elements;
> - an axial-effect bridge through
>   `apply_straight_pipe_equivalent_user_loads_with_axial_effects`;
> - deterministic findings for missing, invalid, unsupported, or unresolved
>   inputs.
>
> This deliverable excludes:
>
> - code-specific load combinations, factors, or default values;
> - protected standard content or proprietary allowables;
> - certification, compliance, or professional approval claims.
> - final result-envelope/API/persistence/GUI/CLI/report integration unless
>   separately dispatched.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-05-05-R1 | Support concentrated force, concentrated moment, and distributed user-load categories as explicit user load inputs. | ScopeLedger.csv row SOW-052; SOFTWARE_DECOMP.md row SOW-052 |
> | DEL-05-05-R2 | Keep user loads separate from code-specific combinations supplied by users or rule packs. | Deliverables.csv row DEL-05-05; ScopeLedger.csv row SOW-052 |
> | DEL-05-05-R3 | Keep load input, application records, and result handoff unit-aware and dimensionally checked. | CONTRACT.md OPS-K-UNIT-1; ScopeLedger.csv SOW-025 context via OBJ-012 |
> | DEL-05-05-R4 | Attach load application to the centerline/frame mechanics model and avoid implying shell/solid FEA is the primary global model. | CONTRACT.md OPS-K-MECH-1 |
> | DEL-05-05-R5 | Provide deterministic verification tests before release use. | CONTRACT.md OPS-K-SOLVER-1; architecture basis AB-00-08 |
> | DEL-05-05-R6 | Preserve status/provenance distinctions in diagnostics and result hooks without certification or compliance claims. | architecture basis AB-00-03 and AB-00-06; CONTRACT.md OPS-K-MECH-2 |
> | DEL-05-05-R7 | For straight-pipe elements, recover full-span and valid partial-span translational distributed loads into equivalent global nodal contributions. | `core/loads/user_loads/src/lib.rs`; `core/loads/user_loads/README.md` |
> | DEL-05-05-R8 | For straight-pipe elements, recover element-station concentrated forces while rejecting station moments and invalid station fractions as findings. | `core/loads/user_loads/src/lib.rs` |
> | DEL-05-05-R9 | Preserve oriented straight-pipe behavior by treating `GlobalX`, `GlobalY`, and `GlobalZ` user-load directions as global force vectors before straight-pipe projection. | `core/loads/user_loads/src/lib.rs` |
> | DEL-05-05-R10 | Bridge already-prepared primitive axial-effect contributions into straight-pipe equivalent nodal loads without duplicating axial-effect mechanics in this crate. | `core/loads/user_loads/src/lib.rs` |
>

### CLM-011 — Standards

> ##### Standards
>
> - Governing invariant catalog: `docs/CONTRACT.md`, location rows listed in `_CONTEXT.md` and this document.
> - Decomposition basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
> - Public schema/interchange baseline: JSON Schema 2020-12 from architecture
>   basis injection. Current implementation binds model-load records to
>   `schemas/model.schema.yaml#/$defs/LoadRecord` and recovery records to model
>   result values or `schemas/results.schema.yaml#/$defs/QuantityResult`.
> - Code-specific rules, combinations, and allowables: excluded from bundled defaults; user/rule-pack supplied.
>

### CLM-012 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | DEL-05-05-R1 | Existing Rust unit tests cover concentrated force, concentrated moment, and uniform distributed load input/application paths. |
> | DEL-05-05-R2 | Existing boundary text and tests preserve the no-default/no-compliance boundary; attempts to embed code combinations remain outside this module. |
> | DEL-05-05-R3 | Existing unit tests exercise `LoadDimension`, `ForcePerLength` metadata, explicit unit metadata, schema binding, payload references, and payload-hash references. |
> | DEL-05-05-R4 | Existing generic contribution records use frame DOFs; straight-pipe recovery uses `StraightPipeElement` equivalent global nodal loads. |
> | DEL-05-05-R5 | Existing deterministic test suite runs with `cargo test --manifest-path core/loads/user_loads/Cargo.toml`. |
> | DEL-05-05-R6 | Existing result-hook tests verify schema-binding separation and provenance-preserving records without compliance claim language. |
> | DEL-05-05-R7 | Existing tests verify full-span and partial-span straight-pipe distributed-load equivalent recovery. |
> | DEL-05-05-R8 | Existing tests verify interior station point-force recovery and deterministic rejection of unsupported station moments or invalid station fractions. |
> | DEL-05-05-R9 | Existing tests verify global-direction transformation for a straight pipe aligned with global `Y`. |
> | DEL-05-05-R10 | Existing tests verify axial-effect contribution inclusion, wrong-element rejection, and nonfinite axial-effect rejection. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> Current implementation records include:
>
> - `core/loads/user_loads/README.md` for crate scope, boundaries, and verification summary;
> - `core/loads/user_loads/src/lib.rs` for API, findings, contribution records, recovery hooks, and tests;
> - deliverable-local `MEMORY.md` for historical implementation slices, validation runs, and remaining TBDs.
>
> Remaining TBDs are final result-envelope/API/persistence/GUI/CLI/report
> integration, production tolerance policy, release thresholds, primitive
> axial-effect provenance beyond `load_id`, and professional reliance.
>

### CLM-014 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. User-load application now covers nodal and distributed inputs plus the straight-pipe equivalent-recovery, oriented, and axial bridge evidenced by current implementation and tests. Other geometry/load breadth remains residual where recorded.

- **AC-001** — The contract preserves the accepted general mechanics-load boundary, explicit units and provenance, deterministic ordering and invalid-input findings, and unresolved result-envelope, persistence, tolerance, and release policies without inventing magnitudes, factors, code combinations, allowables, protected content, or professional claims.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-05-05 Concentrated and distributed user load application

> #### Procedure: DEL-05-05 Concentrated and distributed user load application
>

### CLM-016 — Purpose

> ##### Purpose
>
> Define the operational path for validating and extending the implemented
> DEL-05-05 user-load slice without crossing the deliverable boundary.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Sealed task brief for DEL-05-05 with explicit write scope.
> - Access to `_CONTEXT.md`, `_REFERENCES.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, relevant register rows, and `docs/CONTRACT.md`.
> - Access to implementation evidence in `core/loads/user_loads/README.md` and
>   `core/loads/user_loads/src/lib.rs`.
> - Any extension task must resolve or explicitly inherit applicable unit,
>   solver, schema, diagnostics, and result-envelope contracts.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Confirm the task is scoped to DEL-05-05 and does not request code-specific
>    combinations, default factors, protected standards data, allowables,
>    compliance conclusions, or professional approval.
> 2. Identify whether the change concerns generic `apply_user_loads`,
>    straight-pipe equivalent recovery, the axial-effect bridge, documentation,
>    or downstream integration.
> 3. Preserve SOW-052 support for concentrated force, concentrated moment, and
>    distributed user-load inputs while treating SOW-013 only as primitive
>    load-case context; do not reimplement DEL-05-01.
> 4. Route numeric load values, dimensions, units, directions, target references,
>    station fractions, spans, element lengths, provenance references, schema
>    bindings, payload references, and payload-hash references through explicit
>    validation or boundary records.
> 5. For generic application, confirm nodal concentrated forces and moments emit
>    nodal contributions and recovery hooks, element distributed loads emit
>    distributed contribution records and hooks, and element-station loads remain
>    blocked unless straight-pipe geometry is supplied.
> 6. For straight-pipe equivalent recovery, confirm full-span and partial-span
>    translational distributed loads, oriented global directions, and
>    element-station concentrated forces route through `StraightPipeElement`
>    equivalent-load helpers.
> 7. For axial-effect bridge behavior, accept only already-prepared
>    `PrimitiveAxialEffectContribution` records and preserve findings for wrong
>    element indices, nonfinite axial effects, and missing straight-pipe
>    geometry.
> 8. Preserve architecture-basis constraints AB-00-01, AB-00-02, AB-00-03,
>    AB-00-06, and AB-00-08 in design notes, diagnostics, and validation.
> 9. Run deterministic validation before closeout, at minimum
>    `cargo test --manifest-path core/loads/user_loads/Cargo.toml` for changes
>    that touch or document current implementation behavior.
> 10. Expose result recovery hooks with provenance/status fields sufficient for
>     downstream stress recovery, reporting, export, GUI, and headless execution
>     work, without compliance claims.
>

### CLM-019 — Verification

> ##### Verification
>
> - Four deliverable documents exist and consistently reflect the implemented
>   evidence while excluding code-specific default behavior.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist as lens artifacts only.
> - `Dependencies.csv` validates against v3.1 schema.
> - `_STATUS.md` remains read-only unless an explicit authorized status-change
>   task is dispatched.
> - `cargo test --manifest-path core/loads/user_loads/Cargo.toml` is the
>   targeted validation gate for the documented implementation surface; any
>   failure is a blocker for code-level acceptance unless separately waived by a
>   human ruling.
> - `git diff --check` passes for touched deliverable-local files.
>

### CLM-020 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `MEMORY.md`
> - `core/loads/user_loads/README.md`
> - `core/loads/user_loads/src/lib.rs`
> - `_run_records/TASK_RUN_2026-04-30_1023_*.md`
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. User-load application now covers nodal and distributed inputs plus the straight-pipe equivalent-recovery, oriented, and axial bridge evidenced by current implementation and tests. Other geometry/load breadth remains residual where recorded.

- **VER-001** — Validate the contract and review source parity, concentrated and distributed application, partial spans and orientation, equivalent nodal recovery, axial-effect bridge ownership, unit/provenance metadata, result hooks, deterministic tests, and surviving governed residuals.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-05-05 Concentrated and distributed user load application

> #### Guidance: DEL-05-05 Concentrated and distributed user load application
>

### CLM-023 — Purpose

> ##### Purpose
>
> This guidance frames the implemented DEL-05-05 user-load slice so
> user-applied concentrated and distributed loads can be used and extended
> without crossing into code-specific load combination logic, invented
> engineering defaults, or professional approval claims.
>

### CLM-024 — Principles

> ##### Principles
>
> - Treat load magnitudes, locations, directions, distribution definitions, and coordinate references as user/model data unless a separate sealed source authorizes otherwise.
> - Preserve the solver/rule separation: mechanics may be solved, while acceptability and code compliance remain user/rule-pack and human-judgment concerns.
> - Keep dimensional analysis explicit. A load input that lacks a required unit,
>   direction basis, location basis, distribution basis, target, quantity, or
>   straight-pipe geometry should become a deterministic finding or preserved
>   TBD rather than a silent default.
> - Keep result hooks narrow: they should expose mechanical force/moment/result
>   data and provenance needed by stress recovery, reporting, export, GUI, and
>   headless execution work, not code-specific conclusions.
> - Use `core/loads/user_loads` as the current implementation evidence for this
>   deliverable; do not infer additional solver, schema, GUI, or report behavior
>   beyond that crate without a sealed task.
>

### CLM-025 — Considerations

> ##### Considerations
>
> - SOW-052 is the direct scope item for concentrated forces, concentrated moments, and distributed user loads.
> - SOW-013 is relevant as the primitive load-case context, but this deliverable should not absorb the full primitive load-case engine.
> - AB-00-03 and AB-00-06 make status and result-envelope separation important for downstream GUI, CLI, reports, and rule-pack consumers.
> - AB-00-08 means the implemented module must keep deterministic tests as the
>   evidence basis before release use.
> - Generic `apply_user_loads` handles nodal concentrated forces, nodal
>   concentrated moments, and element uniform distributed-load records. It
>   reports element-station loads as requiring straight-pipe equivalent recovery.
> - `apply_straight_pipe_equivalent_user_loads` handles oriented straight-pipe
>   equivalent recovery for full-span and partial-span distributed loads and
>   element-station concentrated forces.
> - `apply_straight_pipe_equivalent_user_loads_with_axial_effects` appends
>   already-prepared primitive axial-effect contributions through
>   `StraightPipeElement` equivalent-load behavior; it does not introduce a new
>   axial-effect mechanics model in this crate.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> - A narrower user-load module reduces risk of accidentally encoding protected or jurisdiction-specific load combinations.
> - Keeping final result-envelope/API/persistence/GUI/CLI/report integration as
>   TBD avoids overstating the implementation slice while preserving current
>   recovery-hook evidence.
> - The straight-pipe equivalent APIs support the current oriented straight-pipe
>   scope; other element families or higher-level model assembly should remain
>   separately dispatched.
> - Distributed-load support is implemented for uniform translational loads over
>   normalized spans, including valid partial spans. Other distribution shapes,
>   magnitudes, factors, or defaults are not invented here.
>

### CLM-027 — Examples

> ##### Examples
>
> - Existing Rust unit tests use artificial mechanics fixtures for concentrated
>   forces, concentrated moments, uniform distributed loads, partial spans,
>   oriented straight-pipe recovery, station point forces, and axial effects.
> - Public or product-facing examples remain TBD unless separately approved with
>   non-protected fixture values, explicit unit provenance, and no compliance
>   claims.
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict detected in current implementation evidence. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-052 SOW-013 OBJ-003 OBJ-012 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
