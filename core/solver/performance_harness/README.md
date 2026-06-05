# Solver Performance Harness

This crate is the bounded implementation slice for `DEL-04-05`. It provides a deterministic regression harness around the current frame-kernel solve boundary so sparse-solver performance and conditioning evidence can be recorded without changing solver logic.

## Scope

- Invented frame-chain benchmark fixtures with explicit public provenance posture.
- Fixture unit-system and unit identifiers for frame-kernel length, force,
  moment, stress, area, `second_moment_area`, displacement, and rotation
  quantities.
- Repeat-run regression records for the same fixture, solver version, and harness settings, including per-repeat residual and solution-delta observations.
- Matrix size, nonzero-count, residual, repeatability, diagonal conditioning observations, and condition-ratio estimates.
- Integration with solver diagnostics for conditioning classification and unresolved sparse-solver/tolerance-policy `TBD` states.

## Boundary

This crate does not select the final sparse numerical library, set release-quality timing thresholds, alter the frame kernel, define code-specific checks, encode protected standards examples, or make professional/code-compliance claims.

The current solve path uses the temporary dense verification interface from `core/solver/frame_kernel`. The harness preserves a stable observer boundary so a future sparse adapter can be measured behind the same record shape.

Fixture unit metadata declares the calculation basis for reproducibility only. The harness does not define a project conversion catalog, convert units, supply protected benchmark values, or promote fixture results to release-quality performance evidence.

## Verification

The unit tests cover deterministic repeat-run records, per-repeat observation rows, provenance rejection, invalid settings, nonzero-count metrics, conditioning observations, conditioning diagnostics, solve-failure diagnostic recording, and residual calculation.
