# NL-GAP-CLOSURE-ORIGINAL

## Purpose

Invented positive-clearance gap fixture for a closed support that remains
closed under the DEC-067 state-switched complementarity test: the engaged gap
classifies on the bearing sign of its trial reaction.

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
| Trial reaction | -2.0 | N | force |
| Prior state | active | state label | dimensionless |

## Expected Values

The closed gap bears with a negative `-2.0 N` reaction against further
positive motion, so contact persists and the gap remains active. No support
changes state, so the active-set residual is zero and the fixture is
converged.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `clearance` | 0.25 | mm | length |
| `trial_displacement` | 0.25 | mm | length |
| `trial_reaction` | -2.0 | N | force |
| Expected state | active | state label | dimensionless |
| Expected active-set residual | 0.0 | count | dimensionless |

Tolerance policy: `TBD`.
