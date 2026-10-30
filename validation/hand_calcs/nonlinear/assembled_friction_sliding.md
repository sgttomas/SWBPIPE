# NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
friction support releases from sticking into sliding using explicit
normal-reaction evidence.

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
| Explicit normal reaction evidence | 20.0 | N | force |
| Friction limit | 6.0 | N | force |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | sticking | state label | dimensionless |

## Expected Values

The initially sticking friction support restrains the axial degree of freedom.
The first assembled linear solve produces a tangential reaction of `-10.0 N`.
With an explicit normal reaction of `20.0 N` and friction coefficient `0.30`,
the invented friction limit is `6.0 N`, so the support transitions to sliding.
On the sliding iteration the DEC-067 bounded tangential force
`-(0.30 * 20.0) = -6.0 N` opposes the motion, so the net drive is
`10.0 - 6.0 = 4.0 N` and the displacement is `4.0 / 100.0 = 0.04 mm`; the
deterministic anti-chatter rule keeps the support sliding, and the active-set
residual reaches zero.

This fixture uses explicit invented normal-reaction evidence only. It does not
derive a normal-force model, apply hidden friction-load defaults, or close the
governed normal-force model residual.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | sliding | state label | dimensionless |
| Expected first tangential reaction | -10.0 | N | force |
| Expected bounded sliding force | -6.0 | N | force |
| Expected final displacement | 0.04 | mm | length |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
Displacement/reaction delta policy: `DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1`.
