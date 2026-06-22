# Assembled Convergence Observations

## Purpose

Record the measured convergence observations for the current assembled
nonlinear validation fixtures. These observations are fixture evidence under
`DEC-046` and bind the current assembled validation seed to the governed
`DEC-046-CV-B-active-set-count-validation-v1` active-set changed-support-count
policy. The same current seed now also binds final-iteration free-DOF force and
moment equilibrium residuals to
`DEC-046-CV-B-free-dof-force-moment-residual-validation-v1`, and
final-iteration free-DOF work residual products to
`DEC-046-CV-B-free-dof-work-residual-validation-v1`. Displacement and reaction
deltas remain observation-only. This does not define general energy
convergence, sparse live-path behavior, product-preview thresholds, external
validation thresholds, release thresholds, or a release claim.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

All rows are measured by running the current public-original assembled fixture
inventory in `validation/benchmarks/nonlinear/src/lib.rs`.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Fixture count | 6 | count | dimensionless |
| Residual basis | active-set changed-support count | count | dimensionless |
| Convergence policy reference | DEC-046-CV-B-active-set-count-validation-v1 | label | dimensionless |
| Relative residual tolerance | 0.0 | count | dimensionless |
| Absolute residual floor | 0.0 | count | dimensionless |
| Max iteration cap | 4 | count | dimensionless |
| Policy status | accepted for current assembled validation seed | label | dimensionless |
| Free-DOF force residual threshold | 0.0 | N | force |
| Free-DOF moment residual threshold | 0.0 | N-m | moment |
| Free-DOF force/moment residual policy reference | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 | label | dimensionless |
| Free-DOF work residual threshold | 0.0 | N-m | moment |
| Free-DOF work residual policy reference | DEC-046-CV-B-free-dof-work-residual-validation-v1 | label | dimensionless |
| Free-DOF work residual unit | N-m | moment |

## Active-Set Expected Values

| Fixture | Class | Observed iterations | Final residual | Converged | Policy status |
|---|---|---:|---:|---|---|
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | one_way | 2 | 0.0 | true | accepted |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | gap | 2 | 0.0 | true | accepted |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | lift_off | 2 | 0.0 | true | accepted |
| `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` | friction | 1 | 0.0 | true | accepted |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | friction | 2 | 0.0 | true | accepted |
| `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` | friction | 1 | 0.0 | true | accepted |

Each row uses the same governed validation-seed policy reference. The table
records observed fixture behavior only; it is not an external validation
record or reliance statement.

## Force/Displacement Residual Observations

The following values are final-iteration observations from the same invented
assembled fixtures. `none` means the fixture converged in one iteration and
there is no previous iteration to compare against. Translational deltas are
reported in the fixture-local displacement unit (`mm`); reaction deltas are in
`N`; rotational and moment residual observations are zero in the current axial
fixture set.

| Fixture | Max translation delta from previous | Max force-reaction delta from previous | Free-DOF force residual | Free-DOF moment residual | Free-DOF work residual | Threshold policy |
|---|---:|---:|---:|---:|---:|---|
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | 100.0 mm | 10.0 N | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | 50.0 mm | 5.0 N | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | 100.0 mm | 10.0 N | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |
| `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` | none | none | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | 100.0 mm | 10.0 N | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |
| `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` | none | none | 0.0 N | 0.0 N-m | 0.0 N-m | DEC-046-CV-B-free-dof-force-moment-residual-validation-v1 |

The free-DOF force/moment residual threshold policy applies only to the current
public-original assembled validation seed. The free-DOF work residual threshold
policy likewise applies only to final-iteration residual work products in that
seed. Displacement deltas and reaction deltas remain observation-only; these
records do not loosen, tighten, or replace the accepted active-set-count
policy.

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF force/moment residual policy: `DEC-046-CV-B-free-dof-force-moment-residual-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
