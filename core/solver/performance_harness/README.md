# Solver Performance Harness

This crate is the bounded implementation slice for `DEL-04-05`. It provides a deterministic regression harness around the frame-kernel solve boundary so solver performance and conditioning evidence can be recorded without changing solver logic. Since the human ruling `DEC-023` it measures the in-repo sparse skyline path (`core/solver/sparse_direct`) alongside the dense path on the same reduced systems.

## Scope

- Invented frame-chain and planar grid-frame benchmark fixtures with explicit public provenance posture.
- Fixture unit-system and unit identifiers for frame-kernel length, force,
  moment, stress, area, `second_moment_area`, displacement, and rotation
  quantities.
- Repeat-run regression records for the same fixture, solver version, and harness settings, including per-repeat residual and solution-delta observations.
- Matrix size, nonzero-count, residual, repeatability, diagonal conditioning observations, and condition-ratio estimates for the dense path.
- Sparse-path observations per record (`SparseSolveObservation`): deterministic RCM ordering identity, original/ordered profile entry counts and half-bandwidths, accepted-pivot extrema, the pivot-ratio conditioning proxy, nonpositive-pivot count, sparse-vs-dense parity delta, sparse residual, repeat determinism delta, and elapsed-time measurement.
- Sparse-suitability observation records over explicit invented planar-grid
  size bands (`SparseSuitabilityObservationRecord`) for the `DEC-050` evidence
  lane. These records preserve dense as default and mark threshold/default
  promotion status as `TBD`.
- Elapsed-time measurements for the first dense solve and the first sparse order+factor+solve. These are environment-dependent observations recorded for measurement only; no timing thresholds are asserted (thresholds remain governed by D-04).
- Deterministic suite runs over explicit invented cantilever-chain fixture
  sizes with per-fixture records and suite-level summary counts, including
  sparse-observation aggregates.
- Integration with solver diagnostics in a fixed, documented diagnostic order: the DEC-023 sparse-solver adoption-status diagnostic, the tolerance-policy `TBD` diagnostic, dense conditioning classification, dense solve failures, sparse factorization-report diagnostics (nonpositive pivots), sparse conditioning classification, and sparse solve failures.

## Boundary

This crate does not set release-quality timing thresholds, alter the frame kernel or the sparse solver, define code-specific checks, encode protected standards examples, or make professional/code-compliance claims.

Per `DEC-023` the dense path serves as the parity oracle for the in-repo sparse skyline path measured here. Per `DEC-050`, sparse is present as a live evidence lane in product/nonlinear paths while dense remains the default product solve path. Default sparse promotion, timing/memory thresholds, practical-size bands, conditioning/CI thresholds, and hardware-normalized methodology remain `TBD`.

Fixture unit metadata declares the calculation basis for reproducibility only. The harness does not define a project conversion catalog, convert units, supply protected benchmark values, or promote fixture results to release-quality performance evidence.

Sparse-vs-dense parity assertions in the tests cite the `DEC-026` analytic-class relative seed (1.0e-9) scaled by the dense reference solution magnitude; the harness itself records parity deltas without asserting thresholds.

## Verification

The unit tests cover deterministic repeat-run records, per-repeat observation rows, invented suite-runner records, suite summary counts (including sparse aggregates), provenance rejection, invalid settings, nonzero-count metrics, conditioning observations, conditioning diagnostics, dense and sparse solve-failure diagnostic recording (including located sparse singular pivots), sparse-vs-dense parity on chain and grid fixtures (small and larger generated banded models), sparse repeat determinism, grid fixture validation, and residual calculation.
