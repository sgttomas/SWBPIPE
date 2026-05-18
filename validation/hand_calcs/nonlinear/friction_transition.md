# NL-FRICTION-STICK-SLIDE-ORIGINAL

## Purpose

Invented friction fixture for a pair of supports: one remains sticking and one
remains sliding under explicit normal and tangential trial reactions.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Stick support | Slide support | Unit | Canonical dimension |
|---|---:|---:|---|---|
| Iteration | 3 | 3 | count | dimensionless |
| Maximum iterations | 6 | 6 | count | dimensionless |
| Active-set residual tolerance | 0.0 | 0.0 | count | dimensionless |
| Friction coefficient | 0.30 | 0.30 | ratio | dimensionless |
| Normal reaction | 10.0 | 10.0 | N | force |
| Tangential reaction | 2.0 | 3.5 | N | force |
| Prior state | sticking | sliding | state label | dimensionless |

## Expected Values

The fixture classifies the first support as sticking and the second support as
sliding using the committed nonlinear-support API. No support changes state, so
the active-set residual is zero.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `friction_coefficient` | 0.30 | ratio | dimensionless |
| `stick_tangential_reaction` | 2.0 | N | force |
| `slide_tangential_reaction` | 3.5 | N | force |
| Expected active-set residual | 0.0 | count | dimensionless |

Tolerance policy: `TBD`.
