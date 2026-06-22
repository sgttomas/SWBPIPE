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
seed; free-DOF force/moment residuals, free-DOF work residuals, displacement
deltas, sparse live-path behavior, product-preview thresholds, and external
validation thresholds remain outside this active-set-count record.

The current assembled validation seed also carries the governed
`DEC-046-CV-B-free-dof-force-moment-residual-validation-v1` threshold policy for
final-iteration free-DOF force and moment equilibrium residuals only. The
accepted limits are `0.0 N` and `0.0 N-m` on the current invented seed. This
current seed also carries
`DEC-046-CV-B-free-dof-work-residual-validation-v1` for final-iteration
free-DOF work residual products, with an accepted `0.0 N-m` limit. These
records now also bind the same final-iteration residual-work evidence envelope
to `DEC-046-CV-B-general-energy-residual-validation-v1`, with an accepted
`0.0 N-m` limit for the current assembled validation seed only. These records
do not define sparse live-path, product-preview, release, external validation,
total strain-energy, modal-energy, or CI thresholds.

The current assembled validation seed also carries
`DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1` for
fixture-evidence-envelope final-iteration displacement and reaction deltas from
the previous active-set solve, using
`DEC-046-CV-B-displacement-reaction-delta-observation-v1` as the source
observation ledger. The class-tiered accepted limits are `100.0 mm`, `0.0 rad`,
`10.0 N`, and `0.0 N-m` for one-way/lift-off/friction and `50.0 mm`,
`0.0 rad`, `5.0 N`, and `0.0 N-m` for gap. These records do not define general
energy, sparse live-path, product-preview, release, external validation, or CI
thresholds.

The companion note is
`validation/hand_calcs/nonlinear/convergence_observations.md`.
The machine-readable policy records are
`validation/benchmarks/nonlinear/convergence_policy.dec046.json` and
`validation/benchmarks/nonlinear/free_dof_force_moment_policy.dec046.json`,
and `validation/benchmarks/nonlinear/free_dof_work_policy.dec046.json`. The
machine-readable general-energy record is
`validation/benchmarks/nonlinear/general_energy_policy.dec046.json`. The
machine-readable displacement/reaction records are
`validation/benchmarks/nonlinear/displacement_reaction_delta_observation.dec046.json`
and
`validation/benchmarks/nonlinear/displacement_reaction_delta_policy.dec046.json`.

## Multi-Support Acceptance Inventory

The crate exposes `assembled_multisupport_acceptance_inventory()` plus
`assembled_multisupport_acceptance_convergence_observations()` and
`assembled_multisupport_acceptance_residual_observations()` for the
accepted multi-support validation fixture set:
`NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL` and
`NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL` and
`NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-OPPOSING-GAPS-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-ACCEPTED-ORIGINAL`, plus
`NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-OPPOSING-GAPS-ACCEPTED-ORIGINAL`. This is
narrow non-seed acceptance evidence spanning translational-only, mixed
translation/rotation, derived-normal friction, derived-normal rotational, and
sequential active-set cascade, negative gap-direction, and four-class
companions, plus gap-only opposing-direction and multi-node / two-span
companions, including a two-span opposing-gap companion, under
`DEC-046-CV-B-multisupport-active-set-count-validation-v1` and
`DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`.
It accepts active-set changed-support-count convergence, final-iteration
free-DOF force/moment residuals, and final-iteration free-DOF work residual
products for that fixture set only. It also accepts
`DEC-046-CV-B-multisupport-general-energy-residual-validation-v1` for the same
fixture-set final-iteration residual-work energy envelope. It also accepts
`DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1`
as a fixture-evidence-envelope displacement/reaction delta policy for the same
set, using
`DEC-046-CV-B-multisupport-displacement-reaction-delta-observation-v1` as the
source observation ledger. The accepted multi-support limits are `100.0 mm`,
`0.005 rad`, `10.0 N`, and `3.0 N-m`, with a `0.0 N-m` general-energy residual
limit. These records do not define sparse-default, product-preview, release,
external validation, total strain-energy, modal-energy, or CI thresholds.

The companion notes are
`validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof_acceptance.md`
and
`validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_friction_gap_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_three_dof_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_rotational_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_gap_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_rotational_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_cascade_gap_lift_off_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_negative_gap_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_four_class_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_opposing_gaps_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_two_span_acceptance.md`,
and
`validation/hand_calcs/nonlinear/assembled_multi_support_two_span_opposing_gaps_acceptance.md`.
The machine-readable policy records are
`validation/benchmarks/nonlinear/multisupport_convergence_policy.dec046.json`
and
`validation/benchmarks/nonlinear/multisupport_free_dof_force_moment_policy.dec046.json`,
and
`validation/benchmarks/nonlinear/multisupport_free_dof_work_policy.dec046.json`.
The machine-readable multi-support general-energy record is
`validation/benchmarks/nonlinear/multisupport_general_energy_policy.dec046.json`.
The machine-readable multi-support displacement/reaction records are
`validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_observation.dec046.json`
and
`validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_policy.dec046.json`.

## Multi-Support Depth Observation Inventory

The crate also exposes `assembled_multisupport_depth_inventory()` plus
`assembled_multisupport_depth_convergence_observations()` and
`assembled_multisupport_depth_residual_observations()` for the R4/D9
multi-DOF / multi-support depth residual. This inventory is intentionally
outside `assembled_fixture_inventory()` and uses
`TP-R4-D9-MULTISUPPORT-OBS-TBD` with `ConvergencePolicyStatus::Tbd`. It records
mechanical convergence and residual observations for an invented two-support,
two-DOF frame case without promoting non-seed force/displacement, free-DOF
work/energy, reaction-delta, sparse-default, external validation, release, or
CI thresholds.

The companion note is
`validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md`.
