# Datasheet: DEL-04-06 Solver diagnostics and singularity detection

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-04-06-DECL-002`.

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-06 |
| Name | Solver diagnostics and singularity detection |
| Package | PKG-04 Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-053, SOW-035 |
| Objectives | OBJ-003, OBJ-008, OBJ-012 |
| Context envelope | M |

## Attributes

| Attribute | Current value | Source |
|---|---|---|
| Primary subject | `core/solver/diagnostics` implements deterministic mechanics-solver diagnostics for singular, ill-conditioned, nonconverged, invalid-restraint, invalid-topology, invalid-numeric-input, and explicit solver-configuration TBD states. | `_CONTEXT.md`; `core/solver/diagnostics/README.md`; `core/solver/diagnostics/src/lib.rs` |
| Solver-result boundary | Diagnostics describe mechanics-solver/model-validation status under `solver_result_only` authority and do not emit rule-check status, human-acceptance status, professional approval, or code-compliance claims. | `docs/CONTRACT.md` OPS-K-MECH-2; OPS-K-AUTH-1; `core/solver/diagnostics/src/lib.rs` `SolverStatus::analysis_boundary_mapping` |
| Diagnostic envelope fields | `SolverDiagnostic` carries `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`. | `core/solver/diagnostics/src/lib.rs` `SolverDiagnostic` |
| Provenance/remediation/unit metadata | Default diagnostics include solver-generated provenance with `review_status: TBD`, default remediation text by diagnostic code/severity, optional canonical model reference, and optional quantity-unit metadata. | `core/solver/diagnostics/src/lib.rs` `DiagnosticProvenance`, `default_remediation`, `with_canonical_ref`, `with_quantity_unit` |
| Analysis-boundary statuses | `ReadyToSolve`, `ModelIncomplete`, and `SolveFailed` map to `MODEL_INCOMPLETE`; `MechanicsSolved` and `SolvedWithWarnings` map to `MECHANICS_SOLVED`; every mapping keeps `human_review_required: true`, `emits_rule_status: false`, and `emits_human_acceptance: false`. | `core/solver/diagnostics/src/lib.rs` `SolverStatus::analysis_boundary_mapping` |
| Diagnostic classes | Blocking/failure diagnostics map to `SOLVE_BLOCKING`; ill-conditioning warnings map to `ASSUMPTION_WARNING`; warning-level solver-iteration nonconvergence maps to `NONLINEAR_WARNING`; sparse-solver and tolerance-policy deferrals map to `TBD`. | `core/solver/diagnostics/src/lib.rs` `diagnostic_class_for` |
| Numerical thresholds | Conditioning and convergence helper functions accept caller-supplied thresholds/tolerances and reject invalid numeric inputs, but accepted release thresholds remain TBD. | `core/solver/diagnostics/src/lib.rs` `classify_condition_ratio`, `convergence_diagnostic`; `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-005 |
| Sparse solver library/settings | Sparse solver selection remains TBD and is represented by an explicit warning diagnostic; no sparse numerical library/settings are selected in this deliverable. | `_CONTEXT.md` Still TBD; `core/solver/diagnostics/src/lib.rs` `sparse_solver_tbd_diagnostic` |

## Diagnostic Category Register

| Category | Implemented handling | Preserved limit |
|---|---|---|
| Singular system | `FrameKernelError::SingularSystem` maps to `SingularSystem` / `Failure` / `MechanicsSolver` and produces a blocked `SolveFailed` report. | Sparse-solver selection and final solver integration remain outside this document update. |
| Ill-conditioned system | `classify_condition_ratio` returns `IllConditionedSystem` warning diagnostics and `ConditioningFailure` failure diagnostics from finite, nonnegative condition-ratio evidence. | Accepted project thresholds remain TBD. |
| Nonconverged analysis | `convergence_diagnostic` returns `NonConvergence` warning diagnostics before the iteration limit and failure diagnostics at or beyond the limit. | Nonlinear support warning finalization remains deferred pending the nonlinear support slice and accepted tolerance policy. |
| Invalid restraint | Frame restrained/prescribed DOF errors and support findings/application errors map to blocking `InvalidRestraint` model-validation diagnostics, with `dof:*` affected references where implemented. | This does not decide rule compliance or human acceptance. |
| Invalid model topology | Frame topology/orientation errors, support DOF alignment errors, and primitive-load topology findings map to blocking `InvalidModelTopology` diagnostics; missing load IDs use stable `load:<missing-id>` references. | Protected examples and proprietary benchmark cases remain excluded. |
| Invalid numeric input | Frame numeric errors, linear-support errors, primitive-load errors, and primitive-load numeric findings map to blocking `InvalidNumericInput` diagnostics. | Unit conversion/default policy is not introduced here. |
| Sparse/tolerance TBD | `SparseSolverTbd` and `TolerancePolicyTbd` diagnostics are warning-level solver-configuration diagnostics with class `TBD`. | No sparse solver, final tolerance threshold, performance claim, or release claim is selected. |
| Solver status | `SolverDiagnosticReport` carries a solver-local status and diagnostics; analysis-boundary mapping preserves mechanics-only status semantics. | Final application-service/result-envelope integration remains deferred. |

## Conditions

- Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.
- Nonlinear solver diagnostics must report convergence, active-set state, and unresolved non-convergence when nonlinear support behavior is involved. Source: `docs/CONTRACT.md` OPS-K-SOLVER-2.
- Missing solve-required values and invalid restraints must become explicit findings rather than silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
- Units in solver-facing values and outputs remain dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
- Current evidence is implementation evidence for the diagnostic crate, not release acceptance, professional reliance, or code-compliance approval.

## Construction

| Artifact | Description | Current status |
|---|---|---|
| Solver diagnostics module | Rust crate for diagnostic classification, status reporting, mapping helpers, and TBD configuration diagnostics. | Implemented at `core/solver/diagnostics` |
| Frame-kernel mappings | Maps singular systems, invalid restraints, topology/orientation errors, numeric input errors, and matrix/vector shape errors from `FrameKernelError`. | Implemented in `diagnostic_from_frame_error` and `report_frame_error` |
| Support mappings | Maps linear-support errors/findings and support-application errors into blocking diagnostics. | Implemented in `diagnostic_from_linear_support_error`, `diagnostic_from_support_finding`, and `diagnostics_from_support_application_error` |
| Primitive-load mappings | Maps primitive-load errors/findings, including axial-effect findings and missing load IDs, into blocking diagnostics with stable affected references. | Implemented in `diagnostic_from_primitive_load_error` and `diagnostic_from_primitive_load_finding` |
| Conditioning diagnostics | Classifies finite, nonnegative condition ratios into no diagnostic, warning, or failure. | Implemented with caller-supplied thresholds; release thresholds TBD |
| Nonconvergence diagnostics | Reports residual/tolerance nonconvergence as warning or failure based on iteration count. | Implemented with caller-supplied tolerance; accepted tolerance policy TBD |
| Sparse/tolerance TBD diagnostics | Emits warning diagnostics for unresolved sparse-solver adapter and tolerance policy. | Implemented; selections remain TBD |
| Status reporting | Machine-readable solver-local statuses map to PKG-02 analysis-boundary mechanics statuses. | Implemented; final result-envelope integration remains deferred |

## Evidence Snapshot

- `core/solver/diagnostics/README.md` states this crate is the bounded implementation slice for `DEL-04-06`.
- `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md` records `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` passing with 19 tests.
- The same June 5 evidence preserves deferrals for sparse solver selection, tolerance policy, nonlinear support behavior, release claims, professional approval, and code-compliance claims.

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `MEMORY.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- `docs/_Registers/Deliverables.csv` row DEL-04-06
- `docs/_Registers/ScopeLedger.csv` rows SOW-053 and SOW-035
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `core/solver/diagnostics/README.md`
- `core/solver/diagnostics/src/lib.rs`
- `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`
