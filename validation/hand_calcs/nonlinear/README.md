# Nonlinear Support Hand-Calculation Notes

These notes support `DEL-09-03 - Nonlinear support regression suite`.

All numeric values are invented public verification fixtures. They exercise
active-set state transitions, gap closure, lift-off, friction classification,
and non-convergence diagnostics in the committed nonlinear-support API. They do
not use protected standards examples, commercial benchmark outputs,
proprietary engineering values, private project records, code-specific
acceptance criteria, or professional reliance evidence.

## Fixture Unit Basis

The nonlinear fixtures use explicit fixture-local unit identifiers only:

| Quantity family | Unit | Canonical dimension |
|---|---|---|
| Translational support displacement and clearance | `mm` | length |
| Translational displacement-delta observations | `mm` | length |
| Translational support reaction | `N` | force |
| Translational reaction-delta / free-DOF force-residual observations | `N` | force |
| Rotational support reaction | `N-m` | moment |
| Rotational displacement-delta observations | `rad` | rotation |
| Rotational reaction-delta / free-DOF moment-residual observations | `N-m` | moment |
| Free-DOF work residual observations | `N-m` | moment |
| General energy residual envelope observations | `N-m` | moment |
| Friction coefficient | `ratio` | dimensionless |
| Active-set residual and iteration counts | `count` | dimensionless |

The project unit catalog, conversion constants, sparse live-path thresholds,
product-preview release thresholds, external validation thresholds, and CI gate
thresholds remain `TBD`. The current assembled validation seed uses
`DEC-046-CV-B-active-set-count-validation-v1` for active-set changed-support
count and
`DEC-046-CV-B-free-dof-force-moment-residual-validation-v1` for final-iteration
free-DOF force/moment equilibrium residuals, plus
`DEC-046-CV-B-free-dof-work-residual-validation-v1` for final-iteration
free-DOF work residual products,
`DEC-046-CV-B-general-energy-residual-validation-v1` for the corresponding
residual-work energy envelope, and
`DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1` for the
fixture-evidence displacement/reaction delta envelope only. A separate
multi-support companion fixture set uses
`DEC-046-CV-B-multisupport-active-set-count-validation-v1` and
`DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`, plus
`DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1`,
`DEC-046-CV-B-multisupport-general-energy-residual-validation-v1`, and
`DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1`,
for the public-original multi-DOF / multi-support validation fixture set only.

## Notes

| Fixture | Note |
|---|---|
| `NL-ACTIVE-ONE-WAY-ORIGINAL` | [active_set_one_way.md](active_set_one_way.md) |
| `NL-GAP-CLOSURE-ORIGINAL` | [gap_closure.md](gap_closure.md) |
| `NL-LIFT-OFF-ORIGINAL` | [lift_off.md](lift_off.md) |
| `NL-FRICTION-STICK-SLIDE-ORIGINAL` | [friction_transition.md](friction_transition.md) |
| `NL-NONCONVERGENCE-LIMIT-ORIGINAL` | [unresolved_nonconvergence.md](unresolved_nonconvergence.md) |
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | [assembled_one_way_deactivation.md](assembled_one_way_deactivation.md) |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | [assembled_gap_closure.md](assembled_gap_closure.md) |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | [assembled_lift_off.md](assembled_lift_off.md) |
| `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` | [assembled_friction_sticking.md](assembled_friction_sticking.md) |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | [assembled_friction_sliding.md](assembled_friction_sliding.md) |
| `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` | [assembled_friction_derived_normal.md](assembled_friction_derived_normal.md) |
| `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL` | [assembled_multi_support_multi_dof.md](assembled_multi_support_multi_dof.md) |
| `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL` | [assembled_multi_support_multi_dof_acceptance.md](assembled_multi_support_multi_dof_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL` | [assembled_multi_support_gap_lift_off_acceptance.md](assembled_multi_support_gap_lift_off_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL` | [assembled_multi_support_friction_gap_acceptance.md](assembled_multi_support_friction_gap_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL` | [assembled_multi_support_three_dof_acceptance.md](assembled_multi_support_three_dof_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL` | [assembled_multi_support_rotational_acceptance.md](assembled_multi_support_rotational_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL` | [assembled_multi_support_derived_normal_gap_acceptance.md](assembled_multi_support_derived_normal_gap_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL` | [assembled_multi_support_derived_normal_rotational_acceptance.md](assembled_multi_support_derived_normal_rotational_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL` | [assembled_multi_support_cascade_gap_lift_off_acceptance.md](assembled_multi_support_cascade_gap_lift_off_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL` | [assembled_multi_support_negative_gap_acceptance.md](assembled_multi_support_negative_gap_acceptance.md) |
| `NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL` | [assembled_multi_support_four_class_acceptance.md](assembled_multi_support_four_class_acceptance.md) |

## Observation Notes

| Topic | Note |
|---|---|
| Assembled convergence observations | [convergence_observations.md](convergence_observations.md) |
