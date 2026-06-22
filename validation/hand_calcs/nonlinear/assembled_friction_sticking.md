# NL-ASSEMBLED-FRICTION-STICK-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
friction support remains sticking using explicit normal-reaction evidence.

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
| Applied axial force | 10.0 | N | force |
| Friction coefficient | 0.30 | ratio | dimensionless |
| Explicit normal reaction evidence | 100.0 | N | force |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | sticking | state label | dimensionless |

## Expected Values

The initially sticking friction support restrains the axial degree of freedom.
The assembled linear solve produces a tangential reaction of `-10.0 N`. With an
explicit normal reaction of `100.0 N` and friction coefficient `0.30`, the
invented friction limit is `30.0 N`, so the support remains sticking and the
active-set residual is zero.

This fixture uses explicit invented normal-reaction evidence only. It does not
derive a normal-force model and does not close the governed normal-force model
residual.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Expected iteration count | 1 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | sticking | state label | dimensionless |
| Expected tangential reaction | -10.0 | N | force |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
