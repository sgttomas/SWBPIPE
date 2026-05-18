# Nonlinear Support Regression Benchmarks

This crate contains invented nonlinear support regression fixtures for active-set,
gap, lift-off, friction, and non-convergence behavior.

The fixtures are software verification aids only. They do not encode protected
standards examples, real project data, proprietary benchmark outputs, acceptance
criteria for engineering use, or authority claims.

## Fixture Unit Basis

Fixture values carry explicit unit identifiers in code under
`PKG09-NONLINEAR-FIXTURE-UNITS-EXPLICIT-MM-N-NM`. This is a fixture-local basis
only: it records units for evidence review and does not define project
conversion constants or the canonical unit catalog, which remain `TBD`.

Hand-calculation and provenance notes are in `validation/hand_calcs/nonlinear/`.
