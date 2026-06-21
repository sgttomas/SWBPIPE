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
| Translational support reaction | `N` | force |
| Rotational support reaction | `N-m` | moment |
| Friction coefficient | `ratio` | dimensionless |
| Active-set residual and iteration counts | `count` | dimensionless |

The project unit catalog, conversion constants, release tolerances, and CI gate
thresholds remain `TBD`.

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
