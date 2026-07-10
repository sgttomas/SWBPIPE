# NL-ACTIVE-ONE-WAY-ORIGINAL

## Purpose

Invented one-way support fixture for an active-set transition from inactive to
active under the DEC-067 state-switched complementarity test: the released
support re-engages when the trial displacement penetrates toward the side the
support can bear against.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Iteration | 1 | count | dimensionless |
| Maximum iterations | 6 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Trial displacement | -0.02 | mm | length |
| Trial reaction | 0.0 | N | force |
| Prior state | inactive | state label | dimensionless |

## Expected Values

The released positive-reaction support bears against negative displacement, so
the penetrating `-0.02 mm` trial displacement re-engages the support (its
trial reaction at the free DOF is identically zero, so a reaction-sign test
could never observe re-engagement). One support changes state, so the
active-set residual is the changed-support count.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `trial_displacement` | -0.02 | mm | length |
| `state_change_count` | 1.0 | count | dimensionless |
| Expected state | active | state label | dimensionless |
| Expected converged flag | false | boolean | dimensionless |

Tolerance policy: `TBD`.
