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

## Convergence Observation Inventory

The crate exposes `assembled_convergence_observations()` as a structured
inventory of observed iteration counts, final active-set residuals, convergence
flags, policy references, and diagnostics for the current assembled fixture
set. The inventory keeps the `DEC-046-CV-B-assembled-validation-seed-TBD`
policy reference and `TolerancePolicyTbd` diagnostic visible for every row; it
records fixture evidence only and does not define release thresholds.

The companion note is
`validation/hand_calcs/nonlinear/convergence_observations.md`.
