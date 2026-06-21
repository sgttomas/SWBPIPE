# NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
friction support derives its normal-reaction evidence from a named restrained
support-normal degree of freedom.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node count | 2 | count | dimensionless |
| Member length | 1.0 | mm | length |
| Axial stiffness basis | 100.0 | N/mm | linear_stiffness |
| Applied axial friction-direction force | 10.0 | N | force |
| Applied normal-direction force | -100.0 | N | force |
| Friction coefficient | 0.30 | ratio | dimensionless |
| Normal source DOF | node 1 UY | support DOF | dimensionless |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | sticking | state label | dimensionless |

## Expected Values

The initially sticking friction support restrains the axial degree of freedom.
The separate UY support-normal degree of freedom is also restrained. The
assembled linear solve therefore produces a UY support reaction of `100.0 N`
from the invented `-100.0 N` normal-direction load. The fixture uses the
absolute value of that named support reaction as the friction normal-reaction
magnitude.

With friction coefficient `0.30`, the invented friction limit is `30.0 N`.
The axial tangential reaction is `-10.0 N`, which stays within the friction
limit, so the support remains sticking and the active-set residual is zero on
the first iteration.

This fixture derives normal-reaction evidence only from an explicit support
DOF named by the fixture. It does not supply a catalog/default normal force,
copy a protected value, or make any professional acceptance claim.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Expected iteration count | 1 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | sticking | state label | dimensionless |
| Expected derived normal reaction | 100.0 | N | force |
| Expected tangential reaction | -10.0 | N | force |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `TBD`.
