# NL-LIFT-OFF-ORIGINAL

## Purpose

Invented lift-off fixture for a support that loses contact when the trial
reaction reaches zero.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Iteration | 2 | count | dimensionless |
| Maximum iterations | 6 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Trial displacement | 0.04 | mm | length |
| Trial reaction | 0.0 | N | force |
| Prior state | active | state label | dimensionless |

## Expected Values

The zero trial reaction is treated as loss of contact for this invented case.
One support changes state, so the active-set residual is the changed-support
count.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `trial_reaction` | 0.0 | N | force |
| `trial_displacement` | 0.04 | mm | length |
| Expected state | inactive | state label | dimensionless |
| Expected converged flag | false | boolean | dimensionless |

Tolerance policy: `TBD`.
