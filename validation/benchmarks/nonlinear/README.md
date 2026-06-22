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
set. The inventory now binds the current assembled validation seed to the
governed `DEC-046-CV-B-active-set-count-validation-v1` policy: active-set
changed-support-count residual, relative tolerance `0.0`, absolute floor `0.0`,
and max iteration cap `4` for the one-way, gap, lift-off, and friction classes.
This policy applies only to the current public-original assembled validation
seed; free-DOF force/moment residuals, displacement deltas, energy, sparse
live-path behavior, product-preview thresholds, and external validation
thresholds remain outside this record.

The current assembled validation seed also carries the governed
`DEC-046-CV-B-free-dof-force-moment-residual-validation-v1` threshold policy for
final-iteration free-DOF force and moment equilibrium residuals only. The
accepted limits are `0.0 N` and `0.0 N-m` on the current invented seed. This
does not define displacement-delta, reaction-delta, energy, sparse live-path,
product-preview, release, or external validation thresholds.

The companion note is
`validation/hand_calcs/nonlinear/convergence_observations.md`.
The machine-readable policy records are
`validation/benchmarks/nonlinear/convergence_policy.dec046.json` and
`validation/benchmarks/nonlinear/free_dof_force_moment_policy.dec046.json`.

## Multi-Support Depth Observation Inventory

The crate also exposes `assembled_multisupport_depth_inventory()` plus
`assembled_multisupport_depth_convergence_observations()` and
`assembled_multisupport_depth_residual_observations()` for the R4/D9
multi-DOF / multi-support depth residual. This inventory is intentionally
outside `assembled_fixture_inventory()` and uses
`TP-R4-D9-MULTISUPPORT-OBS-TBD` with `ConvergencePolicyStatus::Tbd`. It records
mechanical convergence and residual observations for an invented two-support,
two-DOF frame case without promoting non-seed force/displacement, energy,
reaction-delta, sparse-default, external validation, release, or CI thresholds.

The companion note is
`validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md`.
