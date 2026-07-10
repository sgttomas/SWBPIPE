# NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
closed gap support lifts off under the DEC-067 state-switched complementarity
test: the engaged (closed) gap classifies on the bearing sign of its trial
reaction, so a reaction that would have to pull the node onto the stop means
lift-off of the closed gap.

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
| Applied axial force | -2.0 | N | force |
| Gap clearance | 0.01 | mm | length |
| Gap closing direction | positive displacement | sense label | dimensionless |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | active | state label | dimensionless |

## Expected Values

The gap starts closed, so the first assembled linear solve prescribes the
support DOF at the clearance `+0.01 mm`. The reaction there is
`100.0 * 0.01 - (-2.0) = +3.0 N`. A gap that closes in the positive
displacement direction can only bear with a negative reaction (resisting
further positive motion), so the `+3.0 N` pulling reaction means the closed
gap lifts off and the support releases (a displacement-only test at the
prescribed clearance would have kept the gap closed forever and missed the
transition).

On the released iteration the free displacement is `-2.0 / 100.0 = -0.02 mm`,
which does not reach the `+0.01 mm` clearance, so the gap stays open and the
active-set residual reaches zero.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Closed-gap pulling reaction (iteration 1) | 3.0 | N | force |
| Released free displacement (iteration 2) | -0.02 | mm | length |
| Expected final reaction | 0.0 | N | force |
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | inactive | state label | dimensionless |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
Displacement/reaction delta policy: `DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1`.
