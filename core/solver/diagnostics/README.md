# Solver Diagnostics

This crate is the bounded implementation slice for `DEL-04-06`. It provides code-neutral solver diagnostic records for frame-kernel failures, sparse skyline solver failures, conditioning warnings, nonconvergence, invalid restraints, and the explicit sparse-solver adoption-status / tolerance-policy `TBD` states.

## Scope

- Deterministic diagnostic codes, severities, sources, and solver status values.
- Diagnostic envelope fields for PKG-02 result-boundary integration: diagnostic
  class, remediation, provenance, optional canonical model reference, and
  optional unit metadata.
- Solver-status mapping to the PKG-02 analysis-boundary mechanics status and
  authority fields.
- Mapping from `open_pipe_stress_frame_kernel::FrameKernelError` into diagnostic records, including invalid frame orientation as blocking model topology.
- Mapping from `open_pipe_stress_sparse_direct::SparseDirectError` into diagnostic records: singular pivots are failures located in both the profile-ordered numbering and the original reduced-DOF numbering (stable `reduced-dof:<n>` affected references); invalid numeric inputs are blocking.
- Deterministic diagnostics from sparse LDLᵀ factorization reports: a `NonPositivePivot` warning with count and first location when the reduced stiffness is not positive definite.
- Mapping from linear support findings/application errors and primitive load findings into diagnostic records.
- Stable affected references for primitive-load missing-ID findings.
- Basic finite/nonnegative condition-ratio classification for small verification paths; the same classifier serves the sparse pivot-ratio conditioning proxy (thresholds remain caller-supplied and governed by D-04).
- Nonconvergence diagnostics for iterative methods without implementing nonlinear support behavior.
- Explicit status diagnostics for the sparse-solver adoption state and tolerance policy: per `DEC-023` the sparse solver *selection* is resolved (in-repo skyline LDLᵀ, `core/solver/sparse_direct`), and `DEC-050` binds the R4 sparse evidence lane while dense remains default; the `SparseSolverTbd` code now records that profile-direct assembly and default sparse promotion remain `TBD`. The tolerance-policy `TBD` diagnostic continues to fire per `DEC-026` until governed values are filled by measurement.

## Boundary

This crate reports mechanics-solver diagnostics only. Solver-local statuses map to `MODEL_INCOMPLETE` or `MECHANICS_SOLVED` under `solver_result_only` authority and never emit rule-check or human-acceptance statuses. The crate does not solve models, set final tolerance policy, define conditioning thresholds, perform rule-pack checks, encode protected standards data, or make professional/code-compliance claims.

## Verification

The unit tests cover singular-system mapping (dense and sparse, including ordered/original pivot locations), invalid restraint mapping, prescribed DOF boundary errors, support finding/application error mapping, primitive load finding mapping, invalid model topology, non-finite condition estimates, ill-conditioning warnings, failed conditioning, nonconvergence, completed convergence, nonpositive-pivot report mapping, clean factorization reports, and the DEC-023-aware sparse-solver status diagnostic.
