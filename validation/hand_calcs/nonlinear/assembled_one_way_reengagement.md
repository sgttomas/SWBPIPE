# NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
lifted (released) one-way support re-engages under the DEC-067 state-switched
complementarity test: the released support classifies on trial displacement
penetration toward its bearing side, and the re-engaged support then
classifies on reaction sign.

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
| Applied axial force | -10.0 | N | force |
| One-way activation sense | positive reaction | sense label | dimensionless |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | inactive | state label | dimensionless |

## Expected Values

The support starts released (lifted), so the first assembled linear solve
leaves the axial degree of freedom free. With the applied axial force of
`-10.0 N` and the invented axial stiffness basis of `100.0 N/mm`, the free
trial displacement is `-10.0 / 100.0 = -0.1 mm`. A positive-reaction one-way
support bears against negative displacement, so this penetrating displacement
re-engages the support (a reaction-sign test on a free DOF could never observe
this transition; the trial reaction there is identically zero).

On the re-engaged iteration the support DOF is prescribed to zero and the
reaction is `0.0 - (-10.0) = +10.0 N`, which matches the positive activation
sense, so the support stays active and the active-set residual reaches zero.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Penetrating free displacement (iteration 1) | -0.1 | mm | length |
| Re-engaged support reaction (iteration 2) | 10.0 | N | force |
| Expected final displacement | 0.0 | mm | length |
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | active | state label | dimensionless |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
Displacement/reaction delta policy: `DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1`.
