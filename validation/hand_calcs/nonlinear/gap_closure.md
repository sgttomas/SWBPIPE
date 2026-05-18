# NL-GAP-CLOSURE-ORIGINAL

## Purpose

Invented positive-clearance gap fixture for a support that remains closed when
the trial displacement equals the explicit clearance.

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
| Gap clearance | 0.25 | mm | length |
| Trial displacement | 0.25 | mm | length |
| Trial reaction | 0.0 | N | force |
| Prior state | active | state label | dimensionless |

## Expected Values

At the explicit clearance the gap remains active. No support changes state, so
the active-set residual is zero and the fixture is converged.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `clearance` | 0.25 | mm | length |
| `trial_displacement` | 0.25 | mm | length |
| Expected state | active | state label | dimensionless |
| Expected active-set residual | 0.0 | count | dimensionless |

Tolerance policy: `TBD`.
