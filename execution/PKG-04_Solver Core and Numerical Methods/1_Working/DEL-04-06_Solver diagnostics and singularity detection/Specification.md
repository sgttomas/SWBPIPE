# Specification: DEL-04-06 Solver diagnostics and singularity detection

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-04-06-DECL-001`.

## Scope

This deliverable records the implemented backend solver diagnostic layer at `core/solver/diagnostics`. The implemented slice covers deterministic diagnostic records, solver-local status reporting, analysis-boundary status mapping, frame-kernel error mapping, linear-support mapping, primitive-load mapping, condition-ratio classification, iterative nonconvergence diagnostics, and explicit sparse-solver/tolerance-policy TBD diagnostics.

The implemented boundary remains mechanics-only. This deliverable does not select the sparse numerical library, set accepted release tolerance thresholds, finalize nonlinear support warning policy, integrate the final application result envelope, claim release readiness, or claim professional/code compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-04-06-001 | The diagnostic crate shall preserve deterministic diagnostic records for singular, ill-conditioned, nonconverged, invalid-restraint, invalid-topology, invalid-numeric-input, and unresolved solver-configuration states. | SOW-053; `core/solver/diagnostics/README.md` |
| REQ-04-06-002 | `SolverDiagnostic` shall carry machine-readable `code`, `class`, `severity`, `source`, `message`, optional `affected_ref`, optional `canonical_ref`, optional `remediation`, `provenance`, and `quantity_units`. | AB-00-06; `core/solver/diagnostics/src/lib.rs` `SolverDiagnostic` |
| REQ-04-06-003 | Diagnostic provenance, remediation, canonical-reference, and unit-metadata surfaces shall remain available in the diagnostic envelope. | AB-00-06; OPS-K-UNIT-1; `core/solver/diagnostics/src/lib.rs` |
| REQ-04-06-004 | Solver-local status mapping shall map mechanics states only: incomplete/failed states to `MODEL_INCOMPLETE`, solved/warning states to `MECHANICS_SOLVED`, with `solver_result_only` authority, human review required, no rule status, and no human-acceptance emission. | AB-00-03; AB-00-06; OPS-K-MECH-2; OPS-K-AUTH-1 |
| REQ-04-06-005 | Frame-kernel errors shall map to blocking/failure diagnostics for singular systems, invalid restraints, invalid model topology/orientation, invalid numeric input, and invalid matrix/vector dimensions. | SOW-053; `core/solver/diagnostics/src/lib.rs` `diagnostic_from_frame_error` |
| REQ-04-06-006 | Linear-support errors, support findings, and support-application errors shall map to blocking model-validation diagnostics, with stable DOF references where implemented. | OPS-K-DATA-2; SOW-053; `core/solver/diagnostics/src/lib.rs` |
| REQ-04-06-007 | Primitive-load errors and findings shall map to blocking model-validation diagnostics, including axial-effect findings and missing-load-ID references. | OPS-K-DATA-2; SOW-053; `core/solver/diagnostics/src/lib.rs` |
| REQ-04-06-008 | Conditioning diagnostics shall classify finite nonnegative condition-ratio evidence into no diagnostic, `IllConditionedSystem` warning, or `ConditioningFailure` failure using caller-supplied thresholds. | SOW-035; OI-005; `classify_condition_ratio` |
| REQ-04-06-009 | Nonconvergence diagnostics shall classify residual/tolerance evidence as no diagnostic, warning-level `NonConvergence`, or failure-level `NonConvergence` using caller-supplied tolerance and iteration count. | OPS-K-SOLVER-2; `convergence_diagnostic` |
| REQ-04-06-010 | Sparse-solver adapter selection and accepted tolerance policy shall remain explicit warning-level `TBD` diagnostics until accepted by a later governed decision. | `_CONTEXT.md` Still TBD; OI-005; `sparse_solver_tbd_diagnostic`; `tolerance_policy_tbd_diagnostic` |
| REQ-04-06-011 | The diagnostic slice shall preserve mechanics-only boundaries and shall not claim certification, sealing, approval, professional reliance, release readiness, or code compliance. | OPS-K-MECH-2; OPS-K-AUTH-1; `core/solver/diagnostics/README.md` |

## Standards

No external protected standard text is introduced by this diagnostic slice or by this document update. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-04-06-001 | The June 5 Worker B evidence records `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked` passing with 19 tests. |
| REQ-04-06-002 | Source inspection verifies the `SolverDiagnostic` fields listed above. |
| REQ-04-06-003 | Source inspection verifies `DiagnosticProvenance`, `default_remediation`, `with_canonical_ref`, and `with_quantity_unit`. |
| REQ-04-06-004 | Unit coverage includes solver-status analysis-boundary mapping without human authority emission. |
| REQ-04-06-005 | Unit coverage includes singular-system mapping, invalid-restraint mapping, prescribed DOF boundary errors, invalid orientation, and repeated element node topology; source inspection covers the remaining frame-kernel error mappings. |
| REQ-04-06-006 | Unit coverage includes support finding and support-application error mapping; the June 5 parent fan-in also records linear-support and frame-kernel compatibility validation. |
| REQ-04-06-007 | Unit coverage includes primitive-load finding mapping, missing load ID handling, and axial-effect missing/invalid property mappings. |
| REQ-04-06-008 | Unit coverage includes condition-ratio no-diagnostic, warning, failure, and non-finite-input rejection cases. |
| REQ-04-06-009 | Unit coverage includes failed nonconvergence and converged residual cases. |
| REQ-04-06-010 | Unit coverage includes sparse-solver `TBD` warning behavior; tolerance-policy `TBD` is implemented as the companion unresolved-policy diagnostic and remains a release deferral. |
| REQ-04-06-011 | Boundary review checks wording and run-record evidence for no release, professional approval, or code-compliance claim. |

## Documentation

Current local documentation artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`. This document update does not edit lifecycle state, dependency registers, review finding dispositions, DAG files, coordination prompts, code, or tests.
