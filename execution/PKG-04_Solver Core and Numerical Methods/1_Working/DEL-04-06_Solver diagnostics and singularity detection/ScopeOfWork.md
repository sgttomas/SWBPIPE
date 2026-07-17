---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-06
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-053, SOW-035]
package_objective_refs: [OBJ-003, OBJ-008, OBJ-012]
---

# Scope of Work — DEL-04-06

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-06` in service of project scope [SOW-053, SOW-035] and package objectives [OBJ-003, OBJ-008, OBJ-012].

- **OUT-001** — A solver-diagnostics and singularity-detection contract covering deterministic machine-readable diagnostic envelopes, mechanics-only solver status, singularity and conditioning evidence, nonconvergence, invalid model and numeric inputs, provenance, remediation, unit metadata, and reviewable result-envelope interfaces is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-06 Solver diagnostics and singularity detection

> #### Datasheet: DEL-04-06 Solver diagnostics and singularity detection
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-06-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-04-06 |
> | Name | Solver diagnostics and singularity detection |
> | Package | PKG-04 Solver Core and Numerical Methods |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope items | SOW-053, SOW-035 |
> | Objectives | OBJ-003, OBJ-008, OBJ-012 |
> | Context envelope | M |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source |
> |---|---|---|
> | Primary subject | `core/solver/diagnostics` implements deterministic mechanics-solver diagnostics for singular, ill-conditioned, nonconverged, invalid-restraint, invalid-topology, invalid-numeric-input, and explicit solver-configuration TBD states. | `_CONTEXT.md`; `core/solver/diagnostics/README.md`; `core/solver/diagnostics/src/lib.rs` |
> | Solver-result boundary | Diagnostics describe mechanics-solver/model-validation status under `solver_result_only` authority and do not emit rule-check status, human-acceptance status, professional approval, or code-compliance claims (PRD §21.2). | `docs/CONTRACT.md` OPS-K-MECH-2; OPS-K-AUTH-1; `core/solver/diagnostics/src/lib.rs` `SolverStatus::analysis_boundary_mapping` |
> | Diagnostic envelope fields | `SolverDiagnostic` carries `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`. | `core/solver/diagnostics/src/lib.rs` `SolverDiagnostic` |
> | Provenance/remediation/unit metadata | Default diagnostics include solver-generated provenance with `review_status: TBD`, default remediation text by diagnostic code/severity, optional canonical model reference, and optional quantity-unit metadata. | `core/solver/diagnostics/src/lib.rs` `DiagnosticProvenance`, `default_remediation`, `with_canonical_ref`, `with_quantity_unit` |
> | Analysis-boundary statuses | `ReadyToSolve`, `ModelIncomplete`, and `SolveFailed` map to `MODEL_INCOMPLETE`; `MechanicsSolved` and `SolvedWithWarnings` map to `MECHANICS_SOLVED`; every mapping keeps `human_review_required: true`, `emits_rule_status: false`, and `emits_human_acceptance: false`. | `core/solver/diagnostics/src/lib.rs` `SolverStatus::analysis_boundary_mapping` |
> | Diagnostic classes | Blocking/failure diagnostics map to `SOLVE_BLOCKING`; ill-conditioning warnings map to `ASSUMPTION_WARNING`; warning-level solver-iteration nonconvergence maps to `NONLINEAR_WARNING`; sparse-solver and tolerance-policy deferrals map to `TBD`. | `core/solver/diagnostics/src/lib.rs` `diagnostic_class_for` |
> | Numerical thresholds | Conditioning and convergence helper functions accept caller-supplied thresholds/tolerances and reject invalid numeric inputs, but accepted release thresholds remain TBD. | `core/solver/diagnostics/src/lib.rs` `classify_condition_ratio`, `convergence_diagnostic`; `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-005 |
> | Sparse solver library/settings | Sparse solver selection remains TBD and is represented by an explicit warning diagnostic; no sparse numerical library/settings are selected in this deliverable. | `_CONTEXT.md` Still TBD; `core/solver/diagnostics/src/lib.rs` `sparse_solver_tbd_diagnostic` |
>

### CLM-005 — Diagnostic Category Register

> ##### Diagnostic Category Register
>
> | Category | Implemented handling | Preserved limit |
> |---|---|---|
> | Singular system | `FrameKernelError::SingularSystem` maps to `SingularSystem` / `Failure` / `MechanicsSolver` and produces a blocked `SolveFailed` report. | Sparse-solver selection and final solver integration remain outside this document update. |
> | Ill-conditioned system | `classify_condition_ratio` returns `IllConditionedSystem` warning diagnostics and `ConditioningFailure` failure diagnostics from finite, nonnegative condition-ratio evidence. | Accepted project thresholds remain TBD. |
> | Nonconverged analysis | `convergence_diagnostic` returns `NonConvergence` warning diagnostics before the iteration limit and failure diagnostics at or beyond the limit. | Nonlinear support warning finalization remains deferred pending the nonlinear support slice and accepted tolerance policy. |
> | Invalid restraint | Frame restrained/prescribed DOF errors and support findings/application errors map to blocking `InvalidRestraint` model-validation diagnostics, with `dof:*` affected references where implemented. | This does not decide rule compliance or human acceptance. |
> | Invalid model topology | Frame topology/orientation errors, support DOF alignment errors, and primitive-load topology findings map to blocking `InvalidModelTopology` diagnostics; missing load IDs use stable `load:<missing-id>` references. | Protected examples and proprietary benchmark cases remain excluded. |
> | Invalid numeric input | Frame numeric errors, linear-support errors, primitive-load errors, and primitive-load numeric findings map to blocking `InvalidNumericInput` diagnostics. | Unit conversion/default policy is not introduced here. |
> | Sparse/tolerance TBD | `SparseSolverTbd` and `TolerancePolicyTbd` diagnostics are warning-level solver-configuration diagnostics with class `TBD`. | No sparse solver, final tolerance threshold, performance claim, or release claim is selected. |
> | Solver status | `SolverDiagnosticReport` carries a solver-local status and diagnostics; analysis-boundary mapping preserves mechanics-only status semantics. | Final application-service/result-envelope integration remains deferred. |
>

### CLM-006 — Conditions

> ##### Conditions
>
> - Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.
> - Nonlinear solver diagnostics must report convergence, active-set state, and unresolved non-convergence when nonlinear support behavior is involved. Source: `docs/CONTRACT.md` OPS-K-SOLVER-2.
> - Missing solve-required values and invalid restraints must become explicit findings rather than silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
> - Units in solver-facing values and outputs remain dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
> - Current evidence is implementation evidence for the diagnostic crate, not release acceptance. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-007 — Construction

> ##### Construction
>
> | Artifact | Description | Current status |
> |---|---|---|
> | Solver diagnostics module | Rust crate for diagnostic classification, status reporting, mapping helpers, and TBD configuration diagnostics. | Implemented at `core/solver/diagnostics` |
> | Frame-kernel mappings | Maps singular systems, invalid restraints, topology/orientation errors, numeric input errors, and matrix/vector shape errors from `FrameKernelError`. | Implemented in `diagnostic_from_frame_error` and `report_frame_error` |
> | Support mappings | Maps linear-support errors/findings and support-application errors into blocking diagnostics. | Implemented in `diagnostic_from_linear_support_error`, `diagnostic_from_support_finding`, and `diagnostics_from_support_application_error` |
> | Primitive-load mappings | Maps primitive-load errors/findings, including axial-effect findings and missing load IDs, into blocking diagnostics with stable affected references. | Implemented in `diagnostic_from_primitive_load_error` and `diagnostic_from_primitive_load_finding` |
> | Conditioning diagnostics | Classifies finite, nonnegative condition ratios into no diagnostic, warning, or failure. | Implemented with caller-supplied thresholds; release thresholds TBD |
> | Nonconvergence diagnostics | Reports residual/tolerance nonconvergence as warning or failure based on iteration count. | Implemented with caller-supplied tolerance; accepted tolerance policy TBD |
> | Sparse/tolerance TBD diagnostics | Emits warning diagnostics for unresolved sparse-solver adapter and tolerance policy. | Implemented; selections remain TBD |
> | Status reporting | Machine-readable solver-local statuses map to PKG-02 analysis-boundary mechanics statuses. | Implemented; final result-envelope integration remains deferred |
>

### CLM-008 — Evidence Snapshot

> ##### Evidence Snapshot
>
> - `core/solver/diagnostics/README.md` states this crate is the bounded implementation slice for `DEL-04-06`.
> - `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md` records `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` passing with 19 tests.
> - The same June 5 evidence preserves deferrals for sparse solver selection, tolerance policy, nonlinear support behavior, release claims, professional approval, and code-compliance claims (PRD §21.2).
>

### CLM-009 — References

> ##### References
>
> - `_CONTEXT.md`
> - `_REFERENCES.md`
> - `MEMORY.md`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
> - `docs/_Registers/Deliverables.csv` row DEL-04-06
> - `docs/_Registers/ScopeLedger.csv` rows SOW-053 and SOW-035
> - `docs/CONTRACT.md`
> - `docs/SPEC.md`
> - `docs/TYPES.md`
> - `core/solver/diagnostics/README.md`
> - `core/solver/diagnostics/src/lib.rs`
> - `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-04-06 Solver diagnostics and singularity detection

> #### Specification: DEL-04-06 Solver diagnostics and singularity detection
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-06-DECL-001`.
>

### CLM-012 — Scope

> ##### Scope
>
> This deliverable records the implemented backend solver diagnostic layer at `core/solver/diagnostics`. The implemented slice covers deterministic diagnostic records, solver-local status reporting, analysis-boundary status mapping, frame-kernel error mapping, linear-support mapping, primitive-load mapping, condition-ratio classification, iterative nonconvergence diagnostics, and explicit sparse-solver/tolerance-policy TBD diagnostics.
>
> The implemented boundary remains mechanics-only. This deliverable does not select the sparse numerical library, set accepted release tolerance thresholds, finalize nonlinear support warning policy, integrate the final application result envelope, claim release readiness, or claim professional/code compliance.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source |
> |---|---|---|
> | REQ-04-06-001 | The diagnostic crate shall preserve deterministic diagnostic records for singular, ill-conditioned, nonconverged, invalid-restraint, invalid-topology, invalid-numeric-input, and unresolved solver-configuration states. | SOW-053; `core/solver/diagnostics/README.md` |
> | REQ-04-06-002 | `SolverDiagnostic` shall carry machine-readable `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`. | AB-00-06; `core/solver/diagnostics/src/lib.rs` `SolverDiagnostic` |
> | REQ-04-06-003 | Diagnostic provenance, remediation, canonical-reference, and unit-metadata surfaces shall remain available in the diagnostic envelope. | AB-00-06; OPS-K-UNIT-1; `core/solver/diagnostics/src/lib.rs` |
> | REQ-04-06-004 | Solver-local status mapping shall map mechanics states only: incomplete/failed states to `MODEL_INCOMPLETE`, solved/warning states to `MECHANICS_SOLVED`, with `solver_result_only` authority, human review required, no rule status, and no human-acceptance emission. | AB-00-03; AB-00-06; OPS-K-MECH-2; OPS-K-AUTH-1 |
> | REQ-04-06-005 | Frame-kernel errors shall map to blocking/failure diagnostics for singular systems, invalid restraints, invalid model topology/orientation, invalid numeric input, and invalid matrix/vector dimensions. | SOW-053; `core/solver/diagnostics/src/lib.rs` `diagnostic_from_frame_error` |
> | REQ-04-06-006 | Linear-support errors, support findings, and support-application errors shall map to blocking model-validation diagnostics, with stable DOF references where implemented. | OPS-K-DATA-2; SOW-053; `core/solver/diagnostics/src/lib.rs` |
> | REQ-04-06-007 | Primitive-load errors and findings shall map to blocking model-validation diagnostics, including axial-effect findings and missing-load-ID references. | OPS-K-DATA-2; SOW-053; `core/solver/diagnostics/src/lib.rs` |
> | REQ-04-06-008 | Conditioning diagnostics shall classify finite nonnegative condition-ratio evidence into no diagnostic, `IllConditionedSystem` warning, or `ConditioningFailure` failure using caller-supplied thresholds. | SOW-035; OI-005; `classify_condition_ratio` |
> | REQ-04-06-009 | Nonconvergence diagnostics shall classify residual/tolerance evidence as no diagnostic, warning-level `NonConvergence`, or failure-level `NonConvergence` using caller-supplied tolerance and iteration count. | OPS-K-SOLVER-2; `convergence_diagnostic` |
> | REQ-04-06-010 | Sparse-solver adapter selection and accepted tolerance policy shall remain explicit warning-level `TBD` diagnostics until accepted by a later governed decision. | `_CONTEXT.md` Still TBD; OI-005; `sparse_solver_tbd_diagnostic`; `tolerance_policy_tbd_diagnostic` |
> | REQ-04-06-011 | The diagnostic slice shall preserve mechanics-only boundaries and shall not claim certification, sealing, approval, professional reliance, release readiness, or code compliance (PRD §21.2). | OPS-K-MECH-2; OPS-K-AUTH-1; `core/solver/diagnostics/README.md` |
>

### CLM-014 — Standards

> ##### Standards
>
> No external protected standard text is introduced by this diagnostic slice or by this document update. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | REQ-04-06-001 | The June 5 Worker B evidence records `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` passing with 19 tests. |
> | REQ-04-06-002 | Source inspection verifies the `SolverDiagnostic` fields listed above. |
> | REQ-04-06-003 | Source inspection verifies `DiagnosticProvenance`, `default_remediation`, `with_canonical_ref`, and `with_quantity_unit`. |
> | REQ-04-06-004 | Unit coverage includes solver-status analysis-boundary mapping without human authority emission. |
> | REQ-04-06-005 | Unit coverage includes singular-system mapping, invalid-restraint mapping, prescribed DOF boundary errors, invalid orientation, and repeated element node topology; source inspection covers the remaining frame-kernel error mappings. |
> | REQ-04-06-006 | Unit coverage includes support finding and support-application error mapping; the June 5 parent fan-in also records linear-support and frame-kernel compatibility validation. |
> | REQ-04-06-007 | Unit coverage includes primitive-load finding mapping, missing load ID handling, and axial-effect missing/invalid property mappings. |
> | REQ-04-06-008 | Unit coverage includes condition-ratio no-diagnostic, warning, failure, and non-finite-input rejection cases. |
> | REQ-04-06-009 | Unit coverage includes failed nonconvergence and converged residual cases. |
> | REQ-04-06-010 | Unit coverage includes sparse-solver `TBD` warning behavior; tolerance-policy `TBD` is implemented as the companion unresolved-policy diagnostic and remains a release deferral. |
> | REQ-04-06-011 | Boundary review checks wording and run-record evidence for no release, professional approval, or code-compliance claim (PRD §21.2). |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Current local documentation artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`. This document update does not edit lifecycle state, dependency registers, review finding dispositions, DAG files, coordination prompts, code, or tests.

- **AC-001** — The contract preserves the accepted diagnostic codes and mappings, caller-supplied threshold and tolerance behavior, explicit sparse-solver and tolerance-policy TBD diagnostics, stable affected and canonical references, unit-aware provenance, rights-cleared verification evidence, and mechanics-only authority without inventing numerical policy or release readiness. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-04-06 Solver diagnostics and singularity detection

> #### Procedure: DEL-04-06 Solver diagnostics and singularity detection
>

### CLM-018 — Purpose

> ##### Purpose
>
> Define the review and maintenance procedure for the implemented solver diagnostic deliverable.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> - Sealed context for DEL-04-06 in `_CONTEXT.md`.
> - Decomposition and register rows for SOW-053, SOW-035, OBJ-003, OBJ-008, and OBJ-012.
> - Applicable architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
> - Contract invariants OPS-K-SOLVER-1, OPS-K-SOLVER-2, OPS-K-UNIT-1, OPS-K-MECH-2, OPS-K-DATA-2, OPS-K-REPORT-1, OPS-K-AUTH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
> - Current implementation evidence in `core/solver/diagnostics/README.md` and `core/solver/diagnostics/src/lib.rs`.
> - Recent validation evidence in `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`.
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity and write scope from `_CONTEXT.md`.
> 2. Read the deliverable-local truth set: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> 3. Read the implementation evidence in `core/solver/diagnostics/README.md` and `core/solver/diagnostics/src/lib.rs`.
> 4. Confirm the diagnostic envelope fields: `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`.
> 5. Confirm provenance/remediation/unit metadata surfaces through `DiagnosticProvenance`, default remediation text, `with_canonical_ref`, and `with_quantity_unit`.
> 6. Confirm analysis-boundary status mapping through `SolverStatus::analysis_boundary_mapping`; preserve `solver_result_only` authority, human review required, no rule-status emission, and no human-acceptance emission.
> 7. Confirm frame-kernel mappings for singular systems, invalid restraints, invalid topology/orientation, invalid numeric input, and matrix/vector shape failures.
> 8. Confirm linear-support and support-application mappings, including support IDs and `dof:*` affected references where implemented.
> 9. Confirm primitive-load mappings, including axial-effect finding codes and stable `load:<missing-id>` affected references for missing or blank load IDs.
> 10. Confirm conditioning and nonconvergence diagnostics with caller-supplied thresholds/tolerances and input validation.
> 11. Confirm sparse-solver and tolerance-policy TBD diagnostics remain warning-level unresolved-policy evidence.
> 12. Check recent run records for validation evidence; the June 5 Worker B record reports 19 passing diagnostics tests.
> 13. Preserve deferrals for sparse solver selection, accepted tolerance thresholds, nonlinear support warning finalization, final result-envelope integration, release claims, professional approval, and code-compliance claims (PRD §21.2).
> 14. For document-only alignment work, do not edit code, `_STATUS.md`, review finding dispositions, dependency registers, DAG files, or coordination prompts.
>

### CLM-021 — Verification And Release-Hardening Workflow

> ##### Verification And Release-Hardening Workflow
>
> 1. Use existing diagnostics unit tests as implementation evidence when the code has not changed.
> 2. Rerun `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` when diagnostics source or tests change.
> 3. Add original/public/permissive singular, restraint, topology, load, conditioning, and nonconvergence fixtures only within an authorized implementation scope.
> 4. Add nonlinear support active-set regression coverage only after nonlinear diagnostic policy is accepted.
> 5. Add release gate coverage for accepted sparse solver and tolerance policy only after those decisions are governed.
> 6. Verify that each fixture emits machine-readable diagnostics and does not claim professional approval or code compliance (PRD §21.2).
>

### CLM-022 — Verification

> ##### Verification
>
> - Confirm all four production documents exist.
> - Confirm the four production documents align with current implementation evidence.
> - Confirm current documents include diagnostic envelope fields, provenance/remediation/unit metadata, analysis-boundary status mapping, frame/support/primitive-load mappings, conditioning/nonconvergence diagnostics, sparse/tolerance TBD diagnostics, and 19-test evidence.
> - Confirm obsolete non-implementation wording has been removed from the four production documents except where a preserved deferral is explicitly named.
> - Confirm no files outside the assigned write scope were edited.
>

### CLM-023 — Records

> ##### Records
>
> - Four production documents in this folder.
> - `MEMORY.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_*.md`
> - `core/solver/diagnostics/README.md`
> - `core/solver/diagnostics/src/lib.rs`
>

### CLM-024 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The solver-diagnostics and singularity-detection suite is live with current tests and evidence. Setup-era exact counts and checklists are historical rather than current authority; unresolved solver-policy matters remain only where explicitly recorded.

- **VER-001** — Validate the contract and review source parity, diagnostic-envelope fields and mappings, solver-status authority boundaries, singularity, restraint, topology, numeric, conditioning and nonconvergence coverage, caller-supplied policy inputs, sparse and tolerance deferrals, units and provenance, deterministic evidence, protected-content controls, and every surviving governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-04-06 Solver diagnostics and singularity detection

> #### Guidance: DEL-04-06 Solver diagnostics and singularity detection
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-04-06-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable guides use and review of the implemented `core/solver/diagnostics` slice. The diagnostic layer makes mechanics-solver failure states explicit, reproducible, machine-readable, and reportable while preserving the project boundary between solver facts, rule checks, human review, and professional reliance.
>

### CLM-028 — Principles

> ##### Principles
>
> - Treat diagnostics as solver evidence, not engineering approval.
> - Treat `SparseSolverTbd` and `TolerancePolicyTbd` as explicit unresolved-policy diagnostics, not as acceptable release settings.
> - Preserve diagnostic provenance, remediation, affected references, canonical references, and unit metadata so downstream reports can explain which object, solver stage, and source produced a warning or blocking condition.
> - Keep invalid restraints, invalid topology, invalid numeric inputs, and missing solve-required values visible as findings; do not normalize them away.
> - Use `SolverStatus::analysis_boundary_mapping` for mechanics-only status handoff: solver diagnostics may map to `MODEL_INCOMPLETE` or `MECHANICS_SOLVED`, but they do not emit rule status or human acceptance.
> - Keep release claims outside this deliverable. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-029 — Considerations

> ##### Considerations
>
> - Singular models and invalid restraints overlap semantically but may require different diagnostic codes because one describes numerical rank/solvability and the other describes model-definition validity.
> - Frame-kernel mappings currently include singular systems, repeated/out-of-range restrained or prescribed DOFs, node/topology/orientation failures, non-finite/non-positive inputs, and matrix/vector shape failures.
> - Linear-support mappings currently cover support errors, support findings, and support-application errors. Support finding affected references use the public support ID; DOF application errors use `dof:*`.
> - Primitive-load mappings currently cover primitive-load errors/findings, axial-effect finding codes, and missing load IDs. Missing or blank load IDs use the stable affected reference `load:<missing-id>`.
> - Ill-conditioning is implemented as condition-ratio classification, but accepted threshold values are still a governed policy decision.
> - Nonconvergence diagnostics are implemented for iterative residual/tolerance evidence, but nonlinear support warning finalization and active-set policy remain outside the accepted slice.
> - Result-envelope integration should preserve AB-00-03 and AB-00-06 semantics so GUI, CLI, reports, and tests receive consistent status fields when the application service integrates the final envelope.
> - Thresholds remain `TBD` because the accepted authority requires reportable conditioning and convergence diagnostics but does not define release numerical policy. Later values need solver-library evidence, deterministic tests, and human-approved implementation context, and must not imply code compliance.
>

### CLM-030 — Vocabulary Notes

> ##### Vocabulary Notes
>
> | Term | Current implementation use |
> |---|---|
> | Singular | A mechanics solve state where the assembled/constrained system cannot produce a determinate solution under the chosen solver policy. |
> | Ill-conditioned | A numerical-quality warning state reported by condition-ratio classification when a caller-supplied warning threshold is met or exceeded. |
> | Conditioning failure | A blocking/failure state reported by condition-ratio classification when a caller-supplied failure threshold is met or exceeded. |
> | Nonconverged | An iterative solve state whose residual exceeds caller-supplied tolerance; it is warning-level before the iteration limit and failure-level at or beyond the limit. |
> | Invalid restraint | A model-definition finding for restraint/support data that prevents a valid solve or makes the restraint set contradictory. |
> | Invalid model topology | A model-definition finding for topology, orientation, target, connectivity, or required-reference problems that prevent valid mechanics use. |
> | Invalid numeric input | A model-validation or mechanics-solver finding for non-finite, non-positive, dimensionally invalid, or malformed numeric data. |
> | Blocking diagnostic | A diagnostic that prevents treating the mechanics solve as completed. |
> | TBD diagnostic | A warning-level diagnostic that records a deliberately unresolved solver policy or adapter selection. |
>

### CLM-031 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Current guidance | Open issue |
> |---|---|---|
> | Diagnostic granularity | Use the implemented `SolverDiagnosticCode` registry before adding new diagnostic kinds. | Additional nonlinear-support warning classes remain deferred. |
> | Conditioning warnings | Preserve reportability and provenance while passing threshold values explicitly from accepted solver policy. | Accepted release thresholds and sparse solver settings remain TBD. |
> | Fixture design | Use original/public/invented models and avoid protected examples. | Additional release gate fixture inventory remains TBD. |
> | Status wording | State mechanics-solver facts only and preserve `solver_result_only` authority. | Human review, rule checks, and professional reliance remain outside solver authority. |
> | Result integration | Keep diagnostic records and status mapping stable for the application result envelope. | Final result-envelope integration remains deferred. |
>

### CLM-032 — Examples

> ##### Examples
>
> No example models are added by this document alignment. Any later examples should be invented/public/permissive and should not reproduce protected standard examples or proprietary benchmark cases.
>

### CLM-033 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No new source conflicts were found during this document/evidence alignment. Numerical threshold policy, sparse solver selection, nonlinear support warning finalization, final result-envelope integration, release claims, and professional/code-compliance claims remain deferrals, not conflicts.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-053 SOW-035 OBJ-003 OBJ-008 OBJ-012 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
