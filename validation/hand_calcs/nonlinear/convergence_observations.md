# Assembled Convergence Observations

## Purpose

Record the measured convergence observations for the current assembled
nonlinear validation fixtures. These observations are fixture evidence under
`DEC-046` and bind the current assembled validation seed to the governed
`DEC-046-CV-B-active-set-count-validation-v1` active-set changed-support-count
policy. This does not define force/displacement residuals, sparse live-path
behavior, product-preview thresholds, external validation thresholds, or a
release claim.

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

## Expected Values

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

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
