# Guidance: DEL-04-06 Solver diagnostics and singularity detection

## Purpose

This deliverable guides use and review of the implemented `core/solver/diagnostics` slice. The diagnostic layer makes mechanics-solver failure states explicit, reproducible, machine-readable, and reportable while preserving the project boundary between solver facts, rule checks, human review, and professional reliance.

## Principles

- Treat diagnostics as solver evidence, not engineering approval.
- Treat `SparseSolverTbd` and `TolerancePolicyTbd` as explicit unresolved-policy diagnostics, not as acceptable release settings.
- Preserve diagnostic provenance, remediation, affected references, canonical references, and unit metadata so downstream reports can explain which object, solver stage, and source produced a warning or blocking condition.
- Keep invalid restraints, invalid topology, invalid numeric inputs, and missing solve-required values visible as findings; do not normalize them away.
- Use `SolverStatus::analysis_boundary_mapping` for mechanics-only status handoff: solver diagnostics may map to `MODEL_INCOMPLETE` or `MECHANICS_SOLVED`, but they do not emit rule status or human acceptance.
- Keep release claims, professional approval, sealing, certification, and code-compliance language outside this deliverable.

## Considerations

- Singular models and invalid restraints overlap semantically but may require different diagnostic codes because one describes numerical rank/solvability and the other describes model-definition validity.
- Frame-kernel mappings currently include singular systems, repeated/out-of-range restrained or prescribed DOFs, node/topology/orientation failures, non-finite/non-positive inputs, and matrix/vector shape failures.
- Linear-support mappings currently cover support errors, support findings, and support-application errors. Support finding affected references use the public support ID; DOF application errors use `dof:*`.
- Primitive-load mappings currently cover primitive-load errors/findings, axial-effect finding codes, and missing load IDs. Missing or blank load IDs use the stable affected reference `load:<missing-id>`.
- Ill-conditioning is implemented as condition-ratio classification, but accepted threshold values are still a governed policy decision.
- Nonconvergence diagnostics are implemented for iterative residual/tolerance evidence, but nonlinear support warning finalization and active-set policy remain outside the accepted slice.
- Result-envelope integration should preserve AB-00-03 and AB-00-06 semantics so GUI, CLI, reports, and tests receive consistent status fields when the application service integrates the final envelope.
- Thresholds remain `TBD` because the accepted authority requires reportable conditioning and convergence diagnostics but does not define release numerical policy. Later values need solver-library evidence, deterministic tests, and human-approved implementation context, and must not imply code compliance.

## Vocabulary Notes

| Term | Current implementation use |
|---|---|
| Singular | A mechanics solve state where the assembled/constrained system cannot produce a determinate solution under the chosen solver policy. |
| Ill-conditioned | A numerical-quality warning state reported by condition-ratio classification when a caller-supplied warning threshold is met or exceeded. |
| Conditioning failure | A blocking/failure state reported by condition-ratio classification when a caller-supplied failure threshold is met or exceeded. |
| Nonconverged | An iterative solve state whose residual exceeds caller-supplied tolerance; it is warning-level before the iteration limit and failure-level at or beyond the limit. |
| Invalid restraint | A model-definition finding for restraint/support data that prevents a valid solve or makes the restraint set contradictory. |
| Invalid model topology | A model-definition finding for topology, orientation, target, connectivity, or required-reference problems that prevent valid mechanics use. |
| Invalid numeric input | A model-validation or mechanics-solver finding for non-finite, non-positive, dimensionally invalid, or malformed numeric data. |
| Blocking diagnostic | A diagnostic that prevents treating the mechanics solve as completed. |
| TBD diagnostic | A warning-level diagnostic that records a deliberately unresolved solver policy or adapter selection. |

## Trade-offs

| Decision area | Current guidance | Open issue |
|---|---|---|
| Diagnostic granularity | Use the implemented `SolverDiagnosticCode` registry before adding new diagnostic kinds. | Additional nonlinear-support warning classes remain deferred. |
| Conditioning warnings | Preserve reportability and provenance while passing threshold values explicitly from accepted solver policy. | Accepted release thresholds and sparse solver settings remain TBD. |
| Fixture design | Use original/public/invented models and avoid protected examples. | Additional release gate fixture inventory remains TBD. |
| Status wording | State mechanics-solver facts only and preserve `solver_result_only` authority. | Human review, rule checks, and professional reliance remain outside solver authority. |
| Result integration | Keep diagnostic records and status mapping stable for the application result envelope. | Final result-envelope integration remains deferred. |

## Examples

No example models are added by this document alignment. Any later examples should be invented/public/permissive and should not reproduce protected standard examples or proprietary benchmark cases.

## Conflict Table (for human ruling)

No new source conflicts were found during this document/evidence alignment. Numerical threshold policy, sparse solver selection, nonlinear support warning finalization, final result-envelope integration, release claims, and professional/code-compliance claims remain deferrals, not conflicts.
