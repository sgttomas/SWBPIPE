---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-03
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-011]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-03` in service of project scope [SOW-011] and package objectives [OBJ-003].

- **OUT-001** — A linear-support and restraint contract covering anchors, guides, line stops, vertical supports, springs, imposed-displacement boundary data, frame-kernel DOF mapping, boundary preparation/application, explicit findings, and deterministic tests is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-03 Linear support and restraint models

> #### Datasheet: DEL-04-03 Linear support and restraint models
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-04-03 |
> | Package ID | PKG-04 |
> | Package name | Solver Core and Numerical Methods |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-011 |
> | Objective | OBJ-003 |
> | Context envelope | M |
> | Anticipated artifacts | support model; linear restraint tests |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source |
> |---|---|---|
> | Covered support families | Anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `_CONTEXT.md` Description; `docs/_Registers/ScopeLedger.csv` row SOW-011 |
> | Analysis basis | Support behavior is part of the primary 3D centerline/frame global model with six degrees of freedom per node. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1 |
> | Support implementation surface | `core/solver/linear_supports` implements the bounded mechanics-boundary slice for linear support records, prepared boundary data, and dense-system support application. | `core/solver/linear_supports/README.md`; `core/solver/linear_supports/src/lib.rs` |
> | Frame DOF boundary | `FrameDof` is re-exported from `open_pipe_stress_frame_kernel`; `NodeDof::global_index()` delegates to frame-kernel `node_dof_index`. | `core/solver/linear_supports/src/lib.rs`; `_run_records/TASK_RUN_2026-06-05_0732_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_A.md` |
> | Linear-only boundary | This deliverable covers implemented linear support/restraint behavior. Nonlinear one-way behavior, lift-off, gaps, and friction belong to DEL-04-04 unless explicitly deferred back by human authority. | `execution/_Decomposition/SOFTWARE_DECOMP.md` rows DEL-04-03 and DEL-04-04 |
> | Unit metadata | `SupportQuantity` retains dimension intent and can carry caller-supplied `UnitSystemRef` and `QuantityUnitMetadata`; it validates metadata dimension matches and does not convert or default values. | `core/solver/linear_supports/src/lib.rs`; `core/solver/linear_supports/README.md` |
> | Imposed displacement boundary data | Imposed displacement entries are prepared as prescribed DOFs with explicit displacement or rotation values for the frame-kernel prescribed-displacement boundary. | `core/solver/linear_supports/src/lib.rs` |
> | Default stiffness values | TBD. No invented support stiffness/defaults are introduced by the implementation or this document kit. | `docs/CONTRACT.md` OPS-K-AGENT-1; human hard stop |
>

### CLM-004 — Conditions

> ##### Conditions
>
> - Public artifacts must not include protected standards text, protected tables, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary support/vendor data. Source: `docs/CONTRACT.md` OPS-K-IP-1.
> - Missing solve-required support data must become explicit findings or diagnostics, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
> - Numerical values affecting calculations must be unit-aware and dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
> - Solver output remains a mechanics result; rule-pack acceptability and professional compliance remain outside this deliverable. Source: `docs/CONTRACT.md` OPS-K-MECH-2.
> - Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.
>

### CLM-005 — Construction

> ##### Construction
>
> | Construction slot | Current state |
> |---|---|
> | Support model | Implemented Rust types cover rigid restraints, springs, and imposed displacement boundary data for the SOW-011 families. The crate remains mechanics-only and linear-only. |
> | DOF mapping | Support effects map to frame-kernel `FrameDof` values `Ux`, `Uy`, `Uz`, `Rx`, `Ry`, and `Rz`; global indices are computed through `node_dof_index`. Support coordinate policy beyond explicit node DOFs remains TBD. |
> | Boundary preparation | `prepare_boundary` produces restrained DOFs, spring entries, imposed displacement entries, and deterministic support findings for missing, invalid, duplicate, or out-of-range support data. |
> | Boundary application | `apply_linear_supports` validates a dense global system through the frame kernel, adds prepared spring stiffness to global stiffness diagonals, and reduces rigid plus imposed displacement DOFs through the frame-kernel prescribed-displacement boundary. Sparse solver integration and final result-envelope integration remain downstream. |
> | Diagnostics | Current crate findings include local support finding code, support ID, and message. AB-00-06 result-envelope mapping remains downstream integration work. |
> | Tests | Current evidence records `cargo test --manifest-path core/solver/linear_supports/Cargo.toml --locked` passing 14 tests, including frame-kernel DOF parity, unit metadata, boundary preparation, and boundary application coverage. |
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` for sealed deliverable identity and applicable architecture basis IDs.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-011, OBJ-003, PKG-04, DEL-04-03, and AB-00-01/02/03/06/08.
> - `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` for register rows.
> - `docs/CONTRACT.md` for applicable invariants.
> - `docs/SPEC.md`, `docs/TYPES.md`, and `docs/INTENT.md` for public architecture and vocabulary slices.
>

### CLM-007 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-04-03 Linear support and restraint models

> #### Specification: DEL-04-03 Linear support and restraint models
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable provides the current linear support and restraint mechanics-boundary implementation for PKG-04. The implemented crate is `core/solver/linear_supports`; it covers anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data as listed in SOW-011.
>
> Out of scope:
>
> - Nonlinear active-set behavior such as one-way restraints, lift-off, gaps, and friction, assigned to DEL-04-04.
> - Sparse solver selection, performance harness work, final result-envelope integration, and support coordinate policy beyond explicit node DOFs.
> - Bundled protected standards/vendor values; claims language per the standard claim fence (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-04-03-R01 | The support model shall cover the SOW-011 families: anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `docs/_Registers/ScopeLedger.csv` row SOW-011 |
> | DEL-04-03-R02 | Linear support data shall be expressible against the 3D centerline/frame model and the six node degrees of freedom through the frame-kernel `FrameDof` boundary. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1; `core/solver/linear_supports/src/lib.rs` |
> | DEL-04-03-R03 | The support model shall keep mechanics solving separate from user-rule evaluation and human professional compliance judgment. | `docs/CONTRACT.md` OPS-K-MECH-2 |
> | DEL-04-03-R04 | Support stiffnesses, imposed displacements, rotations, and related numerical data shall retain unit intent through `SupportQuantity`; caller-supplied unit metadata shall match the support quantity dimension. | `docs/CONTRACT.md` OPS-K-UNIT-1; `core/solver/linear_supports/src/lib.rs` |
> | DEL-04-03-R05 | Missing solve-required support data shall be reported as explicit findings or diagnostics, never silently filled by defaults. | `docs/CONTRACT.md` OPS-K-DATA-2 |
> | DEL-04-03-R06 | Public support examples and test fixtures shall avoid protected standards text, protected tables, copied formulas, proprietary vendor data, and unsupported default values. | `docs/CONTRACT.md` OPS-K-IP-1 |
> | DEL-04-03-R07 | Support findings shall remain explicit at the crate boundary; mapping those findings into final AB-00-06 result-envelope fields remains downstream integration work. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 |
> | DEL-04-03-R08 | The support model shall preserve module boundaries and inward dependencies toward domain contracts; adapters/plugins must not bypass validation, diagnostics, or governance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 |
> | DEL-04-03-R09 | Solver-facing support changes shall be covered by deterministic verification tests before release use. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08 |
> | DEL-04-03-R10 | Unknown support coordinate policy, sparse solver integration points, final result-envelope integration, and release-readiness criteria shall remain `TBD` until resolved by a governed brief or human decision. | `docs/CONTRACT.md` OPS-K-AGENT-1; `_CONTEXT.md` Still TBD |
> | DEL-04-03-R11 | Documentation shall not imply nonlinear support behavior, a penalty method, sparse solver integration, release approval, professional approval, or code-compliance acceptance (PRD §21.2). | Human hard stop; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01 |
> | DEL-04-03-R12 | Boundary preparation and application shall use `prepare_boundary` and `apply_linear_supports` as the current implemented surfaces for dense-system support handling. | `core/solver/linear_supports/src/lib.rs`; `core/solver/linear_supports/README.md` |
>

### CLM-011 — Standards

> ##### Standards
>
> - Public project invariants in `docs/CONTRACT.md` apply, especially OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2, OPS-K-AGENT-1..4, and OPS-K-IP-1.
> - SCA-001 architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08 apply as dispatch constraints.
> - External piping design-code clauses, support stiffness defaults, and vendor support catalogs are not accessible sources for this deliverable kit; any such requirement is `TBD` and must be user-supplied or rights-cleared before use.
>

### CLM-012 — Verification

> ##### Verification
>
> | Requirement | Verification approach / current evidence |
> |---|---|
> | R01, R02 | Implementation review confirms all SOW-011 support families are represented without leaving the six-DOF centerline model; `FrameDof` is re-exported from `open_pipe_stress_frame_kernel` and `NodeDof::global_index()` uses `node_dof_index`. |
> | R03 | Result/status review confirms mechanics-solved state is not represented as rule-pass or professional approval. |
> | R04 | Unit tests cover `SupportQuantity` finite/positive validation, canonical dimension IDs, caller-supplied unit metadata, and metadata dimension mismatch rejection. |
> | R05, R07 | Negative tests omit required support data and verify explicit support findings; final result-envelope mapping remains deferred. |
> | R06 | Protected-content/provenance review checks public examples and tests. |
> | R08 | Architecture review checks dependency direction and no-bypass behavior. |
> | R09 | June 5 evidence records `cargo test --manifest-path core/solver/linear_supports/Cargo.toml --locked` passing 14 tests. |
> | R10 | Open policy and integration choices are tracked as `TBD` or decision-record inputs, not hidden assumptions. |
> | R11 | Review confirms artifacts do not claim nonlinear behavior, release approval, professional approval, code compliance, sparse solver integration, or invented support stiffness/defaults (PRD §21.2). |
> | R12 | Tests cover `prepare_boundary` output and `apply_linear_supports` spring diagonal application, imposed displacement force adjustment, rigid zero-displacement reduction, and blocking findings before reduction. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> The implemented evidence for this deliverable includes:
>
> - `core/solver/linear_supports/README.md`;
> - `core/solver/linear_supports/src/lib.rs`;
> - deliverable-local run records for the May 15 and June 5 implementation slices;
> - this four-document kit and `MEMORY.md`.
>
> Decision records are still required for any later resolution of support coordinate policy, sparse solver integration, final result-envelope integration, release criteria, or other governed choices outside the current bounded implementation.
>

### CLM-014 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.

- **AC-001** — The contract preserves the accepted implemented linear-support boundaries, unit-bearing quantities, no-default behavior, frame-kernel indexing and prescribed-displacement integration, rights-safe fixtures, and unresolved support-coordinate, sparse/result-envelope, release, and constant-effort-hanger work without implying nonlinear behavior or approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-04-03 Linear support and restraint models

> #### Procedure: DEL-04-03 Linear support and restraint models
>

### CLM-016 — Purpose

> ##### Purpose
>
> Use this procedure to verify DEL-04-03 review-readiness evidence for the implemented linear support/restraint slice without expanding scope into nonlinear supports, sparse solver integration, final result-envelope integration, or release approval. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Confirm the active scope is DEL-04-03 / PKG-04 and SOW-011 only.
> - Read `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, and the SOFTWARE_DECOMP rows for SOW-011, OBJ-003, DEL-04-03, DEL-04-04, and AB-00-01/02/03/06/08.
> - Confirm protected data and professional-compliance hard stops before drafting support examples or tests.
> - Confirm the current implementation evidence in `core/solver/linear_supports/README.md`, `core/solver/linear_supports/src/lib.rs`, and the June 5 support-boundary-hardening run record.
> - Confirm any support coordinate policy, sparse solver integration, final result-envelope integration, release-readiness criterion, or professional/code-compliance conclusion (PRD §21.2) is either already accepted by a governed decision or marked `TBD`.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Confirm the support family taxonomy in `LinearSupport` covers anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data.
> 2. Confirm `FrameDof` is re-exported from `open_pipe_stress_frame_kernel` and `NodeDof::global_index()` delegates to frame-kernel `node_dof_index`.
> 3. Confirm each support family maps to explicit node DOFs and that support coordinate policy beyond explicit DOF selection remains `TBD`.
> 4. Confirm `SupportQuantity` preserves value, dimension, optional unit-system reference, and optional unit metadata; verify dimension mismatch is rejected rather than converted or defaulted.
> 5. Confirm `prepare_boundary` returns restrained DOFs, spring entries, imposed displacement entries, and deterministic support findings for missing, invalid, duplicate, or out-of-range data.
> 6. Confirm `apply_linear_supports` validates a dense global system through the frame kernel, adds spring stiffness to global diagonals, and reduces rigid plus imposed displacement DOFs through the frame-kernel prescribed-displacement boundary.
> 7. Confirm current local evidence records 14 passing `linear_supports` tests and does not rely on protected support catalog values, code-derived defaults, or copied commercial benchmarks.
> 8. Confirm nonlinear one-way, gap, lift-off, friction, active-set behavior, sparse solver integration, final result-envelope integration, support coordinate policy, release claims, and professional/code-compliance claims remain outside this deliverable alignment.
> 9. Record any later resolved architecture or representation choices through the approved decision-record path. The exact repo-level ADR location is outside this task and remains `TBD`.
>

### CLM-019 — Verification

> ##### Verification
>
> - Four-document kit exists and uses consistent terminology for linear support/restraint models.
> - Required support-data gaps are represented as `TBD` or support findings, not defaults.
> - Current test evidence references deterministic mechanics verification and unit/dimensional checks for support stiffness and imposed displacement values.
> - Implementation evidence includes `prepare_boundary`, `apply_linear_supports`, frame-kernel `FrameDof` re-export, `node_dof_index` indexing, `SupportQuantity` unit metadata, and 14 passing `linear_supports` tests.
> - No protected standards text, copied formulas, protected tables, proprietary vendor data, or certification/compliance claims are introduced.
> - Nonlinear support behavior is not implemented or specified as part of this linear deliverable.
>

### CLM-020 — Records

> ##### Records
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` historical semantic artifacts.
> - `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
> - `_run_records/TASK_RUN_*.md` entries for four-documents P1/P2, semantic matrix, lens register, four-documents P3, dependency extraction, implementation evidence, support boundary hardening, and this review-readiness alignment.
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.

- **VER-001** — Validate the contract and review source parity, all SOW-011 families, FrameDof and boundary surfaces, dimensional metadata, missing/invalid-data findings, deterministic test evidence, protected-content controls, linear/nonlinear separation, and surviving governed residuals.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-04-03 Linear support and restraint models

> #### Guidance: DEL-04-03 Linear support and restraint models
>

### CLM-023 — Purpose

> ##### Purpose
>
> This deliverable documents the implemented linear support/restraint slice of the global solver. It makes boundary conditions explicit enough for deterministic dense-system mechanics preparation and application while staying protected-data safe and outside professional/code-compliance judgment.
>

### CLM-024 — Principles

> ##### Principles
>
> - Treat supports as mechanics-boundary data in the 3D centerline/frame model, not as local shell/solid FEA features. Source: `docs/CONTRACT.md` OPS-K-MECH-1.
> - Keep the solver/rule/human boundary visible. A restraint reaction or displacement result is a mechanics result, not an automatic code-compliance finding. Source: OPS-K-MECH-2.
> - Prefer explicit required inputs over defaults. If a support stiffness, direction, coordinate basis, imposed displacement value, or target reference is required and absent, emit a finding. Source: OPS-K-DATA-2.
> - Keep every numerical support input unit-aware. Translational stiffness, rotational stiffness, displacement, and rotation units need dimensional checks before solve use. Source: OPS-K-UNIT-1.
> - Keep linear and nonlinear scope separate. Linear springs and fixed restraints belong here; activation, lift-off, gaps, one-way supports, and friction are DEL-04-04 unless a later sealed brief changes scope.
> - Use original or permissive test fixtures. Do not import protected support catalog values, code examples, vendor tables, or copied commercial benchmark cases. Source: OPS-K-IP-1.
> - Keep frame-kernel DOF authority centralized. `FrameDof` is re-exported from `open_pipe_stress_frame_kernel`, and support global DOF indices are computed through `node_dof_index`.
>

### CLM-025 — Considerations

> ##### Considerations
>
> - `core/solver/linear_supports` now represents anchors, guides, line stops, vertical supports, springs, and imposed displacements through explicit node DOFs, restrained DOF lists, spring entries, and imposed displacement entries.
> - `SupportQuantity` records value, quantity dimension, optional unit-system reference, and optional unit metadata. It validates finite or positive finite values and rejects unit metadata whose canonical dimension does not match the support quantity. It does not perform unit conversion or provide defaults.
> - `prepare_boundary` is the implemented preparation surface. It returns restrained DOFs, spring entries, imposed displacement entries, and findings for missing DOFs, missing stiffness/displacement values, invalid dimensions, repeated restraints, and out-of-range nodes.
> - `apply_linear_supports` is the implemented dense-system application surface. It validates through the frame kernel, adds spring stiffness to global stiffness diagonals, and reduces rigid and imposed displacement DOFs through the frame-kernel prescribed-displacement boundary.
> - Support coordinate policy remains TBD. The current crate maps explicit node DOFs and does not define arbitrary global, local, or user-defined support direction conventions.
> - Imposed displacement boundary data may interact with load-case handling in PKG-05, but this deliverable currently implements the support-side boundary data and frame-kernel prescribed-displacement reduction only.
> - Diagnostics should be useful to GUI, CLI, report, and adapter consumers through result-envelope fields required by AB-00-06.
> - Final result-envelope integration remains downstream; current `SupportFinding` values are crate-local findings with code, support ID, and message.
> - AB-00-01 requires accepted decisions to be recorded when this deliverable resolves deferred choices. This review-readiness alignment does not create repo-level ADR files.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Risk / TBD |
> |---|---|---|
> | Represent rigid restraints as prescribed zero displacements in dense boundary application | Clear mechanics boundary condition using the frame-kernel prescribed-displacement reducer. | Sparse-solver integration, final result-envelope mapping, and release criteria remain downstream. |
> | Represent linear springs as unit-bearing stiffness data | Keeps support behavior testable and explicit. | No default stiffnesses may be invented; source/provenance remains required. |
> | Keep nonlinear behavior out of this deliverable | Preserves a bounded linear slice. | Users may expect guides/stops to include gaps or one-way action; that expectation must route to DEL-04-04 or remain TBD. |
> | Use invented/public benchmark fixtures | Protects IP boundary. | Validation breadth is limited until lawful engineering examples are reviewed. |
>

### CLM-027 — Examples

> ##### Examples
>
> - Valid example category: an invented frame/support fixture with public-domain dimensions and unit-bearing user-entered stiffness values.
> - Invalid example category: a copied support stiffness table, code-derived default, vendor catalog value without redistribution rights, or commercial software benchmark copied into public tests.
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were identified during this alignment. Open policy and integration details remain `TBD`, not conflicts.
>

### CLM-029 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-011 OBJ-003 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
