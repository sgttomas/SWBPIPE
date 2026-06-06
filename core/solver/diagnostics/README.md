# Solver Diagnostics

This crate is the bounded implementation slice for `DEL-04-06`. It provides code-neutral solver diagnostic records for frame-kernel failures, conditioning warnings, nonconvergence, invalid restraints, and explicit sparse-solver `TBD` state.

## Scope

- Deterministic diagnostic codes, severities, sources, and solver status values.
- Diagnostic envelope fields for PKG-02 result-boundary integration: diagnostic
  class, remediation, provenance, optional canonical model reference, and
  optional unit metadata.
- Solver-status mapping to the PKG-02 analysis-boundary mechanics status and
  authority fields.
- Mapping from `open_pipe_stress_frame_kernel::FrameKernelError` into diagnostic records, including invalid frame orientation as blocking model topology.
- Mapping from linear support findings/application errors and primitive load findings into diagnostic records.
- Stable affected references for primitive-load missing-ID findings.
- Basic finite/nonnegative condition-ratio classification for small verification paths.
- Nonconvergence diagnostics for iterative methods without implementing nonlinear support behavior.
- Explicit `TBD` diagnostics for sparse-solver adapter selection and tolerance policy.

## Boundary

This crate reports mechanics-solver diagnostics only. Solver-local statuses map to `MODEL_INCOMPLETE` or `MECHANICS_SOLVED` under `solver_result_only` authority and never emit rule-check or human-acceptance statuses. The crate does not solve models, select the sparse numerical library, set final tolerance policy, perform rule-pack checks, encode protected standards data, or make professional/code-compliance claims.

## Verification

The unit tests cover singular-system mapping, invalid restraint mapping, prescribed DOF boundary errors, support finding/application error mapping, primitive load finding mapping, invalid model topology, non-finite condition estimates, ill-conditioning warnings, failed conditioning, nonconvergence, completed convergence, and sparse-solver `TBD` diagnostics.
