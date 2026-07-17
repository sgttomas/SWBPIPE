---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-01
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-005, SOW-035]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-01` in service of project scope [SOW-005, SOW-035] and package objectives [OBJ-003].

- **OUT-001** — A 3D frame-stiffness-kernel contract covering six-degree-of-freedom node mapping, global assembly, coordinate transforms, boundary conditions, sparse-solve interfaces, mechanics-only result envelopes, and deterministic verification is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-01 3D frame stiffness kernel

> #### Datasheet: DEL-04-01 3D frame stiffness kernel
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-04-01 |
> | Deliverable name | 3D frame stiffness kernel |
> | Package ID | PKG-04 |
> | Package name | Solver Core and Numerical Methods |
> | Type | BACKEND_FEATURE_SLICE |
> | Decomposition basis | execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 |
> | Register basis | docs/_Registers/Deliverables.csv row DEL-04-01 |
> | Scope items | SOW-005, SOW-035 |
> | Objective support | OBJ-003 |
> | Context envelope | L |
> | Context QA risk | WATCH |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value |
> |---|---|
> | Primary model class | 3D centerline/frame model |
> | Nodal degrees of freedom | Six degrees of freedom per node |
> | Intended kernel responsibility | Global frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface |
> | Anticipated implementation location | core/solver/frame_kernel |
> | Anticipated verification artifacts | Unit tests |
> | Sparse performance scope | Required design concern; concrete performance targets are TBD |
> | Reproducibility scope | Required for practical piping models; exact reproducibility envelope is TBD |
> | Solver numerical library | TBD |
> | Tolerance policy | TBD; no numerical tolerances are defined by this setup kit |
> | Physical formulation details | TBD; future implementation must use lawful, source-grounded mechanics references and avoid protected formulas/data |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - This deliverable is bounded to the global stiffness kernel and sparse solve interface setup for a 3D centerline/frame system.
> - This deliverable does not implement straight pipe element mechanics, support families, nonlinear support active-set logic, load cases, stress recovery, diagnostics, rule-pack evaluation, GUI behavior, reports, or packaging except as interface constraints named by the sealed context.
> - The kernel must remain unit-aware and dimensionally checked through the project unit contracts.
> - Missing solve-required values must be surfaced as explicit findings rather than silently defaulted.
> - Solver mechanics and rule-pack acceptability decisions remain separated. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
> - Protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data are out of bounds.
>

### CLM-006 — Construction

> ##### Construction
>
> The setup kit identifies these future implementation surfaces without creating code:
>
> | Surface | Setup expectation |
> |---|---|
> | Model topology input | Node and element connectivity for a 3D frame/centerline model, details TBD |
> | Degree-of-freedom mapping | Stable six-DOF-per-node indexing contract, exact ordering TBD |
> | Coordinate transform handling | Transform interface for local-to-global assembly, convention TBD |
> | Boundary-condition handling | Interface for applying restraints/imposed conditions supplied by other deliverables, details TBD |
> | Sparse assembly | Sparse global matrix assembly interface, storage format TBD |
> | Sparse solve interface | Solver adapter boundary for selected numerical library, library TBD |
> | Result envelope | Must fit architecture-basis command/query/job result and diagnostic envelope constraints |
> | Verification | Deterministic unit tests are required before release use |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - local sealed context for DEL-04-01.
> - `_REFERENCES.md` - governing references and register pointers.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 - PKG-04, DEL-04-01, SOW-005, SOW-035, OBJ-003, AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08.
> - `docs/_Registers/Deliverables.csv` - row DEL-04-01.
> - `docs/_Registers/ScopeLedger.csv` - rows SOW-005 and SOW-035.
> - `docs/_Registers/ContextBudgetQA.csv` - row DEL-04-01.
> - `docs/CONTRACT.md` - applicable invariants named in the task brief.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-04-01 3D frame stiffness kernel

> #### Specification: DEL-04-01 3D frame stiffness kernel
>

### CLM-009 — Scope

> ##### Scope
>
> This setup specification covers the future backend feature slice for a 3D frame stiffness kernel. The sealed scope is DEL-04-01 in PKG-04, implementing the global 3D frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface for a 3D centerline/frame model with six degrees of freedom per node.
>
> Out of scope for this deliverable:
>
> - Solver implementation in this setup run.
> - Numeric tolerances, convergence thresholds, performance targets, or solver library selection.
> - Protected standards formulas, examples, tables, or proprietary commercial data.
> - Straight pipe element details owned by DEL-04-02.
> - Support/restraint model families owned by DEL-04-03 and DEL-04-04.
> - Solver diagnostics layer owned by DEL-04-06.
> - Rule-pack acceptability decisions; claims language per the standard claim fence (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-04-01-REQ-001 | Future implementation shall model the primary global analysis system as a 3D centerline/frame model. | SOW-005; OPS-K-MECH-1 |
> | DEL-04-01-REQ-002 | Future implementation shall represent each frame node with six degrees of freedom. | SOW-005 |
> | DEL-04-01-REQ-003 | Future implementation shall provide global stiffness assembly for the frame system. | Deliverables.csv row DEL-04-01 |
> | DEL-04-01-REQ-004 | Future implementation shall provide coordinate transform handling between local and global frame representations; exact convention is TBD. | Deliverables.csv row DEL-04-01 |
> | DEL-04-01-REQ-005 | Future implementation shall provide a boundary-condition application interface; supported restraint semantics are delegated to later support deliverables unless explicitly sealed in this deliverable. | Deliverables.csv row DEL-04-01; package scope |
> | DEL-04-01-REQ-006 | Future implementation shall expose a sparse solve interface designed for sparse numerical performance and reproducible practical-model results; performance targets are TBD. | SOW-035 |
> | DEL-04-01-REQ-007 | Future implementation shall remain unit-aware and dimensionally checked. | OPS-K-UNIT-1 |
> | DEL-04-01-REQ-008 | Future implementation shall report missing solve-required values as explicit findings and shall not supply silent defaults. | OPS-K-DATA-2 |
> | DEL-04-01-REQ-009 | Solver outputs shall compute mechanics only and shall not decide rule-pack acceptability or professional compliance. | OPS-K-MECH-2 |
> | DEL-04-01-REQ-010 | Solver changes shall require deterministic verification tests before release. | OPS-K-SOLVER-1; AB-00-08 |
> | DEL-04-01-REQ-011 | Diagnostics and result envelopes crossing service boundaries shall preserve code, class, severity, source, affected object, message, remediation, and provenance fields where applicable. | AB-00-03; AB-00-06 |
> | DEL-04-01-REQ-012 | Future implementation shall not import protected formulas/data or embed protected standards text, tables, figures, examples, or proprietary data. | OPS-K-IP-1 |
>

### CLM-011 — Standards

> ##### Standards
>
> - Project invariants in `docs/CONTRACT.md` are governing for this setup kit.
> - Decomposition and register rows cited in `_CONTEXT.md` are governing for scope and objectives.
> - External mechanics references for future implementation are TBD and must be lawful, source-grounded, and compatible with the protected-data boundary.
> - No code standard clause, protected formula, or professional compliance criterion is adopted by this setup kit.
>

### CLM-012 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | DEL-04-01-REQ-001, DEL-04-01-REQ-002 | Unit tests should verify model topology and six-DOF mapping after implementation; exact cases TBD. |
> | DEL-04-01-REQ-003, DEL-04-01-REQ-004 | Unit tests should verify deterministic assembly and transform behavior using rights-cleared fixtures; equations and expected values TBD. |
> | DEL-04-01-REQ-005 | Unit tests should verify boundary-condition application semantics once restraint interfaces are sealed. |
> | DEL-04-01-REQ-006 | Sparse performance and reproducibility checks should be coordinated with DEL-04-05; target sizes and metrics TBD. |
> | DEL-04-01-REQ-007 | Unit and schema tests should verify dimensional compatibility and unit-aware inputs/outputs. |
> | DEL-04-01-REQ-008, DEL-04-01-REQ-011 | Tests should verify missing-value and diagnostics/result-envelope behavior without silent defaults. |
> | DEL-04-01-REQ-009 | Tests/reviews should verify mechanics outputs remain separate from rule-pack or compliance determinations. |
> | DEL-04-01-REQ-010 | CI must include deterministic solver verification gates before release use. |
> | DEL-04-01-REQ-012 | Protected-content review gates must check that fixtures, docs, and source comments contain no protected standards or proprietary data. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> Future implementation should maintain:
>
> - `core/solver/frame_kernel` module documentation.
> - Unit test records for frame assembly, coordinate transforms, boundary conditions, unit handling, missing-value diagnostics, and sparse solve interface behavior.
> - Result-envelope and diagnostics examples that disclose assumptions, warnings, model/solver versions, and provenance without certification claims.
> - TBD register entries for unresolved formulation, tolerance, solver-library, and performance-target decisions.
>

### CLM-014 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The 3D frame kernel, DOF contract, dense solution path, and DEC-023 sparse evidence now exist in the implemented slice. Sparse-as-default policy and any broader performance/validation thresholds remain held where recorded; lifecycle remains `IN_PROGRESS`.

- **AC-001** — The contract preserves the accepted frame-kernel requirements and current declarations, including unit and provenance boundaries, explicit missing-input and solver findings, rights-cleared verification data, and unresolved formulation, tolerance, sparse-policy, arc-pressure-thrust, and mechanics-assessment items without inventing engineering values or approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-04-01 3D frame stiffness kernel

> #### Procedure: DEL-04-01 3D frame stiffness kernel
>

### CLM-016 — Purpose

> ##### Purpose
>
> This procedure defines how a future TASK worker should proceed from the local setup kit into implementation without expanding beyond DEL-04-01.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Confirm the active deliverable is DEL-04-01 and the write scope is the assigned deliverable or future implementation path explicitly authorized by a sealed brief.
> - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, and `_SEMANTIC_LENSING.md`.
> - Confirm applicable invariants: OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2, OPS-K-REPORT-1, OPS-K-AGENT-1..4, and OPS-K-IP-1.
> - Confirm architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
> - Obtain or cite lawful mechanics/numerics references before implementing formulas, expected numerical values, tolerances, or benchmark thresholds.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Reconfirm scope boundaries against SOW-005 and SOW-035.
> 2. Identify any implementation decisions still TBD, including DOF ordering, coordinate convention, sparse storage, solver library, error handling policy, tolerance policy, and reproducibility target.
> 3. If a TBD decision is needed to write code, record it as a required human/architecture ruling rather than guessing.
> 4. Define interfaces for model topology, six-DOF node mapping, frame assembly, coordinate transforms, boundary-condition application, sparse solve invocation, unit validation, and result/diagnostic output.
> 5. Keep straight pipe element behavior, supports, nonlinear supports, loads, stress recovery, diagnostics specialization, reports, and rule packs at their assigned deliverable boundaries.
> 6. Implement only from lawful, source-grounded mechanics and numerics references after the implementation brief authorizes code work.
> 7. Create deterministic unit tests before release use; use rights-cleared fixtures with explicit units and no protected data.
> 8. Ensure missing solve-required inputs produce explicit findings and do not become silent defaults.
> 9. Ensure output envelopes preserve mechanics/rule/human approval separation and do not claim certification or compliance.
> 10. Run protected-content and provenance checks before any release or handoff.
>

### CLM-019 — Verification

> ##### Verification
>
> - Confirm no implementation values were invented from this setup kit.
> - Confirm unit-aware and dimensionally checked interfaces are present in implementation scope.
> - Confirm deterministic tests exist for every implemented kernel behavior.
> - Confirm sparse performance and reproducibility checks are coordinated with DEL-04-05 when targets are sealed.
> - Confirm diagnostics/result envelopes align with AB-00-03 and AB-00-06.
> - Confirm protected-content review passes before release.
>

### CLM-020 — Records

> ##### Records
>
> - Implementation brief and sealed write scope.
> - Architecture decisions for TBD items.
> - Source/provenance records for mechanics and numerical references.
> - Unit test and CI results.
> - Protected-content review result.
> - Result-envelope and diagnostics samples.
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The 3D frame kernel, DOF contract, dense solution path, and DEC-023 sparse evidence now exist in the implemented slice. Sparse-as-default policy and any broader performance/validation thresholds remain held where recorded; lifecycle remains `IN_PROGRESS`.

- **VER-001** — Validate the contract and review source parity, frame/DOF and assembly coverage, coordinate and boundary interfaces, sparse/reproducibility obligations, units and diagnostics, protected-content and professional boundaries, and every unresolved governed item.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-04-01 3D frame stiffness kernel

> #### Guidance: DEL-04-01 3D frame stiffness kernel
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-01-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> This deliverable prepares the bounded implementation surface for the central 3D frame stiffness kernel. Its value is to keep global centerline mechanics, sparse numerical behavior, reproducibility, units, diagnostics, protected-data controls, and professional-boundary language aligned before code is written.
>

### CLM-025 — Principles

> ##### Principles
>
> - Keep the global model line-element based unless a later handoff explicitly routes local analysis to shell/solid FEA.
> - Treat six degrees of freedom per node as a scope fact, not as permission to invent an ordering or sign convention.
> - Keep coordinate conventions, matrix storage format, solver library, tolerances, and benchmark targets as TBD until a source-backed implementation brief or human ruling seals them.
> - Separate mechanics computation from rule-pack evaluation and professional approval.
> - Use deterministic, rights-cleared verification fixtures. Do not use copied standards examples or protected commercial data.
> - Surface missing values, singular/ill-conditioned states, and assumptions as explicit result-envelope findings rather than silent behavior.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The accessible source set is the decomposition, registers, contract, and local context. It does not provide detailed mechanics formulas, reference-element derivations, numerical tolerances, sparse storage formats, or benchmark thresholds. Future implementation must therefore source those details from lawful references or human-approved architecture decisions.
>
> Architecture basis rows constrain the future implementation shape:
>
> - AB-00-01 requires decision records for accepted architecture choices and reconsideration triggers.
> - AB-00-02 keeps dependencies pointed inward toward domain contracts and preserves layer responsibilities.
> - AB-00-03 separates commands, queries, jobs, result envelopes, progress/cancellation, and mechanics/rule/human approval states.
> - AB-00-06 requires structured diagnostics and avoids certification/compliance claims.
> - AB-00-08 requires layered solver tests and protected-content/provenance gates.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Topic | Current guidance |
> |---|---|
> | Sparse solver library | TBD. Choosing one now would exceed setup scope. |
> | Coordinate convention | TBD. Must be consistent across element, support, load, result, and report interfaces. |
> | Performance targets | TBD. SOW-035 requires sparse performance and reproducibility, but concrete thresholds are not available. |
> | Verification fixtures | Use invented, rights-cleared, non-authoritative fixtures only after implementation is scoped; label them as test fixtures, not engineering examples. |
> | Kernel boundaries | Keep element stiffness, support behavior, diagnostics, load application, and stress recovery in their assigned deliverables unless a human-approved change amends the decomposition. |
>

### CLM-028 — Examples

> ##### Examples
>
> - `TBD`: A future fixture set may include a minimal frame topology to test DOF mapping, but this setup kit does not define values, tolerances, or expected results.
> - `TBD`: A future architecture decision may choose a sparse matrix representation, but this setup kit does not select one.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict found in setup sources. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-005 SOW-035 OBJ-003 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
