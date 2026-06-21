# Assembled Convergence Observations

## Purpose

Record the measured convergence observations for the current assembled
nonlinear validation fixtures. These observations are fixture evidence under
`DEC-046`; they do not promote governed release thresholds or remove the
`TolerancePolicyTbd` diagnostic.

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
| Convergence policy reference | DEC-046-CV-B-assembled-validation-seed-TBD | label | dimensionless |
| Policy status | TBD | label | dimensionless |

## Expected Values

| Fixture | Class | Observed iterations | Final residual | Converged | Policy status |
|---|---|---:|---:|---|---|
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | one_way | 2 | 0.0 | true | TBD |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | gap | 2 | 0.0 | true | TBD |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | lift_off | 2 | 0.0 | true | TBD |
| `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` | friction_sticking_explicit_normal | 1 | 0.0 | true | TBD |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | friction_sliding_explicit_normal | 2 | 0.0 | true | TBD |
| `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` | friction_sticking_derived_normal | 1 | 0.0 | true | TBD |

Each row keeps the `DEC-046-CV-B-assembled-validation-seed-TBD` policy
reference and the visible `TolerancePolicyTbd` diagnostic. The table records
observed fixture behavior only; it is not a release threshold, external
validation record, or reliance statement.

Tolerance policy: `TBD`.
